# Bedrock/Indubitably Smoke Tests (Fork-Specific)

This checklist validates OpenAI baseline behavior, Indubitably login/proxy behavior, and current ECS headless constraints.

## Preconditions

- Build CLI: `cd codex-rs && cargo build -p codex-cli`
- Use repo binary first in PATH or run via `cargo run -p codex-cli -- ...`.

## 1) OpenAI Provider Baseline

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
base_url = "https://api.indubitably.ai"
env_key = "INDUBITABLY_API_TOKEN"
```

Command:

```sh
codex exec --skip-git-repo-check --indubitably --model claude-3-5-sonnet "summarize this repo in one line"
```

Expected:
- command exits `0`
- no calls to OpenAI `/v1/responses` in provider path
- `/model` shows Bedrock provider models for the same token

## 4) Headless ECS Proxy Token Run (Supported)

In ECS task env:
- set `INDUBITABLY_API_TOKEN=<token>`
- keep `model_providers.bedrock.base_url=https://api.indubitably.ai`

Command:

```sh
codex exec --skip-git-repo-check --indubitably --model claude-3-5-sonnet "health check"
```

Expected:
- command exits `0`
- Bedrock proxy paths succeed (`/cli/models`, `/cli/bedrock/invoke`)

## 5) Direct AWS Endpoint Run (Current Expected Failure)

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
