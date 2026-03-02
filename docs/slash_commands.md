# Slash commands

For an overview of Codex CLI slash commands, see [this documentation](https://developers.openai.com/codex/cli/slash-commands).

## Provider-Aware `/model` Behavior (Fork-Specific)

In this fork, `/model` is provider-aware:

- OpenAI provider sessions show OpenAI-available models.
- Bedrock provider sessions (`model_provider = "bedrock"` or `--indubitably`) show models from the active Bedrock/Indubitably provider path.
- In Bedrock mode, `/model` options come from the provider's `/cli/models` allowlist (or Bedrock fallback catalog when remote fetch is unavailable).

This matches `--model` validation, which is also resolved against the active provider.
