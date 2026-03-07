# Bedrock/Indubitably Smoke Tests (Fork-Specific)

This checklist validates OpenAI baseline behavior, Indubitably login/proxy behavior, invocation defaults, and current ECS headless constraints.

## Preconditions

- Build CLI: `cd codex-rs && cargo build -p codex-cli`
- Use repo binary first in PATH or run via `cargo run -p codex-cli -- ...`.

## 1) OpenAI Provider Baseline (`codex`)

```sh
codex exec --skip-git-repo-check --model gpt-5.1 "say hello in one sentence"
```

Expected:
- command exits `0`
- non-empty assistant output

## 2) Indubitably Browser Login

```sh
codex login --indubitably
```

Expected:
- browser flow opens `app.indubitably.ai`
- token is saved in `$CODEX_HOME/indubitably-auth.json` (or `INDUBITABLY_AUTH_FILE` path)

## 3) Indubitably Bedrock Proxy Run

Config:

```toml
model_provider = "bedrock"

[model_providers.bedrock]
name = "AWS Bedrock"
env_key = "INDUBITABLY_API_TOKEN"
```

`base_url` is optional here. The built-in `bedrock` provider now defaults to `https://api.indubitably.ai`, so only set it when you need to override the proxy endpoint.

Command:

```sh
codex exec --skip-git-repo-check --indubitably --model claude-3-5-sonnet "summarize this repo in one line"
```

Expected:
- command exits `0`
- no calls to OpenAI `/v1/responses` in provider path
- `/model` shows Bedrock provider models for the same token

## 4) Invocation Matrix (`indubitably` + override)

Default path:

```sh
indubitably exec --skip-git-repo-check --model claude-3-5-sonnet "health check"
```

Expected:
- routes to Bedrock/Indubitably provider path by default

Override path:

```sh
indubitably --openai exec --skip-git-repo-check --model gpt-5.1 "health check"
```

Expected:
- does **not** route to Bedrock proxy paths
- follows OpenAI/provider-default auth and routing behavior

Legacy compatibility path:

```sh
codex --indubitably exec --skip-git-repo-check --model claude-3-5-sonnet "health check"
```

Expected:
- still routes to Bedrock/Indubitably provider path

## 5) Headless ECS Proxy Token Run (Supported)

In ECS task env:
- set `INDUBITABLY_API_TOKEN=<token>`
- use the built-in default Bedrock proxy endpoint, or explicitly keep `model_providers.bedrock.base_url=https://api.indubitably.ai` if you want the config to be explicit

Command:

```sh
codex exec --skip-git-repo-check --indubitably --model claude-3-5-sonnet "health check"
```

Expected:
- command exits `0`
- Bedrock proxy paths succeed (`/cli/models`, `/cli/bedrock/invoke`)

## 6) Direct AWS Endpoint Run (Current Expected Failure)

Config (intentionally unsupported right now):

```toml
model_provider = "bedrock"

[model_providers.bedrock]
name = "AWS Bedrock"
base_url = "https://bedrock-runtime.us-east-1.amazonaws.com"
```

Command:

```sh
codex exec --skip-git-repo-check --indubitably --model claude-3-5-sonnet "health check"
```

Expected today:
- model discovery/runtime path fails because CLI proxy paths are used (`/cli/models`, `/cli/bedrock/invoke`)
- errors may include HTTP 404 / `UnknownOperationException`

This is the tracked ECS direct-AWS gap.

## 7) Homebrew Artifact Sanity (Both macOS Architectures)

Build and validate artifact set:

```sh
./packaging/homebrew/scripts/release_checklist.py --version <VERSION>
```

Expected:
- validates both:
  - `codex-rs/dist/aarch64-apple-darwin/indubitably-aarch64-apple-darwin.tar.gz`
  - `codex-rs/dist/x86_64-apple-darwin/indubitably-x86_64-apple-darwin.tar.gz`
- confirms tarball entry names and executable bit
- emits generated Homebrew cask content
- creates a temporary Homebrew tap, installs the generated cask with `brew install --cask <temp-tap>/indubitably`, and validates:
  - `indubitably --version`
  - `indubitably --help` contains `Usage: indubitably`
- uninstalls the temporary cask and force-untaps the temporary tap
- restores any previously installed `indubitably` tap/version state

To skip only the tap-based Homebrew install/uninstall smoke (while keeping artifact/cask checks):

```sh
./packaging/homebrew/scripts/release_checklist.py --version <VERSION> --skip-brew-install-smoke
```

Use `--unsigned` only for local debugging. Release artifacts should be codesigned and notarized.
