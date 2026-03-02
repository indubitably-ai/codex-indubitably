use anyhow::Result;
use codex_core::ModelClient;
use codex_core::Prompt;
use codex_core::ResponseEvent;
use codex_core::auth::AuthManager;
use codex_core::built_in_model_providers;
use codex_core::error::CodexErr;
use codex_otel::OtelManager;
use codex_protocol::ThreadId;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::protocol::SessionSource;
use core_test_support::bedrock::provider_for_mock_server_uri;
use core_test_support::bedrock::test_model_info;
use core_test_support::bedrock_fixtures::load_stream_fixture;
use core_test_support::responses::start_mock_server;
use core_test_support::skip_if_no_network;
use futures::StreamExt;
use pretty_assertions::assert_eq;
use std::sync::Arc;
use wiremock::Mock;
use wiremock::ResponseTemplate;
use wiremock::matchers::method;
use wiremock::matchers::path;
use wiremock::matchers::query_param;

const BEDROCK_PROVIDER_ID: &str = "bedrock";

fn test_otel_manager() -> OtelManager {
    OtelManager::new(
        ThreadId::new(),
        "claude-3-7-sonnet",
        "claude-3-7-sonnet",
        None,
        None,
        None,
        "test-originator".to_string(),
        false,
        "test-terminal".to_string(),
        SessionSource::Cli,
    )
}

#[test]
fn bedrock_stream_fixture_parses_expected_chunk_sequence() {
    let stream_fixture = load_stream_fixture("stream_basic.json");
    let chunk_kinds: Vec<&str> = stream_fixture
        .chunks
        .iter()
        .map(|chunk| chunk.kind.as_str())
        .collect();
    assert_eq!(
        chunk_kinds,
        vec!["message_delta", "message_delta", "usage", "done"]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bedrock_provider_stream_without_runtime_config_returns_unsupported_operation() {
    let provider = built_in_model_providers()
        .get(BEDROCK_PROVIDER_ID)
        .unwrap_or_else(|| panic!("bedrock provider should exist"))
        .clone();

    let client = ModelClient::new(
        None::<Arc<AuthManager>>,
        ThreadId::new(),
        BEDROCK_PROVIDER_ID.to_string(),
        provider,
        SessionSource::Cli,
        None,
        None,
        false,
        false,
        None,
    );
    let mut session = client.new_session();

    let result = session
        .stream(
            &Prompt::default(),
            &test_model_info(),
            &test_otel_manager(),
            None,
            ReasoningSummary::Auto,
            None,
        )
        .await;

    match result {
        Err(CodexErr::UnsupportedOperation(message)) => {
            assert!(message.contains("Bedrock runtime adapter is not configured"));
        }
        Err(other) => panic!("expected UnsupportedOperation, got {other:?}"),
        Ok(_) => panic!("expected bedrock stream to return an error"),
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bedrock_provider_stream_uses_proxy_runtime_and_avoids_responses_api() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cli/bedrock/invoke"))
        .and(query_param("stream", "true"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "text/event-stream")
                .set_body_string(bedrock_sse()),
        )
        .expect(1)
        .mount(&server)
        .await;

    let provider = provider_for_mock_server_uri(server.uri().as_str());

    let client = ModelClient::new(
        None::<Arc<AuthManager>>,
        ThreadId::new(),
        BEDROCK_PROVIDER_ID.to_string(),
        provider,
        SessionSource::Cli,
        None,
        None,
        false,
        false,
        None,
    );
    let mut session = client.new_session();

    let mut stream = session
        .stream(
            &Prompt::default(),
            &test_model_info(),
            &test_otel_manager(),
            None,
            ReasoningSummary::Auto,
            None,
        )
        .await?;

    let mut aggregated_delta = String::new();
    let mut usage = None;

    while let Some(event) = stream.next().await {
        match event? {
            ResponseEvent::OutputTextDelta(delta) => {
                aggregated_delta.push_str(&delta);
            }
            ResponseEvent::Completed { token_usage, .. } => {
                usage = token_usage;
                break;
            }
            _ => {}
        }
    }

    assert_eq!(aggregated_delta, "Hello world");

    let usage = usage.unwrap_or_else(|| panic!("expected usage in completed event"));
    assert_eq!(usage.input_tokens, 12);
    assert_eq!(usage.output_tokens, 8);
    assert_eq!(usage.total_tokens, 20);

    let requests = server
        .received_requests()
        .await
        .unwrap_or_else(|| panic!("wiremock request log should be readable"));
    let bedrock_calls = requests
        .iter()
        .filter(|request| request.url.path() == "/cli/bedrock/invoke")
        .count();
    let responses_calls = requests
        .iter()
        .filter(|request| request.url.path() == "/v1/responses")
        .count();

    assert_eq!(bedrock_calls, 1, "bedrock runtime should call proxy invoke");
    assert_eq!(
        responses_calls, 0,
        "bedrock provider stream should not issue OpenAI responses requests"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bedrock_provider_stream_surfaces_auth_expired_for_proxy_unauthorized() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cli/bedrock/invoke"))
        .and(query_param("stream", "true"))
        .respond_with(
            ResponseTemplate::new(401)
                .insert_header("content-type", "application/json")
                .set_body_string(r#"{"error":"Unauthorized"}"#),
        )
        .expect(1)
        .mount(&server)
        .await;

    let provider = provider_for_mock_server_uri(server.uri().as_str());

    let client = ModelClient::new(
        None::<Arc<AuthManager>>,
        ThreadId::new(),
        BEDROCK_PROVIDER_ID.to_string(),
        provider,
        SessionSource::Cli,
        None,
        None,
        false,
        false,
        None,
    );
    let mut session = client.new_session();

    let result = session
        .stream(
            &Prompt::default(),
            &test_model_info(),
            &test_otel_manager(),
            None,
            ReasoningSummary::Auto,
            None,
        )
        .await;

    match result {
        Err(CodexErr::Stream(message, _)) => {
            assert_eq!(
                message,
                "indubitably authentication expired; run `codex login --indubitably`"
            );
        }
        Err(other) => panic!("expected stream auth error, got {other:?}"),
        Ok(_) => panic!("expected bedrock stream to return an auth error"),
    }

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
        "bedrock provider stream should not issue OpenAI responses requests"
    );

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn bedrock_provider_stream_reports_unknown_operation_from_non_proxy_endpoint() -> Result<()> {
    skip_if_no_network!(Ok(()));

    let server = start_mock_server().await;

    Mock::given(method("POST"))
        .and(path("/cli/bedrock/invoke"))
        .and(query_param("stream", "true"))
        .respond_with(
            ResponseTemplate::new(404)
                .insert_header("content-type", "application/xml")
                .set_body_string("<UnknownOperationException/>"),
        )
        .expect(1)
        .mount(&server)
        .await;

    let provider = provider_for_mock_server_uri(server.uri().as_str());

    let client = ModelClient::new(
        None::<Arc<AuthManager>>,
        ThreadId::new(),
        BEDROCK_PROVIDER_ID.to_string(),
        provider,
        SessionSource::Cli,
        None,
        None,
        false,
        false,
        None,
    );
    let mut session = client.new_session();

    let result = session
        .stream(
            &Prompt::default(),
            &test_model_info(),
            &test_otel_manager(),
            None,
            ReasoningSummary::Auto,
            None,
        )
        .await;

    match result {
        Err(CodexErr::Stream(message, _)) => {
            assert!(
                message.contains("bedrock proxy returned status 404 Not Found"),
                "unexpected error message: {message}"
            );
            assert!(
                message.contains("UnknownOperationException"),
                "unexpected error message: {message}"
            );
        }
        Err(other) => panic!("expected stream status error, got {other:?}"),
        Ok(_) => panic!("expected bedrock stream to return an endpoint error"),
    }

    Ok(())
}

fn bedrock_sse() -> String {
    [
        "event: bedrock.chunk\n",
        "data: {\"type\":\"message_delta\",\"payload\":{\"delta\":{\"text\":\"Hello\"}}}\n\n",
        "event: bedrock.chunk\n",
        "data: {\"type\":\"message_delta\",\"payload\":{\"delta\":{\"text\":\" world\"}}}\n\n",
        "event: bedrock.chunk\n",
        "data: {\"type\":\"usage\",\"payload\":{\"inputTokens\":12,\"outputTokens\":8,\"totalTokens\":20}}\n\n",
        "event: bedrock.done\n",
        "data: [DONE]\n\n",
    ]
    .join("")
}
