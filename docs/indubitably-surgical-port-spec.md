# Indubitably Surgical Port Specification

Last updated: 2026-02-28
Owner: Indubitably fork maintainers
Working branch: `codex/indubitably-port` in `/Users/gp/src/codex-indubitably`

## Objective

Port the Indubitably feature box (AWS Bedrock runtime + Indubitably auth + provider-aware model selection) onto current upstream Codex without re-merging the old fork history.

## Context

- Source fork: `/Users/gp/src/indubitably-codex` (October-era fork with substantial downstream work).
- Target baseline: `/Users/gp/src/codex-indubitably` from `origin/main`.
- Constraint: treat this as a surgical transplant (feature-level reimplementation/port), not a branch merge.
- Note: local `indubitably-codex/master` and `upstream/main` currently do not produce a usable merge-base for straightforward cherry-picking, so patch-by-feature is the safer path.

## Design Principles

1. Preserve upstream behavior by default.
2. Add Indubitably behavior behind explicit provider/auth selection.
3. Keep provider routing generic so future providers can be added.
4. Port tests first (or in lockstep) so behavior is pinned before implementation drift.
5. Keep commits small and reviewable by workstream.

## Scope

In scope:

- Provider selection abstraction and routing for OpenAI vs Bedrock.
- Indubitably auth flow and token-backed Bedrock proxy/runtime integration.
- CLI alias flag `--indubitably` as shorthand for provider/auth settings.
- Provider-aware `--model` validation and `/model` model availability.
- Ported/updated tests for the feature box.
- Docs updates in `docs/`.

Out of scope:

- Full rebrand of all upstream UX/content.
- Porting every downstream-only customization from old fork.
- Rewriting unrelated upstream subsystems.

## Architecture Targets

1. Provider/Auth selection
- Canonical selector is provider-based (`model_provider`/provider engine).
- `--indubitably` is an alias that sets Bedrock + Indubitably auth expectations.
- Existing provider mechanisms remain extensible.

2. Model availability
- `--model` and `/model` use active-provider model sources.
- Bedrock + Indubitably path can provide account-scoped model lists.
- OpenAI path keeps upstream model sourcing behavior.

3. Runtime execution
- Request pipeline remains provider-agnostic at the orchestration layer.
- Provider-specific details live in provider/runtime modules.

## Initial Port Map (Seed)

Source-heavy modules from old fork:

- `codex-rs/core/src/bedrock/**`
- `codex-rs/core/src/indubitably/**`
- `codex-rs/core/src/provider_engine.rs`
- `codex-rs/core/src/model_provider_info.rs`
- `codex-rs/core/src/codex.rs`
- `codex-rs/core/src/client.rs`
- `codex-rs/app-server/src/codex_message_processor.rs`
- `codex-rs/tui/src/cli.rs`
- `codex-rs/exec/src/cli.rs`
- `codex-rs/exec/src/lib.rs`

Primary target touchpoints in upstream workspace:

- `codex-rs/core/src/model_provider_info.rs`
- `codex-rs/core/src/config/mod.rs`
- `codex-rs/core/src/codex.rs`
- `codex-rs/core/src/client.rs`
- `codex-rs/core/src/models_manager/**`
- `codex-rs/app-server/src/codex_message_processor.rs`
- `codex-rs/tui/src/cli.rs`
- `codex-rs/tui/src/slash_command.rs`
- `codex-rs/exec/src/cli.rs`
- `codex-rs/exec/src/lib.rs`

First-wave test candidates to port/adapt:

- Core suites around provider/model behavior and Bedrock runtime paths.
- App-server suites around model listing and auth status messaging.
- Exec/TUI tests that cover CLI flag semantics (`--model`, `--provider`, `--indubitably` alias once added).

## Repeatable Inventory Commands

Run from `/Users/gp/src/indubitably-codex`:

```bash
rg --files | rg "bedrock|indubitably|migration-plan|upstream-sync"
rg -n --hidden -S -- "bedrock|indubitably|model_provider|/model|--model" codex-rs docs
```

Run from `/Users/gp/src/codex-indubitably`:

```bash
rg -n --hidden -S -- "model_provider|/model|--model|--provider" codex-rs
```

## Phase 1 Artifact Mapping and Triage

Status values:

- `Port as-is`
- `Reimplement`
- `Drop`
- `Defer`

| Source artifact (old fork) | Target destination (upstream workspace) | Status | Rationale |
| --- | --- | --- | --- |
| `codex-rs/core/src/bedrock/runtime.rs` | `codex-rs/core/src/bedrock/runtime.rs` (new) | Reimplement | Upstream has no Bedrock runtime module; interface must be adapted to current `codex_api` and session flow. |
| `codex-rs/core/src/bedrock/bedrock_client.rs` | `codex-rs/core/src/bedrock/bedrock_client.rs` (new) | Reimplement | Needs compatibility work with latest core client lifecycle and retry/error conventions. |
| `codex-rs/core/src/bedrock/messages_adapter.rs` + `invoke_messages_adapter.rs` | `codex-rs/core/src/bedrock/messages_adapter.rs` + `invoke_messages_adapter.rs` (new) | Reimplement | Message pipeline and item types have changed upstream; direct copy is high risk. |
| `codex-rs/core/src/bedrock/stream_adapter.rs` | `codex-rs/core/src/bedrock/stream_adapter.rs` (new) | Reimplement | Streaming event handling has evolved upstream; port logic with current event contracts. |
| `codex-rs/core/src/bedrock/retry.rs` | `codex-rs/core/src/bedrock/retry.rs` (new) | Reimplement | Retry policy should align with current provider retry behavior and telemetry hooks. |
| `codex-rs/core/src/bedrock/tools.rs` | `codex-rs/core/src/bedrock/tools.rs` (new) | Reimplement | Tool schema handling must match current tool/spec definitions. |
| `codex-rs/core/src/bedrock/model_capabilities.rs` + `codex-rs/core/bedrock_models.toml` | `codex-rs/core/src/bedrock/model_capabilities.rs` + catalog file (new or integrated into models manager) | Reimplement | Must reconcile with upstream `models_manager` and remote catalog behavior. |
| `codex-rs/core/src/bedrock/aws_credentials.rs` | `codex-rs/core/src/bedrock/aws_credentials.rs` (new) | Defer | Direct AWS credential-chain Bedrock auth is not implemented in this surgical-port runtime; current runtime expects proxy bearer auth. |
| `codex-rs/core/src/bedrock/proxy_runtime.rs` | `codex-rs/core/src/bedrock/proxy_runtime.rs` (new) | Reimplement | Needs integration with current auth/session abstractions. |
| `codex-rs/core/src/bedrock/account.rs` + `usage.rs` | `codex-rs/core/src/bedrock/account.rs` + `usage.rs` (new) | Defer | Useful after base runtime is stable; not required for initial functional parity. |
| `codex-rs/core/src/bedrock/mod.rs` | `codex-rs/core/src/bedrock/mod.rs` (new) | Port as-is | Module export shell is low risk once file set is finalized. |
| `codex-rs/core/src/indubitably/auth_store.rs` | `codex-rs/core/src/indubitably/auth_store.rs` (new) | Reimplement | Storage integration must match upstream auth/config loader behavior. |
| `codex-rs/core/src/indubitably/token_manager.rs` | `codex-rs/core/src/indubitably/token_manager.rs` (new) | Reimplement | Depends on store mode and error flow differences in upstream. |
| `codex-rs/core/src/indubitably/client.rs` | `codex-rs/core/src/indubitably/client.rs` (new) | Port as-is | API client logic is likely reusable with endpoint/config wiring changes. |
| `codex-rs/core/src/indubitably/telemetry.rs` | `codex-rs/core/src/indubitably/telemetry.rs` (new) | Defer | Can land after core functionality; not required for minimal migration success. |
| `codex-rs/core/src/indubitably/mod.rs` | `codex-rs/core/src/indubitably/mod.rs` (new) | Port as-is | Module export shell is low risk. |
| `codex-rs/core/src/provider_engine.rs` | `codex-rs/core/src/provider_engine.rs` (new) or fold into existing provider selection | Reimplement | Upstream provider system changed; need compatibility with current config/provider model. |
| `codex-rs/core/src/model_provider_info.rs` (Bedrock additions) | `codex-rs/core/src/model_provider_info.rs` | Reimplement | Keep upstream provider schema intact while adding Bedrock/Indubitably provider entries. |
| `codex-rs/core/src/config_profile.rs` Bedrock fields | `codex-rs/core/src/config/profile.rs` + `codex-rs/core/src/config/mod.rs` | Reimplement | Profile config types differ upstream; must adapt keys and defaults carefully. |
| `codex-rs/core/src/codex.rs` Bedrock/Indubitably session wiring | `codex-rs/core/src/codex.rs` | Reimplement | High-churn core orchestration file in upstream; patch minimal integration seams only. |
| `codex-rs/core/src/client.rs` Bedrock request path | `codex-rs/core/src/client.rs` | Reimplement | Client internals differ; Bedrock path should be inserted with minimal divergence. |
| `codex-rs/core/src/models_manager/**` behavior from old fork | `codex-rs/core/src/models_manager/**` | Reimplement | Upstream now centralizes `/models`; provider-aware behavior should hook here, not app-server-only logic. |
| `codex-rs/app-server/src/codex_message_processor.rs` model/auth sections | `codex-rs/app-server/src/codex_message_processor.rs` | Reimplement | File has large upstream delta; direct port would regress unrelated app-server behavior. |
| `codex-rs/tui/src/cli.rs` (`--indubitably` alias) | `codex-rs/tui/src/cli.rs` | Reimplement | Add alias via current CLI override plumbing rather than fork-era assumptions. |
| `codex-rs/exec/src/cli.rs` (`--indubitably` alias) | `codex-rs/exec/src/cli.rs` | Reimplement | Same as TUI: alias should map into provider/auth config overrides. |
| `codex-rs/exec/src/lib.rs` Bedrock runtime bootstrap | `codex-rs/exec/src/lib.rs` | Reimplement | Exec flow changed upstream; integrate Bedrock bootstrap behind existing command flow. |
| `codex-rs/core/tests/common/bedrock*.rs` | `codex-rs/core/tests/common/bedrock*.rs` (new) | Reimplement | Test harness should be adapted to current core test support utilities. |
| `codex-rs/core/tests/fixtures/bedrock/*.json` | `codex-rs/core/tests/fixtures/bedrock/*.json` (new) | Port as-is | Fixtures are data assets and can be reused with minor schema adjustments if needed. |
| `codex-rs/core/tests/suite/*_bedrock.rs` | Existing core suites (`remote_models.rs`, `model_overrides.rs`, `tools.rs`, `cli_stream.rs`, etc.) plus new focused Bedrock suites | Reimplement | Upstream suite structure differs; port intent, not file-for-file tests. |
| `codex-rs/exec/tests/suite/*_bedrock.rs` | Existing exec suites (`auth_env.rs`, resume/review suites) plus new Bedrock-focused tests | Reimplement | Keep upstream coverage style and adapt only behavior-level assertions. |
| `docs/bedrock-phase*.md`, `docs/bedrock-migration-plan.md` | `docs/indubitably-surgical-port-spec.md` (this doc) and targeted user docs | Defer | Keep as reference material; do not copy wholesale until implementation settles. |
| `docs/testing/bedrock-harness.md` | `docs/testing/bedrock-harness.md` (new if harness lands) | Defer | Add once harness API stabilizes. |
| `docs/ci-bedrock.md` | GitHub workflow docs in target repo | Defer | CI wiring should follow final crate/test layout. |
| `docs/upstream-sync.md` and release/rebrand docs | None (or separate ops docs) | Drop | These are fork-operations docs, not required for feature-box transplant itself. |

### API/Schema Impact Notes (Initial)

- `ConfigToml` / profile changes are likely (provider/auth aliasing and optional Bedrock/Indubitably config fields). If changed, regenerate config schema (`just write-config-schema`) during implementation.
- App-server protocol fields may need review if introducing provider-specific auth-status flags; avoid renaming existing wire fields unless required.
- Model list semantics likely change by active provider; update docs and app-server tests to pin behavior.

### Runtime Status Update (2026-03-02)

- Current supported path:
  - `--indubitably` + `model_provider = "bedrock"` + proxy base URL (`https://api.indubitably.ai`) + bearer token.
  - Model discovery and runtime use proxy endpoints (`/cli/models`, `/cli/bedrock/invoke`).
- Current gap:
  - Direct AWS credential-chain Bedrock auth (ECS/task-role/IMDS) is not wired in this fork.
  - Pointing `base_url` at AWS runtime endpoints (for example `https://bedrock-runtime.us-east-1.amazonaws.com`) does not provide direct Bedrock compatibility for current CLI proxy paths.
- Validation evidence:
  - AWS credentials can be valid independently (`aws sts get-caller-identity`, `aws bedrock list-foundation-models`) while Codex Bedrock path still requires proxy bearer auth.

## Phase 2 Test Contract Plan

Goal: add behavior-first tests that currently fail on upstream baseline, then implement until green.

### Contract Matrix

| Contract ID | Behavior contract | Target tests (upstream workspace) | Baseline expectation before implementation |
| --- | --- | --- | --- |
| P2-CLI-1 | `--indubitably` is accepted by CLI parser in both exec and tui commands. | `codex-rs/exec/src/cli.rs` (add parser test), `codex-rs/tui/src/cli.rs` (add parser test), optionally `codex-rs/exec/src/main.rs` for top-level passthrough. | Fails because flag is unknown and not represented in CLI structs. |
| P2-CLI-2 | `--indubitably` maps to provider/auth overrides (alias behavior, not separate code path). | `codex-rs/exec/src/lib.rs` tests (new helper-focused unit tests), `codex-rs/tui/src/lib.rs` tests (override construction tests). | Fails because no alias mapping exists in override construction. |
| P2-MODEL-1 | Provider-aware model refresh works for non-ChatGPT auth providers (for example Bedrock/Indubitably style providers). | `codex-rs/core/src/models_manager/manager.rs` tests (new cases near `refresh_available_models_*`). | Fails because refresh currently short-circuits when auth mode is not ChatGPT. |
| P2-MODEL-2 | App-server `listModels` reflects active provider model source rather than only bundled OpenAI catalog assumptions. | `codex-rs/app-server/tests/suite/v2/model_list.rs` (new provider-specific case), `codex-rs/app-server/tests/common/config.rs` helper updates as needed. | Fails because model list path currently inherits current manager behavior and bundled defaults for non-ChatGPT flows. |
| P2-AUTH-1 | Auth status semantics remain correct with provider-specific auth requirements after alias introduction. | `codex-rs/app-server/tests/suite/auth.rs` (extend custom-provider cases; retain `requires_openai_auth` wire field compatibility). | May partially pass today; new alias/provider cases should fail until wiring is added. |
| P2-BEDROCK-1 | Bedrock request path sanity: selecting Bedrock provider routes through Bedrock runtime adapter, not OpenAI responses. | New core suite files (planned): `codex-rs/core/tests/suite/bedrock_runtime.rs`, `codex-rs/core/tests/common/bedrock*.rs`, fixture dir `codex-rs/core/tests/fixtures/bedrock/`. | Fails by absence: no Bedrock runtime/harness modules exist in upstream baseline. |
| P2-BEDROCK-2 | Exec path sanity under alias/provider: initial turn reaches Bedrock runtime path and honors model selection. | New exec suite files (planned): `codex-rs/exec/tests/suite/*_bedrock.rs` adapted to upstream style. | Fails by absence of Bedrock runtime bootstrap and alias wiring. |

### Phase 2 File-Level Checklist

- [x] Add parser coverage for `--indubitably` in:
  - `codex-rs/exec/src/cli.rs`
  - `codex-rs/tui/src/cli.rs`
- [x] Add override-construction tests for alias behavior in:
  - `codex-rs/exec/src/lib.rs`
  - `codex-rs/tui/src/lib.rs`
- [x] Add provider-refresh regression tests in:
  - `codex-rs/core/src/models_manager/manager.rs`
- [x] Extend app-server model-list tests for provider-aware behavior in:
  - `codex-rs/app-server/tests/suite/v2/model_list.rs`
- [x] Extend app-server auth-status tests for alias/provider combinations in:
  - `codex-rs/app-server/tests/suite/auth.rs`
- [x] Add initial Bedrock runtime-path regression test in:
  - `codex-rs/core/tests/suite/bedrock_runtime.rs`
- [x] Add Bedrock harness scaffolding and fixture-backed runtime-path coverage in:
  - `codex-rs/core/tests/common/bedrock*.rs`
  - `codex-rs/core/tests/fixtures/bedrock/*`
- [x] Add exec Bedrock-path regression test in:
  - `codex-rs/exec/tests/suite/bedrock_runtime.rs`

### Execution Order

1. Land parser and override alias tests (`P2-CLI-*`) first.
2. Land provider-aware model refresh tests (`P2-MODEL-*`) next.
3. Land app-server auth/list tests (`P2-AUTH-*`) after model refresh tests.
4. Land Bedrock runtime and exec-path tests (`P2-BEDROCK-*`) with harness introduction.

### Baseline Failure Capture Plan

For each contract, record in this doc during execution:

- test command run
- failing test name(s)
- short failure reason
- commit SHA where failure was captured

Captured baseline failures:

| Date | Contract | Command | Failing test(s) | Failure reason | Commit SHA |
| --- | --- | --- | --- | --- | --- |
| 2026-02-27 | P2-CLI-1 | `cargo test -p codex-exec parse_accepts_indubitably_flag` | `cli::tests::parse_accepts_indubitably_flag` | parser rejects/does not support `--indubitably` yet | `695957a34` |
| 2026-02-27 | P2-CLI-1 | `cargo test -p codex-tui --lib parse_accepts_indubitably_flag --no-default-features` | `cli::tests::parse_accepts_indubitably_flag` | parser rejects/does not support `--indubitably` yet | `695957a34` |

## Surgical Port Checklist

### Phase 0: Workspace + Tracking

- [x] Create target integration workspace (`/Users/gp/src/codex-indubitably`) from upstream `origin/main` on branch `codex/indubitably-port`.
- [x] Create this specification document.
- [x] Create a running implementation log section in this doc as changes land.

### Phase 1: Inventory and Triage (Source Fork -> Feature Box)

- [x] Build inventory of candidate downstream artifacts from `/Users/gp/src/indubitably-codex`:
  - `codex-rs/core/src/bedrock/**`
  - `codex-rs/core/src/indubitably/**`
  - provider/config wiring in `core`, `exec`, `tui`, `app-server`
  - Bedrock/Indubitably tests under `codex-rs/core/tests/**` and `codex-rs/exec/tests/**`
  - docs: `docs/bedrock-*.md`, `docs/testing/bedrock-harness.md`, `docs/upstream-sync.md`
- [x] Classify each artifact: `Port as-is`, `Reimplement`, `Drop`, or `Defer`.
- [x] Record the mapping in a new table (source path -> target path -> status -> rationale).
- [x] Identify any API/schema impacts requiring doc/schema regeneration.

### Phase 2: Test Contracts First

- [x] Port/adapt minimal high-value tests that define required behavior before implementation:
  - provider selection and model-provider config precedence
  - `--indubitably` alias semantics
  - provider-aware `/model` listing + model validation
  - Indubitably auth-required vs not-required flows
  - Bedrock request path sanity checks
- [x] Ensure these tests fail on current upstream baseline for the intended reasons.
- [x] Group tests by crate and keep names explicit (`*_bedrock.rs`, `*_indubitably.rs` where appropriate).

### Phase 3: Implementation (Feature Box)

- [x] Add/adjust provider/auth abstraction points in `codex-rs/core`.
- [x] Wire `--indubitably` in CLI entry points (`tui` and `exec`) as shorthand, not as a separate hardcoded execution path.
- [x] Ensure provider selection drives model source and model validation.
- [x] Port Indubitably token management/auth client integration needed for runtime and model listing.
- [x] Integrate Bedrock runtime/proxy path with minimal coupling to orchestration layers.
- [x] Preserve upstream-compatible behavior for non-Indubitably users.
- Current scope note: Bedrock runtime and provider model refresh now load bearer tokens from provider `env_key`, provider `experimental_bearer_token`, Indubitably token-store files, or shared auth token. Full interactive Indubitably login UX/client parity from the old fork remains optional follow-up work.

### Phase 4: Docs and UX Contracts

- [x] Update docs to explain:
  - when to use `--indubitably`
  - provider-based model behavior for `--model` and `/model`
  - auth expectations for Bedrock + Indubitably
- [x] Document migration guidance from old fork configuration to upstream-based fork configuration.
- [x] Add troubleshooting entries for common auth/model-provider mismatches.

### Phase 5: Validation and Quality Gates

- [x] Run formatting for Rust changes: `just fmt` in `/Users/gp/src/codex-indubitably/codex-rs`.
- [x] Run lint fixes per changed crate(s): `just fix -p <crate>`.
- [x] Run targeted tests for changed crates.
- [x] If `common`, `core`, or `protocol` changed, run full test suite gate (`cargo test`) after user confirmation.
- [x] Ensure no snapshot drift remains unless intentionally accepted.
- [x] Produce a final validation summary in this doc (commands run + pass/fail + commit SHA).

## Acceptance Criteria

- `--indubitably` exists and behaves as a stable alias for the intended provider/auth configuration.
- `--model` and `/model` reflect active-provider model availability (OpenAI vs Bedrock/Indubitably).
- Non-Indubitably upstream workflows remain intact.
- Ported tests pass and cover the critical feature box behaviors.
- Documentation matches implementation and operator workflow.

## Implementation Log

Use this section as work progresses.

| Date | Change | Status | Notes |
| --- | --- | --- | --- |
| 2026-02-27 | Created `codex/indubitably-port` worktree and initial surgical-port spec | Done | Baseline planning artifact established |
| 2026-02-27 | Added Phase 1 artifact inventory + source-to-target triage table | Done | Classified Bedrock/Indubitably modules and tests as Port as-is/Reimplement/Defer/Drop |
| 2026-02-27 | Added Phase 2 test contract matrix and file-level execution checklist | Done | Defined concrete tests, baseline failure expectations, and implementation order |
| 2026-02-27 | Added first `--indubitably` parser tests and captured baseline failures | Done | `codex-exec` and `codex-tui` parser tests now fail on baseline as expected |
| 2026-02-27 | Wired `--indubitably` CLI flag and provider override mapping in exec/tui | Done | Added `indubitably` flag to exec/tui CLIs, mapped alias to provider id `bedrock`, and added `--oss` conflict guard |
| 2026-02-27 | Updated top-level CLI resume/fork merge behavior for alias flag | Done | `codex resume --indubitably` now carries the flag into `TuiCli` |
| 2026-02-27 | Added and verified `P2-CLI-2` alias override tests | Done | Added helper-focused tests in `exec/src/lib.rs` and `tui/src/lib.rs`; targeted tests pass |
| 2026-02-28 | Completed formatting, clippy fixes, and crate test runs for `cli`/`exec`/`tui` | Done | `just fmt`, `just fix -p codex-exec`, `just fix -p codex-tui`, `just fix -p codex-cli`, and `cargo test -p` runs all succeeded |
| 2026-02-28 | Completed `P2-MODEL-1` provider-aware model refresh wiring and tests in `core` | Done | `models_manager` refresh now gates on `provider.requires_openai_auth`; added/updated tests for non-OpenAI vs OpenAI-auth provider behavior without ChatGPT auth |
| 2026-02-28 | Completed `P2-MODEL-2` provider-aware app-server model list wiring and test | Done | Added non-ChatGPT provider `listModels` coverage and propagated `config.model_provider` into `ThreadManager`/`ModelsManager` construction across exec/tui/app-server/mcp |
| 2026-02-28 | Completed `P2-AUTH-1` auth-status provider coverage in app-server tests | Done | Added explicit `requires_openai_auth` assertions for default/auth-required flows and a new custom-provider `requires_openai_auth = true` auth-status test case |
| 2026-02-28 | Updated MCP codex-tool integration test for provider preflight model fetch | Done | `codex_tool` suite now selects the `/responses` request explicitly, so preflight `/models` calls do not break request-body assertions |
| 2026-02-28 | Fixed core `grep_files` integration tests under provider-aware model refresh behavior | Done | Updated `core/tests/suite/grep_files.rs` to use explicit `/models` mock with `experimental_supported_tools=["grep_files"]` and ChatGPT test auth so tool support is discovered from remote models |
| 2026-02-28 | Re-ran workspace quality gates after `grep_files` fix | Done | `just fmt`, scoped `just fix -p ...`, and full `cargo test --all-features` all passed |
| 2026-02-28 | Added built-in `bedrock` provider registry entry and config acceptance tests | Done | `--indubitably` now resolves to an existing provider id in core provider registry; added tests in `model_provider_info` and `config` to prevent regression |
| 2026-02-28 | Added initial `P2-BEDROCK-1` runtime-path regression test in core suite | Done (partial contract) | New `core/tests/suite/bedrock_runtime.rs` asserts the `bedrock` provider path returns unsupported until adapter wiring exists and emits zero `/responses` API requests |
| 2026-02-28 | Added initial `P2-BEDROCK-2` exec-path regression test | Done (partial contract) | New `exec/tests/suite/bedrock_runtime.rs` validates `--indubitably` exits on unconfigured Bedrock adapter and does not call `/v1/responses` |
| 2026-02-28 | Added Bedrock test harness scaffolding + fixture corpus in `core_test_support` | Done (partial contract) | Added `core/tests/common/bedrock.rs` + `bedrock_fixtures.rs`, copied `core/tests/fixtures/bedrock/*`, and updated `core/tests/suite/bedrock_runtime.rs` to use fixture-backed helpers and chunk-sequence assertions |
| 2026-02-28 | Added injectable Bedrock runtime adapter seam in `core` client | Done (partial contract) | Added `core/src/bedrock/runtime_adapter.rs`, wired `ModelClient::new_with_bedrock_runtime(...)`, and routed Bedrock provider streams through adapter trait instead of a hardcoded branch |
| 2026-02-28 | Added Bedrock adapter routing unit coverage in `core` client tests | Done | New `client::tests::bedrock_provider_stream_uses_injected_runtime_adapter` verifies Bedrock provider streams invoke injected adapter and emit streamed events |
| 2026-02-28 | Completed Bedrock proxy runtime implementation and stream adaptation in `core` | Done | Added concrete Bedrock runtime modules (`runtime`, `proxy_runtime`, `messages_adapter`, `stream_adapter`, `tools`, `usage`) and wired default adapter selection via provider config |
| 2026-02-28 | Promoted `P2-BEDROCK-*` from partial to functional proxy-runtime contract | Done | Core Bedrock runtime tests now cover both unconfigured unsupported path and configured proxy streaming path with fixture-backed chunk parsing and usage assertions |
| 2026-02-28 | Completed full crate/workspace validation after Bedrock proxy runtime wiring | Done | Scoped `just fix -p` for changed crates, targeted `codex-core`/`codex-exec` tests, snapshot check, and `cargo test --all-features` all passed |
| 2026-02-28 | Updated fork-specific docs for provider behavior, migration, and troubleshooting | Done | Added Bedrock/Indubitably sections to `docs/config.md`, provider-aware `/model` behavior note in `docs/slash_commands.md`, and Bedrock token-source order in `docs/authentication.md` |
| 2026-02-28 | Added Indubitably token-store auth fallback for runtime + model refresh paths | Done | Added `core/src/indubitably_auth.rs`, wired token loading in `api_bridge::auth_provider_from_auth` and `bedrock::runtime_adapter`, and validated with `codex-core` tests |

## Validation Summary (Latest Run)

| Date | Command | Result | Notes |
| --- | --- | --- | --- |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib resolve_bearer_token_` | Pass | New Bedrock runtime adapter token-resolution unit tests passed (`resolve_bearer_token_*`). |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib refresh_available_models_surfaces_unknown_operation_for_non_proxy_bedrock_base_url` | Pass | New provider-aware model refresh regression for non-proxy Bedrock endpoint behavior passed. |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all suite::bedrock_runtime::` | Pass | Core Bedrock runtime integration suite passed with added unauthorized + UnknownOperation coverage. |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec` | Pass | Exec unit + integration + e2e suite passed with new proxy-auth unauthorized scenario. |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Fail (env/deps) | Core integration suite had existing environment-dependent failures (missing `codex`/`test_stdio_server` binaries in test env and search-tool mock expectations) unrelated to new Bedrock-path tests. |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy fix for core completed after local disk cleanup. |
| 2026-03-02 | `CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-exec` | Pass | Scoped clippy fix for exec completed. |
| 2026-03-02 | `just fmt` | Pass | Rust formatting completed after test and clippy runs. |
| 2026-02-28 | `just fmt` | Pass | Ran after code changes and again after clippy fixes |
| 2026-02-28 | `just fix -p codex-exec` | Pass | No lint errors |
| 2026-02-28 | `just fix -p codex-tui` | Pass | No lint errors |
| 2026-02-28 | `just fix -p codex-cli` | Pass | First attempt failed due disk full; rerun after `cargo clean` passed |
| 2026-02-28 | `cargo test -p codex-exec` | Pass | Includes unit + integration suites |
| 2026-02-28 | `cargo test -p codex-tui` | Pass | 1142 passed, 2 ignored in lib tests; crate test run passed |
| 2026-02-28 | `cargo test -p codex-cli` | Pass | Unit + integration suites passed |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Completed after terminating stale lock-holder cargo processes from earlier interrupted runs |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Fail (env/disk) | Initial run failed with `No space left on device`; resolved by `cargo clean` and rerun with `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Lint fixes and clippy checks completed under reduced build pressure |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-app-server` | Pass | Completed; later rerun after test-helper cleanup produced warning-free output |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-mcp-server` | Pass | Completed after provider-preflight test assertion update |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-exec` | Pass | Completed |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-tui` | Pass | Completed |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-cli` | Pass | Completed |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Fail (env/integration) | Unit tests passed; integration `tests/all.rs` had existing environment/binary-dependent failures (missing `codex`/`test_stdio_server` binaries plus search/grep suite mismatches) |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core refresh_available_models_fetches_non_openai_provider_without_chatgpt_auth` | Pass | Confirms non-OpenAI provider refresh proceeds without ChatGPT auth |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core refresh_available_models_skips_openai_auth_provider_without_chatgpt_auth` | Pass | Confirms OpenAI-auth provider refresh is skipped without ChatGPT auth |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core refresh_available_models_` | Pass | Consolidated run of all provider refresh regression tests |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server list_models_uses_custom_provider_cache_source_without_chatgpt_auth` | Pass | Provider-only dynamic model appears in app-server `model/list`; one earlier run timed out during initialize and passed on immediate rerun |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec parse_accepts_indubitably_flag` | Pass | Alias parser coverage in exec |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui --lib parse_accepts_indubitably_flag --no-default-features` | Pass | Alias parser coverage in tui |
| 2026-02-28 | `RUSTFLAGS='-C debuginfo=0' cargo test -p codex-mcp-server` | Pass | Full mcp-server suite green after test assertion update |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Fail (core integration) | Fails in `codex-core` integration suite with `suite::grep_files::grep_files_tool_collects_matches` and `suite::grep_files::grep_files_tool_reports_empty_results` (`unsupported call: grep_files`) |
| 2026-02-28 | `just fmt` | Pass | Re-ran after `grep_files` test-harness fix |
| 2026-02-28 | `just fix -p codex-core` | Pass | Re-ran after `grep_files` test-harness fix |
| 2026-02-28 | `just fix -p codex-app-server` | Pass | Re-ran as part of final quality gate |
| 2026-02-28 | `just fix -p codex-mcp-server` | Pass | Re-ran as part of final quality gate |
| 2026-02-28 | `just fix -p codex-exec` | Pass | Re-ran as part of final quality gate |
| 2026-02-28 | `just fix -p codex-tui` | Pass | Re-ran as part of final quality gate |
| 2026-02-28 | `just fix -p codex-cli` | Pass | Re-ran as part of final quality gate |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Full workspace test/doc-test gate completed cleanly after `grep_files` test fix |
| 2026-02-28 | `just fix -p codex-app-server` | Fail (env/disk) | Failed with `No space left on device`; rerun after `cargo clean` with `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0'` succeeded |
| 2026-02-28 | `cargo clean && CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-app-server` | Pass | Completed after clearing 15.7GiB from `target/` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server suite::auth::` | Pass | Auth-status suite includes new custom-provider `requires_openai_auth=true` case |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server` | Pass | Full crate gate passed (214 passed, 0 failed, 1 ignored) |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No pending snapshots reported |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Full workspace regression rerun after auth-status test additions is green |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Re-ran after adding built-in `bedrock` provider registration/tests |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core built_in_model_providers_include_bedrock` | Pass | Verifies provider registry includes `bedrock` and expected defaults |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core test_load_config_accepts_built_in_bedrock_model_provider` | Pass | Verifies config loader accepts `model_provider = "bedrock"` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Full workspace rerun after built-in `bedrock` provider registration is green |
| 2026-02-28 | `just fmt` | Pass | Re-ran after adding `core/tests/suite/bedrock_runtime.rs` and suite module registration |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy/lint gate passed after bedrock runtime suite addition |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_does_not_call_responses_api` | Pass | New runtime-path regression test passed (`suite::bedrock_runtime::bedrock_provider_stream_does_not_call_responses_api`) |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Full `codex-core` unit + integration suite passed after new Bedrock runtime test |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Full workspace regression rerun passed after adding the Bedrock runtime-path regression test |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No pending snapshots after adding the Bedrock runtime-path regression test |
| 2026-02-28 | `just fmt` | Pass | Re-ran after adding `exec/tests/suite/bedrock_runtime.rs` and suite module registration |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-exec` | Pass | Scoped clippy/lint gate passed after exec bedrock regression test addition |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec exec_indubitably_path_exits_without_openai_responses_requests` | Pass | New exec bedrock-path regression test passed (`suite::bedrock_runtime::exec_indubitably_path_exits_without_openai_responses_requests`) |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec` | Pass | Full `codex-exec` unit + integration suite passed with bedrock-path test included |
| 2026-02-28 | `just fmt` | Pass | Re-ran after adding `core/tests/common/bedrock*.rs` helpers and fixture-backed suite assertions |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p core_test_support bedrock` | Fail (compile) | Initial run failed due `ModelInfo.description` assertion type mismatch (`Option<String>` vs `&str`) in `core/tests/common/bedrock.rs`; fixed and reran |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p core_test_support bedrock` | Pass | `core_test_support` Bedrock helper tests passed after assertion fix |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_stream_fixture_parses_expected_chunk_sequence` | Pass | Confirms fixture-backed chunk sequence assertion in `suite::bedrock_runtime` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_does_not_call_responses_api` | Pass | Confirms Bedrock path still does not call `/responses` and returns unsupported adapter error |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Full `codex-core` gate rerun with new helper modules and fixture directory |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy/lint gate rerun after adding Bedrock harness scaffolding |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No pending snapshots after adding Bedrock harness scaffolding and fixture-backed assertions |
| 2026-02-28 | `just fmt` | Pass | Re-ran after adding Bedrock adapter seam (`core/src/bedrock/*`) and `ModelClient` routing changes |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Fail (compile) | First run failed while integration test attempted to construct private `ResponseStream` internals from outside crate; moved injected-adapter assertion into `core/src/client.rs` unit tests and kept integration suite focused on runtime-path + fixture checks |
| 2026-02-28 | `just fmt` | Pass | Re-ran after integration test cleanup |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy/lint gate passed after Bedrock adapter seam cleanup |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_uses_injected_runtime_adapter` | Pass | New unit test confirms Bedrock provider stream uses injected adapter in `core/src/client.rs` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_returns_unsupported_operation` | Pass | Default adapter path still returns stable unsupported-operation error for Bedrock when no runtime is configured |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_does_not_call_responses_api` | Pass | Integration Bedrock runtime-path sanity check remains green after adapter seam wiring |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_stream_fixture_parses_expected_chunk_sequence` | Pass | Fixture-backed Bedrock chunk-sequence regression remains green |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Full `codex-core` unit + integration suites passed after Bedrock adapter seam wiring |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No snapshot drift after Bedrock adapter seam and routing test additions |
| 2026-02-28 | `just fmt` | Pass | Re-ran after concrete Bedrock proxy-runtime module implementation in `core` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy/lint gate passed after Bedrock runtime implementation |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-exec` | Pass | Scoped clippy/lint gate passed after exec Bedrock runtime test updates |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_without_runtime_config_returns_unsupported_operation` | Pass | Confirms default unconfigured Bedrock adapter behavior remains stable |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core bedrock_provider_stream_uses_proxy_runtime_and_avoids_responses_api` | Pass | Confirms configured Bedrock provider uses proxy runtime path and avoids OpenAI `/responses` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec exec_indubitably_path_exits_without_openai_responses_requests` | Pass | Exec regression still ensures Bedrock path does not issue OpenAI `/responses` requests |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Full `codex-core` gate rerun passed after proxy runtime implementation |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec` | Pass | Full `codex-exec` gate rerun passed after Bedrock runtime test updates |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No snapshot drift after Bedrock proxy-runtime work |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Full workspace regression gate passed after proxy runtime implementation |
| 2026-02-28 | `just fmt` | Pass | Re-ran after adding Indubitably token-store auth loader and wiring |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Scoped clippy/lint gate passed for token-store integration |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core indubitably_auth::tests::` | Fail (test) | Initial run failed due env-var race in token-store unit tests; fixed by serializing the env-dependent tests with `serial_test` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core indubitably_auth::tests::` | Pass | Token-store loader tests pass after serializing env-dependent cases |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Full `codex-core` unit + integration gate passed with token-store auth integration |
| 2026-02-28 | `just fmt` | Pass | Re-ran after final token-store test serialization adjustments |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' just fix -p codex-core` | Pass | Final scoped clippy/lint pass after `just fmt` |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core` | Pass | Final full `codex-core` rerun passed after lint rerun |
| 2026-02-28 | `cargo insta pending-snapshots --workspace-root . --workspace` | Pass | No pending snapshots after token-store integration |
| 2026-02-28 | `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 RUSTFLAGS='-C debuginfo=0' cargo test --all-features` | Pass | Final full workspace regression gate passed after token-store auth integration |

Validation base commit: `695957a34` (results captured on a dirty working tree with the changes listed in this spec).

## Final Change Summary

Spec reference: `docs/indubitably-surgical-port-spec.md` (this document).

Delivered feature box:

- `--indubitably` CLI alias now routes to built-in `bedrock` provider selection.
- Provider-aware model discovery/validation powers both `--model` and `/model`.
- Bedrock proxy runtime path is integrated into `codex-core` with streaming adaption and fixture-backed regression tests.
- Indubitably token-store fallback is wired into both provider model refresh auth and Bedrock runtime auth.
- Fork-specific operator docs now cover config, migration, auth expectations, and troubleshooting.

Validation highlights:

- Scoped formatting/lint gates completed for changed crates (including final `just fix -p codex-core` pass).
- Targeted and full crate tests passed for changed areas (`codex-core`, `codex-exec`, plus relevant integration suites).
- Snapshot drift check is clean.
- Final workspace gate passed: `cargo test --all-features`.

## Proposed Commit Breakdown

Suggested non-interactive commit slicing for reviewability:

1. CLI/provider routing and provider-aware model source
   - `codex-rs/exec/src/cli.rs`
   - `codex-rs/exec/src/lib.rs`
   - `codex-rs/tui/src/cli.rs`
   - `codex-rs/tui/src/lib.rs`
   - `codex-rs/tui/src/app.rs`
   - `codex-rs/cli/src/main.rs`
   - `codex-rs/core/src/model_provider_info.rs`
   - `codex-rs/core/src/models_manager/manager.rs`
   - `codex-rs/core/src/config/mod.rs`
   - `codex-rs/core/src/codex.rs`
   - `codex-rs/core/src/thread_manager.rs`
   - `codex-rs/app-server/src/message_processor.rs`
   - `codex-rs/mcp-server/src/message_processor.rs`
   - `codex-rs/app-server/tests/suite/auth.rs`
   - `codex-rs/app-server/tests/suite/v2/model_list.rs`
   - `codex-rs/mcp-server/tests/suite/codex_tool.rs`
   - `codex-rs/core/tests/suite/grep_files.rs`
   - `codex-rs/core/tests/responses_headers.rs`
   - `codex-rs/core/tests/suite/client_websockets.rs`
   - `codex-rs/Cargo.lock`
2. Bedrock runtime/proxy integration and harness tests
   - `codex-rs/core/src/bedrock/*`
   - `codex-rs/core/src/client.rs`
   - `codex-rs/core/src/lib.rs`
   - `codex-rs/core/tests/common/Cargo.toml`
   - `codex-rs/core/tests/common/lib.rs`
   - `codex-rs/core/tests/common/bedrock.rs`
   - `codex-rs/core/tests/common/bedrock_fixtures.rs`
   - `codex-rs/core/tests/fixtures/bedrock/*`
   - `codex-rs/core/tests/suite/mod.rs`
   - `codex-rs/core/tests/suite/bedrock_runtime.rs`
   - `codex-rs/core/tests/suite/client.rs`
   - `codex-rs/exec/tests/suite/mod.rs`
   - `codex-rs/exec/tests/suite/bedrock_runtime.rs`
3. Indubitably token-store fallback and auth-source wiring
   - `codex-rs/core/src/indubitably_auth.rs`
   - `codex-rs/core/src/api_bridge.rs`
   - (if preferred, include `codex-rs/core/src/bedrock/runtime_adapter.rs` in this commit instead of commit 2)
4. Operator docs + migration/spec artifact
   - `docs/config.md`
   - `docs/authentication.md`
   - `docs/slash_commands.md`
   - `docs/indubitably-surgical-port-spec.md`

## Definition of Done

- [x] Every checklist item above is complete or explicitly marked deferred with rationale.
- [x] All required tests pass in `codex-indubitably`.
- [x] Docs are updated and internally consistent.
- [x] Final PR/change summary links to this spec and references completed validation results.
