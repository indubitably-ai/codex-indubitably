use std::path::Path;
use std::path::PathBuf;

use anyhow::Result;
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;
use tempfile::TempDir;

fn codex_command(codex_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(codex_utils_cargo_bin::cargo_bin("codex")?);
    cmd.env("CODEX_HOME", codex_home);
    Ok(cmd)
}

fn create_indubitably_alias() -> Result<(TempDir, PathBuf)> {
    let alias_dir = TempDir::new()?;
    let codex_bin = codex_utils_cargo_bin::cargo_bin("codex")?;

    #[cfg(unix)]
    {
        let alias_path = alias_dir.path().join("indubitably");
        std::os::unix::fs::symlink(&codex_bin, &alias_path)?;
        Ok((alias_dir, alias_path))
    }

    #[cfg(windows)]
    {
        let alias_path = alias_dir.path().join("indubitably.exe");
        std::fs::copy(&codex_bin, &alias_path)?;
        Ok((alias_dir, alias_path))
    }
}

fn indubitably_command(codex_home: &Path, alias_path: &Path) -> assert_cmd::Command {
    let mut cmd = assert_cmd::Command::new(alias_path);
    cmd.env("CODEX_HOME", codex_home);
    cmd
}

fn write_openai_only_config(codex_home: &Path) -> Result<()> {
    std::fs::create_dir_all(codex_home)?;
    std::fs::write(
        codex_home.join("config.toml"),
        "model_provider = \"openai\"\n",
    )?;
    Ok(())
}

#[test]
fn help_uses_indubitably_name_when_invoked_via_alias() -> Result<()> {
    let codex_home = TempDir::new()?;
    let (_alias_dir, alias_path) = create_indubitably_alias()?;

    indubitably_command(codex_home.path(), &alias_path)
        .args(["--help"])
        .assert()
        .success()
        .stdout(contains("Usage: indubitably"));

    Ok(())
}

#[test]
fn indubitably_login_defaults_to_indubitably_flow() -> Result<()> {
    let codex_home = TempDir::new()?;
    let (_alias_dir, alias_path) = create_indubitably_alias()?;

    indubitably_command(codex_home.path(), &alias_path)
        .args(["login", "--with-api-key"])
        .write_stdin("sk-test\n")
        .assert()
        .failure()
        .stderr(contains(
            "Indubitably login does not support device or API key flows.",
        ));

    Ok(())
}

#[test]
fn indubitably_openai_login_uses_openai_flow() -> Result<()> {
    let codex_home = TempDir::new()?;
    let (_alias_dir, alias_path) = create_indubitably_alias()?;

    indubitably_command(codex_home.path(), &alias_path)
        .args(["--openai", "login", "--with-api-key"])
        .write_stdin("sk-test\n")
        .assert()
        .success()
        .stderr(contains("Successfully logged in"));

    Ok(())
}

#[test]
fn codex_indubitably_exec_path_still_parses() -> Result<()> {
    let codex_home = TempDir::new()?;
    let mut cmd = codex_command(codex_home.path())?;
    cmd.args(["--indubitably", "exec", "--help"])
        .assert()
        .success()
        .stdout(contains("--indubitably"));

    Ok(())
}

#[test]
fn indubitably_exec_defaults_to_bedrock_provider_path() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_openai_only_config(codex_home.path())?;
    let (_alias_dir, alias_path) = create_indubitably_alias()?;

    indubitably_command(codex_home.path(), &alias_path)
        .args(["exec", "--skip-git-repo-check", "health check"])
        .assert()
        .failure()
        .stderr(contains("indubitably authentication expired"))
        .stderr(contains("Bedrock runtime adapter is not configured").not());

    Ok(())
}

#[test]
fn indubitably_openai_exec_overrides_implicit_bedrock_default() -> Result<()> {
    let codex_home = TempDir::new()?;
    write_openai_only_config(codex_home.path())?;
    let (_alias_dir, alias_path) = create_indubitably_alias()?;

    indubitably_command(codex_home.path(), &alias_path)
        .args(["--openai", "exec", "--skip-git-repo-check", "health check"])
        .assert()
        .failure()
        .stderr(contains("Bedrock runtime adapter is not configured").not());

    Ok(())
}
