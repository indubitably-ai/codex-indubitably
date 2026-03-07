#![cfg(not(debug_assertions))]

use crate::update_action;
use crate::update_action::UpdateAction;
use chrono::DateTime;
use chrono::Duration;
use chrono::Utc;
use codex_core::config::Config;
use codex_core::default_client::create_client;
use serde::Deserialize;
use serde::Serialize;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;

use crate::version::CODEX_CLI_VERSION;

pub fn get_upgrade_version(config: &Config) -> Option<String> {
    if !config.check_for_update_on_startup {
        return None;
    }

    let version_file = version_filepath(config);
    let info = read_version_info(&version_file).ok();

    if match &info {
        None => true,
        Some(info) => info.last_checked_at < Utc::now() - Duration::hours(20),
    } {
        // Refresh the cached latest version in the background so TUI startup
        // isn’t blocked by a network call. The UI reads the previously cached
        // value (if any) for this run; the next run shows the banner if needed.
        tokio::spawn(async move {
            check_for_update(&version_file)
                .await
                .inspect_err(|e| tracing::error!("Failed to update version: {e}"))
        });
    }

    info.and_then(|info| {
        if is_newer(&info.latest_version, CODEX_CLI_VERSION).unwrap_or(false) {
            Some(info.latest_version)
        } else {
            None
        }
    })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VersionInfo {
    latest_version: String,
    // ISO-8601 timestamp (RFC3339)
    last_checked_at: DateTime<Utc>,
    #[serde(default)]
    dismissed_version: Option<String>,
}

const VERSION_FILENAME: &str = "version.json";
const INDUBITABLY_VERSION_FILENAME: &str = "indubitably-version.json";
const INDUBITABLY_COMMAND_NAME: &str = "indubitably";
const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/openai/codex/releases/latest";

#[derive(Deserialize, Debug, Clone)]
struct ReleaseInfo {
    tag_name: String,
}

#[derive(Deserialize, Debug, Clone)]
struct HomebrewCaskInfo {
    version: String,
}

#[derive(Deserialize, Debug, Clone)]
struct BrewInfo {
    casks: Vec<BrewCaskInfo>,
}

#[derive(Deserialize, Debug, Clone)]
struct BrewCaskInfo {
    version: String,
}

fn version_filepath(config: &Config) -> PathBuf {
    let filename = if is_indubitably_update_channel() {
        INDUBITABLY_VERSION_FILENAME
    } else {
        VERSION_FILENAME
    };
    config.codex_home.join(filename)
}

fn read_version_info(version_file: &Path) -> anyhow::Result<VersionInfo> {
    let contents = std::fs::read_to_string(version_file)?;
    Ok(serde_json::from_str(&contents)?)
}

async fn check_for_update(version_file: &Path) -> anyhow::Result<()> {
    let latest_version = if is_indubitably_update_channel() {
        let Some(version) = fetch_latest_indubitably_version_from_brew().await? else {
            return Ok(());
        };
        version
    } else {
        match update_action::get_update_action() {
            Some(UpdateAction::BrewUpgrade) => {
                let cask_name = update_action::brew_cask_name();
                let cask_token = cask_name.rsplit('/').next().unwrap_or(cask_name);
                let cask_api_url = format!("https://formulae.brew.sh/api/cask/{cask_token}.json");
                let HomebrewCaskInfo { version } = create_client()
                    .get(&cask_api_url)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<HomebrewCaskInfo>()
                    .await?;
                version
            }
            _ => {
                let ReleaseInfo {
                    tag_name: latest_tag_name,
                } = create_client()
                    .get(LATEST_RELEASE_URL)
                    .send()
                    .await?
                    .error_for_status()?
                    .json::<ReleaseInfo>()
                    .await?;
                extract_version_from_latest_tag(&latest_tag_name)?
            }
        }
    };

    // Preserve any previously dismissed version if present.
    let prev_info = read_version_info(version_file).ok();
    let info = VersionInfo {
        latest_version,
        last_checked_at: Utc::now(),
        dismissed_version: prev_info.and_then(|p| p.dismissed_version),
    };

    let json_line = format!("{}\n", serde_json::to_string(&info)?);
    if let Some(parent) = version_file.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(version_file, json_line).await?;
    Ok(())
}

fn is_indubitably_update_channel() -> bool {
    codex_core::util::cli_command_name().eq_ignore_ascii_case(INDUBITABLY_COMMAND_NAME)
}

async fn fetch_latest_indubitably_version_from_brew() -> anyhow::Result<Option<String>> {
    let output = match tokio::process::Command::new("brew")
        .env("HOMEBREW_NO_AUTO_UPDATE", "1")
        .env("HOMEBREW_NO_ENV_HINTS", "1")
        .args([
            "info",
            "--cask",
            "--json=v2",
            update_action::brew_cask_name(),
        ])
        .stdin(Stdio::null())
        .output()
        .await
    {
        Ok(output) => output,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(err.into()),
    };

    if !output.status.success() {
        return Ok(None);
    }

    let info: BrewInfo = serde_json::from_slice(&output.stdout)?;
    Ok(info.casks.first().map(|cask| cask.version.clone()))
}

fn is_newer(latest: &str, current: &str) -> Option<bool> {
    match (parse_version(latest), parse_version(current)) {
        (Some(l), Some(c)) => Some(l > c),
        _ => None,
    }
}

fn extract_version_from_latest_tag(latest_tag_name: &str) -> anyhow::Result<String> {
    latest_tag_name
        .strip_prefix("rust-v")
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse latest tag name '{latest_tag_name}'"))
}

/// Returns the latest version to show in a popup, if it should be shown.
/// This respects the user's dismissal choice for the current latest version.
pub fn get_upgrade_version_for_popup(config: &Config) -> Option<String> {
    if !config.check_for_update_on_startup {
        return None;
    }

    let version_file = version_filepath(config);
    let latest = get_upgrade_version(config)?;
    // If the user dismissed this exact version previously, do not show the popup.
    if let Ok(info) = read_version_info(&version_file)
        && info.dismissed_version.as_deref() == Some(latest.as_str())
    {
        return None;
    }
    Some(latest)
}

/// Persist a dismissal for the current latest version so we don't show
/// the update popup again for this version.
pub async fn dismiss_version(config: &Config, version: &str) -> anyhow::Result<()> {
    let version_file = version_filepath(config);
    let mut info = match read_version_info(&version_file) {
        Ok(info) => info,
        Err(_) => return Ok(()),
    };
    info.dismissed_version = Some(version.to_string());
    let json_line = format!("{}\n", serde_json::to_string(&info)?);
    if let Some(parent) = version_file.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(version_file, json_line).await?;
    Ok(())
}

fn parse_version(v: &str) -> Option<(u64, u64, u64)> {
    let mut iter = v.trim().split('.');
    let maj = iter.next()?.parse::<u64>().ok()?;
    let min = iter.next()?.parse::<u64>().ok()?;
    let pat = iter.next()?.parse::<u64>().ok()?;
    Some((maj, min, pat))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_version_from_brew_api_json() {
        //
        // https://formulae.brew.sh/api/cask/codex.json
        let cask_json = r#"{
            "token": "codex",
            "full_token": "codex",
            "tap": "homebrew/cask",
            "version": "0.96.0",
        }"#;
        let HomebrewCaskInfo { version } = serde_json::from_str::<HomebrewCaskInfo>(cask_json)
            .expect("failed to parse version from cask json");
        assert_eq!(version, "0.96.0");
    }

    #[test]
    fn extract_version_from_brew_info_json() {
        let cask_json = r#"{
            "casks": [
                {
                    "token": "indubitably",
                    "version": "0.1.16"
                }
            ]
        }"#;
        let info = serde_json::from_str::<BrewInfo>(cask_json).expect("failed to parse cask json");
        assert_eq!(info.casks[0].version, "0.1.16");
    }

    #[test]
    fn extracts_version_from_latest_tag() {
        assert_eq!(
            extract_version_from_latest_tag("rust-v1.5.0").expect("failed to parse version"),
            "1.5.0"
        );
    }

    #[test]
    fn latest_tag_without_prefix_is_invalid() {
        assert!(extract_version_from_latest_tag("v1.5.0").is_err());
    }

    #[test]
    fn prerelease_version_is_not_considered_newer() {
        assert_eq!(is_newer("0.11.0-beta.1", "0.11.0"), None);
        assert_eq!(is_newer("1.0.0-rc.1", "1.0.0"), None);
    }

    #[test]
    fn plain_semver_comparisons_work() {
        assert_eq!(is_newer("0.11.1", "0.11.0"), Some(true));
        assert_eq!(is_newer("0.11.0", "0.11.1"), Some(false));
        assert_eq!(is_newer("1.0.0", "0.9.9"), Some(true));
        assert_eq!(is_newer("0.9.9", "1.0.0"), Some(false));
    }

    #[test]
    fn whitespace_is_ignored() {
        assert_eq!(parse_version(" 1.2.3 \n"), Some((1, 2, 3)));
        assert_eq!(is_newer(" 1.2.3 ", "1.2.2"), Some(true));
    }
}
