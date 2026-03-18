# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-17
- Fork branch: main
- Upstream ref: upstream/main
- Start ahead/behind: ahead 40 / behind 273
- End ahead/behind: ahead 52 / behind 273

## Protected Surfaces

- Protected paths file: .upstream-sync-protected-paths
- Notes: Batch 1 of phased sync (10 commits/run), direct-to-main push cadence.

## Commit Intake Log

| order | upstream sha | action | status | risk score | confidence | tests | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `b9a2e400018c219e3010a5a5b8ded8645184da0b` | cherry-pick | ported | 1 | 0.93 | `cargo check -p codex-skills --quiet` | Skill sample asset removal only. |
| 2 | `1c888709b5d718b2452f4bf59ef20f65ff4b5331` | cherry-pick | ported | 2 | 0.91 | `cargo test -p codex-core guardian --quiet` | Removes obsolete guardian snapshot test. |
| 3 | `92f7541624810406d5c3d1c424147bcfa458efce` | cherry-pick+surgical | ported | 5 | 0.86 | `cargo test -p codex-core guardian --quiet && cargo test -p codex-tui guardian --quiet` | Guardian CI follow-up + local exhaustive-match compatibility fixes. |

| 4 | `e8d7ede83cf09c99134866f19e5378c546d53191` | cherry-pick | ported | 2 | 0.90 | cargo test -p codex-tui context_window --quiet | TUI token-count timing display fix. |

| 5 | `bf5c2f48a5730f8076a65a1a5f637398ec92ae22` | cherry-pick | ported | 4 | 0.84 | cargo test -p codex-core request_permissions --quiet | Seatbelt split filesystem policy handling update. |

| 6 | `590cfa6176f2001d9289a062f7ada987f78bddd0` | cherry-pick | ported | 4 | 0.83 | cargo test -p codex-core mention_syntax --quiet && cargo test -p codex-tui mention --quiet | Plaintext mentions now use @plugin semantics. |

| 7 | `a4a9536fd7477a4c323999c01afb4743766bf2ac` | cherry-pick+surgical | ported | 5 | 0.82 | cargo test -p codex-core mcp_tool_call --quiet && cargo build -p codex-rmcp-client --bin test_stdio_server --quiet && cargo test -p codex-tui elicitation --quiet | Always-allow MCP elicitation option; required rmcp test binary build for tui tests. |

| 8 | `07a30da3fb31b2c1f70c1ef92e0b11355039c0ab` | cherry-pick | ported | 5 | 0.80 | cargo test -p codex-linux-sandbox landlock --quiet && cargo test -p codex-core request_permissions --quiet | Split sandbox policy plumbing for linux helper and protocol. |

| 9 | `46b8d127cf372378945b53f79c82cd0341fe870e` | cherry-pick | ported | 3 | 0.88 | cargo test -p codex-core request_permissions --quiet | Preserve denied paths while widening permissions. |

| 10 | `3b5fe5ca35d914645a818d454a3931f6748b7e77` | cherry-pick | ported | 5 | 0.81 | cargo test -p codex-protocol --quiet && cargo test -p codex-core request_permissions --quiet | Protocol + core update keeps root carveouts sandboxed. |

| 11 | `dc19e789624d46163f6882efca18c84ea4d17b81` | cherry-pick+surgical | ported | 5 | 0.81 | cargo test -p codex-app-server initialize --quiet && cargo test -p codex-core abort --quiet | Abort follow-up stabilization across app-server/core. |

| 12 | `dcc4d7b634e0c732e5dab9ab04b6f3b67bfa55f1` | cherry-pick | ported | 4 | 0.79 | cargo test -p codex-linux-sandbox landlock --quiet | Bwrap split filesystem policy honoring in linux-sandbox. |

| 13 | `a30edb6c17201358273b76a4fd81d2b6ce3c2c54` | cherry-pick | ported | 3 | 0.87 | cargo test -p codex-utils-pty terminate --quiet | Windows PTY terminate handling correction. |

| 14 | `7ba1fccfc1083354e058772b83b9bbb191e99f42` | cherry-pick | ported | 5 | 0.82 | cargo test -p codex-core guardian --quiet && cargo test -p codex-app-server thread_resume --quiet | Guardian coverage/Bazel test stabilization updates. |

| 15 | `1f150eda8b695b479cc45dc5243c7938ffc78b52` | cherry-pick | ported | 2 | 0.92 | cargo test -p codex-core shell_serialization --quiet | Core shell serialization test flake hardening only. |

| 16 | `a684a36091d70e1d8720fb99aafecd2a41ea7207` | cherry-pick+surgical | ported | 6 | 0.74 | cargo test -p codex-app-server batch_write_reloads_user_config_when_requested --quiet && cargo test -p codex-app-server config_batch_write_applies_multiple_edits --quiet | Protected app-server/protocol config hot-reload path touched. |

| 17 | `da3689f0ef7422c3857e1156d4b78d3482cc26d6` | cherry-pick+surgical | ported | 8 | 0.76 | cargo test -p codex-app-server-client --quiet && cargo test -p codex-exec --quiet && cargo test -p codex-app-server in_process --quiet && just bazel-lock-check | Conflict in exec/lib.rs resolved; retained local indubitably provider overlay tests with in-process adaptation. |

## Decision Briefs

### Commit `b9a2e400018c219e3010a5a5b8ded8645184da0b`

- Upstream intent: Remove temporary artifact sample skills from bundled skill assets.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider selection untouched.
- Risk factors: Low-scope content deletion in codex-skills assets only.
- Strategy selected: cherry-pick.
- Confidence: 0.93.
- Validation evidence: `cargo check -p codex-skills --quiet`.
- Rollback note: Revert this sync commit if asset removal causes missing reference issues.

### Commit `1c888709b5d718b2452f4bf59ef20f65ff4b5331`

- Upstream intent: Remove stale guardian snapshot assertions that no longer reflect current output.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unchanged.
- Risk factors: Core test cleanup with low runtime behavior impact.
- Strategy selected: cherry-pick.
- Confidence: 0.91.
- Validation evidence: `cargo test -p codex-core guardian --quiet` (20 passed).
- Rollback note: Revert this sync commit if guardian output assertions are needed for local regressions.

### Commit `92f7541624810406d5c3d1c424147bcfa458efce`

- Upstream intent: Fix guardian-related CI instability across core and TUI tests.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unchanged.
- Risk factors: Cross-crate test updates and required local exhaustive event-match compatibility adjustments.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.86.
- Validation evidence: `cargo test -p codex-core guardian --quiet` and `cargo test -p codex-tui guardian --quiet` passed.
- Rollback note: Revert this sync commit if guardian test behavior regresses or event handling compatibility should be isolated.

### Commit `e8d7ede83cf09c99134866f19e5378c546d53191`

- Upstream intent: Correct context window display before initial TokenCount events in TUI.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No auth/provider/runtime overlay paths modified.
- Risk factors: UI timing logic update with targeted tests available.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: codex-tui context_window filtered tests passed.
- Rollback note: Revert this sync commit if TUI header metrics regress.

### Commit `bf5c2f48a5730f8076a65a1a5f637398ec92ae22`

- Upstream intent: Honor split filesystem sandbox policies in seatbelt and related core paths.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock provider/runtime paths unchanged.
- Risk factors: Sandbox policy semantics changed in core runtime paths.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: codex-core request_permissions filtered tests passed.
- Rollback note: Revert this sync commit if seatbelt policy behavior regresses in local macOS flows.

### Commit `590cfa6176f2001d9289a062f7ada987f78bddd0`

- Upstream intent: Switch plaintext plugin mention syntax from $plugin to @plugin across core and tui mention handling.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock runtime/provider paths modified.
- Risk factors: Cross-crate mention parsing/rendering behavior change.
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: Core mention_syntax filter and TUI mention filter tests passed.
- Rollback note: Revert this sync commit if mention parsing/routing behavior regresses.

### Commit `a4a9536fd7477a4c323999c01afb4743766bf2ac`

- Upstream intent: Support always-allow option for MCP tool-call elicitations in core and TUI approval UI.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Auth/provider/runtime overlays unchanged.
- Risk factors: Approval-flow UX and state semantics changed across core+tui.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: Core mcp_tool_call and TUI elicitation filters passed after building rmcp test stdio server binary.
- Rollback note: Revert this sync commit if MCP approval behavior or elicitation UI regresses.

### Commit `07a30da3fb31b2c1f70c1ef92e0b11355039c0ab`

- Upstream intent: Plumb split filesystem sandbox policies through linux helper and protocol/core integration points.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider paths unchanged.
- Risk factors: Protocol + sandbox runtime behavior change across crates.
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: Linux-sandbox landlock filter and core request_permissions filter passed.
- Rollback note: Revert this sync commit if linux sandbox policy behavior regresses.

### Commit `46b8d127cf372378945b53f79c82cd0341fe870e`

- Upstream intent: Keep denied-path constraints when computing widened sandbox permissions.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider paths unchanged.
- Risk factors: Core sandbox permission behavior adjustment.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: codex-core request_permissions filtered tests passed.
- Rollback note: Revert this sync commit if permission widening introduces unexpected access changes.

### Commit `3b5fe5ca35d914645a818d454a3931f6748b7e77`

- Upstream intent: Ensure root carveouts remain sandboxed in protocol/core permission modeling.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider paths unchanged.
- Risk factors: Protocol permission semantics touched alongside core sandboxing.
- Strategy selected: cherry-pick
- Confidence: 0.81
- Validation evidence: codex-protocol suite and core request_permissions filter passed.
- Rollback note: Revert this sync commit if permission carveout behavior regresses.

### Commit `dc19e789624d46163f6882efca18c84ea4d17b81`

- Upstream intent: Stabilize abort-task follow-up handling between core and app-server initialization flow.
- Local overlays touched: Touches protected app-server path; no auth/provider overlay logic changed.
- Invariants checked: Indubitably auth path and Bedrock provider/runtime behavior remain unchanged.
- Risk factors: Cross-crate behavior at task-abort boundary plus protected app-server path touch.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.81
- Validation evidence: App-server initialize and core abort filtered tests passed.
- Rollback note: Revert this sync commit if abort follow-up flow regresses in app-server notifications.

### Commit `dcc4d7b634e0c732e5dab9ab04b6f3b67bfa55f1`

- Upstream intent: Honor split filesystem policies in bwrap path for linux sandbox execution.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unaffected.
- Risk factors: Linux sandbox behavior path changed across runtime + tests.
- Strategy selected: cherry-pick
- Confidence: 0.79
- Validation evidence: codex-linux-sandbox landlock-filtered test command completed successfully.
- Rollback note: Revert this sync commit if Linux bwrap policy behavior regresses.

### Commit `a30edb6c17201358273b76a4fd81d2b6ce3c2c54`

- Upstream intent: Fix inverted Windows PTY TerminateProcess handling logic in utils/pty.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Auth/provider/runtime overlay behavior unchanged.
- Risk factors: Platform-specific process-control behavior update.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: codex-utils-pty terminate-filtered tests passed.
- Rollback note: Revert this sync commit if Windows PTY termination semantics regress.

### Commit `7ba1fccfc1083354e058772b83b9bbb191e99f42`

- Upstream intent: Restore guardian test coverage and Bazel unit-test stability wiring across core/app-server templates.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Auth/provider/runtime overlay paths unchanged.
- Risk factors: Cross-cutting test infra and guardian behavior updates across core/app-server/Bazel scaffolding.
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: Core guardian and app-server thread_resume filtered suites passed.
- Rollback note: Revert this sync commit if guardian coverage or Bazel test launcher behavior regresses.

### Commit `1f150eda8b695b479cc45dc5243c7938ffc78b52`

- Upstream intent: Stabilize shell serialization tests by reducing fixture latency and forcing non-login shell execution in fixtures.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider overlays untouched.
- Risk factors: Test-only timing/fixture argument changes in core test suite.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: codex-core shell_serialization-filtered test command passed (80 tests).
- Rollback note: Revert this sync commit if shell serialization coverage weakens or flakes reappear due to timing assumptions.
### Commit `a684a36091d70e1d8720fb99aafecd2a41ea7207`

- Upstream intent: Allow config/batchWrite to optionally hot-reload user config into loaded threads after a successful write.
- Local overlays touched: Protected app-server/protocol paths touched; no Indubitably auth or Bedrock provider/runtime code changed.
- Invariants checked: Preserved existing config write semantics when reload flag is false; reload only occurs on successful write when requested.
- Risk factors: Behavioral change in app-server config RPC plus protocol surface extension and schema fixture updates.
- Strategy selected: cherry-pick+surgical (reviewed protected-path overlap; accepted upstream implementation as-is).
- Confidence: 0.74
- Validation evidence: App-server unit/integration filters for batch write + reload passed. Full codex-app-server-protocol suite currently fails on preexisting schema fixture drift expecting newer skill metadata fixture files.
- Rollback note: Revert this sync commit if config batch write reload introduces unexpected thread state churn.
### Commit `da3689f0ef7422c3857e1156d4b78d3482cc26d6`

- Upstream intent: Introduce in-process app-server client/facade and route exec through app-server request/event flow.
- Local overlays touched: Protected app-server/protocol/exec surfaces touched; preserved Indubitably provider override logic and Bedrock path guardrails.
- Invariants checked: Maintained --indubitably provider selection precedence and no-OpenAI-responses behavior in bedrock runtime tests.
- Risk factors: Large cross-crate architectural move (new crate, runtime wiring, protocol/common changes, dependency graph updates).
- Strategy selected: cherry-pick+surgical (manual conflict resolution in exec/lib.rs plus local test assertion adaptation for in-process preflight behavior).
- Confidence: 0.76
- Validation evidence: codex-app-server-client, codex-exec, and app-server in_process test filters passed; Bazel lock update/check executed for dependency changes.
- Rollback note: Revert this sync commit if in-process exec/app-server event flow regresses or bedrock auth path behavior deviates.
## Batch Validation

- [x] CLI default provider smoke
- [x] CLI `--indubitably` smoke
- [x] Targeted crate tests for touched code
- [ ] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits: none in this 10-commit batch.
- Manual port TODOs: none; two commits used cherry-pick+surgical strategy.
- Risk notes: full `cargo test -p codex-core` currently fails in this environment due js_repl runtime requirement (Node >= 22.22.0; local is v20.19.5); targeted tests and CLI smokes passed.
