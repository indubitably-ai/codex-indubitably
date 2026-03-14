use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use codex_protocol::ThreadId;
use rand::Rng;
use tracing::debug;
use tracing::error;

use crate::parse_command::shlex_join;

const INITIAL_DELAY_MS: u64 = 200;
const BACKOFF_FACTOR: f64 = 2.0;
pub const INVOKED_COMMAND_NAME_ENV_VAR: &str = "CODEX_INVOKED_COMMAND_NAME";

/// Emit structured feedback metadata as key/value pairs.
///
/// This logs a tracing event with `target: "feedback_tags"`. If
/// `codex_feedback::CodexFeedback::metadata_layer()` is installed, these fields are captured and
/// later attached as tags when feedback is uploaded.
///
/// Values are wrapped with [`tracing::field::DebugValue`], so the expression only needs to
/// implement [`std::fmt::Debug`].
///
/// Example:
///
/// ```rust
/// codex_core::feedback_tags!(model = "gpt-5", cached = true);
/// codex_core::feedback_tags!(provider = provider_id, request_id = request_id);
/// ```
#[macro_export]
macro_rules! feedback_tags {
    ($( $key:ident = $value:expr ),+ $(,)?) => {
        ::tracing::info!(
            target: "feedback_tags",
            $( $key = ::tracing::field::debug(&$value) ),+
        );
    };
}

pub(crate) struct FeedbackRequestTags<'a> {
    pub endpoint: &'a str,
    pub auth_header_attached: bool,
    pub auth_header_name: Option<&'a str>,
    pub auth_mode: Option<&'a str>,
    pub auth_retry_after_unauthorized: Option<bool>,
    pub auth_recovery_mode: Option<&'a str>,
    pub auth_recovery_phase: Option<&'a str>,
    pub auth_connection_reused: Option<bool>,
    pub auth_request_id: Option<&'a str>,
    pub auth_cf_ray: Option<&'a str>,
    pub auth_error: Option<&'a str>,
    pub auth_error_code: Option<&'a str>,
    pub auth_recovery_followup_success: Option<bool>,
    pub auth_recovery_followup_status: Option<u16>,
}

struct Auth401FeedbackSnapshot<'a> {
    request_id: &'a str,
    cf_ray: &'a str,
    error: &'a str,
    error_code: &'a str,
}

impl<'a> Auth401FeedbackSnapshot<'a> {
    fn from_optional_fields(
        request_id: Option<&'a str>,
        cf_ray: Option<&'a str>,
        error: Option<&'a str>,
        error_code: Option<&'a str>,
    ) -> Self {
        Self {
            request_id: request_id.unwrap_or(""),
            cf_ray: cf_ray.unwrap_or(""),
            error: error.unwrap_or(""),
            error_code: error_code.unwrap_or(""),
        }
    }
}

pub(crate) fn emit_feedback_request_tags(tags: &FeedbackRequestTags<'_>) {
    let auth_header_name = tags.auth_header_name.unwrap_or("");
    let auth_mode = tags.auth_mode.unwrap_or("");
    let auth_retry_after_unauthorized = tags
        .auth_retry_after_unauthorized
        .map_or_else(String::new, |value| value.to_string());
    let auth_recovery_mode = tags.auth_recovery_mode.unwrap_or("");
    let auth_recovery_phase = tags.auth_recovery_phase.unwrap_or("");
    let auth_connection_reused = tags
        .auth_connection_reused
        .map_or_else(String::new, |value| value.to_string());
    let auth_request_id = tags.auth_request_id.unwrap_or("");
    let auth_cf_ray = tags.auth_cf_ray.unwrap_or("");
    let auth_error = tags.auth_error.unwrap_or("");
    let auth_error_code = tags.auth_error_code.unwrap_or("");
    let auth_recovery_followup_success = tags
        .auth_recovery_followup_success
        .map_or_else(String::new, |value| value.to_string());
    let auth_recovery_followup_status = tags
        .auth_recovery_followup_status
        .map_or_else(String::new, |value| value.to_string());
    feedback_tags!(
        endpoint = tags.endpoint,
        auth_header_attached = tags.auth_header_attached,
        auth_header_name = auth_header_name,
        auth_mode = auth_mode,
        auth_retry_after_unauthorized = auth_retry_after_unauthorized,
        auth_recovery_mode = auth_recovery_mode,
        auth_recovery_phase = auth_recovery_phase,
        auth_connection_reused = auth_connection_reused,
        auth_request_id = auth_request_id,
        auth_cf_ray = auth_cf_ray,
        auth_error = auth_error,
        auth_error_code = auth_error_code,
        auth_recovery_followup_success = auth_recovery_followup_success,
        auth_recovery_followup_status = auth_recovery_followup_status
    );
}

pub(crate) fn emit_feedback_auth_recovery_tags(
    auth_recovery_mode: &str,
    auth_recovery_phase: &str,
    auth_recovery_outcome: &str,
    auth_request_id: Option<&str>,
    auth_cf_ray: Option<&str>,
    auth_error: Option<&str>,
    auth_error_code: Option<&str>,
) {
    let auth_401 = Auth401FeedbackSnapshot::from_optional_fields(
        auth_request_id,
        auth_cf_ray,
        auth_error,
        auth_error_code,
    );
    feedback_tags!(
        auth_recovery_mode = auth_recovery_mode,
        auth_recovery_phase = auth_recovery_phase,
        auth_recovery_outcome = auth_recovery_outcome,
        auth_401_request_id = auth_401.request_id,
        auth_401_cf_ray = auth_401.cf_ray,
        auth_401_error = auth_401.error,
        auth_401_error_code = auth_401.error_code
    );
}

pub fn backoff(attempt: u64) -> Duration {
    let exp = BACKOFF_FACTOR.powi(attempt.saturating_sub(1) as i32);
    let base = (INITIAL_DELAY_MS as f64 * exp) as u64;
    let jitter = rand::rng().random_range(0.9..1.1);
    Duration::from_millis((base as f64 * jitter) as u64)
}

pub(crate) fn error_or_panic(message: impl std::string::ToString) {
    if cfg!(debug_assertions) {
        panic!("{}", message.to_string());
    } else {
        error!("{}", message.to_string());
    }
}

pub(crate) fn try_parse_error_message(text: &str) -> String {
    debug!("Parsing server error response: {}", text);
    let json = serde_json::from_str::<serde_json::Value>(text).unwrap_or_default();
    if let Some(error) = json.get("error")
        && let Some(message) = error.get("message")
        && let Some(message_str) = message.as_str()
    {
        return message_str.to_string();
    }
    if text.is_empty() {
        return "Unknown error".to_string();
    }
    text.to_string()
}

pub fn resolve_path(base: &Path, path: &PathBuf) -> PathBuf {
    if path.is_absolute() {
        path.clone()
    } else {
        base.join(path)
    }
}

pub fn cli_command_name() -> String {
    std::env::var(INVOKED_COMMAND_NAME_ENV_VAR)
        .ok()
        .and_then(|value| normalize_command_name(&value))
        .unwrap_or_else(|| "codex".to_string())
}

pub fn command_with_args(args: &str) -> String {
    let command_name = cli_command_name();
    format!("{command_name} {args}")
}

fn normalize_command_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }

    let basename = Path::new(trimmed)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(trimmed);
    let normalized = basename
        .strip_suffix(".exe")
        .or_else(|| basename.strip_suffix(".EXE"))
        .unwrap_or(basename);

    if normalized.is_empty() {
        None
    } else {
        Some(normalized.to_string())
    }
}

/// Trim a thread name and return `None` if it is empty after trimming.
pub fn normalize_thread_name(name: &str) -> Option<String> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

pub fn resume_command(thread_name: Option<&str>, thread_id: Option<ThreadId>) -> Option<String> {
    let resume_target = thread_name
        .filter(|name| !name.is_empty())
        .map(str::to_string)
        .or_else(|| thread_id.map(|thread_id| thread_id.to_string()));
    resume_target.map(|target| {
        let needs_double_dash = target.starts_with('-');
        let escaped = shlex_join(&[target]);
        if needs_double_dash {
            command_with_args(&format!("resume -- {escaped}"))
        } else {
            command_with_args(&format!("resume {escaped}"))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_parse_error_message() {
        let text = r#"{
  "error": {
    "message": "Your refresh token has already been used to generate a new access token. Please try signing in again.",
    "type": "invalid_request_error",
    "param": null,
    "code": "refresh_token_reused"
  }
}"#;
        let message = try_parse_error_message(text);
        assert_eq!(
            message,
            "Your refresh token has already been used to generate a new access token. Please try signing in again."
        );
    }

    #[test]
    fn test_try_parse_error_message_no_error() {
        let text = r#"{"message": "test"}"#;
        let message = try_parse_error_message(text);
        assert_eq!(message, r#"{"message": "test"}"#);
    }

    #[test]
    fn feedback_tags_macro_compiles() {
        #[derive(Debug)]
        struct OnlyDebug;

        feedback_tags!(model = "gpt-5", cached = true, debug_only = OnlyDebug);
    }

    #[test]
    fn normalize_thread_name_trims_and_rejects_empty() {
        assert_eq!(normalize_thread_name("   "), None);
        assert_eq!(
            normalize_thread_name("  my thread  "),
            Some("my thread".to_string())
        );
    }

    #[test]
    fn resume_command_prefers_name_over_id() {
        let thread_id = ThreadId::from_string("123e4567-e89b-12d3-a456-426614174000").unwrap();
        let command = resume_command(Some("my-thread"), Some(thread_id));
        assert_eq!(command, Some(command_with_args("resume my-thread")));
    }

    #[test]
    fn resume_command_with_only_id() {
        let thread_id = ThreadId::from_string("123e4567-e89b-12d3-a456-426614174000").unwrap();
        let command = resume_command(None, Some(thread_id));
        assert_eq!(
            command,
            Some(command_with_args(
                "resume 123e4567-e89b-12d3-a456-426614174000"
            ))
        );
    }

    #[test]
    fn resume_command_with_no_name_or_id() {
        let command = resume_command(None, None);
        assert_eq!(command, None);
    }

    #[test]
    fn resume_command_quotes_thread_name_when_needed() {
        let command = resume_command(Some("-starts-with-dash"), None);
        assert_eq!(
            command,
            Some(command_with_args("resume -- -starts-with-dash"))
        );

        let command = resume_command(Some("two words"), None);
        assert_eq!(command, Some(command_with_args("resume 'two words'")));

        let command = resume_command(Some("quote'case"), None);
        assert_eq!(command, Some(command_with_args("resume \"quote'case\"")));
    }

    #[test]
    fn normalize_command_name_handles_paths_and_exe_suffix() {
        assert_eq!(
            normalize_command_name("/tmp/indubitably.exe"),
            Some("indubitably".to_string())
        );
        assert_eq!(normalize_command_name("codex"), Some("codex".to_string()));
        assert_eq!(normalize_command_name("   "), None);
    }
}
