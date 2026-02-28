#![cfg(not(target_os = "windows"))]
#![allow(clippy::expect_used, clippy::unwrap_used)]

use core_test_support::responses;
use core_test_support::test_codex_exec::test_codex_exec;
use predicates::str::contains;
use pretty_assertions::assert_eq;

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
            "unsupported operation: Bedrock runtime adapter is not configured",
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
