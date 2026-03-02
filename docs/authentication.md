# Authentication

For information about Codex CLI authentication, see [this documentation](https://developers.openai.com/codex/auth).

## Bedrock/Indubitably Token Sources (Fork-Specific)

### Browser Login Flow

Use:

```sh
codex login --indubitably
```

This opens the Indubitably web login flow (for example `app.indubitably.ai`) and saves the issued bearer token in the Indubitably token store (`$CODEX_HOME/indubitably-auth.json` by default, or `INDUBITABLY_AUTH_FILE` when set).

When `model_provider = "bedrock"` (including runs started with `--indubitably`), bearer token resolution follows this order:

1. Provider `env_key` (recommended).
2. Provider `experimental_bearer_token`.
3. Indubitably token store (`INDUBITABLY_AUTH_FILE`, then `$CODEX_HOME/indubitably-auth.json`, then `~/.indubitably/indubitably-auth.json`).
4. Shared auth manager token (when available).

Recommended setup:

- Define `env_key` on `[model_providers.bedrock]`.
- Export that environment variable before launching the CLI.
- Keep long-lived static bearer tokens out of checked-in config files.
