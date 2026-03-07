#!/usr/bin/env python3
"""Generate a Homebrew cask from the repository template.

Example:
  ./packaging/homebrew/scripts/generate_homebrew_cask.py \
    --version 0.55.0 \
    --base-url https://downloads.indubitably.ai/cli/indubitably \
    --arm-tar codex-rs/dist/aarch64-apple-darwin/indubitably-aarch64-apple-darwin.tar.gz \
    --intel-tar codex-rs/dist/x86_64-apple-darwin/indubitably-x86_64-apple-darwin.tar.gz
"""

from __future__ import annotations

import argparse
import hashlib
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[3]
TEMPLATE_PATH = REPO_ROOT / "packaging" / "homebrew" / "Casks" / "indubitably.rb.template"


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True, help="Release version (for example 0.55.0)")
    parser.add_argument(
        "--base-url",
        default="https://downloads.indubitably.ai/cli/indubitably",
        help="Base URL for release artifacts (no trailing slash)",
    )

    arm_group = parser.add_mutually_exclusive_group(required=True)
    arm_group.add_argument("--sha-arm", help="SHA256 for ARM tarball")
    arm_group.add_argument("--arm-tar", type=Path, help="Path to ARM tarball")

    intel_group = parser.add_mutually_exclusive_group(required=True)
    intel_group.add_argument("--sha-intel", help="SHA256 for Intel tarball")
    intel_group.add_argument("--intel-tar", type=Path, help="Path to Intel tarball")

    return parser.parse_args()


def resolve_sha(path: Path | None, explicit: str | None, flag: str) -> str:
    if explicit is not None:
        return explicit
    if path is None or not path.is_file():
        raise FileNotFoundError(f"{flag} not found: {path}")
    return sha256_file(path)


def render(template: str, *, version: str, base_url: str, sha_arm: str, sha_intel: str) -> str:
    return (
        template.replace("{{VERSION}}", version)
        .replace("{{BASE_URL}}", base_url.rstrip("/"))
        .replace("{{SHA_ARM}}", sha_arm)
        .replace("{{SHA_INTEL}}", sha_intel)
    )


def main() -> int:
    args = parse_args()
    template = TEMPLATE_PATH.read_text(encoding="utf-8")

    sha_arm = resolve_sha(args.arm_tar, args.sha_arm, "--arm-tar")
    sha_intel = resolve_sha(args.intel_tar, args.sha_intel, "--intel-tar")

    sys.stdout.write(
        render(
            template,
            version=args.version,
            base_url=args.base_url,
            sha_arm=sha_arm,
            sha_intel=sha_intel,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
