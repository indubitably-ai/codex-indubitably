# Configuration

For basic configuration instructions, see [this documentation](https://developers.openai.com/codex/config-basic).

For advanced configuration instructions, see [this documentation](https://developers.openai.com/codex/config-advanced).

For a full configuration reference, see [this documentation](https://developers.openai.com/codex/config-reference).

## Indubitably Bedrock Provider (Fork-Specific)

This fork adds a provider alias flag, `--indubitably`, which selects `model_provider = "bedrock"` for the current run.

Minimal config example:

```toml
model_provider = "bedrock"

[model_providers.bedrock]
name = "AWS Bedrock"
base_url = "https://api.indubitably.ai"
env_key = "INDUBITABLY_API_TOKEN"
```

Notes:

- `base_url` is required for the Bedrock proxy runtime path.
- `env_key` is recommended for bearer token auth (`INDUBITABLY_API_TOKEN` in this example).
- `experimental_bearer_token` is also supported but discouraged for long-lived config.
- `--indubitably` is an alias for provider selection; it is not a separate execution engine.

## Provider-Aware Model Selection

- `--model` is validated against the active provider model catalog.
- `/model` reads from the active provider as well.
- When `model_provider = "bedrock"` (or `--indubitably` is used), model discovery comes from the configured Bedrock/Indubitably provider path instead of OpenAI defaults.

## Migration From Old Fork Config

If you are migrating from the older October-era fork:

- Keep: `model_provider = "bedrock"` (still valid).
- Replace old auth/feature toggles with provider config on `model_providers.bedrock`.
- The old keys `features.indubitably_auth`, `auth.indubitably.*`, and `bedrock_region` are not part of this surgical-port runtime path.
- Set token delivery through provider fields (`env_key` or `experimental_bearer_token`) and keep endpoint routing in `base_url`.

## Bedrock Troubleshooting

- Error: `Bedrock runtime adapter is not configured`
  - Fix: set `[model_providers.bedrock].base_url`.
- Error about missing environment variable for provider API key
  - Fix: export the variable referenced by `env_key`.
- Bedrock runs but `/model` does not show expected models
  - Fix: verify the configured provider endpoint serves the model catalog for the same token/provider.
- Authentication expired error from Bedrock proxy
  - Fix: refresh your Indubitably token and re-run.

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
