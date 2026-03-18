# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-17
- Fork branch: main
- Upstream ref: upstream/main
- Batch 1 start ahead/behind: ahead 40 / behind 273
- Batch 1 end ahead/behind: ahead 52 / behind 273
- Batch 2 start ahead/behind: ahead 57 / behind 292
- Batch 2 end ahead/behind: ahead 63 / behind 292
- Batch 3 start ahead/behind: ahead 64 / behind 293
- Batch 3 end ahead/behind: ahead 79 / behind 293

## Protected Surfaces

- Protected paths file: .upstream-sync-protected-paths
- Notes: Batch 1 through Batch 3 of phased sync (10 commits/run), direct-to-main push cadence.

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

| 18 | `340f9c9ecb0b02a89e88c6dc068809007185f645` | cherry-pick+surgical | ported | 8 | 0.79 | cargo test -p codex-core skill_approval --quiet && cargo test -p codex-app-server command_execution_request --quiet && cargo test -p codex-tui chatwidget --quiet && cargo test -p codex-app-server-protocol command_execution_request_approval --quiet | Protected protocol/app-server surfaces plus core+tui approval plumbing updated for skill metadata. |

| 19 | `f41b1638c98deddd0d8f89d821999d30f73de599` | cherry-pick | ported | 2 | 0.91 | cargo test -p codex-core record_responses_sets_span_fields_for_response_events --quiet | Core OTEL test fixture completion sequencing fix only. |

| 20 | `4ad3b59de322dc75c2b257b2eec365309b195ab7` | cherry-pick | ported | 4 | 0.87 | cargo test -p codex-tui pending_input_preview --quiet && cargo test -p codex-tui interrupted_turn_pending_steers --quiet | TUI pending-steer/queued-follow-up UX and interrupt semantics update with snapshot refresh. |

| 21 | `e6b93841c585f8b56b2c9b38cb07708f278ea227` | cherry-pick+surgical | ported | 9 | 0.74 | cargo test -p codex-core request_permissions --quiet && cargo test -p codex-app-server request_permissions --quiet && cargo test -p codex-app-server-protocol permissions_request --quiet && cargo test -p codex-tui chatwidget --quiet && cargo test -p codex-exec --quiet | Large protected-surface request_permissions tool integration across core/protocol/app-server/exec/tui. |

| 22 | `05332b0e96197573a618b6df9adc41752cc87de9` | cherry-pick | ported | 2 | 0.95 | cargo check -p codex-app-server-client --quiet | Adds missing Bazel target file for previously introduced app-server-client crate. |

| 23 | `06f82c123c6ed295f0ef19b5cbf49cf78bbc092e` | cherry-pick | ported | 4 | 0.88 | cargo test -p codex-tui permissions_prompt_snapshot --quiet && cargo test -p codex-tui handle_request_permissions_opens_approval_modal --quiet | TUI render/queue handling for request_permissions approvals with new snapshot coverage. |

| 24 | `2bc3e52a91bb88a0e067a95f8f8559f8711d30e6` | cherry-pick | ported | 2 | 0.93 | cargo test -p codex-app-server list_apps_waits_for_accessible_data_before_emitting_directory_updates --quiet | App-server v2 app-list ordering test stabilization only. |

| 25 | `f23fcd6ced0035f2aa4e34d4f12da4f04c8a7fa4` | cherry-pick | ported | 6 | 0.83 | cargo test -p codex-core guardian --quiet && cargo test -p codex-protocol --quiet && cargo test -p codex-tui --lib experimental_popup_includes_guardian_approval --quiet | Guardian review copy/plumbing refinements across core+protocol with matching tui snapshot updates. |

| 26 | `3f1280ce1c3ba33c7be769f3b44c4fb610aef3e7` | cherry-pick | ported | 3 | 0.90 | cargo test -p codex-app-server test_fuzzy_file_search_session_stops_sending_updates_after_stop --quiet && cargo test -p codex-app-server auth --quiet && cargo test -p codex-app-server account_read_returns_auth_status --quiet | App-server timeout-pressure reduction in auth/account/fuzzy-search tests only. |

| 27 | `615ed0e437afd9bfe0af4a8a8c9a2254227060e1` | cherry-pick | ported | 3 | 0.92 | cargo test -p codex-app-server turn_start_shell_zsh_fork --quiet | Stabilizes zsh-fork interrupt/decline tests by synchronizing on explicit completion events. |

| 28 | `6052558a017b89cc62820b388f2cdd3ad5a3feda` | cherry-pick | ported | 2 | 0.95 | cargo test -p codex-rmcp-client drop_kills_wrapper_process_group --quiet | RMCP pid-file cleanup test now waits for non-empty pid content before assertions. |

| 29 | `5d9db0f9959a5ee2eec75e6bbd2f6b3a543802d2` | cherry-pick | ported | 2 | 0.94 | cargo test -p codex-utils-pty pty_python_repl_emits_output_and_exits --quiet | PTY Python REPL test now waits for a startup marker emitted at process launch. |

| 30 | `6b68d1ef661263b0fa6bf9b1e1badffebfd64ee9` | cherry-pick | ported | 3 | 0.92 | cargo test -p codex-app-server plan_mode_ --quiet | Plan-item app-server tests now use multi-thread runtime and explicit /responses request-count waits. |

| 31 | `0dc242a67229c99ac1de63dbdd5adc1d17481575` | cherry-pick+surgical | ported | 7 | 0.82 | cargo test -p codex-app-server initialize_opt_out_notification_methods_filters_notifications --quiet && cargo test -p codex-app-server turn_start_notify_payload_includes_initialize_client_name --quiet | Websocket initialize ordering fixed by sending connection-scoped notifications before outbound-ready flip. |

| 32 | `10bf6008f4d76c56db86cafb2a45c1fc88024aaf` | cherry-pick | ported | 3 | 0.88 | cargo test -p codex-app-server thread_resume_replays_pending_command_execution_request_approval --quiet && cargo test -p codex-app-server thread_resume_replays_pending_file_change_request_approval --quiet | Thread-resume replay tests now poll deterministic /responses request counts with unchecked mock sequencing. |

| 33 | `4a0e6dc9163eccf8141a5478711ccdf1630f787c` | cherry-pick | ported | 3 | 0.85 | cargo test -p codex-core --lib snapshot_shell_does_not_inherit_stdin --quiet | Shell snapshot stdin test now records read status and runs with a wider timeout for deterministic EOF assertions. |

| 34 | `75e608343cfea3f667d5d0001b035af51b009cc7` | cherry-pick | ported | 4 | 0.84 | cargo test -p codex-core startup_context --quiet | Realtime startup-context tests now match outbound websocket requests by payload instead of fixed connection/request indices. |

| 35 | `c1f3ef16ec57ccf64c32411b3a2927bc57d80465` | cherry-pick+surgical | ported | 6 | 0.80 | CARGO_INCREMENTAL=0 cargo test -p codex-app-server plugin_list --quiet && CARGO_INCREMENTAL=0 cargo check -p codex-tui --quiet | Starts curated plugin repo sync during TUI startup in parity with app-server startup behavior. |

| 36 | `b15cfe93291185bd4b5df8f3a572d50fc236e706` | cherry-pick | ported | 8 | 0.78 | CARGO_INCREMENTAL=0 cargo test -p codex-backend-client --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-cloud-requirements unauthorized --quiet | Adds explicit backend RequestError status handling and 401-aware cloud-requirements auth recovery flow with new coverage. |

| 37 | `203a70a1915d9e2f308f110f90b9790bb53c09f2` | cherry-pick | ported | 2 | 0.92 | CARGO_INCREMENTAL=0 cargo test -p codex-mcp-server shell_command_approval_triggers_elicitation --quiet | MCP shell approval test now uses native touch/New-Item commands and exact argv-derived expected shell command. |

| 38 | `ad57505ef5ca82a3ba5e182d01b27b572042079f` | cherry-pick | ported | 6 | 0.81 | CARGO_INCREMENTAL=0 cargo test -p codex-core --test all interrupt_long_running_tool_emits_turn_aborted --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core --test all user_shell_cmd_can_be_interrupted --quiet | Turn abort cleanup now drains active tasks before clearing pending approvals to avoid pre-abort approval rejection races. |

| 39 | `e03e9b63eac0a7f374fb4387fbd0b4c49371a461` | cherry-pick | ported | 5 | 0.84 | CARGO_INCREMENTAL=0 cargo test -p codex-core --lib guardian_allows_shell_additional_permissions_requests_past_policy_validation --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-tui --lib experimental_popup_includes_guardian_approval --quiet | Guardian coverage stabilized by aligning sandbox policy setup and replacing brittle popup snapshots with targeted assertions. |

| 40 | `fefd01b9e011380a2f081d8337736602e7e87ee0` | cherry-pick | ported | 3 | 0.90 | CARGO_INCREMENTAL=0 cargo test -p codex-core --test all resume_includes_initial_messages_from_rollout_events --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core --test all resume_includes_initial_messages_from_reasoning_events --quiet | Resume tests now poll until initial_messages reach final persisted shape before asserting. |

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
### Commit `340f9c9ecb0b02a89e88c6dc068809007185f645`

- Upstream intent: Expose experimental skill metadata on exec command approval requests end-to-end (core/protocol/app-server/tui).
- Local overlays touched: Protected app-server and app-server-protocol paths touched; no Indubitably auth or Bedrock runtime/provider selection logic changed.
- Invariants checked: Retained local provider/auth overlays while threading approval metadata through protocol and UI surfaces.
- Risk factors: Cross-crate wire-shape changes with experimental gating and regenerated schema artifacts.
- Strategy selected: cherry-pick+surgical (accepted upstream patch; verified protected-surface behavior with focused tests).
- Confidence: 0.79
- Validation evidence: Focused core/app-server/protocol/tui approval-related test filters passed; tui filter showed only legacy snapshot-format notices.
- Rollback note: Revert this sync commit if approval request payload compatibility or tui approval rendering regresses.
### Commit `f41b1638c98deddd0d8f89d821999d30f73de599`

- Upstream intent: Stabilize OTEL response-span test by including complete follow-up SSE sequence.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock runtime/provider overlays untouched.
- Risk factors: Test-only fixture/event-sequence update in core otel suite.
- Strategy selected: cherry-pick
- Confidence: 0.91
- Validation evidence: Targeted codex-core otel test passed.
- Rollback note: Revert this sync commit if OTEL event-span test behavior regresses.
### Commit `4ad3b59de322dc75c2b257b2eec365309b195ab7`

- Upstream intent: Clarify pending steer follow-up messaging and make Esc interrupt + immediate steer resubmit behavior explicit.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth/provider/runtime overlay paths touched.
- Risk factors: User-visible TUI text/layout and interrupt-flow behavior updates with extensive snapshot changes.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: Focused codex-tui pending_input_preview and interrupted_turn_pending_steers filters passed.
- Rollback note: Revert this sync commit if pending steer queue/interrupt UX regresses or snapshots prove unstable.
### Commit `e6b93841c585f8b56b2c9b38cb07708f278ea227`

- Upstream intent: Add request_permissions as a first-class tool flow with approval request/response plumbing across protocol, app-server, core, and UI surfaces.
- Local overlays touched: Protected app-server and app-server-protocol files touched; local Indubitably auth and Bedrock provider/runtime routing preserved.
- Invariants checked: No regressions in Indubitably/Bedrock exec behavior observed; provider selection and no-openai-responses guardrails remain intact.
- Risk factors: Very large cross-crate behavioral and wire-shape change (new tool semantics, protocol schema additions, approval flow persistence).
- Strategy selected: cherry-pick+surgical (accepted upstream patch; additional local validation and environment recovery for disk-pressure linker failure).
- Confidence: 0.74
- Validation evidence: Core/app-server/protocol request-permissions filters plus codex-tui chatwidget and full codex-exec suite passed after freeing disk space.
- Rollback note: Revert this sync commit if permission-approval flow, tool execution semantics, or app-server approval transport regress.
### Commit `05332b0e96197573a618b6df9adc41752cc87de9`

- Upstream intent: Restore Bazel build metadata for codex-app-server-client by adding BUILD.bazel.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No auth/provider/runtime logic touched.
- Risk factors: Build metadata only; no runtime code path changes.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: codex-app-server-client cargo check passed.
- Rollback note: Revert this sync commit if Bazel rule ownership must be managed differently.
### Commit `06f82c123c6ed295f0ef19b5cbf49cf78bbc092e`

- Upstream intent: Render request_permissions approval calls in TUI overlays and interactive replay paths.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No auth/provider/runtime overlay behavior changed.
- Risk factors: User-visible approval overlay and replay queue behavior update in TUI only.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: Focused codex-tui snapshot and modal-behavior tests for permissions approvals passed.
- Rollback note: Revert this sync commit if permissions approval rendering or replay handling regresses in TUI.
### Commit `2bc3e52a91bb88a0e067a95f8f8559f8711d30e6`

- Upstream intent: Stabilize app list update ordering test by accepting valid interim accessible-only updates before merged final payload.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No auth/provider/runtime behavior paths modified.
- Risk factors: Test-only control-flow change in app-server integration test.
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: Targeted codex-app-server app-list ordering test passed.
- Rollback note: Revert this sync commit if app-list ordering test should retain prior strict sequencing assumption.
### Commit `f23fcd6ced0035f2aa4e34d4f12da4f04c8a7fa4`

- Upstream intent: Refine guardian wording and review-context plumbing while removing model-visible guardian-specific prompt append paths.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth and Bedrock provider/runtime routing untouched.
- Risk factors: Cross-module behavior/copy updates in guardian review flow plus protocol model adjustments.
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: Core guardian and full protocol tests passed; targeted tui guardian popup test passed after freeing disk pressure from local build artifacts.
- Rollback note: Revert this sync commit if guardian approval messaging or review retry-context behavior regresses.
### Commit `3f1280ce1c3ba33c7be769f3b44c4fb610aef3e7`

- Upstream intent: Reduce app-server test flakiness by removing incidental shell-snapshot setup and heavy fixture volume from unrelated checks.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No runtime auth/provider/sandbox behavior changes; test harness only.
- Risk factors: Test-config and fixture volume adjustments in app-server suite.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: Targeted app-server fuzzy-search/auth/account filters passed.
- Rollback note: Revert this sync commit if original test fixture pressure assumptions are needed.
### Commit `615ed0e437afd9bfe0af4a8a8c9a2254227060e1`

- Upstream intent: Stabilize zsh-fork app-server tests by holding command execution until interrupt and broadening valid terminal outcomes.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only async ordering and timing assumptions in app-server integration tests.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: Targeted codex-app-server zsh-fork filter passed (4 tests).
- Rollback note: Revert this sync commit if zsh-fork interrupt/decline integration coverage regresses.

### Commit `6052558a017b89cc62820b388f2cdd3ad5a3feda`

- Upstream intent: Stabilize RMCP pid-file cleanup by treating empty pid files as not-ready and continuing polling.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only timing fix in rmcp-client cleanup path.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: Targeted rmcp-client drop_kills_wrapper_process_group test passed.
- Rollback note: Revert this sync commit if pid-file cleanup test assumptions need to return to strict immediate parsing.

### Commit `5d9db0f9959a5ee2eec75e6bbd2f6b3a543802d2`

- Upstream intent: Stabilize PTY Python REPL readiness by switching from live probe commands to startup marker synchronization.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only synchronization update in utils/pty.
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: Targeted codex-utils-pty python REPL test passed.
- Rollback note: Revert this sync commit if PTY readiness detection needs to revert to probe-based behavior.

### Commit `6b68d1ef661263b0fa6bf9b1e1badffebfd64ee9`

- Upstream intent: Stabilize plan-item app-server tests by using multi-thread runtime and deterministic wiremock request-count synchronization.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only runtime scheduling/synchronization update in app-server tests.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: Targeted codex-app-server plan_mode_ filter passed (2 tests).
- Rollback note: Revert this sync commit if plan-item test harness should return to prior wiremock teardown semantics.

### Commit `0dc242a67229c99ac1de63dbdd5adc1d17481575`

- Upstream intent: Fix websocket initialize ordering race by targeting initialize notifications to the specific connection before general outbound readiness.
- Local overlays touched: Protected app-server surfaces touched (codex-rs/app-server/src/lib.rs, codex-rs/app-server/src/message_processor.rs); no Indubitably/Bedrock-specific logic changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged; only websocket initialize sequencing adjusted.
- Risk factors: Production logic change in initialize path across protected app-server files.
- Strategy selected: cherry-pick+surgical (protected-path review, accepted upstream implementation).
- Confidence: 0.82
- Validation evidence: Focused app-server initialize-notification tests passed.
- Rollback note: Revert this sync commit if websocket initialize notification ordering or readiness gating regresses.

### Commit `10bf6008f4d76c56db86cafb2a45c1fc88024aaf`

- Upstream intent: Stabilize thread-resume replay tests by waiting for settled outbound /responses counts and removing strict sequencing assumptions.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only synchronization updates in app-server integration tests.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: Both targeted thread-resume replay filters passed; first filter needed one rerun after an initialize timeout.
- Rollback note: Revert this sync commit if thread-resume replay tests need stricter sequencing behavior.

### Commit `4a0e6dc9163eccf8141a5478711ccdf1630f787c`

- Upstream intent: Stabilize shell-snapshot stdin behavior by asserting startup read EOF via persisted status instead of timing-sensitive assumptions.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only adjustment in codex-core shell snapshot unit test.
- Strategy selected: cherry-pick
- Confidence: 0.85
- Validation evidence: Targeted codex-core --lib snapshot shell stdin test passed after recovering disk pressure from full-harness linking.
- Rollback note: Revert this sync commit if shell snapshot stdin test semantics need to revert.

### Commit `75e608343cfea3f667d5d0001b035af51b009cc7`

- Upstream intent: Stabilize realtime startup-context tests by synchronizing on the first request containing session.instructions and serializing env-mutating fallback coverage.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Large test-only refactor in codex-core realtime_conversation integration coverage.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: Targeted codex-core startup_context filter passed after full clean to recover disk headroom.
- Rollback note: Revert this sync commit if realtime startup-context test harness needs prior fixed-index assertions.

### Commit `c1f3ef16ec57ccf64c32411b3a2927bc57d80465`

- Upstream intent: Ensure curated plugin metadata sync is kicked off for both app-server and TUI initialization paths.
- Local overlays touched: Protected app-server message processor touched plus tui app startup path.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged; plugin sync initialization only.
- Risk factors: Production startup behavior change spanning app-server and tui initialization lifecycle.
- Strategy selected: cherry-pick+surgical (protected-path review, accepted upstream patch).
- Confidence: 0.80
- Validation evidence: App-server plugin_list filter passed and codex-tui crate compiled cleanly with incremental disabled.
- Rollback note: Revert this sync commit if startup-time curated plugin sync causes regressions or unwanted side effects.

### Commit `b15cfe93291185bd4b5df8f3a572d50fc236e706`

- Upstream intent: Handle cloud requirements 401 responses via auth-refresh recovery path and surface clear user-facing auth failure messaging.
- Local overlays touched: No protected-path overlap; touched backend-client and cloud-requirements auth/error handling code.
- Invariants checked: Indubitably auth and Bedrock provider/runtime overlays unchanged; cloud requirements still fail-closed on unrecoverable auth mismatch.
- Risk factors: Large production behavior change in backend-client error surface and cloud requirements retry/auth recovery logic.
- Strategy selected: cherry-pick
- Confidence: 0.78
- Validation evidence: Backend-client full tests and cloud-requirements unauthorized-focused tests passed with incremental disabled.
- Rollback note: Revert this sync commit if cloud requirements auth recovery introduces incorrect messaging or retry behavior.

### Commit `203a70a1915d9e2f308f110f90b9790bb53c09f2`

- Upstream intent: Stabilize MCP shell approval elicitation test by removing Python startup/quoting variance and asserting on exact forwarded command.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only update in mcp-server suite.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: Targeted codex-mcp-server shell approval elicitation test passed after disk-space cleanup.
- Rollback note: Revert this sync commit if MCP shell approval test should retain Python-based command fixtures.

### Commit `ad57505ef5ca82a3ba5e182d01b27b572042079f`

- Upstream intent: Stabilize interrupted-task cleanup by reordering abort flow so task cancellation is observed before pending approvals are cleared.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Production task-abort sequencing change in core task session management.
- Strategy selected: cherry-pick
- Confidence: 0.81
- Validation evidence: Targeted codex-core abort/interruption tests passed using explicit --test all filters.
- Rollback note: Revert this sync commit if interruption cleanup ordering causes regressions in task/approval lifecycle.

### Commit `e03e9b63eac0a7f374fb4387fbd0b4c49371a461`

- Upstream intent: Stabilize guardian approval coverage across core and tui by tightening policy setup and reducing snapshot churn.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Cross-crate test and helper visibility adjustments in guardian-related coverage paths.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: Focused core guardian and tui guardian popup tests passed after full clean to restore disk headroom.
- Rollback note: Revert this sync commit if guardian approval coverage should retain prior snapshot-based assertions.

### Commit `fefd01b9e011380a2f081d8337736602e7e87ee0`

- Upstream intent: Stabilize resumed rollout message tests by waiting for persisted initial message sequences instead of asserting on transient timing.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: No Indubitably auth or Bedrock provider/runtime behavior changed.
- Risk factors: Test-only polling helper and assertion timing changes in core resume suite.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: Both targeted core resume initial-message tests passed via explicit --test all filters.
- Rollback note: Revert this sync commit if resume suite should keep single-shot post-turn assertions.

## Batch Validation

- [x] CLI default provider smoke
- [x] CLI `--indubitably` smoke
- [x] Targeted crate tests for touched code
- [x] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits: none in this 10-commit batch.
- Manual port TODOs: none; three commits in this batch used cherry-pick+surgical strategy (orders 16-18).
- Batch 2 summary: processed 6 (orders 15-20), blocked 0, skipped 0, branch now ahead 63 / behind 292 vs upstream/main.
- Batch 3 summary: processed 10 (orders 21-30), blocked 0, skipped 0, branch now ahead 79 / behind 293 vs upstream/main.
- Risk notes: full `cargo test -p codex-core` and full `cargo test -p codex-app-server-protocol` remain outside this batch gate; targeted crate filters passed for all processed commits. Disk pressure (os error 28) required `cargo clean` recovery during app-server validation.
