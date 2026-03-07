# Configuration

For basic configuration instructions, see [this documentation](https://developers.openai.com/codex/config-basic).

For advanced configuration instructions, see [this documentation](https://developers.openai.com/codex/config-advanced).

For a full configuration reference, see [this documentation](https://developers.openai.com/codex/config-reference).

## Indubitably Bedrock Provider (Fork-Specific)

This fork supports both invocation styles:

- `codex --indubitably ...`
- `indubitably ...` (implicit `--indubitably`)

It also adds `--openai` as an explicit provider override to force OpenAI behavior when using the `indubitably` command (for example `indubitably --openai exec ...`).

Minimal config example:

```toml
model_provider = "bedrock"

[model_providers.bedrock]
name = "AWS Bedrock"
env_key = "INDUBITABLY_API_TOKEN"
```

Notes:

- The built-in `bedrock` provider defaults to `https://api.indubitably.ai`.
- `base_url` is optional unless you need to override the default Bedrock proxy endpoint.
- `env_key` is recommended for bearer token auth (`INDUBITABLY_API_TOKEN` in this example).
- `experimental_bearer_token` is also supported but discouraged for long-lived config.
- `--indubitably` is an alias for provider selection; it is not a separate execution engine.
- `--openai` conflicts with `--indubitably` and `--oss`.

### Provider Precedence

When selecting provider from CLI flags, this fork applies:

1. explicit `--openai` => `model_provider = "openai"`
2. explicit or implicit indubitably (`--indubitably` or invoked as `indubitably`) => `model_provider = "bedrock"`
3. explicit `--oss` => OSS provider resolution (`--local-provider`, then config)
4. otherwise => config/profile defaults

The implicit `indubitably` invocation default applies to interactive, `exec`, `review`, `resume`/`fork`, and `login`.
This change does not include npm/bin alias behavior.

### Supported Bedrock Modes

- Supported: Indubitably proxy mode (the built-in `bedrock` provider defaults to `https://api.indubitably.ai`; bearer token still required).
- Not implemented in this fork: direct AWS credential-chain auth (`AWS_ACCESS_KEY_ID`, task role, IMDS/ECS role) for Bedrock runtime calls.
  - Today, Bedrock runtime/model discovery uses CLI proxy endpoints (`/cli/models`, `/cli/bedrock/invoke`) and bearer auth.
  - Pointing `base_url` directly at AWS Bedrock runtime endpoints (for example `https://bedrock-runtime.us-east-1.amazonaws.com`) is not supported.

### Headless/ECS Recommended Setup (Current)

Use the Indubitably proxy from ECS and inject a short-lived bearer token via environment variables:

```toml
model_provider = "bedrock"

[model_providers.bedrock]
name = "AWS Bedrock"
base_url = "https://api.indubitably.ai"
env_key = "INDUBITABLY_API_TOKEN"
```

## Provider-Aware Model Selection

- `--model` is validated against the active provider model catalog.
- `/model` reads from the active provider as well.
- When `model_provider = "bedrock"` (or `--indubitably` is used), model discovery comes from the active Bedrock/Indubitably provider path instead of OpenAI defaults.

## Migration From Old Fork Config

If you are migrating from the older October-era fork:

- Keep: `model_provider = "bedrock"` (still valid).
- Replace old auth/feature toggles with provider config on `model_providers.bedrock`.
- The old keys `features.indubitably_auth`, `auth.indubitably.*`, and `bedrock_region` are not part of this surgical-port runtime path.
- Set token delivery through provider fields (`env_key` or `experimental_bearer_token`) and keep endpoint routing in `base_url`.

## Bedrock Troubleshooting

- Error: `indubitably authentication expired`
  - Fix: run `indubitably login` (or `codex login --indubitably`) again, or provide a bearer token via `env_key` / `experimental_bearer_token`.
- Error about missing environment variable for provider API key
  - Fix: export the variable referenced by `env_key`.
- Bedrock runs but `/model` does not show expected models
  - Fix: verify the active provider endpoint serves the model catalog for the same token/provider. If you overrode `base_url`, make sure it still points at the Indubitably CLI proxy.
- Authentication expired error from Bedrock proxy
  - Fix: refresh your Indubitably token and re-run.
- Error like `UnknownOperationException` or HTTP 404 from Bedrock paths
  - Cause: `base_url` points at a direct AWS Bedrock endpoint instead of the Indubitably CLI proxy.
  - Fix: remove the override or set `[model_providers.bedrock].base_url = "https://api.indubitably.ai"`, then provide a bearer token (`env_key` recommended).

## Connecting to MCP servers

Codex can connect to MCP servers configured in `~/.codex/config.toml`. See the configuration reference for the latest MCP server options:

- https://developers.openai.com/codex/config-reference

## Apps (Connectors)

Use `$` in the composer to insert a ChatGPT connector; the popover lists accessible
apps. The `/apps` command lists available and installed apps. Connected apps appear first
and are labeled as connected; others are marked as can be installed.

## Notify

Codex can run a notification hook when the agent finishes a turn. See the configuration reference for the latest notification settings:

- https://developers.openai.com/codex/config-reference

When Codex knows which client started the turn, the legacy notify JSON payload also includes a top-level `client` field. The TUI reports `codex-tui`, and the app server reports the `clientInfo.name` value from `initialize`.

## JSON Schema

The generated JSON Schema for `config.toml` lives at `codex-rs/core/config.schema.json`.

## SQLite State DB

Codex stores the SQLite-backed state DB under `sqlite_home` (config key) or the
`CODEX_SQLITE_HOME` environment variable. When unset, WorkspaceWrite sandbox
sessions default to a temp directory; other modes default to `CODEX_HOME`.

## Notices

Codex stores "do not show again" flags for some UI prompts under the `[notice]` table.

## Plan mode defaults

`plan_mode_reasoning_effort` lets you set a Plan-mode-specific default reasoning
effort override. When unset, Plan mode uses the built-in Plan preset default
(currently `medium`). When explicitly set (including `none`), it overrides the
Plan preset. The string value `none` means "no reasoning" (an explicit Plan
override), not "inherit the global default". There is currently no separate
config value for "follow the global default in Plan mode".

Ctrl+C/Ctrl+D quitting uses a ~1 second double-press hint (`ctrl + c again to quit`).
