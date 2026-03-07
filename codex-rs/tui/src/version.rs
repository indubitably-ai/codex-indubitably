/// The current Codex CLI version as embedded at compile time.
///
/// Release packaging can inject `INDUBITABLY_CLI_VERSION` so user-visible
/// version strings don't inherit the workspace placeholder (`0.0.0`).
pub const CODEX_CLI_VERSION: &str = match option_env!("INDUBITABLY_CLI_VERSION") {
    Some(version) => version,
    None => env!("CARGO_PKG_VERSION"),
};
