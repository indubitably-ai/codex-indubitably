#![cfg(not(target_os = "windows"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use core_test_support::responses;
use core_test_support::test_codex_exec::test_codex_exec;
use predicates::str::contains;
use pretty_assertions::assert_eq;
use wiremock::Mock;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::matchers::query_param;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exec_indubitably_path_exits_without_openai_responses_requests() -> anyhow::Result<()> {
    let test = test_codex_exec();
    let server = responses::start_mock_server().await;
    let config_toml = r#"
[model_providers.bedrock]
name = "AWS Bedrock"
requires_openai_auth = false
supports_websockets = false
"#
    .to_string();
    std::fs::write(test.home_path().join("config.toml"), config_toml)?;

    test.cmd()
        .arg("--skip-git-repo-check")
        .arg("--experimental-json")
        .arg("--indubitably")
        .arg("--model")
        .arg("claude-3-7-sonnet")
        .arg("bedrock path check")
        .assert()
        .code(1)
        .stdout(contains(
            "indubitably authentication expired; run `codex login --indubitably`",
        ));

    let requests = server
        .received_requests()
        .await
        .unwrap_or_else(|| panic!("wiremock request log should be readable"));
    let responses_calls = requests
        .iter()
        .filter(|request| request.url.path() == "/v1/responses")
        .count();
    assert_eq!(
        responses_calls, 0,
        "indubitably/bedrock path should not call OpenAI responses endpoint",
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn exec_indubitably_proxy_unauthorized_exits_with_login_hint() -> anyhow::Result<()> {
    let test = test_codex_exec();
    let server = responses::start_mock_server().await;
    Mock::given(method("GET"))
        .and(path("/cli/models"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "models": [
                {
                    "id": "claude-3-7-sonnet",
                    "display_name": "Claude 3.7 Sonnet",
                    "provider": "bedrock",
                    "tool_use": true
                }
            ]
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/cli/bedrock/invoke"))
        .and(query_param("stream", "true"))
        .respond_with(
            ResponseTemplate::new(401)
                .insert_header("content-type", "application/json")
                .set_body_string(r#"{"error":"Unauthorized"}"#),
        )
        .mount(&server)
        .await;

    let config_toml = format!(
        r#"
[model_providers.bedrock]
name = "AWS Bedrock"
base_url = "{}/v1"
requires_openai_auth = false
supports_websockets = false
request_max_retries = 0
stream_max_retries = 0
"#,
        server.uri()
    );
    std::fs::write(test.home_path().join("config.toml"), config_toml)?;

    test.cmd()
        .arg("--skip-git-repo-check")
        .arg("--experimental-json")
        .arg("--indubitably")
        .arg("--model")
        .arg("claude-3-7-sonnet")
        .arg("bedrock auth check")
        .assert()
        .code(1)
        .stdout(contains(
            "indubitably authentication expired; run `codex login --indubitably`",
        ));

    let requests = server
        .received_requests()
        .await
        .unwrap_or_else(|| panic!("wiremock request log should be readable"));
    let responses_calls = requests
        .iter()
        .filter(|request| request.url.path() == "/v1/responses")
        .count();
    assert_eq!(
        responses_calls, 0,
        "indubitably/bedrock path should not call OpenAI responses endpoint",
    );
    let bedrock_invoke_calls = requests
        .iter()
        .filter(|request| request.url.path() == "/cli/bedrock/invoke")
        .count();
    assert!(
        bedrock_invoke_calls <= 1,
        "unexpected repeated bedrock invoke attempts: {bedrock_invoke_calls}"
    );

    Ok(())
}
