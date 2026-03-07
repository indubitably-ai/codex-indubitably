#!/usr/bin/env python3
"""Build macOS Homebrew artifacts for this fork.

Outputs:
  codex-rs/dist/aarch64-apple-darwin/indubitably-aarch64-apple-darwin.tar.gz
  codex-rs/dist/x86_64-apple-darwin/indubitably-x86_64-apple-darwin.tar.gz

Each tarball contains one executable named indubitably-<target>.

By default this script signs and notarizes each binary. Pass --unsigned for
local-only unsigned artifacts, or --skip-notarize to sign without notarization.
"""

from __future__ import annotations

import argparse
import hashlib
import os
import shutil
import subprocess
import tarfile
import tempfile
import zipfile
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
CODEX_RS_ROOT = REPO_ROOT / "codex-rs"
DIST_ROOT = CODEX_RS_ROOT / "dist"
SOURCE_BIN_NAME = "codex"
ARTIFACT_BIN_NAME = "indubitably"
MAC_TARGETS = ("aarch64-apple-darwin", "x86_64-apple-darwin")


@dataclass(frozen=True)
class BuiltArtifact:
    target: str
    tar_gz: Path
    sha256: str


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--cli-version",
        default=os.environ.get("INDUBITABLY_CLI_VERSION"),
        help=(
            "Optional version embedded in the binary for `indubitably --version` "
            "(sets INDUBITABLY_CLI_VERSION at compile time)"
        ),
    )
    parser.add_argument("--skip-build", action="store_true", help="Skip cargo build steps")
    parser.add_argument(
        "--unsigned",
        action="store_true",
        help="Skip codesigning and notarization (local validation only)",
    )
    parser.add_argument(
        "--codesign-identity",
        default=os.environ.get("INDUBITABLY_CODESIGN_IDENTITY"),
        help="Developer ID Application identity",
    )
    parser.add_argument(
        "--notarytool-profile",
        default=os.environ.get("INDUBITABLY_NOTARYTOOL_PROFILE"),
        help="notarytool keychain profile name (required unless --unsigned/--skip-notarize)",
    )
    parser.add_argument(
        "--skip-notarize",
        action="store_true",
        help="Skip notarization (codesign only; release-quality flow should not use this)",
    )
    return parser.parse_args()


def run(cmd: list[str], *, cwd: Path, env_updates: dict[str, str] | None = None) -> None:
    print("+", " ".join(cmd))
    env = os.environ.copy()
    if env_updates:
        env.update(env_updates)
    subprocess.run(cmd, cwd=cwd, check=True, env=env)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def build_target(target: str, cli_version: str | None) -> None:
    env_updates: dict[str, str] | None = None
    if cli_version is not None:
        env_updates = {"INDUBITABLY_CLI_VERSION": cli_version}
    run(
        [
            "cargo",
            "build",
            "--release",
            "--bin",
            SOURCE_BIN_NAME,
            "--target",
            target,
            "-p",
            "codex-cli",
        ],
        cwd=CODEX_RS_ROOT,
        env_updates=env_updates,
    )


def source_binary_for_target(target: str) -> Path:
    return CODEX_RS_ROOT / "target" / target / "release" / SOURCE_BIN_NAME


def codesign_binary(exe_path: Path, identity: str) -> None:
    run(
        [
            "codesign",
            "--force",
            "--options",
            "runtime",
            "--timestamp",
            "--sign",
            identity,
            str(exe_path),
        ],
        cwd=CODEX_RS_ROOT,
    )
    run(["codesign", "--verify", "--strict", "--verbose=2", str(exe_path)], cwd=CODEX_RS_ROOT)


def notarize_binary(exe_path: Path, profile: str) -> None:
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)
        zip_path = temp_path / f"{exe_path.name}.zip"
        with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED) as zf:
            zf.write(exe_path, arcname=exe_path.name)

        run(
            [
                "xcrun",
                "notarytool",
                "submit",
                str(zip_path),
                "--keychain-profile",
                profile,
                "--wait",
            ],
            cwd=CODEX_RS_ROOT,
        )


def assess_binary_for_gatekeeper(exe_path: Path) -> None:
    cmd = ["spctl", "--assess", "--type", "execute", "--verbose=4", str(exe_path)]
    print("+", " ".join(cmd))
    result = subprocess.run(
        cmd,
        cwd=CODEX_RS_ROOT,
        check=False,
        text=True,
        capture_output=True,
    )
    if result.stdout:
        print(result.stdout.rstrip())
    if result.stderr:
        print(result.stderr.rstrip())
    if result.returncode == 0:
        return

    combined = f"{result.stdout}\n{result.stderr}".lower()
    if "does not seem to be an app" in combined:
        print(
            f"warning: spctl reported a non-app executable for {exe_path.name}; "
            "continuing because notarization was accepted"
        )
        return

    raise subprocess.CalledProcessError(result.returncode, cmd)


def ensure_tool(name: str) -> None:
    if shutil.which(name) is None:
        raise RuntimeError(f"Missing required tool: {name}")


def package_target(target: str) -> BuiltArtifact:
    source_bin = source_binary_for_target(target)
    if not source_bin.is_file():
        raise FileNotFoundError(f"Built binary not found: {source_bin}")

    stage_dir = DIST_ROOT / target
    stage_dir.mkdir(parents=True, exist_ok=True)

    artifact_name = f"{ARTIFACT_BIN_NAME}-{target}"
    staged_bin = stage_dir / artifact_name
    shutil.copy2(source_bin, staged_bin)

    tar_gz = stage_dir / f"{artifact_name}.tar.gz"
    if tar_gz.exists():
        tar_gz.unlink()

    with tarfile.open(tar_gz, "w:gz") as tar_file:
        tar_file.add(staged_bin, arcname=artifact_name)

    return BuiltArtifact(target=target, tar_gz=tar_gz, sha256=sha256_file(tar_gz))


def main() -> int:
    args = parse_args()

    if args.unsigned:
        if args.notarytool_profile:
            raise ValueError("--notarytool-profile is incompatible with --unsigned")
        if args.skip_notarize:
            raise ValueError("--skip-notarize is incompatible with --unsigned")
    else:
        if not args.codesign_identity:
            raise ValueError(
                "codesigning is required by default; provide --codesign-identity or pass --unsigned"
            )
        if not args.skip_notarize and not args.notarytool_profile:
            raise ValueError(
                "notarization is required by default; provide --notarytool-profile "
                "(or set INDUBITABLY_NOTARYTOOL_PROFILE), or pass --skip-notarize for non-release builds"
            )

    ensure_tool("cargo")
    if not args.unsigned:
        ensure_tool("codesign")
        if not args.skip_notarize:
            ensure_tool("xcrun")
            ensure_tool("spctl")

    artifacts: list[BuiltArtifact] = []
    for target in MAC_TARGETS:
        if not args.skip_build:
            build_target(target, args.cli_version)

        source_bin = source_binary_for_target(target)
        if not args.unsigned:
            codesign_binary(source_bin, args.codesign_identity)
            if not args.skip_notarize:
                notarize_binary(source_bin, args.notarytool_profile)
                assess_binary_for_gatekeeper(source_bin)

        artifacts.append(package_target(target))

    print("\nArtifacts:")
    for artifact in artifacts:
        print(f"  {artifact.tar_gz}")

    print("\nSHA256:")
    for artifact in artifacts:
        print(f"  {artifact.target}: {artifact.sha256}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
