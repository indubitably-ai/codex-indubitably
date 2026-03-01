# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-01
- Fork branch: `codex/indubitably-port`
- Upstream ref: `origin/main`
- Start ahead/behind: `4 / 24`
- End ahead/behind: `29 / 0`

## Protected Surfaces

- Protected paths file: `.upstream-sync-protected-paths`
- Notes: Protected overlays for Indubitably auth/provider routing and Bedrock runtime were preserved. One semantic merge conflict was resolved manually in `codex-rs/exec/src/lib.rs` by keeping both `BEDROCK_PROVIDER_ID` and upstream analytics default constant.

## Commit Intake Log

| order | upstream sha | action | status | risk score | confidence | tests | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `392fa7de5` | cherry-pick | ported | 4 | 0.90 | app-server app_list | Protected app-server path, behavioral stabilization only |
| 2 | `a39d76dc4` | cherry-pick | ported | 1 | 0.97 | compile via downstream test runs | Linux sandbox enhancement |
| 3 | `c3c75878e` | cherry-pick | ported | 1 | 0.95 | tui model_availability_nux suite | TUI rendering/theme fix |
| 4 | `61c42396a` | cherry-pick | ported | 1 | 0.96 | tui suite compile/run | Chat composer placeholder preservation |
| 5 | `ff5cbfd7d` | cherry-pick | ported | 3 | 0.92 | core auth test `missing_plan_type_maps_to_unknown` | Auth robustness with missing plan claim |
| 6 | `ec6f6aacb` | cherry-pick | ported | 5 | 0.86 | core config test + tui model_availability_nux | Protected config/TUI model NUX behavior |
| 7 | `70ed6cbc7` | cherry-pick | ported | 2 | 0.94 | app-server app_list | Test-client event watch support |
| 8 | `6604608ba` | cherry-pick | ported | 1 | 0.95 | exec event_processor_with_human_output tests | Stdout duplication suppression |
| 9 | `8fa792868` | cherry-pick | ported | 4 | 0.88 | app-server thread_start/thread_read coverage | Protected app-server thread/start non-blocking |
| 10 | `1a8d93026` | cherry-pick | ported | 3 | 0.89 | core + cli regression sweeps | Host executable rule adoption |
| 11 | `8c1e3f3e6` | cherry-pick | ported | 6 | 0.84 | app-server thread_read/thread_start | Protected protocol + core thread metadata (`ephemeral`) |
| 12 | `033ef9cb9` | cherry-pick | ported | 4 | 0.88 | cli `debug_clear_memories` test | CLI debug command + state runtime change |
| 13 | `e6032eb0b` | cherry-pick | ported | 1 | 0.98 | downstream compile/tests | TUI feedback link fix |
| 14 | `e2fef7a3d` | cherry-pick | ported | 4 | 0.89 | downstream compile/tests | Fail-closed cloud requirements path |
| 15 | `83177ed7a` | cherry-pick + surgical edits | ported | 6 | 0.87 | exec/core/cli regressions + smoke | Conflict resolved to preserve Bedrock const while adopting analytics defaults |
| 16 | `2b38b4e03` | cherry-pick | ported | 3 | 0.90 | tui approval filter run | Sub-agent approval UX behavior |
| 17 | `d33f4b54a` | cherry-pick | ported | 3 | 0.91 | downstream compile/tests | Skill disable respects config layer |
| 18 | `bee93ca2f` | cherry-pick | ported | 2 | 0.93 | downstream compile/tests | Memory default tweak |
| 19 | `5f7c38baa` | cherry-pick | ported | 2 | 0.93 | downstream compile/tests | Memory read-path template tuning |
| 20 | `3bfee6fcb` | cherry-pick | ported | 1 | 0.98 | tui model_availability_nux suite | Test ignore nit |
| 21 | `eec3b1e23` | cherry-pick | ported | 3 | 0.89 | downstream compile/tests | Subagent startup performance |
| 22 | `84b662e74` | cherry-pick | ported | 2 | 0.94 | downstream compile/tests | Windows-gated test/config adjustments |
| 23 | `74e5150b1` | cherry-pick | ported | 2 | 0.93 | downstream compile/tests | Bazel models packaging fix |
| 24 | `6a673e733` | cherry-pick | ported | 4 | 0.88 | core/exec/cli regression + smoke | Preflight host executable safety alignment |

## Decision Briefs

### Commit `83177ed7a` (highest-risk conflict)

- Upstream intent: enable analytics by default in `codex exec` and `codex mcp-server`.
- Local overlays touched: `codex-rs/exec/src/lib.rs` where Bedrock/Indubitably path constant exists.
- Invariants checked:
  - `--indubitably` provider routing still selected Bedrock path.
  - Bedrock runtime tests still pass and do not call OpenAI responses endpoint.
- Risk factors: protected path overlap, semantic constant collision, metrics plumbing change.
- Strategy selected: cherry-pick + surgical edit.
- Confidence: 0.87.
- Validation evidence: `cargo test -p codex-core bedrock_runtime`, `cargo test -p codex-exec bedrock_runtime`, CLI smoke default/indubitably.
- Rollback note: revert `14bc36e72` and re-apply upstream without local-const retention if Bedrock path changes are intentionally dropped.

### Commit `ec6f6aacb`

- Upstream intent: model availability NUX tooltip persistence and behavior.
- Local overlays touched: core config path under protected list.
- Invariants checked: provider-aware model behavior retained; Bedrock model path unaffected.
- Risk factors: protected config files + broad TUI touch.
- Strategy selected: direct cherry-pick after patch review.
- Confidence: 0.86.
- Validation evidence: core config serialization test + tui model availability tests.
- Rollback note: revert `e9964fabc`.

### Commit `8c1e3f3e6`

- Upstream intent: include `ephemeral` in app-server `Thread` shape.
- Local overlays touched: app-server protocol/core thread metadata.
- Invariants checked: app-server initialize/model/account/thread RPC smoke still succeeds.
- Risk factors: protocol schema + core state + protected paths.
- Strategy selected: direct cherry-pick after review.
- Confidence: 0.84.
- Validation evidence: app-server thread_read/thread_start targeted tests, JSON-RPC smoke.
- Rollback note: revert `190cf3908`.

## Batch Validation

- [x] CLI default provider smoke
- [x] CLI `--indubitably` smoke (expected runtime-adapter-not-configured in this environment)
- [x] Targeted crate tests for touched code
- [x] App-server protocol smoke (initialize/model/list/account/read/thread/list/listConversations/getAuthStatus)

## Follow-ups

- Blocked commits: none
- Manual port TODOs: none
- Risk notes: post-cherry-pick merge with `origin/main` was required to close graph parity (`behind 24 -> 0`).

## Run Start Snapshot

- Date: 2026-03-01T01:30:38Z
- Fork branch: codex/indubitably-port
- Upstream ref: origin/main
- Start ahead/behind: 4\t24
- End ahead/behind: 29\t0
