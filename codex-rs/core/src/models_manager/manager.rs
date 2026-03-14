use super::cache::ModelsCacheManager;
use crate::api_bridge::CoreAuthProvider;
use crate::api_bridge::auth_provider_from_auth;
use crate::api_bridge::map_api_error;
use crate::auth::AuthManager;
use crate::auth::AuthMode;
use crate::auth::CodexAuth;
use crate::config::Config;
use crate::default_client::build_reqwest_client;
use crate::error::CodexErr;
use crate::error::Result as CoreResult;
use crate::error::UnexpectedResponseError;
use crate::model_provider_info::ModelProviderInfo;
use crate::models_manager::collaboration_mode_presets::CollaborationModesConfig;
use crate::models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use crate::models_manager::model_info;
use crate::util::command_with_args;
use crate::response_debug_context::extract_response_debug_context;
use crate::response_debug_context::telemetry_transport_error_message;
use crate::util::FeedbackRequestTags;
use crate::util::emit_feedback_request_tags;
use codex_api::AuthProvider as ApiAuthProvider;
use codex_api::ModelsClient;
use codex_api::Provider as ApiProvider;
use codex_api::RequestTelemetry;
use codex_api::ReqwestTransport;
use codex_api::TransportError;
use codex_otel::TelemetryAuthMode;
use codex_protocol::config_types::CollaborationModeMask;
use codex_protocol::openai_models::ModelInfo;
use codex_protocol::openai_models::ModelPreset;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::ModelsResponse;
use http::HeaderMap;
use reqwest::header::ETAG;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::sync::TryLockError;
use tokio::time::timeout;
use tracing::error;
use tracing::info;
use tracing::instrument;

const MODEL_CACHE_FILE: &str = "models_cache.json";
const DEFAULT_MODEL_CACHE_TTL: Duration = Duration::from_secs(300);
const MODELS_REFRESH_TIMEOUT: Duration = Duration::from_secs(5);
const BEDROCK_FALLBACK_MODEL_SLUG: &str = "claude-3-5-sonnet";
const BEDROCK_FALLBACK_MODEL_DISPLAY_NAME: &str = "Claude 3.5 Sonnet";

#[derive(Debug, Deserialize)]
struct CliModelsResponse {
    models: Vec<CliModelInfo>,
}

#[derive(Debug, Deserialize)]
struct CliModelInfo {
    id: String,
    display_name: String,
    #[serde(default)]
    tool_use: bool,
}

const MODELS_ENDPOINT: &str = "/models";

#[derive(Clone)]
struct ModelsRequestTelemetry {
    auth_mode: Option<String>,
    auth_header_attached: bool,
    auth_header_name: Option<&'static str>,
}

impl RequestTelemetry for ModelsRequestTelemetry {
    fn on_request(
        &self,
        attempt: u64,
        status: Option<http::StatusCode>,
        error: Option<&TransportError>,
        duration: Duration,
    ) {
        let success = status.is_some_and(|code| code.is_success()) && error.is_none();
        let error_message = error.map(telemetry_transport_error_message);
        let response_debug = error
            .map(extract_response_debug_context)
            .unwrap_or_default();
        let status = status.map(|status| status.as_u16());
        tracing::event!(
            target: "codex_otel.log_only",
            tracing::Level::INFO,
            event.name = "codex.api_request",
            duration_ms = %duration.as_millis(),
            http.response.status_code = status,
            success = success,
            error.message = error_message.as_deref(),
            attempt = attempt,
            endpoint = MODELS_ENDPOINT,
            auth.header_attached = self.auth_header_attached,
            auth.header_name = self.auth_header_name,
            auth.request_id = response_debug.request_id.as_deref(),
            auth.cf_ray = response_debug.cf_ray.as_deref(),
            auth.error = response_debug.auth_error.as_deref(),
            auth.error_code = response_debug.auth_error_code.as_deref(),
            auth.mode = self.auth_mode.as_deref(),
        );
        tracing::event!(
            target: "codex_otel.trace_safe",
            tracing::Level::INFO,
            event.name = "codex.api_request",
            duration_ms = %duration.as_millis(),
            http.response.status_code = status,
            success = success,
            error.message = error_message.as_deref(),
            attempt = attempt,
            endpoint = MODELS_ENDPOINT,
            auth.header_attached = self.auth_header_attached,
            auth.header_name = self.auth_header_name,
            auth.request_id = response_debug.request_id.as_deref(),
            auth.cf_ray = response_debug.cf_ray.as_deref(),
            auth.error = response_debug.auth_error.as_deref(),
            auth.error_code = response_debug.auth_error_code.as_deref(),
            auth.mode = self.auth_mode.as_deref(),
        );
        emit_feedback_request_tags(&FeedbackRequestTags {
            endpoint: MODELS_ENDPOINT,
            auth_header_attached: self.auth_header_attached,
            auth_header_name: self.auth_header_name,
            auth_mode: self.auth_mode.as_deref(),
            auth_retry_after_unauthorized: None,
            auth_recovery_mode: None,
            auth_recovery_phase: None,
            auth_connection_reused: None,
            auth_request_id: response_debug.request_id.as_deref(),
            auth_cf_ray: response_debug.cf_ray.as_deref(),
            auth_error: response_debug.auth_error.as_deref(),
            auth_error_code: response_debug.auth_error_code.as_deref(),
            auth_recovery_followup_success: None,
            auth_recovery_followup_status: None,
        });
    }
}

/// Strategy for refreshing available models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshStrategy {
    /// Always fetch from the network, ignoring cache.
    Online,
    /// Only use cached data, never fetch from the network.
    Offline,
    /// Use cache if available and fresh, otherwise fetch from the network.
    OnlineIfUncached,
}

impl RefreshStrategy {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::Offline => "offline",
            Self::OnlineIfUncached => "online_if_uncached",
        }
    }
}

impl fmt::Display for RefreshStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// How the manager's base catalog is sourced for the lifetime of the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CatalogMode {
    /// Start from bundled `models.json` and allow cache/network refresh updates.
    Default,
    /// Use a caller-provided catalog as authoritative and do not mutate it via refresh.
    Custom,
}

/// Coordinates remote model discovery plus cached metadata on disk.
#[derive(Debug)]
pub struct ModelsManager {
    remote_models: RwLock<Vec<ModelInfo>>,
    catalog_mode: CatalogMode,
    collaboration_modes_config: CollaborationModesConfig,
    auth_manager: Arc<AuthManager>,
    etag: RwLock<Option<String>>,
    cache_manager: ModelsCacheManager,
    provider: ModelProviderInfo,
}

impl ModelsManager {
    /// Construct a manager scoped to the provided `AuthManager`.
    ///
    /// Uses `codex_home` to store cached model metadata and initializes with bundled catalog
    /// When `model_catalog` is provided, it becomes the authoritative remote model list and
    /// background refreshes from `/models` are disabled.
    pub fn new(
        codex_home: PathBuf,
        auth_manager: Arc<AuthManager>,
        model_catalog: Option<ModelsResponse>,
        collaboration_modes_config: CollaborationModesConfig,
    ) -> Self {
        Self::new_with_provider(
            codex_home,
            auth_manager,
            model_catalog,
            collaboration_modes_config,
            ModelProviderInfo::create_openai_provider(/* base_url */ None),
        )
    }

    /// Construct a manager with an explicit provider used for remote model refreshes.
    pub fn new_with_provider(
        codex_home: PathBuf,
        auth_manager: Arc<AuthManager>,
        model_catalog: Option<ModelsResponse>,
        collaboration_modes_config: CollaborationModesConfig,
        provider: ModelProviderInfo,
    ) -> Self {
        let cache_path = codex_home.join(MODEL_CACHE_FILE);
        let cache_manager = ModelsCacheManager::new(cache_path, DEFAULT_MODEL_CACHE_TTL);
        let catalog_mode = if model_catalog.is_some() {
            CatalogMode::Custom
        } else {
            CatalogMode::Default
        };
        let remote_models = model_catalog.map_or_else(
            || Self::load_default_remote_models(&provider),
            |catalog| catalog.models,
        );
        Self {
            remote_models: RwLock::new(remote_models),
            catalog_mode,
            collaboration_modes_config,
            auth_manager,
            etag: RwLock::new(None),
            cache_manager,
            provider,
        }
    }

    /// List all available models, refreshing according to the specified strategy.
    ///
    /// Returns model presets sorted by priority and filtered by auth mode and visibility.
    #[instrument(
        level = "info",
        skip(self),
        fields(refresh_strategy = %refresh_strategy)
    )]
    pub async fn list_models(&self, refresh_strategy: RefreshStrategy) -> Vec<ModelPreset> {
        match self.list_models_with_refresh_status(refresh_strategy).await {
            Ok(models) => return models,
            Err(err) => {
                error!("failed to refresh available models: {err}");
            }
        }
        self.build_available_models(self.get_remote_models().await)
    }

    /// List all available models and surface refresh failures to the caller.
    pub async fn list_models_with_refresh_status(
        &self,
        refresh_strategy: RefreshStrategy,
    ) -> CoreResult<Vec<ModelPreset>> {
        self.refresh_available_models(refresh_strategy).await?;
        let remote_models = self.get_remote_models().await;
        Ok(self.build_available_models(remote_models))
    }

    /// List collaboration mode presets.
    ///
    /// Returns a static set of presets seeded with the configured model.
    pub fn list_collaboration_modes(&self) -> Vec<CollaborationModeMask> {
        self.list_collaboration_modes_for_config(self.collaboration_modes_config)
    }

    pub fn list_collaboration_modes_for_config(
        &self,
        collaboration_modes_config: CollaborationModesConfig,
    ) -> Vec<CollaborationModeMask> {
        builtin_collaboration_mode_presets(collaboration_modes_config)
    }

    /// Attempt to list models without blocking, using the current cached state.
    ///
    /// Returns an error if the internal lock cannot be acquired.
    pub fn try_list_models(&self) -> Result<Vec<ModelPreset>, TryLockError> {
        let remote_models = self.try_get_remote_models()?;
        Ok(self.build_available_models(remote_models))
    }

    // todo(aibrahim): should be visible to core only and sent on session_configured event
    /// Get the model identifier to use, refreshing according to the specified strategy.
    ///
    /// If `model` is provided, returns it directly. Otherwise selects the default based on
    /// auth mode and available models.
    #[instrument(
        level = "info",
        skip(self, model),
        fields(
            model.provided = model.is_some(),
            refresh_strategy = %refresh_strategy
        )
    )]
    pub async fn get_default_model(
        &self,
        model: &Option<String>,
        refresh_strategy: RefreshStrategy,
    ) -> String {
        if let Some(model) = model.as_ref() {
            return model.to_string();
        }
        if let Err(err) = self.refresh_available_models(refresh_strategy).await {
            error!("failed to refresh available models: {err}");
        }
        let remote_models = self.get_remote_models().await;
        let available = self.build_available_models(remote_models);
        available
            .iter()
            .find(|model| model.is_default)
            .or_else(|| available.first())
            .map(|model| model.model.clone())
            .unwrap_or_default()
    }

    // todo(aibrahim): look if we can tighten it to pub(crate)
    /// Look up model metadata, applying remote overrides and config adjustments.
    #[instrument(level = "info", skip(self, config), fields(model = model))]
    pub async fn get_model_info(&self, model: &str, config: &Config) -> ModelInfo {
        let remote_models = self.get_remote_models().await;
        Self::construct_model_info_from_candidates(model, &remote_models, config)
    }

    fn find_model_by_longest_prefix(model: &str, candidates: &[ModelInfo]) -> Option<ModelInfo> {
        let mut best: Option<ModelInfo> = None;
        for candidate in candidates {
            if !model.starts_with(&candidate.slug) {
                continue;
            }
            let is_better_match = if let Some(current) = best.as_ref() {
                candidate.slug.len() > current.slug.len()
            } else {
                true
            };
            if is_better_match {
                best = Some(candidate.clone());
            }
        }
        best
    }

    /// Retry metadata lookup for a single namespaced slug like `namespace/model-name`.
    ///
    /// This only strips one leading namespace segment and only when the namespace is ASCII
    /// alphanumeric/underscore (`\\w+`) to avoid broadly matching arbitrary aliases.
    fn find_model_by_namespaced_suffix(model: &str, candidates: &[ModelInfo]) -> Option<ModelInfo> {
        let (namespace, suffix) = model.split_once('/')?;
        if suffix.contains('/') {
            return None;
        }
        if !namespace
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            return None;
        }
        Self::find_model_by_longest_prefix(suffix, candidates)
    }

    fn construct_model_info_from_candidates(
        model: &str,
        candidates: &[ModelInfo],
        config: &Config,
    ) -> ModelInfo {
        // First use the normal longest-prefix match. If that misses, allow a narrowly scoped
        // retry for namespaced slugs like `custom/gpt-5.3-codex`.
        let remote = Self::find_model_by_longest_prefix(model, candidates)
            .or_else(|| Self::find_model_by_namespaced_suffix(model, candidates));
        let model_info = if let Some(remote) = remote {
            ModelInfo {
                slug: model.to_string(),
                used_fallback_model_metadata: false,
                ..remote
            }
        } else {
            model_info::model_info_from_slug(model)
        };
        model_info::with_config_overrides(model_info, config)
    }

    /// Refresh models if the provided ETag differs from the cached ETag.
    ///
    /// Uses `Online` strategy to fetch latest models when ETags differ.
    pub(crate) async fn refresh_if_new_etag(&self, etag: String) {
        let current_etag = self.get_etag().await;
        if current_etag.clone().is_some() && current_etag.as_deref() == Some(etag.as_str()) {
            if !self.provider.is_bedrock()
                && let Err(err) = self.cache_manager.renew_cache_ttl().await
            {
                error!("failed to renew cache TTL: {err}");
            }
            return;
        }
        if let Err(err) = self.refresh_available_models(RefreshStrategy::Online).await {
            error!("failed to refresh available models: {err}");
        }
    }

    /// Refresh available models according to the specified strategy.
    async fn refresh_available_models(&self, refresh_strategy: RefreshStrategy) -> CoreResult<()> {
        // don't override the custom model catalog if one was provided by the user
        if matches!(self.catalog_mode, CatalogMode::Custom) {
            return Ok(());
        }

        // Providers that require OpenAI auth (for example OpenAI/ChatGPT-backed providers)
        // should only refresh `/models` when ChatGPT auth is active. Other providers can
        // refresh without ChatGPT auth.
        if self.provider.requires_openai_auth
            && self.auth_manager.auth_mode() != Some(AuthMode::Chatgpt)
        {
            if matches!(
                refresh_strategy,
                RefreshStrategy::Offline | RefreshStrategy::OnlineIfUncached
            ) {
                self.try_load_cache().await;
            }
            return Ok(());
        }

        if self.provider.is_bedrock() {
            return self
                .refresh_bedrock_available_models(refresh_strategy)
                .await;
        }

        match refresh_strategy {
            RefreshStrategy::Offline => {
                // Only try to load from cache, never fetch
                self.try_load_cache().await;
                Ok(())
            }
            RefreshStrategy::OnlineIfUncached => {
                // Try cache first, fall back to online if unavailable
                if self.try_load_cache().await {
                    info!("models cache: using cached models for OnlineIfUncached");
                    return Ok(());
                }
                info!("models cache: cache miss, fetching remote models");
                self.fetch_and_update_models().await
            }
            RefreshStrategy::Online => {
                // Always fetch from network
                self.fetch_and_update_models().await
            }
        }
    }

    async fn refresh_bedrock_available_models(
        &self,
        refresh_strategy: RefreshStrategy,
    ) -> CoreResult<()> {
        match refresh_strategy {
            RefreshStrategy::Offline => Ok(()),
            RefreshStrategy::OnlineIfUncached => {
                if self.get_etag().await.is_some() {
                    return Ok(());
                }
                self.fetch_and_update_models().await
            }
            RefreshStrategy::Online => self.fetch_and_update_models().await,
        }
    }

    async fn fetch_and_update_models(&self) -> CoreResult<()> {
        let _timer =
            codex_otel::start_global_timer("codex.remote_models.fetch_update.duration_ms", &[]);
        let auth = self.auth_manager.auth().await;
        let auth_mode = auth.as_ref().map(CodexAuth::auth_mode);
        let api_provider = self.provider.to_api_provider(auth_mode)?;
        let api_auth = auth_provider_from_auth(auth.clone(), &self.provider)?;
        let client_version = crate::models_manager::client_version_to_whole();
        let (models, etag) = if self.provider.is_bedrock() {
            timeout(
                MODELS_REFRESH_TIMEOUT,
                self.fetch_bedrock_models(&api_provider, &api_auth),
            )
            .await
            .map_err(|_| CodexErr::Timeout)??
        } else {
            let transport = ReqwestTransport::new(build_reqwest_client());
            let request_telemetry: Arc<dyn RequestTelemetry> = Arc::new(ModelsRequestTelemetry {
                auth_mode: auth_mode.map(|mode| TelemetryAuthMode::from(mode).to_string()),
                auth_header_attached: api_auth.auth_header_attached(),
                auth_header_name: api_auth.auth_header_name(),
            });
            let client = ModelsClient::new(transport, api_provider, api_auth)
                .with_telemetry(Some(request_telemetry));
            timeout(
                MODELS_REFRESH_TIMEOUT,
                client.list_models(&client_version, HeaderMap::new()),
            )
            .await
            .map_err(|_| CodexErr::Timeout)?
            .map_err(map_api_error)?
        };

        self.apply_remote_models(models.clone()).await;
        *self.etag.write().await = Some(
            etag.clone()
                .unwrap_or_else(|| "bedrock-fetched".to_string()),
        );
        if !self.provider.is_bedrock() {
            self.cache_manager
                .persist_cache(&models, etag, client_version)
                .await;
        }
        Ok(())
    }

    async fn fetch_bedrock_models(
        &self,
        api_provider: &ApiProvider,
        api_auth: &CoreAuthProvider,
    ) -> CoreResult<(Vec<ModelInfo>, Option<String>)> {
        let url = match Self::bedrock_models_url(&api_provider.base_url) {
            Some(url) => url,
            None => return Ok((Self::bedrock_fallback_models(), None)),
        };
        let mut req = build_reqwest_client().get(&url);

        if !api_provider.headers.is_empty() {
            req = req.headers(api_provider.headers.clone());
        }
        if let Some(query_params) = api_provider.query_params.as_ref() {
            req = req.query(query_params);
        }
        let Some(token) = api_auth.bearer_token() else {
            return Err(CodexErr::Stream(
                format!(
                    "indubitably authentication expired; run `{}`",
                    command_with_args("login --indubitably")
                ),
                None,
            ));
        };
        req = req.bearer_auth(token);

        let response = req.send().await.map_err(|err| {
            CodexErr::Stream(
                format!("failed to fetch bedrock model allowlist: {err}"),
                None,
            )
        })?;
        let status = response.status();
        let header_etag = response
            .headers()
            .get(ETAG)
            .and_then(|value| value.to_str().ok())
            .map(ToString::to_string);
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            return Err(CodexErr::UnexpectedStatus(UnexpectedResponseError {
                status,
                body,
                url: Some(url),
                cf_ray: None,
                request_id: None,
            }));
        }

        let cli_models: CliModelsResponse = serde_json::from_str(&body)?;
        Ok((Self::map_bedrock_models(cli_models.models), header_etag))
    }

    fn bedrock_models_url(base_url: &str) -> Option<String> {
        let trimmed = base_url.trim().trim_end_matches('/');
        if trimmed.is_empty() {
            return None;
        }
        let root = if let Some(without_v1) = trimmed.strip_suffix("/v1") {
            without_v1
        } else {
            trimmed
        };
        Some(format!("{root}/cli/models"))
    }

    fn map_bedrock_models(models: Vec<CliModelInfo>) -> Vec<ModelInfo> {
        models
            .into_iter()
            .enumerate()
            .map(|(index, model)| {
                let mut mapped = model_info::model_info_from_slug(&model.id);
                mapped.display_name = model.display_name;
                mapped.priority = i32::try_from(index).unwrap_or(i32::MAX);
                mapped.visibility = if model.tool_use {
                    ModelVisibility::List
                } else {
                    ModelVisibility::Hide
                };
                mapped.used_fallback_model_metadata = false;
                mapped
            })
            .collect()
    }

    async fn get_etag(&self) -> Option<String> {
        self.etag.read().await.clone()
    }

    /// Replace the cached remote models and rebuild the derived presets list.
    async fn apply_remote_models(&self, models: Vec<ModelInfo>) {
        if self.provider.is_bedrock() {
            *self.remote_models.write().await = models;
            return;
        }

        let mut existing_models = Self::load_remote_models_from_file().unwrap_or_default();
        for model in models {
            if let Some(existing_index) = existing_models
                .iter()
                .position(|existing| existing.slug == model.slug)
            {
                existing_models[existing_index] = model;
            } else {
                existing_models.push(model);
            }
        }
        *self.remote_models.write().await = existing_models;
    }

    fn load_remote_models_from_file() -> Result<Vec<ModelInfo>, std::io::Error> {
        let file_contents = include_str!("../../models.json");
        let response: ModelsResponse = serde_json::from_str(file_contents)?;
        Ok(response.models)
    }

    fn load_default_remote_models(provider: &ModelProviderInfo) -> Vec<ModelInfo> {
        if provider.is_bedrock() {
            return Self::bedrock_fallback_models();
        }
        Self::load_remote_models_from_file()
            .unwrap_or_else(|err| panic!("failed to load bundled models.json: {err}"))
    }

    fn bedrock_fallback_models() -> Vec<ModelInfo> {
        let mut fallback = model_info::model_info_from_slug(BEDROCK_FALLBACK_MODEL_SLUG);
        fallback.display_name = BEDROCK_FALLBACK_MODEL_DISPLAY_NAME.to_string();
        fallback.visibility = ModelVisibility::List;
        fallback.priority = 0;
        fallback.used_fallback_model_metadata = false;
        vec![fallback]
    }

    /// Attempt to satisfy the refresh from the cache when it matches the provider and TTL.
    async fn try_load_cache(&self) -> bool {
        let _timer =
            codex_otel::start_global_timer("codex.remote_models.load_cache.duration_ms", &[]);
        let client_version = crate::models_manager::client_version_to_whole();
        info!(client_version, "models cache: evaluating cache eligibility");
        let cache = match self.cache_manager.load_fresh(&client_version).await {
            Some(cache) => cache,
            None => {
                info!("models cache: no usable cache entry");
                return false;
            }
        };
        let models = cache.models.clone();
        *self.etag.write().await = cache.etag.clone();
        self.apply_remote_models(models.clone()).await;
        info!(
            models_count = models.len(),
            etag = ?cache.etag,
            "models cache: cache entry applied"
        );
        true
    }

    /// Build picker-ready presets from the active catalog snapshot.
    fn build_available_models(&self, mut remote_models: Vec<ModelInfo>) -> Vec<ModelPreset> {
        remote_models.sort_by(|a, b| a.priority.cmp(&b.priority));

        let mut presets: Vec<ModelPreset> = remote_models.into_iter().map(Into::into).collect();
        let chatgpt_mode = matches!(self.auth_manager.auth_mode(), Some(AuthMode::Chatgpt));
        presets = ModelPreset::filter_by_auth(presets, chatgpt_mode);

        ModelPreset::mark_default_by_picker_visibility(&mut presets);

        presets
    }

    async fn get_remote_models(&self) -> Vec<ModelInfo> {
        self.remote_models.read().await.clone()
    }

    fn try_get_remote_models(&self) -> Result<Vec<ModelInfo>, TryLockError> {
        Ok(self.remote_models.try_read()?.clone())
    }

    /// Construct a manager with a specific provider for testing.
    pub(crate) fn with_provider_for_tests(
        codex_home: PathBuf,
        auth_manager: Arc<AuthManager>,
        provider: ModelProviderInfo,
    ) -> Self {
        Self::new_with_provider(
            codex_home,
            auth_manager,
            None,
            CollaborationModesConfig::default(),
            provider,
        )
    }

    /// Get model identifier without consulting remote state or cache.
    pub(crate) fn get_model_offline_for_tests(model: Option<&str>) -> String {
        if let Some(model) = model {
            return model.to_string();
        }
        let mut models = Self::load_remote_models_from_file().unwrap_or_default();
        models.sort_by(|a, b| a.priority.cmp(&b.priority));
        let presets: Vec<ModelPreset> = models.into_iter().map(Into::into).collect();
        presets
            .iter()
            .find(|preset| preset.show_in_picker)
            .or_else(|| presets.first())
            .map(|preset| preset.model.clone())
            .unwrap_or_default()
    }

    /// Build `ModelInfo` without consulting remote state or cache.
    pub(crate) fn construct_model_info_offline_for_tests(
        model: &str,
        config: &Config,
    ) -> ModelInfo {
        let candidates: &[ModelInfo] = if let Some(model_catalog) = config.model_catalog.as_ref() {
            &model_catalog.models
        } else {
            &[]
        };
        Self::construct_model_info_from_candidates(model, candidates, config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodexAuth;
    use crate::auth::AuthCredentialsStoreMode;
    use crate::config::ConfigBuilder;
    use crate::model_provider_info::WireApi;
    use chrono::Utc;
    use codex_protocol::openai_models::ModelsResponse;
    use core_test_support::responses::mount_models_once;
    use pretty_assertions::assert_eq;
    use reqwest::StatusCode;
    use serde_json::json;
    use tempfile::tempdir;
    use wiremock::Mock;
    use wiremock::MockServer;
    use wiremock::ResponseTemplate;
    use wiremock::matchers::method;
    use wiremock::matchers::path;

    fn remote_model(slug: &str, display: &str, priority: i32) -> ModelInfo {
        remote_model_with_visibility(slug, display, priority, "list")
    }

    fn remote_model_with_visibility(
        slug: &str,
        display: &str,
        priority: i32,
        visibility: &str,
    ) -> ModelInfo {
        serde_json::from_value(json!({
            "slug": slug,
            "display_name": display,
            "description": format!("{display} desc"),
            "default_reasoning_level": "medium",
            "supported_reasoning_levels": [{"effort": "low", "description": "low"}, {"effort": "medium", "description": "medium"}],
            "shell_type": "shell_command",
            "visibility": visibility,
            "minimal_client_version": [0, 1, 0],
            "supported_in_api": true,
            "priority": priority,
            "upgrade": null,
            "base_instructions": "base instructions",
            "supports_reasoning_summaries": false,
            "support_verbosity": false,
            "default_verbosity": null,
            "apply_patch_tool_type": null,
            "truncation_policy": {"mode": "bytes", "limit": 10_000},
            "supports_parallel_tool_calls": false,
            "supports_image_detail_original": false,
            "context_window": 272_000,
            "experimental_supported_tools": [],
        }))
        .expect("valid model")
    }

    fn assert_models_contain(actual: &[ModelInfo], expected: &[ModelInfo]) {
        for model in expected {
            assert!(
                actual.iter().any(|candidate| candidate.slug == model.slug),
                "expected model {} in cached list",
                model.slug
            );
        }
    }

    fn provider_for(base_url: String) -> ModelProviderInfo {
        ModelProviderInfo {
            name: "mock".into(),
            base_url: Some(base_url),
            env_key: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            wire_api: WireApi::Responses,
            query_params: None,
            http_headers: None,
            env_http_headers: None,
            request_max_retries: Some(0),
            stream_max_retries: Some(0),
            stream_idle_timeout_ms: Some(5_000),
            requires_openai_auth: false,
            supports_websockets: false,
        }
    }

    fn openai_provider_for(base_url: String) -> ModelProviderInfo {
        let mut provider = ModelProviderInfo::create_openai_provider(/* base_url */ None);
        provider.base_url = Some(base_url);
        provider.request_max_retries = Some(0);
        provider.stream_max_retries = Some(0);
        provider.stream_idle_timeout_ms = Some(5_000);
        provider
    }

    fn bedrock_provider_for(base_url: String) -> ModelProviderInfo {
        let mut provider =
            crate::model_provider_info::built_in_model_providers(/* openai_base_url */ None)
                .get(crate::model_provider_info::BEDROCK_PROVIDER_ID)
                .expect("bedrock provider should exist")
                .clone();
        provider.base_url = Some(base_url);
        provider.experimental_bearer_token = Some("bedrock-test-token".to_string());
        provider.request_max_retries = Some(0);
        provider.stream_max_retries = Some(0);
        provider.stream_idle_timeout_ms = Some(5_000);
        provider
    }

    #[tokio::test]
    async fn get_model_info_tracks_fallback_usage() {
        let codex_home = tempdir().expect("temp dir");
        let config = ConfigBuilder::default()
            .codex_home(codex_home.path().to_path_buf())
            .build()
            .await
            .expect("load default test config");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let manager = ModelsManager::new(
            codex_home.path().to_path_buf(),
            auth_manager,
            None,
            CollaborationModesConfig::default(),
        );
        let known_slug = manager
            .get_remote_models()
            .await
            .first()
            .expect("bundled models should include at least one model")
            .slug
            .clone();

        let known = manager.get_model_info(known_slug.as_str(), &config).await;
        assert!(!known.used_fallback_model_metadata);
        assert_eq!(known.slug, known_slug);

        let unknown = manager
            .get_model_info("model-that-does-not-exist", &config)
            .await;
        assert!(unknown.used_fallback_model_metadata);
        assert_eq!(unknown.slug, "model-that-does-not-exist");
    }

    #[tokio::test]
    async fn get_model_info_uses_custom_catalog() {
        let codex_home = tempdir().expect("temp dir");
        let config = ConfigBuilder::default()
            .codex_home(codex_home.path().to_path_buf())
            .build()
            .await
            .expect("load default test config");
        let mut overlay = remote_model("gpt-overlay", "Overlay", 0);
        overlay.supports_image_detail_original = true;

        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let manager = ModelsManager::new(
            codex_home.path().to_path_buf(),
            auth_manager,
            Some(ModelsResponse {
                models: vec![overlay],
            }),
            CollaborationModesConfig::default(),
        );

        let model_info = manager
            .get_model_info("gpt-overlay-experiment", &config)
            .await;

        assert_eq!(model_info.slug, "gpt-overlay-experiment");
        assert_eq!(model_info.display_name, "Overlay");
        assert_eq!(model_info.context_window, Some(272_000));
        assert!(model_info.supports_image_detail_original);
        assert!(!model_info.supports_parallel_tool_calls);
        assert!(!model_info.used_fallback_model_metadata);
    }

    #[tokio::test]
    async fn get_model_info_matches_namespaced_suffix() {
        let codex_home = tempdir().expect("temp dir");
        let config = ConfigBuilder::default()
            .codex_home(codex_home.path().to_path_buf())
            .build()
            .await
            .expect("load default test config");
        let mut remote = remote_model("gpt-image", "Image", 0);
        remote.supports_image_detail_original = true;
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let manager = ModelsManager::new(
            codex_home.path().to_path_buf(),
            auth_manager,
            Some(ModelsResponse {
                models: vec![remote],
            }),
            CollaborationModesConfig::default(),
        );
        let namespaced_model = "custom/gpt-image".to_string();

        let model_info = manager.get_model_info(&namespaced_model, &config).await;

        assert_eq!(model_info.slug, namespaced_model);
        assert!(model_info.supports_image_detail_original);
        assert!(!model_info.used_fallback_model_metadata);
    }

    #[tokio::test]
    async fn get_model_info_rejects_multi_segment_namespace_suffix_matching() {
        let codex_home = tempdir().expect("temp dir");
        let config = ConfigBuilder::default()
            .codex_home(codex_home.path().to_path_buf())
            .build()
            .await
            .expect("load default test config");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let manager = ModelsManager::new(
            codex_home.path().to_path_buf(),
            auth_manager,
            None,
            CollaborationModesConfig::default(),
        );
        let known_slug = manager
            .get_remote_models()
            .await
            .first()
            .expect("bundled models should include at least one model")
            .slug
            .clone();
        let namespaced_model = format!("ns1/ns2/{known_slug}");

        let model_info = manager.get_model_info(&namespaced_model, &config).await;

        assert_eq!(model_info.slug, namespaced_model);
        assert!(model_info.used_fallback_model_metadata);
    }

    #[tokio::test]
    async fn refresh_available_models_fetches_bedrock_allowlist_from_cli_models_endpoint() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cli/models"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "models": [
                    {
                        "id": "claude-3-5-sonnet",
                        "display_name": "Claude 3.5 Sonnet",
                        "provider": "bedrock",
                        "tool_use": true
                    },
                    {
                        "id": "claude-lite-no-tools",
                        "display_name": "Claude Lite",
                        "provider": "bedrock",
                        "tool_use": false
                    }
                ]
            })))
            .mount(&server)
            .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("unused-auth-manager-key"));
        let provider = bedrock_provider_for(format!("{}/v1", server.uri()));
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("bedrock refresh should succeed");

        let requests = server
            .received_requests()
            .await
            .expect("requests should be captured");
        let cli_models_calls = requests
            .iter()
            .filter(|request| request.url.path() == "/cli/models")
            .count();
        let models_calls = requests
            .iter()
            .filter(|request| request.url.path() == "/models")
            .count();
        assert_eq!(cli_models_calls, 1);
        assert_eq!(models_calls, 0);

        let authorization_header = requests
            .iter()
            .find(|request| request.url.path() == "/cli/models")
            .and_then(|request| request.headers.get("authorization"))
            .and_then(|value| value.to_str().ok());
        assert_eq!(authorization_header, Some("Bearer bedrock-test-token"));

        let available = manager.list_models(RefreshStrategy::Offline).await;
        assert!(
            available
                .iter()
                .any(|preset| preset.model == "claude-3-5-sonnet"),
            "bedrock allowlist model should appear in picker"
        );
        assert!(
            !available.iter().any(|preset| preset.model == "gpt-5-codex"),
            "bedrock picker should not include bundled OpenAI models"
        );
    }

    #[tokio::test]
    async fn bedrock_provider_uses_bedrock_only_fallback_catalog() {
        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let provider = bedrock_provider_for("https://api.indubitably.ai/v1".to_string());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        let available = manager.list_models(RefreshStrategy::Offline).await;
        assert!(
            available
                .iter()
                .any(|preset| preset.model == BEDROCK_FALLBACK_MODEL_SLUG),
            "bedrock fallback model should exist"
        );
        assert!(
            !available.iter().any(|preset| preset.model == "gpt-5-codex"),
            "bedrock fallback catalog should not include bundled OpenAI models"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_surfaces_unknown_operation_for_non_proxy_bedrock_base_url() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/cli/models"))
            .respond_with(
                ResponseTemplate::new(404)
                    .insert_header("content-type", "application/xml")
                    .set_body_string("<UnknownOperationException/>"),
            )
            .mount(&server)
            .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("unused-auth-manager-key"));
        let provider = bedrock_provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        let result = manager
            .refresh_available_models(RefreshStrategy::Online)
            .await;
        let Err(CodexErr::UnexpectedStatus(error)) = result else {
            panic!("expected refresh to return UnexpectedStatus");
        };
        assert_eq!(error.status, StatusCode::NOT_FOUND);
        assert!(error.body.contains("UnknownOperationException"));
    }

    #[tokio::test]
    async fn refresh_available_models_without_bearer_token_returns_auth_error_before_network() {
        let server = MockServer::start().await;
        let codex_home = tempdir().expect("temp dir");
        let auth_manager = Arc::new(AuthManager::new(
            codex_home.path().to_path_buf(),
            false,
            AuthCredentialsStoreMode::File,
        ));
        let mut provider = bedrock_provider_for(server.uri());
        provider.experimental_bearer_token = None;
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        let result = manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await;
        let Err(CodexErr::Stream(message, None)) = result else {
            panic!("expected refresh to return auth stream error");
        };
        assert!(message.contains("indubitably authentication expired"));

        let requests = server
            .received_requests()
            .await
            .expect("requests should be captured");
        assert!(requests.is_empty(), "auth failure should avoid network I/O");
    }

    #[tokio::test]
    async fn refresh_available_models_sorts_by_priority() {
        let server = MockServer::start().await;
        let remote_models = vec![
            remote_model("priority-low", "Low", 1),
            remote_model("priority-high", "High", 0),
        ];
        let models_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: remote_models.clone(),
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::create_dummy_chatgpt_auth_for_testing());
        let provider = provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("refresh succeeds");
        let cached_remote = manager.get_remote_models().await;
        assert_models_contain(&cached_remote, &remote_models);

        let available = manager.list_models(RefreshStrategy::OnlineIfUncached).await;
        let high_idx = available
            .iter()
            .position(|model| model.model == "priority-high")
            .expect("priority-high should be listed");
        let low_idx = available
            .iter()
            .position(|model| model.model == "priority-low")
            .expect("priority-low should be listed");
        assert!(
            high_idx < low_idx,
            "higher priority should be listed before lower priority"
        );
        assert_eq!(
            models_mock.requests().len(),
            1,
            "expected a single /models request"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_uses_cache_when_fresh() {
        let server = MockServer::start().await;
        let remote_models = vec![remote_model("cached", "Cached", 5)];
        let models_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: remote_models.clone(),
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::create_dummy_chatgpt_auth_for_testing());
        let provider = provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("first refresh succeeds");
        assert_models_contain(&manager.get_remote_models().await, &remote_models);

        // Second call should read from cache and avoid the network.
        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("cached refresh succeeds");
        assert_models_contain(&manager.get_remote_models().await, &remote_models);
        assert_eq!(
            models_mock.requests().len(),
            1,
            "cache hit should avoid a second /models request"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_refetches_when_cache_stale() {
        let server = MockServer::start().await;
        let initial_models = vec![remote_model("stale", "Stale", 1)];
        let initial_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: initial_models.clone(),
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::create_dummy_chatgpt_auth_for_testing());
        let provider = provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("initial refresh succeeds");

        // Rewrite cache with an old timestamp so it is treated as stale.
        manager
            .cache_manager
            .manipulate_cache_for_test(|fetched_at| {
                *fetched_at = Utc::now() - chrono::Duration::hours(1);
            })
            .await
            .expect("cache manipulation succeeds");

        let updated_models = vec![remote_model("fresh", "Fresh", 9)];
        server.reset().await;
        let refreshed_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: updated_models.clone(),
            },
        )
        .await;

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("second refresh succeeds");
        assert_models_contain(&manager.get_remote_models().await, &updated_models);
        assert_eq!(
            initial_mock.requests().len(),
            1,
            "initial refresh should only hit /models once"
        );
        assert_eq!(
            refreshed_mock.requests().len(),
            1,
            "stale cache refresh should fetch /models once"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_refetches_when_version_mismatch() {
        let server = MockServer::start().await;
        let initial_models = vec![remote_model("old", "Old", 1)];
        let initial_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: initial_models.clone(),
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::create_dummy_chatgpt_auth_for_testing());
        let provider = provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("initial refresh succeeds");

        manager
            .cache_manager
            .mutate_cache_for_test(|cache| {
                let client_version = crate::models_manager::client_version_to_whole();
                cache.client_version = Some(format!("{client_version}-mismatch"));
            })
            .await
            .expect("cache mutation succeeds");

        let updated_models = vec![remote_model("new", "New", 2)];
        server.reset().await;
        let refreshed_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: updated_models.clone(),
            },
        )
        .await;

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("second refresh succeeds");
        assert_models_contain(&manager.get_remote_models().await, &updated_models);
        assert_eq!(
            initial_mock.requests().len(),
            1,
            "initial refresh should only hit /models once"
        );
        assert_eq!(
            refreshed_mock.requests().len(),
            1,
            "version mismatch should fetch /models once"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_drops_removed_remote_models() {
        let server = MockServer::start().await;
        let initial_models = vec![remote_model("remote-old", "Remote Old", 1)];
        let initial_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: initial_models,
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::create_dummy_chatgpt_auth_for_testing());
        let provider = provider_for(server.uri());
        let mut manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );
        manager.cache_manager.set_ttl(Duration::ZERO);

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("initial refresh succeeds");

        server.reset().await;
        let refreshed_models = vec![remote_model("remote-new", "Remote New", 1)];
        let refreshed_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: refreshed_models,
            },
        )
        .await;

        manager
            .refresh_available_models(RefreshStrategy::OnlineIfUncached)
            .await
            .expect("second refresh succeeds");

        let available = manager
            .try_list_models()
            .expect("models should be available");
        assert!(
            available.iter().any(|preset| preset.model == "remote-new"),
            "new remote model should be listed"
        );
        assert!(
            !available.iter().any(|preset| preset.model == "remote-old"),
            "removed remote model should not be listed"
        );
        assert_eq!(
            initial_mock.requests().len(),
            1,
            "initial refresh should only hit /models once"
        );
        assert_eq!(
            refreshed_mock.requests().len(),
            1,
            "second refresh should only hit /models once"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_fetches_non_openai_provider_without_chatgpt_auth() {
        let server = MockServer::start().await;
        let dynamic_slug = "dynamic-model-only-for-test-noauth";
        let models_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: vec![remote_model(dynamic_slug, "No Auth", 1)],
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager = Arc::new(AuthManager::new(
            codex_home.path().to_path_buf(),
            false,
            AuthCredentialsStoreMode::File,
        ));
        let provider = provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::Online)
            .await
            .expect("non-OpenAI providers should refresh without chatgpt auth");
        let cached_remote = manager.get_remote_models().await;
        assert!(
            cached_remote
                .iter()
                .any(|candidate| candidate.slug == dynamic_slug),
            "non-OpenAI providers should refresh /models without chatgpt auth"
        );
        assert_eq!(
            models_mock.requests().len(),
            1,
            "non-OpenAI providers should fetch /models without chatgpt auth"
        );
    }

    #[tokio::test]
    async fn refresh_available_models_skips_openai_auth_provider_without_chatgpt_auth() {
        let server = MockServer::start().await;
        let dynamic_slug = "dynamic-model-only-for-test-openai-auth";
        let models_mock = mount_models_once(
            &server,
            ModelsResponse {
                models: vec![remote_model(dynamic_slug, "OpenAI Auth", 1)],
            },
        )
        .await;

        let codex_home = tempdir().expect("temp dir");
        let auth_manager = Arc::new(AuthManager::new(
            codex_home.path().to_path_buf(),
            false,
            AuthCredentialsStoreMode::File,
        ));
        let provider = openai_provider_for(server.uri());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        manager
            .refresh_available_models(RefreshStrategy::Online)
            .await
            .expect("OpenAI auth providers should no-op without chatgpt auth");
        let cached_remote = manager.get_remote_models().await;
        assert!(
            !cached_remote
                .iter()
                .any(|candidate| candidate.slug == dynamic_slug),
            "OpenAI auth providers should skip /models refresh without chatgpt auth"
        );
        assert_eq!(
            models_mock.requests().len(),
            0,
            "OpenAI auth providers should avoid /models requests without chatgpt auth"
        );
    }

    #[test]
    fn build_available_models_picks_default_after_hiding_hidden_models() {
        let codex_home = tempdir().expect("temp dir");
        let auth_manager =
            AuthManager::from_auth_for_testing(CodexAuth::from_api_key("Test API Key"));
        let provider = provider_for("http://example.test".to_string());
        let manager = ModelsManager::with_provider_for_tests(
            codex_home.path().to_path_buf(),
            auth_manager,
            provider,
        );

        let hidden_model = remote_model_with_visibility("hidden", "Hidden", 0, "hide");
        let visible_model = remote_model_with_visibility("visible", "Visible", 1, "list");

        let expected_hidden = ModelPreset::from(hidden_model.clone());
        let mut expected_visible = ModelPreset::from(visible_model.clone());
        expected_visible.is_default = true;

        let available = manager.build_available_models(vec![hidden_model, visible_model]);

        assert_eq!(available, vec![expected_hidden, expected_visible]);
    }

    #[test]
    fn bundled_models_json_roundtrips() {
        let file_contents = include_str!("../../models.json");
        let response: ModelsResponse =
            serde_json::from_str(file_contents).expect("bundled models.json should deserialize");

        let serialized =
            serde_json::to_string(&response).expect("bundled models.json should serialize");
        let roundtripped: ModelsResponse =
            serde_json::from_str(&serialized).expect("serialized models.json should deserialize");

        assert_eq!(
            response, roundtripped,
            "bundled models.json should round trip through serde"
        );
        assert!(
            !response.models.is_empty(),
            "bundled models.json should contain at least one model"
        );
    }
}
