const OPENAI_NPM_PACKAGE_NAME: &str = "@openai/codex";
const OPENAI_BREW_CASK_NAME: &str = "codex";
const INDUBITABLY_BREW_CASK_NAME: &str = "indubitably-ai/tap/indubitably";
const INDUBITABLY_COMMAND_NAME: &str = "indubitably";
const OPENAI_RELEASE_NOTES_URL: &str = "https://github.com/openai/codex/releases/latest";
const OPENAI_INSTALL_OPTIONS_URL: &str = "https://github.com/openai/codex";
const INDUBITABLY_RELEASE_NOTES_URL: &str = "https://github.com/indubitably-ai/homebrew-tap";
const INDUBITABLY_INSTALL_OPTIONS_URL: &str = "https://github.com/indubitably-ai/homebrew-tap";

/// Update action the CLI should perform after the TUI exits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateAction {
    /// Update via `npm install -g @openai/codex@latest`.
    NpmGlobalLatest,
    /// Update via `bun install -g @openai/codex@latest`.
    BunGlobalLatest,
    /// Update via `brew upgrade --cask <cask>`.
    BrewUpgrade,
}

impl UpdateAction {
    /// Returns the list of command-line arguments for invoking the update.
    pub fn command_args(self) -> (&'static str, &'static [&'static str]) {
        let command_name = codex_core::util::cli_command_name();
        self.command_args_for_command_name(&command_name)
    }

    fn command_args_for_command_name(
        self,
        command_name: &str,
    ) -> (&'static str, &'static [&'static str]) {
        match self {
            UpdateAction::NpmGlobalLatest => ("npm", &["install", "-g", OPENAI_NPM_PACKAGE_NAME]),
            UpdateAction::BunGlobalLatest => ("bun", &["install", "-g", OPENAI_NPM_PACKAGE_NAME]),
            UpdateAction::BrewUpgrade => {
                if uses_indubitably_update_channel(command_name) {
                    ("brew", &["upgrade", "--cask", INDUBITABLY_BREW_CASK_NAME])
                } else {
                    ("brew", &["upgrade", "--cask", OPENAI_BREW_CASK_NAME])
                }
            }
        }
    }

    /// Returns string representation of the command-line arguments for invoking the update.
    pub fn command_str(self) -> String {
        let (command, args) = self.command_args();
        shlex::try_join(std::iter::once(command).chain(args.iter().copied()))
            .unwrap_or_else(|_| format!("{command} {}", args.join(" ")))
    }
}

pub fn brew_cask_name() -> &'static str {
    let command_name = codex_core::util::cli_command_name();
    brew_cask_name_for_command_name(&command_name)
}

pub fn release_notes_url() -> &'static str {
    let command_name = codex_core::util::cli_command_name();
    release_notes_url_for_command_name(&command_name)
}

pub fn install_options_url() -> &'static str {
    let command_name = codex_core::util::cli_command_name();
    install_options_url_for_command_name(&command_name)
}

fn uses_indubitably_update_channel(command_name: &str) -> bool {
    command_name.eq_ignore_ascii_case(INDUBITABLY_COMMAND_NAME)
}

fn brew_cask_name_for_command_name(command_name: &str) -> &'static str {
    if uses_indubitably_update_channel(command_name) {
        INDUBITABLY_BREW_CASK_NAME
    } else {
        OPENAI_BREW_CASK_NAME
    }
}

fn release_notes_url_for_command_name(command_name: &str) -> &'static str {
    if uses_indubitably_update_channel(command_name) {
        INDUBITABLY_RELEASE_NOTES_URL
    } else {
        OPENAI_RELEASE_NOTES_URL
    }
}

fn install_options_url_for_command_name(command_name: &str) -> &'static str {
    if uses_indubitably_update_channel(command_name) {
        INDUBITABLY_INSTALL_OPTIONS_URL
    } else {
        OPENAI_INSTALL_OPTIONS_URL
    }
}

#[cfg(not(debug_assertions))]
pub(crate) fn get_update_action() -> Option<UpdateAction> {
    let exe = std::env::current_exe().unwrap_or_default();
    let managed_by_npm = std::env::var_os("CODEX_MANAGED_BY_NPM").is_some();
    let managed_by_bun = std::env::var_os("CODEX_MANAGED_BY_BUN").is_some();

    detect_update_action(
        cfg!(target_os = "macos"),
        &exe,
        managed_by_npm,
        managed_by_bun,
    )
}

#[cfg(any(not(debug_assertions), test))]
fn detect_update_action(
    is_macos: bool,
    current_exe: &std::path::Path,
    managed_by_npm: bool,
    managed_by_bun: bool,
) -> Option<UpdateAction> {
    if managed_by_npm {
        Some(UpdateAction::NpmGlobalLatest)
    } else if managed_by_bun {
        Some(UpdateAction::BunGlobalLatest)
    } else if is_macos
        && (current_exe.starts_with("/opt/homebrew") || current_exe.starts_with("/usr/local"))
    {
        Some(UpdateAction::BrewUpgrade)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_update_action_without_env_mutation() {
        assert_eq!(
            detect_update_action(false, std::path::Path::new("/any/path"), false, false),
            None
        );
        assert_eq!(
            detect_update_action(false, std::path::Path::new("/any/path"), true, false),
            Some(UpdateAction::NpmGlobalLatest)
        );
        assert_eq!(
            detect_update_action(false, std::path::Path::new("/any/path"), false, true),
            Some(UpdateAction::BunGlobalLatest)
        );
        assert_eq!(
            detect_update_action(
                true,
                std::path::Path::new("/opt/homebrew/bin/codex"),
                false,
                false
            ),
            Some(UpdateAction::BrewUpgrade)
        );
        assert_eq!(
            detect_update_action(
                true,
                std::path::Path::new("/usr/local/bin/codex"),
                false,
                false
            ),
            Some(UpdateAction::BrewUpgrade)
        );
    }

    #[test]
    fn brew_update_command_changes_for_indubitably_invocation() {
        assert_eq!(
            UpdateAction::BrewUpgrade.command_args_for_command_name("indubitably"),
            (
                "brew",
                &["upgrade", "--cask", "indubitably-ai/tap/indubitably"][..]
            )
        );
        assert_eq!(
            UpdateAction::BrewUpgrade.command_args_for_command_name("codex"),
            ("brew", &["upgrade", "--cask", "codex"][..])
        );
    }

    #[test]
    fn release_urls_change_for_indubitably_invocation() {
        assert_eq!(
            release_notes_url_for_command_name("indubitably"),
            "https://github.com/indubitably-ai/homebrew-tap"
        );
        assert_eq!(
            install_options_url_for_command_name("indubitably"),
            "https://github.com/indubitably-ai/homebrew-tap"
        );
        assert_eq!(
            release_notes_url_for_command_name("codex"),
            "https://github.com/openai/codex/releases/latest"
        );
        assert_eq!(
            install_options_url_for_command_name("codex"),
            "https://github.com/openai/codex"
        );
    }
}
