#!/usr/bin/env python3
"""Run local release checks for Homebrew tap publishing.

This script validates:
1) macOS Homebrew artifact generation for both architectures,
2) tarball structure and checksums,
3) invocation smoke checks for `indubitably` and `codex --indubitably`,
4) generated Homebrew cask output,
5) tap-based `brew install --cask` behavior for the generated cask.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import shutil
import subprocess
import tarfile
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
CODEX_RS_ROOT = REPO_ROOT / "codex-rs"
DIST_ROOT = CODEX_RS_ROOT / "dist"
MAC_TARGETS = ("aarch64-apple-darwin", "x86_64-apple-darwin")
DEFAULT_BASE_URL = "https://downloads.indubitably.ai/cli/indubitably"
BREW_TIMEOUT_SEC = 60
CLI_TIMEOUT_SEC = 15
HOMEBREW_ENV_UPDATES = {"HOMEBREW_NO_AUTO_UPDATE": "1"}


class ChecklistError(RuntimeError):
    pass


@dataclass(frozen=True)
class Artifact:
    target: str
    tar_gz: Path


@dataclass(frozen=True)
class InstalledCaskState:
    tap: str
    version: str
    receipt_path: Path


def run(
    cmd: list[str],
    *,
    cwd: Path,
    check: bool = True,
    timeout_sec: int | None = None,
    env_updates: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    print("+", " ".join(cmd))
    env = os.environ.copy()
    if env_updates:
        env.update(env_updates)

    try:
        result = subprocess.run(
            cmd,
            cwd=cwd,
            text=True,
            capture_output=True,
            check=False,
            timeout=timeout_sec,
            env=env,
        )
    except subprocess.TimeoutExpired as exc:
        raise ChecklistError(
            f"Command timed out after {timeout_sec}s: {' '.join(cmd)}"
        ) from exc

    if check and result.returncode != 0:
        stderr = (result.stderr or "").strip()
        stdout = (result.stdout or "").strip()
        details = stderr if stderr else stdout
        raise ChecklistError(
            f"Command failed ({result.returncode}): {' '.join(cmd)}\n{details}"
        )

    return result


def run_brew(args: list[str], *, check: bool = True) -> subprocess.CompletedProcess[str]:
    return run(
        ["brew", *args],
        cwd=REPO_ROOT,
        check=check,
        timeout_sec=BREW_TIMEOUT_SEC,
        env_updates=HOMEBREW_ENV_UPDATES,
    )


def check_tool(name: str) -> None:
    if shutil.which(name) is None:
        raise ChecklistError(f"Missing required tool: {name}")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True, help="Release version (for example 0.55.0)")
    parser.add_argument("--base-url", default=DEFAULT_BASE_URL, help="Base URL for artifact hosting")
    parser.add_argument("--skip-build", action="store_true", help="Skip build step")
    parser.add_argument(
        "--skip-cli-smoke",
        action="store_true",
        help="Skip CLI smoke checks (artifact/cask checks still run)",
    )
    parser.add_argument(
        "--skip-brew-install-smoke",
        action="store_true",
        help="Skip tap-based `brew install --cask` smoke check",
    )
    parser.add_argument("--unsigned", action="store_true", help="Pass through to build script")
    parser.add_argument(
        "--codesign-identity",
        default=None,
        help="Pass through to build script",
    )
    parser.add_argument(
        "--notarytool-profile",
        default=None,
        help="Pass through to build script (required unless --unsigned/--skip-notarize)",
    )
    parser.add_argument(
        "--skip-notarize",
        action="store_true",
        help="Pass through to build script (codesign only, non-release use)",
    )
    return parser.parse_args()


def expected_tarball(target: str) -> Path:
    return DIST_ROOT / target / f"indubitably-{target}.tar.gz"


def validate_tarball(target: str, tar_gz: Path) -> None:
    expected_entry = f"indubitably-{target}"
    if not tar_gz.is_file():
        raise ChecklistError(f"Missing tarball: {tar_gz}")

    with tarfile.open(tar_gz, "r:gz") as tf:
        entries = [member for member in tf.getmembers() if member.isfile()]
        if len(entries) != 1:
            names = ", ".join(entry.name for entry in entries)
            raise ChecklistError(
                f"Expected exactly one file in {tar_gz}, found {len(entries)}: {names}"
            )
        entry = entries[0]
        if entry.name != expected_entry:
            raise ChecklistError(f"Expected tar entry {expected_entry}, found {entry.name}")
        if (entry.mode or 0) & 0o111 == 0:
            raise ChecklistError(
                f"Tar entry {entry.name} is not executable (mode {entry.mode:o})"
            )


def native_target() -> str:
    machine = platform.machine().lower()
    if machine in {"arm64", "aarch64"}:
        return "aarch64-apple-darwin"
    if machine == "x86_64":
        return "x86_64-apple-darwin"
    raise ChecklistError(f"Unsupported macOS architecture: {machine}")


def write_minimal_config(codex_home: Path) -> None:
    codex_home.mkdir(parents=True, exist_ok=True)
    (codex_home / "config.toml").write_text('model_provider = "openai"\n', encoding="utf-8")


def run_cli(binary: Path, args: list[str], codex_home: Path) -> subprocess.CompletedProcess[str]:
    env = {"CODEX_HOME": str(codex_home)}
    return subprocess.run(
        [str(binary), *args],
        text=True,
        capture_output=True,
        check=False,
        env={**os.environ, **env},
    )


def run_cli_smoke_checks(codex_binary: Path) -> None:
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        codex_home = temp_path / "codex-home"
        write_minimal_config(codex_home)

        alias_path = temp_path / "indubitably"
        alias_path.symlink_to(codex_binary)

        bedrock_default = run_cli(
            alias_path,
            ["exec", "--skip-git-repo-check", "health check"],
            codex_home,
        )
        if bedrock_default.returncode == 0:
            raise ChecklistError("Expected indubitably exec smoke command to fail in local checklist")
        bedrock_stderr = (bedrock_default.stderr or "").lower()
        if "bedrock runtime adapter is not configured" not in bedrock_stderr:
            raise ChecklistError(
                "Expected `indubitably exec` to route to bedrock path by default "
                "(missing bedrock adapter message in stderr)."
            )

        openai_override = run_cli(
            alias_path,
            ["--openai", "exec", "--skip-git-repo-check", "health check"],
            codex_home,
        )
        if openai_override.returncode == 0:
            raise ChecklistError(
                "Expected `indubitably --openai exec` smoke command to fail without auth"
            )
        openai_stderr = (openai_override.stderr or "").lower()
        if "bedrock runtime adapter is not configured" in openai_stderr:
            raise ChecklistError(
                "Expected `indubitably --openai exec` to avoid bedrock default path"
            )

        legacy_codex = run_cli(
            codex_binary,
            ["--indubitably", "exec", "--skip-git-repo-check", "health check"],
            codex_home,
        )
        if legacy_codex.returncode == 0:
            raise ChecklistError("Expected `codex --indubitably exec` smoke command to fail")
        legacy_stderr = (legacy_codex.stderr or "").lower()
        if "bedrock runtime adapter is not configured" not in legacy_stderr:
            raise ChecklistError(
                "Expected `codex --indubitably exec` to route to bedrock path"
            )


def generate_cask(version: str, base_url: str, arm_tar: Path, intel_tar: Path) -> str:
    cmd = [
        str(REPO_ROOT / "packaging" / "homebrew" / "scripts" / "generate_homebrew_cask.py"),
        "--version",
        version,
        "--base-url",
        base_url,
        "--arm-tar",
        str(arm_tar),
        "--intel-tar",
        str(intel_tar),
    ]
    return run(cmd, cwd=REPO_ROOT).stdout


def detect_existing_install_state() -> InstalledCaskState | None:
    brew_prefix = run_brew(["--prefix"]).stdout.strip()
    receipt_path = (
        Path(brew_prefix)
        / "Caskroom"
        / "indubitably"
        / ".metadata"
        / "INSTALL_RECEIPT.json"
    )

    if not receipt_path.is_file():
        return None

    try:
        data = json.loads(receipt_path.read_text(encoding="utf-8"))
    except json.JSONDecodeError as exc:
        raise ChecklistError(
            f"Could not parse Homebrew install receipt: {receipt_path}"
        ) from exc

    source = data.get("source")
    if not isinstance(source, dict):
        raise ChecklistError(
            f"Homebrew install receipt missing source metadata: {receipt_path}"
        )

    tap = source.get("tap")
    version = source.get("version")
    if not isinstance(tap, str) or not tap:
        raise ChecklistError(
            f"Homebrew install receipt missing source.tap: {receipt_path}"
        )
    if not isinstance(version, str) or not version:
        raise ChecklistError(
            f"Homebrew install receipt missing source.version: {receipt_path}"
        )

    return InstalledCaskState(tap=tap, version=version, receipt_path=receipt_path)


def create_temporary_tap_with_cask(cask: str) -> tuple[str, Path]:
    tap_name = f"indubitably/smoke-{int(time.time())}-{os.getpid()}"
    run_brew(["tap-new", tap_name])

    org, repo = tap_name.split("/", maxsplit=1)
    brew_repository = Path(run_brew(["--repository"]).stdout.strip())
    tap_dir = brew_repository / "Library" / "Taps" / org / f"homebrew-{repo}"
    casks_dir = tap_dir / "Casks"
    casks_dir.mkdir(parents=True, exist_ok=True)

    cask_path = casks_dir / "indubitably.rb"
    cask_path.write_text(cask, encoding="utf-8")

    return tap_name, cask_path


def clear_binary_quarantine(binary_path: Path) -> None:
    result = run(
        ["xattr", "-d", "com.apple.quarantine", str(binary_path)],
        cwd=REPO_ROOT,
        check=False,
        timeout_sec=CLI_TIMEOUT_SEC,
    )
    stderr = (result.stderr or "").strip()
    if result.returncode != 0 and "No such xattr" not in stderr:
        raise ChecklistError(
            f"Could not clear quarantine from installed binary {binary_path}: {stderr}"
        )


def clear_binary_xattrs(binary_path: Path) -> None:
    result = run(
        ["xattr", "-c", str(binary_path)],
        cwd=REPO_ROOT,
        check=False,
        timeout_sec=CLI_TIMEOUT_SEC,
    )
    stderr = (result.stderr or "").strip()
    if result.returncode != 0 and "Operation not permitted" not in stderr:
        raise ChecklistError(
            f"Could not clear xattrs from smoke binary {binary_path}: {stderr}"
        )


def run_brew_install_smoke(tap_name: str, *, unsigned: bool) -> None:
    run_brew(["install", "--cask", f"{tap_name}/indubitably"])

    indubitably_path = shutil.which("indubitably")
    if indubitably_path is None:
        raise ChecklistError("`indubitably` is not on PATH after Homebrew cask installation")
    indubitably_binary = Path(indubitably_path).resolve()
    smoke_binary: str = "indubitably"

    if unsigned:
        # For local unsigned artifacts, Homebrew applies execution policy metadata in Caskroom.
        # Verify the installed link, then execute a scrubbed temporary copy for deterministic checks.
        clear_binary_quarantine(indubitably_binary)
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_binary = Path(temp_dir) / "indubitably-smoke"
            shutil.copy2(indubitably_binary, temp_binary)
            clear_binary_xattrs(temp_binary)
            temp_binary.chmod(temp_binary.stat().st_mode | 0o111)
            smoke_binary = str(temp_binary)

            version_output = run(
                [smoke_binary, "--version"],
                cwd=REPO_ROOT,
                timeout_sec=CLI_TIMEOUT_SEC,
            ).stdout.strip()
            if not version_output:
                raise ChecklistError("`indubitably --version` returned empty output")

            help_output = run(
                [smoke_binary, "--help"],
                cwd=REPO_ROOT,
                timeout_sec=CLI_TIMEOUT_SEC,
            ).stdout
            if "Usage: indubitably" not in help_output:
                raise ChecklistError("`indubitably --help` did not contain expected usage text")
        return

    version_output = run(
        [smoke_binary, "--version"],
        cwd=REPO_ROOT,
        timeout_sec=CLI_TIMEOUT_SEC,
    ).stdout.strip()
    if not version_output:
        raise ChecklistError("`indubitably --version` returned empty output")

    help_output = run(
        [smoke_binary, "--help"],
        cwd=REPO_ROOT,
        timeout_sec=CLI_TIMEOUT_SEC,
    ).stdout
    if "Usage: indubitably" not in help_output:
        raise ChecklistError("`indubitably --help` did not contain expected usage text")


def cleanup_brew_smoke(
    *,
    tap_name: str,
    previous_install: InstalledCaskState | None,
) -> None:
    errors: list[str] = []

    uninstall_result = run_brew(["uninstall", "--cask", f"{tap_name}/indubitably"], check=False)
    uninstall_stderr = (uninstall_result.stderr or "").strip()
    if uninstall_result.returncode != 0 and (
        "is not installed" not in uninstall_stderr
        and "is unavailable" not in uninstall_stderr
    ):
        stderr = (uninstall_result.stderr or "").strip()
        errors.append(
            f"Could not uninstall test cask `{tap_name}/indubitably`: {stderr}"
        )

    if previous_install is not None:
        reinstall_result = run_brew(
            ["install", "--cask", f"{previous_install.tap}/indubitably"],
            check=False,
        )
        if reinstall_result.returncode != 0:
            stderr = (reinstall_result.stderr or "").strip()
            errors.append(
                f"Could not restore previous install `{previous_install.tap}/indubitably`: {stderr}"
            )
        else:
            info_result = run_brew(
                ["info", "--cask", f"{previous_install.tap}/indubitably"],
                check=False,
            )
            if info_result.returncode != 0 or previous_install.version not in info_result.stdout:
                stderr = (info_result.stderr or "").strip()
                errors.append(
                    "Restored cask version could not be verified "
                    f"(expected {previous_install.version} from {previous_install.tap}): {stderr}"
                )

    untap_result = run_brew(["untap", "--force", tap_name], check=False)
    untap_stderr = (untap_result.stderr or "").strip()
    if untap_result.returncode != 0 and (
        "No such keg" not in untap_stderr and "No such tap" not in untap_stderr
    ):
        stderr = (untap_result.stderr or "").strip()
        errors.append(f"Could not remove temporary tap `{tap_name}`: {stderr}")

    if errors:
        recovery = ""
        if previous_install is not None:
            recovery = (
                "\nManual recovery:\n"
                f"  brew install --cask {previous_install.tap}/indubitably\n"
                f"  brew untap --force {tap_name}"
            )
        raise ChecklistError("\n".join(errors) + recovery)


def main() -> int:
    args = parse_args()

    check_tool("cargo")
    check_tool("python3")
    if not args.skip_brew_install_smoke:
        check_tool("brew")

    if not args.skip_build:
        build_cmd = [
            str(REPO_ROOT / "packaging" / "homebrew" / "scripts" / "build_macos_homebrew_artifacts.py"),
        ]
        if args.unsigned:
            build_cmd.append("--unsigned")
        if args.codesign_identity:
            build_cmd.extend(["--codesign-identity", args.codesign_identity])
        if args.notarytool_profile:
            build_cmd.extend(["--notarytool-profile", args.notarytool_profile])
        if args.skip_notarize:
            build_cmd.append("--skip-notarize")
        run(build_cmd, cwd=REPO_ROOT)

    artifacts: list[Artifact] = []
    for target in MAC_TARGETS:
        tar_gz = expected_tarball(target)
        validate_tarball(target, tar_gz)
        artifacts.append(Artifact(target=target, tar_gz=tar_gz))

    if not args.skip_cli_smoke:
        target = native_target()
        codex_binary = CODEX_RS_ROOT / "target" / target / "release" / "codex"
        if not codex_binary.is_file():
            raise ChecklistError(
                f"Native codex binary not found for smoke checks: {codex_binary}"
            )
        run_cli_smoke_checks(codex_binary)

    arm_tar = next(item.tar_gz for item in artifacts if item.target == "aarch64-apple-darwin")
    intel_tar = next(item.tar_gz for item in artifacts if item.target == "x86_64-apple-darwin")

    cask = generate_cask(args.version, args.base_url, arm_tar, intel_tar)

    if not args.skip_brew_install_smoke:
        previous_install = detect_existing_install_state()
        tap_name, _cask_path = create_temporary_tap_with_cask(cask)

        smoke_error: Exception | None = None
        try:
            run_brew_install_smoke(tap_name, unsigned=args.unsigned)
        except Exception as err:  # propagate after cleanup
            smoke_error = err

        cleanup_error: Exception | None = None
        try:
            cleanup_brew_smoke(tap_name=tap_name, previous_install=previous_install)
        except Exception as err:
            cleanup_error = err

        if cleanup_error is not None and smoke_error is not None:
            raise ChecklistError(f"{smoke_error}\nCleanup failed: {cleanup_error}")
        if cleanup_error is not None:
            raise cleanup_error
        if smoke_error is not None:
            raise smoke_error

    print("\nArtifacts:")
    for artifact in artifacts:
        print(f"  {artifact.tar_gz}")

    print("\nGenerated cask:")
    print(cask.rstrip())

    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ChecklistError as error:
        print(f"ERROR: {error}")
        raise SystemExit(1)
