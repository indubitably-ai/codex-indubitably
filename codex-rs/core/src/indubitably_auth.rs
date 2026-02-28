use std::collections::BTreeMap;
use std::path::PathBuf;

use chrono::Utc;
use serde::Deserialize;
use tracing::debug;

use crate::config::find_codex_home;

const INDUBITABLY_AUTH_FILE_ENV_VAR: &str = "INDUBITABLY_AUTH_FILE";
const INDUBITABLY_AUTH_FILE_NAME: &str = "indubitably-auth.json";

#[derive(Debug, Deserialize)]
struct TokenStore {
    #[serde(default)]
    entries: BTreeMap<String, StoredToken>,
}

#[derive(Debug, Deserialize)]
struct StoredToken {
    access_token: String,
    expires_at: Option<i64>,
}

pub(crate) fn load_access_token_for_base_url(base_url: &str) -> Option<String> {
    let base_urls = candidate_base_urls(base_url);
    if base_urls.is_empty() {
        return None;
    }

    for path in candidate_token_store_paths() {
        let Some(store) = read_token_store(&path) else {
            continue;
        };

        for base in &base_urls {
            if let Some(token) = store.entries.get(base).and_then(select_access_token) {
                return Some(token);
            }
        }
    }

    None
}

fn select_access_token(token: &StoredToken) -> Option<String> {
    if token.access_token.trim().is_empty() {
        return None;
    }

    if let Some(expires_at) = token.expires_at
        && expires_at <= Utc::now().timestamp_millis()
    {
        return None;
    }

    Some(token.access_token.clone())
}

fn candidate_base_urls(base_url: &str) -> Vec<String> {
    let normalized = base_url.trim().trim_end_matches('/');
    if normalized.is_empty() {
        return Vec::new();
    }

    let mut candidates = vec![normalized.to_string()];
    if let Some(without_v1) = normalized.strip_suffix("/v1") {
        candidates.push(without_v1.to_string());
    } else {
        candidates.push(format!("{normalized}/v1"));
    }

    candidates.sort_unstable();
    candidates.dedup();
    candidates
}

fn candidate_token_store_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Ok(path) = std::env::var(INDUBITABLY_AUTH_FILE_ENV_VAR)
        && !path.trim().is_empty()
    {
        paths.push(PathBuf::from(path));
    }

    if let Ok(codex_home) = find_codex_home() {
        paths.push(codex_home.join(INDUBITABLY_AUTH_FILE_NAME));
    }

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

fn read_token_store(path: &PathBuf) -> Option<TokenStore> {
    let contents = match std::fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return None,
    };

    match serde_json::from_str::<TokenStore>(&contents) {
        Ok(store) => Some(store),
        Err(error) => {
            debug!(
                "failed to parse indubitably auth token store {}: {error}",
                path.display()
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serial_test::serial;

    struct EnvGuard {
        key: &'static str,
        previous: Option<String>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: impl Into<String>) -> Self {
            let previous = std::env::var(key).ok();
            // SAFETY: tests use this guard in a scoped manner and restore on drop.
            unsafe {
                std::env::set_var(key, value.into());
            }
            Self { key, previous }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            // SAFETY: tests use this guard in a scoped manner and restore on drop.
            unsafe {
                match self.previous.take() {
                    Some(value) => std::env::set_var(self.key, value),
                    None => std::env::remove_var(self.key),
                }
            }
        }
    }

    #[test]
    #[serial]
    fn loads_token_for_base_url_and_v1_variant() {
        let tempdir = tempfile::tempdir().expect("temp dir");
        let token_file = tempdir.path().join("tokens.json");
        std::fs::write(
            &token_file,
            r#"
            {
              "entries": {
                "https://api.indubitably.ai": {
                  "access_token": "token-1",
                  "expires_at": null
                }
              }
            }
            "#,
        )
        .expect("write token file");

        let _guard = EnvGuard::set(
            INDUBITABLY_AUTH_FILE_ENV_VAR,
            token_file.to_string_lossy().to_string(),
        );

        assert_eq!(
            load_access_token_for_base_url("https://api.indubitably.ai/v1"),
            Some("token-1".to_string())
        );
    }

    #[test]
    #[serial]
    fn ignores_expired_tokens() {
        let tempdir = tempfile::tempdir().expect("temp dir");
        let token_file = tempdir.path().join("tokens.json");
        let expired = Utc::now().timestamp_millis() - 10_000;
        std::fs::write(
            &token_file,
            format!(
                r#"
            {{
              "entries": {{
                "https://api.indubitably.ai": {{
                  "access_token": "token-1",
                  "expires_at": {expired}
                }}
              }}
            }}
            "#
            ),
        )
        .expect("write token file");

        let _guard = EnvGuard::set(
            INDUBITABLY_AUTH_FILE_ENV_VAR,
            token_file.to_string_lossy().to_string(),
        );

        assert_eq!(
            load_access_token_for_base_url("https://api.indubitably.ai"),
            None
        );
    }
}
