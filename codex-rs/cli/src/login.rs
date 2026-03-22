use anyhow::Context;
use base64::Engine;
use chrono::Duration as ChronoDuration;
use chrono::Utc;
// CLI login commands and their direct-user observability surfaces.
//
// The TUI path already installs a broader tracing stack with feedback, OpenTelemetry, and other
// interactive-session layers. Direct `codex login` intentionally does less: it preserves the
// existing stderr/browser UX and adds only a small file-backed tracing layer for login-specific
// targets. Keeping that setup local avoids pulling the TUI's session-oriented logging machinery
// into a one-shot CLI command while still producing a durable `codex-login.log` artifact that
// support can request from users.
use codex_core::CodexAuth;
use codex_core::auth::AuthCredentialsStoreMode;
use codex_core::auth::AuthMode;
use codex_core::auth::CLIENT_ID;
use codex_core::auth::login_with_api_key;
use codex_core::auth::logout;
use codex_core::config::Config;
use codex_core::util::command_with_args;
use codex_login::ServerOptions;
use codex_login::run_device_code_login;
use codex_login::run_login_server;
use codex_protocol::config_types::ForcedLoginMethod;
use codex_utils_cli::CliConfigOverrides;
use rand::RngCore;
use serde::Deserialize;
use serde::Serialize;
use sha2::Digest;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::IsTerminal;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use tiny_http::Header;
use tiny_http::Response;
use tiny_http::Server;
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

const CHATGPT_LOGIN_DISABLED_MESSAGE: &str =
    "ChatGPT login is disabled. Use API key login instead.";
const API_KEY_LOGIN_DISABLED_MESSAGE: &str =
    "API key login is disabled. Use ChatGPT login instead.";
const LOGIN_SUCCESS_MESSAGE: &str = "Successfully logged in";

const INDUBITABLY_DEFAULT_BACKEND_BASE_URL: &str = "https://api.indubitably.ai";
const INDUBITABLY_DEFAULT_REDIRECT_PORT: u16 = 1455;
const INDUBITABLY_CALLBACK_PATH: &str = "/auth/callback";
const INDUBITABLY_AUTH_FILE_ENV_VAR: &str = "INDUBITABLY_AUTH_FILE";
const INDUBITABLY_AUTH_FILE_NAME: &str = "indubitably-auth.json";
/// Installs a small file-backed tracing layer for direct `codex login` flows.
///
/// This deliberately duplicates a narrow slice of the TUI logging setup instead of reusing it
/// wholesale. The TUI stack includes session-oriented layers that are valuable for interactive
/// runs but unnecessary for a one-shot login command. Keeping the direct CLI path local lets this
/// command produce a durable `codex-login.log` artifact without coupling it to the TUI's broader
/// telemetry and feedback initialization.
fn init_login_file_logging(config: &Config) -> Option<WorkerGuard> {
    let log_dir = match codex_core::config::log_dir(config) {
        Ok(log_dir) => log_dir,
        Err(err) => {
            eprintln!("Warning: failed to resolve login log directory: {err}");
            return None;
        }
    };

    if let Err(err) = std::fs::create_dir_all(&log_dir) {
        eprintln!(
            "Warning: failed to create login log directory {}: {err}",
            log_dir.display()
        );
        return None;
    }

    let mut log_file_opts = OpenOptions::new();
    log_file_opts.create(true).append(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        log_file_opts.mode(0o600);
    }

    let log_path = log_dir.join("codex-login.log");
    let log_file = match log_file_opts.open(&log_path) {
        Ok(log_file) => log_file,
        Err(err) => {
            eprintln!(
                "Warning: failed to open login log file {}: {err}",
                log_path.display()
            );
            return None;
        }
    };

    let (non_blocking, guard) = non_blocking(log_file);
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("codex_cli=info,codex_core=info,codex_login=info"));
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_target(true)
        .with_ansi(false)
        .with_filter(env_filter);

    // Direct `codex login` otherwise relies on ephemeral stderr and browser output.
    // Persist the same login targets to a file so support can inspect auth failures
    // without reproducing them through TUI or app-server.
    if let Err(err) = tracing_subscriber::registry().with(file_layer).try_init() {
        eprintln!(
            "Warning: failed to initialize login log file {}: {err}",
            log_path.display()
        );
        return None;
    }

    Some(guard)
}

fn print_login_server_start(actual_port: u16, auth_url: &str) {
    let device_auth_command = command_with_args("login --device-auth");
    eprintln!(
        "Starting local login server on http://localhost:{actual_port}.\nIf your browser did not open, navigate to this URL to authenticate:\n\n{auth_url}\n\nOn a remote or headless machine? Use `{device_auth_command}` instead."
    );
}

pub async fn login_with_chatgpt(
    codex_home: PathBuf,
    forced_chatgpt_workspace_id: Option<String>,
    cli_auth_credentials_store_mode: AuthCredentialsStoreMode,
) -> std::io::Result<()> {
    let opts = ServerOptions::new(
        codex_home,
        CLIENT_ID.to_string(),
        forced_chatgpt_workspace_id,
        cli_auth_credentials_store_mode,
    );
    let server = run_login_server(opts)?;

    print_login_server_start(server.actual_port, &server.auth_url);

    server.block_until_done().await
}

pub async fn run_login_with_chatgpt(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    let _login_log_guard = init_login_file_logging(&config);
    tracing::info!("starting browser login flow");

    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Api)) {
        eprintln!("{CHATGPT_LOGIN_DISABLED_MESSAGE}");
        std::process::exit(1);
    }

    let forced_chatgpt_workspace_id = config.forced_chatgpt_workspace_id.clone();

    match login_with_chatgpt(
        config.codex_home,
        forced_chatgpt_workspace_id,
        config.cli_auth_credentials_store_mode,
    )
    .await
    {
        Ok(_) => {
            eprintln!("{LOGIN_SUCCESS_MESSAGE}");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_login_with_indubitably(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    match perform_indubitably_login(&config).await {
        Ok(()) => {
            eprintln!("{LOGIN_SUCCESS_MESSAGE}");
            std::process::exit(0);
        }
        Err(err) => {
            eprintln!("Error logging in: {err}");
            std::process::exit(1);
        }
    }
}

pub async fn run_login_with_api_key(
    cli_config_overrides: CliConfigOverrides,
    api_key: String,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    let _login_log_guard = init_login_file_logging(&config);
    tracing::info!("starting api key login flow");

    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Chatgpt)) {
        eprintln!("{API_KEY_LOGIN_DISABLED_MESSAGE}");
        std::process::exit(1);
    }

    match login_with_api_key(
        &config.codex_home,
        &api_key,
        config.cli_auth_credentials_store_mode,
    ) {
        Ok(_) => {
            eprintln!("{LOGIN_SUCCESS_MESSAGE}");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in: {e}");
            std::process::exit(1);
        }
    }
}

pub fn read_api_key_from_stdin() -> String {
    let mut stdin = std::io::stdin();

    if stdin.is_terminal() {
        let command = command_with_args("login --with-api-key");
        eprintln!(
            "--with-api-key expects the API key on stdin. Try piping it, e.g. `printenv OPENAI_API_KEY | {command}`."
        );
        std::process::exit(1);
    }

    eprintln!("Reading API key from stdin...");

    let mut buffer = String::new();
    if let Err(err) = stdin.read_to_string(&mut buffer) {
        eprintln!("Failed to read API key from stdin: {err}");
        std::process::exit(1);
    }

    let api_key = buffer.trim().to_string();
    if api_key.is_empty() {
        eprintln!("No API key provided via stdin.");
        std::process::exit(1);
    }

    api_key
}

/// Login using the OAuth device code flow.
pub async fn run_login_with_device_code(
    cli_config_overrides: CliConfigOverrides,
    issuer_base_url: Option<String>,
    client_id: Option<String>,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    let _login_log_guard = init_login_file_logging(&config);
    tracing::info!("starting device code login flow");
    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Api)) {
        eprintln!("{CHATGPT_LOGIN_DISABLED_MESSAGE}");
        std::process::exit(1);
    }
    let forced_chatgpt_workspace_id = config.forced_chatgpt_workspace_id.clone();
    let mut opts = ServerOptions::new(
        config.codex_home,
        client_id.unwrap_or(CLIENT_ID.to_string()),
        forced_chatgpt_workspace_id,
        config.cli_auth_credentials_store_mode,
    );
    if let Some(iss) = issuer_base_url {
        opts.issuer = iss;
    }
    match run_device_code_login(opts).await {
        Ok(()) => {
            eprintln!("{LOGIN_SUCCESS_MESSAGE}");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging in with device code: {e}");
            std::process::exit(1);
        }
    }
}

/// Prefers device-code login (with `open_browser = false`) when headless environment is detected, but keeps
/// CLI login working in environments where device-code may be disabled/feature-gated.
/// If `run_device_code_login` returns `ErrorKind::NotFound` ("device-code unsupported"), this
/// falls back to starting the local browser login server.
pub async fn run_login_with_device_code_fallback_to_browser(
    cli_config_overrides: CliConfigOverrides,
    issuer_base_url: Option<String>,
    client_id: Option<String>,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;
    let _login_log_guard = init_login_file_logging(&config);
    tracing::info!("starting login flow with device code fallback");
    if matches!(config.forced_login_method, Some(ForcedLoginMethod::Api)) {
        eprintln!("{CHATGPT_LOGIN_DISABLED_MESSAGE}");
        std::process::exit(1);
    }

    let forced_chatgpt_workspace_id = config.forced_chatgpt_workspace_id.clone();
    let mut opts = ServerOptions::new(
        config.codex_home,
        client_id.unwrap_or(CLIENT_ID.to_string()),
        forced_chatgpt_workspace_id,
        config.cli_auth_credentials_store_mode,
    );
    if let Some(iss) = issuer_base_url {
        opts.issuer = iss;
    }
    opts.open_browser = false;

    match run_device_code_login(opts.clone()).await {
        Ok(()) => {
            eprintln!("{LOGIN_SUCCESS_MESSAGE}");
            std::process::exit(0);
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("Device code login is not enabled; falling back to browser login.");
                match run_login_server(opts) {
                    Ok(server) => {
                        print_login_server_start(server.actual_port, &server.auth_url);
                        match server.block_until_done().await {
                            Ok(()) => {
                                eprintln!("{LOGIN_SUCCESS_MESSAGE}");
                                std::process::exit(0);
                            }
                            Err(e) => {
                                eprintln!("Error logging in: {e}");
                                std::process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error logging in: {e}");
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error logging in with device code: {e}");
                std::process::exit(1);
            }
        }
    }
}

pub async fn run_login_status(
    cli_config_overrides: CliConfigOverrides,
    use_indubitably: bool,
) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    if use_indubitably {
        let backend_base_url = indubitably_backend_base_url(&config);
        match load_indubitably_tokens(&backend_base_url, &config.codex_home) {
            Ok(Some(tokens)) => {
                let tenant_id = tokens.tenant_id.as_deref().unwrap_or("unknown");
                let user_id = tokens.user_id.as_deref().unwrap_or("unknown");
                eprintln!("Logged in to Indubitably (tenant: {tenant_id}, user: {user_id})");
                std::process::exit(0);
            }
            Ok(None) => {
                eprintln!("Not logged in");
                std::process::exit(1);
            }
            Err(err) => {
                eprintln!("Error checking Indubitably login status: {err}");
                std::process::exit(1);
            }
        }
    }

    match CodexAuth::from_auth_storage(&config.codex_home, config.cli_auth_credentials_store_mode) {
        Ok(Some(auth)) => match auth.auth_mode() {
            AuthMode::ApiKey => match auth.get_token() {
                Ok(api_key) => {
                    eprintln!("Logged in using an API key - {}", safe_format_key(&api_key));
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("Unexpected error retrieving API key: {e}");
                    std::process::exit(1);
                }
            },
            AuthMode::Chatgpt | AuthMode::ChatgptAuthTokens => {
                eprintln!("Logged in using ChatGPT");
                std::process::exit(0);
            }
        },
        Ok(None) => {
            eprintln!("Not logged in");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("Error checking login status: {e}");
            std::process::exit(1);
        }
    }
}

pub async fn run_logout(cli_config_overrides: CliConfigOverrides) -> ! {
    let config = load_config_or_exit(cli_config_overrides).await;

    match logout(&config.codex_home, config.cli_auth_credentials_store_mode) {
        Ok(true) => {
            eprintln!("Successfully logged out");
            std::process::exit(0);
        }
        Ok(false) => {
            eprintln!("Not logged in");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("Error logging out: {e}");
            std::process::exit(1);
        }
    }
}

async fn load_config_or_exit(cli_config_overrides: CliConfigOverrides) -> Config {
    let cli_overrides = match cli_config_overrides.parse_overrides() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing -c overrides: {e}");
            std::process::exit(1);
        }
    };

    match Config::load_with_cli_overrides(cli_overrides).await {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error loading configuration: {e}");
            std::process::exit(1);
        }
    }
}

async fn perform_indubitably_login(config: &Config) -> anyhow::Result<()> {
    let backend_base_url = indubitably_backend_base_url(config);
    let auth_config = fetch_auth_config(&backend_base_url).await?;

    let redirect_uri = select_redirect_uri(&auth_config.redirect_uris, None)?;
    let server = bind_login_server(&redirect_uri)?;

    let pkce = generate_pkce();
    let state = generate_state();

    let web_login_url = auth_config
        .web_login_url
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let use_web_handoff = web_login_url.is_some();
    let auth_url = if let Some(web_login_url) = web_login_url {
        build_web_login_url(
            web_login_url,
            &redirect_uri,
            &pkce.code_challenge,
            &state,
            auth_config.code_challenge_method.as_deref(),
        )?
    } else {
        build_auth_url(&auth_config, &redirect_uri, &pkce.code_challenge, &state)?
    };

    let _ = webbrowser::open(&auth_url);
    eprintln!(
        "Starting local login server on {redirect_uri}.\nIf your browser did not open, navigate to this URL to authenticate:\n\n{auth_url}"
    );

    let code = wait_for_auth_code(server, &state).await?;
    let token_response = exchange_token(
        &backend_base_url,
        &TokenExchangeRequest {
            grant_type: if use_web_handoff {
                "cli_login_code".to_string()
            } else {
                "authorization_code".to_string()
            },
            code: Some(code),
            refresh_token: None,
            redirect_uri: (!use_web_handoff).then_some(redirect_uri),
            code_verifier: Some(pkce.code_verifier),
        },
    )
    .await?;

    save_indubitably_tokens(&backend_base_url, &config.codex_home, token_response)
}

fn indubitably_backend_base_url(config: &Config) -> String {
    let configured = config
        .model_providers
        .get("bedrock")
        .and_then(|provider| provider.base_url.clone());
    let configured = configured
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    normalize_base_url(configured.unwrap_or(INDUBITABLY_DEFAULT_BACKEND_BASE_URL))
}

fn normalize_base_url(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.is_empty() {
        return INDUBITABLY_DEFAULT_BACKEND_BASE_URL.to_string();
    }

    if let Some(without_v1) = trimmed.strip_suffix("/v1")
        && !without_v1.trim().is_empty()
    {
        return without_v1.to_string();
    }

    trimmed.to_string()
}

async fn fetch_auth_config(base_url: &str) -> anyhow::Result<AuthConfigResponse> {
    let url = format!("{}/cli/auth/config", normalize_base_url(base_url));
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .context("failed to send Indubitably auth config request")?;

    let status = response.status();
    if !status.is_success() {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".to_string());
        anyhow::bail!("Indubitably auth config request failed ({status}): {body}");
    }

    response
        .json::<AuthConfigResponse>()
        .await
        .context("failed to parse Indubitably auth config response")
}

fn select_redirect_uri(
    redirect_uris: &[String],
    configured_port: Option<u16>,
) -> anyhow::Result<String> {
    if redirect_uris.is_empty() {
        return Ok(format!(
            "http://localhost:{INDUBITABLY_DEFAULT_REDIRECT_PORT}{INDUBITABLY_CALLBACK_PATH}"
        ));
    }

    let candidates: Vec<&String> = if let Some(configured_port) = configured_port {
        let filtered: Vec<&String> = redirect_uris
            .iter()
            .filter(|uri| redirect_port_from_uri(uri).ok() == Some(configured_port.into()))
            .collect();

        if filtered.is_empty() {
            anyhow::bail!(
                "redirect port {configured_port} is not allowed by this backend; allowed redirect_uris: {}",
                redirect_uris.join(", ")
            );
        }
        filtered
    } else {
        redirect_uris.iter().collect()
    };

    for preferred_host in ["localhost", "127.0.0.1"] {
        if let Some(uri) = candidates.iter().find(|uri| {
            Url::parse(uri)
                .ok()
                .and_then(|url| {
                    url.host_str()
                        .map(|host| host.eq_ignore_ascii_case(preferred_host))
                })
                .unwrap_or(false)
        }) {
            return Ok((*uri).clone());
        }
    }

    Ok((*candidates[0]).clone())
}

fn redirect_port_from_uri(redirect_uri: &str) -> anyhow::Result<i64> {
    let url = Url::parse(redirect_uri)
        .map_err(|err| anyhow::anyhow!("invalid redirect_uri {redirect_uri:?}: {err}"))?;
    if url.path() != INDUBITABLY_CALLBACK_PATH {
        anyhow::bail!(
            "redirect_uri must use path {INDUBITABLY_CALLBACK_PATH}, got {}",
            url.path()
        );
    }

    let port = url
        .port()
        .map(i64::from)
        .ok_or_else(|| anyhow::anyhow!("redirect_uri is missing an explicit port"))?;

    Ok(port)
}

fn bind_login_server(redirect_uri: &str) -> anyhow::Result<Server> {
    let url = Url::parse(redirect_uri)
        .map_err(|err| anyhow::anyhow!("invalid redirect_uri {redirect_uri:?}: {err}"))?;

    if url.path() != INDUBITABLY_CALLBACK_PATH {
        anyhow::bail!(
            "redirect_uri must use path {INDUBITABLY_CALLBACK_PATH}, got {}",
            url.path()
        );
    }

    let host = url
        .host_str()
        .ok_or_else(|| anyhow::anyhow!("redirect_uri is missing a host"))?;
    let port = url
        .port()
        .ok_or_else(|| anyhow::anyhow!("redirect_uri is missing an explicit port"))?;

    let bind_host = match host.to_ascii_lowercase().as_str() {
        "127.0.0.1" => "127.0.0.1",
        "localhost" => "localhost",
        _ => anyhow::bail!("redirect_uri host must be localhost or 127.0.0.1, got {host}"),
    };

    Server::http(format!("{bind_host}:{port}")).map_err(|err| anyhow::anyhow!(err))
}

async fn wait_for_auth_code(server: Server, expected_state: &str) -> anyhow::Result<String> {
    let server = std::sync::Arc::new(server);
    let (tx, mut rx) = tokio::sync::mpsc::channel(16);

    {
        let server = server.clone();
        std::thread::spawn(move || {
            while let Ok(request) = server.recv() {
                if tx.blocking_send(request).is_err() {
                    break;
                }
            }
        });
    }

    loop {
        let request = rx.recv().await.ok_or_else(|| {
            anyhow::anyhow!("login server stopped before completing authentication")
        })?;
        let url_raw = request.url().to_string();
        if let Some(code) = handle_login_request(request, &url_raw, expected_state)? {
            return Ok(code);
        }
    }
}

fn handle_login_request(
    request: tiny_http::Request,
    url_raw: &str,
    expected_state: &str,
) -> anyhow::Result<Option<String>> {
    let url = if url_raw.starts_with("http://") || url_raw.starts_with("https://") {
        Url::parse(url_raw).map_err(|err| anyhow::anyhow!("invalid callback URL: {err}"))?
    } else {
        Url::parse(&format!("http://localhost{url_raw}"))
            .map_err(|err| anyhow::anyhow!("invalid callback URL: {err}"))?
    };

    if url.path() != INDUBITABLY_CALLBACK_PATH {
        request.respond(html_response(build_callback_html(CallbackPage::Info {
            heading: "Login In Progress",
            message: "This tab is used to complete CLI login. Return to your terminal to continue.",
            details: None,
        }))?)?;
        return Ok(None);
    }

    let mut code = None;
    let mut state = None;
    let mut error = None;

    for (key, value) in url.query_pairs() {
        match key.as_ref() {
            "code" => code = Some(value.to_string()),
            "state" => state = Some(value.to_string()),
            "error" => error = Some(value.to_string()),
            _ => {}
        }
    }

    if let Some(error) = error {
        request.respond(html_response(build_callback_html(CallbackPage::Error {
            heading: "Authentication Failed",
            message: "The login callback included an error.",
            details: Some(error.clone()),
        }))?)?;
        anyhow::bail!("authentication failed: {error}");
    }

    if state.as_deref() != Some(expected_state) {
        let command = command_with_args("login --indubitably");
        let message =
            format!("State mismatch. Please return to your terminal and run `{command}` again.");
        request.respond(html_response(build_callback_html(CallbackPage::Error {
            heading: "Authentication Failed",
            message: message.as_str(),
            details: None,
        }))?)?;
        return Ok(None);
    }

    let Some(code) = code else {
        let command = command_with_args("login --indubitably");
        let message = format!(
            "Missing authorization code. Please return to your terminal and run `{command}` again."
        );
        request.respond(html_response(build_callback_html(CallbackPage::Error {
            heading: "Authentication Failed",
            message: message.as_str(),
            details: None,
        }))?)?;
        return Ok(None);
    };

    request.respond(html_response(build_callback_html(CallbackPage::Success {
        heading: "Authentication Complete",
        message: "You are signed in. You can close this tab and return to your terminal.",
        details: None,
    }))?)?;

    Ok(Some(code))
}

enum CallbackPage<'a> {
    Success {
        heading: &'a str,
        message: &'a str,
        details: Option<String>,
    },
    Error {
        heading: &'a str,
        message: &'a str,
        details: Option<String>,
    },
    Info {
        heading: &'a str,
        message: &'a str,
        details: Option<String>,
    },
}

fn html_response(html: String) -> anyhow::Result<Response<std::io::Cursor<Vec<u8>>>> {
    let content_type = Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..])
        .map_err(|_| anyhow::anyhow!("failed to build response headers"))?;
    let cache_control = Header::from_bytes(&b"Cache-Control"[..], &b"no-store"[..])
        .map_err(|_| anyhow::anyhow!("failed to build response headers"))?;

    Ok(Response::from_data(html.into_bytes())
        .with_status_code(200)
        .with_header(content_type)
        .with_header(cache_control))
}

fn build_callback_html(page: CallbackPage<'_>) -> String {
    let (status_class, heading, message, details) = match page {
        CallbackPage::Success {
            heading,
            message,
            details,
        } => ("success", heading, message, details),
        CallbackPage::Error {
            heading,
            message,
            details,
        } => ("error", heading, message, details),
        CallbackPage::Info {
            heading,
            message,
            details,
        } => ("info", heading, message, details),
    };

    let details_html = details.map_or(String::new(), |text| {
        format!(
            "<pre style=\"margin-top:16px;padding:12px;border:1px solid #ccc;white-space:pre-wrap\">{}</pre>",
            html_escape(&text)
        )
    });

    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"/><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"/><title>Indubitably Codex CLI</title><style>body{{font-family:Arial,sans-serif;margin:0;background:#fff;color:#111}}main{{max-width:640px;margin:48px auto;padding:0 20px}}.card{{border:2px solid #111;padding:24px}}.kicker{{font-size:12px;letter-spacing:0.2em;text-transform:uppercase;color:#555}}h1{{margin:12px 0 0;font-size:28px}}p{{line-height:1.5}}.success h1{{color:#0b7a1c}}.error h1{{color:#b00020}}</style></head><body><main><section class=\"card {status_class}\"><p class=\"kicker\">CLI LOGIN</p><h1>{}</h1><p>{}</p>{details_html}</section></main></body></html>",
        html_escape(heading),
        html_escape(message),
    )
}

fn html_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
    out
}

fn build_auth_url(
    auth_config: &AuthConfigResponse,
    redirect_uri: &str,
    code_challenge: &str,
    state: &str,
) -> anyhow::Result<String> {
    let hosted_ui_base_url = auth_config
        .hosted_ui_base_url
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!("backend did not return web_login_url or hosted_ui_base_url")
        })?;
    let client_id = auth_config
        .client_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| anyhow::anyhow!("backend did not return client_id for OAuth login"))?;

    let mut url = Url::parse(hosted_ui_base_url)?;
    let mut path = url.path().trim_end_matches('/').to_string();
    if !path.ends_with("/oauth2/authorize") {
        path.push_str("/oauth2/authorize");
    }
    url.set_path(&path);

    let scope = auth_config
        .scopes
        .as_ref()
        .map(|scopes| scopes.join(" "))
        .unwrap_or_else(|| "openid email profile".to_string());

    url.query_pairs_mut()
        .append_pair("response_type", "code")
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
        .append_pair("code_challenge", code_challenge)
        .append_pair(
            "code_challenge_method",
            auth_config
                .code_challenge_method
                .as_deref()
                .unwrap_or("S256"),
        )
        .append_pair("scope", &scope);

    Ok(url.to_string())
}

fn build_web_login_url(
    web_login_url: &str,
    redirect_uri: &str,
    code_challenge: &str,
    state: &str,
    code_challenge_method: Option<&str>,
) -> anyhow::Result<String> {
    let mut url = Url::parse(web_login_url)?;
    url.query_pairs_mut()
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("state", state)
        .append_pair("code_challenge", code_challenge)
        .append_pair(
            "code_challenge_method",
            code_challenge_method.unwrap_or("S256"),
        );

    Ok(url.to_string())
}

fn generate_state() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

fn generate_pkce() -> PkceCodes {
    let mut bytes = [0u8; 64];
    rand::rng().fill_bytes(&mut bytes);
    let code_verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    let digest = sha2::Sha256::digest(code_verifier.as_bytes());
    let code_challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest);
    PkceCodes {
        code_verifier,
        code_challenge,
    }
}

async fn exchange_token(
    base_url: &str,
    request: &TokenExchangeRequest,
) -> anyhow::Result<TokenExchangeResponse> {
    let url = format!("{}/cli/auth/token", normalize_base_url(base_url));
    let response = reqwest::Client::new()
        .post(url)
        .json(request)
        .send()
        .await
        .context("failed to send Indubitably token exchange request")?;

    let status = response.status();
    if !status.is_success() {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unreadable body>".to_string());
        anyhow::bail!("Indubitably token exchange failed ({status}): {body}");
    }

    response
        .json::<TokenExchangeResponse>()
        .await
        .context("failed to parse Indubitably token exchange response")
}

fn save_indubitably_tokens(
    backend_base_url: &str,
    codex_home: &Path,
    token_response: TokenExchangeResponse,
) -> anyhow::Result<()> {
    let path = default_indubitably_auth_file(codex_home);
    let mut store = read_token_store(&path).unwrap_or_default();

    let expires_at = Utc::now()
        .checked_add_signed(ChronoDuration::seconds(token_response.expires_in))
        .map(|value| value.timestamp_millis());

    store.entries.insert(
        normalize_base_url(backend_base_url),
        StoredIndubitablyToken {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            id_token: token_response.id_token,
            expires_at,
            tenant_id: Some(token_response.tenant_id),
            user_id: Some(token_response.user_id),
            active_model_id: None,
        },
    );

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)?;
    }

    let serialized = serde_json::to_string_pretty(&store)
        .context("failed to serialize Indubitably auth token store")?;
    std::fs::write(&path, serialized)
        .with_context(|| format!("failed to write token store at {}", path.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

fn load_indubitably_tokens(
    backend_base_url: &str,
    codex_home: &Path,
) -> anyhow::Result<Option<StoredIndubitablyToken>> {
    let candidate_base_urls = candidate_base_urls(backend_base_url);
    if candidate_base_urls.is_empty() {
        return Ok(None);
    }

    for path in candidate_token_store_paths(codex_home) {
        let Some(store) = read_token_store(&path) else {
            continue;
        };

        for base_url in &candidate_base_urls {
            if let Some(token) = store.entries.get(base_url)
                && token_is_valid(token)
            {
                return Ok(Some(token.clone()));
            }
        }
    }

    Ok(None)
}

fn token_is_valid(token: &StoredIndubitablyToken) -> bool {
    if token.access_token.trim().is_empty() {
        return false;
    }

    if let Some(expires_at) = token.expires_at
        && expires_at <= Utc::now().timestamp_millis()
    {
        return false;
    }

    true
}

fn read_token_store(path: &Path) -> Option<IndubitablyTokenStore> {
    let contents = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<IndubitablyTokenStore>(&contents).ok()
}

fn candidate_base_urls(base_url: &str) -> Vec<String> {
    let normalized = normalize_base_url(base_url);
    if normalized.trim().is_empty() {
        return Vec::new();
    }

    let mut candidates = vec![normalized.clone()];
    candidates.push(format!("{normalized}/v1"));
    candidates.sort_unstable();
    candidates.dedup();
    candidates
}

fn candidate_token_store_paths(codex_home: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(path) = std::env::var(INDUBITABLY_AUTH_FILE_ENV_VAR)
        && !path.trim().is_empty()
    {
        paths.push(PathBuf::from(path));
    }

    paths.push(codex_home.join(INDUBITABLY_AUTH_FILE_NAME));

    if let Some(home_dir) = dirs::home_dir() {
        paths.push(
            home_dir
                .join(".indubitably")
                .join(INDUBITABLY_AUTH_FILE_NAME),
        );
    }

    paths.sort_unstable();
    paths.dedup();
    paths
}

fn default_indubitably_auth_file(codex_home: &Path) -> PathBuf {
    if let Ok(path) = std::env::var(INDUBITABLY_AUTH_FILE_ENV_VAR)
        && !path.trim().is_empty()
    {
        return PathBuf::from(path);
    }

    codex_home.join(INDUBITABLY_AUTH_FILE_NAME)
}

fn safe_format_key(key: &str) -> String {
    if key.len() <= 13 {
        return "***".to_string();
    }
    let prefix = &key[..8];
    let suffix = &key[key.len() - 5..];
    format!("{prefix}***{suffix}")
}

#[derive(Debug, Clone)]
struct PkceCodes {
    code_verifier: String,
    code_challenge: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthConfigResponse {
    #[serde(default)]
    client_id: Option<String>,
    #[serde(default)]
    hosted_ui_base_url: Option<String>,
    #[serde(default)]
    web_login_url: Option<String>,
    #[serde(default)]
    redirect_uris: Vec<String>,
    #[serde(default)]
    code_challenge_method: Option<String>,
    #[serde(default)]
    scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize)]
struct TokenExchangeRequest {
    grant_type: String,
    code: Option<String>,
    refresh_token: Option<String>,
    redirect_uri: Option<String>,
    code_verifier: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenExchangeResponse {
    access_token: String,
    refresh_token: Option<String>,
    id_token: String,
    expires_in: i64,
    tenant_id: String,
    user_id: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct IndubitablyTokenStore {
    #[serde(default)]
    entries: BTreeMap<String, StoredIndubitablyToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredIndubitablyToken {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    id_token: String,
    #[serde(default)]
    expires_at: Option<i64>,
    #[serde(default)]
    tenant_id: Option<String>,
    #[serde(default)]
    user_id: Option<String>,
    #[serde(default)]
    active_model_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::candidate_base_urls;
    use super::normalize_base_url;
    use super::safe_format_key;
    use super::select_redirect_uri;
    use pretty_assertions::assert_eq;

    #[test]
    fn formats_long_key() {
        let key = "sk-proj-1234567890ABCDE";
        assert_eq!(safe_format_key(key), "sk-proj-***ABCDE");
    }

    #[test]
    fn short_key_returns_stars() {
        let key = "sk-proj-12345";
        assert_eq!(safe_format_key(key), "***");
    }

    #[test]
    fn normalize_base_url_drops_trailing_v1() {
        assert_eq!(
            normalize_base_url("https://api.indubitably.ai/v1"),
            "https://api.indubitably.ai"
        );
        assert_eq!(
            normalize_base_url("https://api.indubitably.ai/v1/"),
            "https://api.indubitably.ai"
        );
    }

    #[test]
    fn candidate_base_urls_include_v1_variant() {
        let candidates = candidate_base_urls("https://api.indubitably.ai");
        assert_eq!(
            candidates,
            vec![
                "https://api.indubitably.ai".to_string(),
                "https://api.indubitably.ai/v1".to_string()
            ]
        );
    }

    #[test]
    fn select_redirect_uri_prefers_localhost() {
        let redirect_uris = vec![
            "http://127.0.0.1:1455/auth/callback".to_string(),
            "http://localhost:1455/auth/callback".to_string(),
        ];

        let selected = select_redirect_uri(&redirect_uris, None).expect("select redirect uri");
        assert_eq!(selected, "http://localhost:1455/auth/callback");
    }
}
