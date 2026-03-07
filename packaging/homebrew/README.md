# Homebrew Packaging Helpers

This directory contains fork-owned helpers for publishing the `indubitably` Homebrew cask without depending on `../indubitably-codex`.

## Layout

- `Casks/indubitably.rb.template`: source template for cask generation.
- `scripts/build_macos_homebrew_artifacts.py`: builds macOS artifacts and packages `indubitably-<target>.tar.gz` files from this repo.
- `scripts/generate_homebrew_cask.py`: renders the cask from version/base URL/SHA values.
- `scripts/release_checklist.py`: end-to-end local checklist (artifact validation + smoke checks + cask rendering).

## Build artifacts for Homebrew

```sh
./packaging/homebrew/scripts/build_macos_homebrew_artifacts.py \
  --codesign-identity "Developer ID Application: <TEAM/NAME>" \
  --notarytool-profile <NOTARY_PROFILE>
```

Expected outputs:

- `codex-rs/dist/aarch64-apple-darwin/indubitably-aarch64-apple-darwin.tar.gz`
- `codex-rs/dist/x86_64-apple-darwin/indubitably-x86_64-apple-darwin.tar.gz`

## Generate a cask file

```sh
./packaging/homebrew/scripts/generate_homebrew_cask.py \
  --version <VERSION> \
  --base-url https://downloads.indubitably.ai/cli/indubitably \
  --arm-tar codex-rs/dist/aarch64-apple-darwin/indubitably-aarch64-apple-darwin.tar.gz \
  --intel-tar codex-rs/dist/x86_64-apple-darwin/indubitably-x86_64-apple-darwin.tar.gz
```

## Full local release checklist

```sh
./packaging/homebrew/scripts/release_checklist.py \
  --version <VERSION> \
  --base-url https://downloads.indubitably.ai/cli/indubitably
```

This validates:

- tarball structure for both macOS architectures,
- `indubitably exec` default bedrock routing,
- `indubitably --openai exec` override behavior,
- `codex --indubitably exec` compatibility,
- generated cask content,
- real Homebrew install behavior via a temporary tap (`brew install --cask <temp-tap>/indubitably`),
- `indubitably --version` and `indubitably --help` after Homebrew install,
- cleanup (`brew uninstall`) and forced temporary tap removal,
- restoration of any prior `indubitably` tap/version state.

To skip the tap-based Homebrew install smoke:

```sh
./packaging/homebrew/scripts/release_checklist.py \
  --version <VERSION> \
  --base-url https://downloads.indubitably.ai/cli/indubitably \
  --skip-brew-install-smoke
```

For local-only unsigned artifacts (not for release publishing), pass `--unsigned`.
The default signed/notarized flow also runs a `spctl` Gatekeeper assessment on each built binary.
