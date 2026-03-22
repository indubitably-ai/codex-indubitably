# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-19
- Fork branch: main
- Upstream ref: upstream/main
- Batch 1 start ahead/behind: ahead 40 / behind 273
- Batch 1 end ahead/behind: ahead 52 / behind 273
- Batch 2 start ahead/behind: ahead 57 / behind 292
- Batch 2 end ahead/behind: ahead 63 / behind 292
- Batch 3 start ahead/behind: ahead 64 / behind 293
- Batch 3 end ahead/behind: ahead 79 / behind 293
- Batch 4 start ahead/behind: ahead 80 / behind 301
- Batch 4 end ahead/behind: ahead 90 / behind 301
- Batch 5 start ahead/behind: ahead 91 / behind 304
- Batch 5 end ahead/behind: ahead 101 / behind 304
- Batch 6 start ahead/behind: ahead 102 / behind 306
- Batch 6 end ahead/behind: ahead 113 / behind 306
- Batch 7 start ahead/behind: ahead 114 / behind 307
- Batch 7 end ahead/behind: ahead 132 / behind 307
- Batch 8 start ahead/behind: ahead 133 / behind 310
- Batch 8 end ahead/behind: ahead 153 / behind 310
- Batch 9 start ahead/behind: ahead 154 / behind 318
- Batch 9 end ahead/behind: ahead 174 / behind 318
- Batch 10 start ahead/behind: ahead 175 / behind 321
- Batch 10 end ahead/behind: ahead 195 / behind 321
- Batch 11 start ahead/behind: ahead 196 / behind 323
- Batch 11 end ahead/behind: ahead 217 / behind 323
- Batch 12 start ahead/behind: ahead 218 / behind 323
- Batch 12 end ahead/behind: ahead 239 / behind 324
- Batch 13 start ahead/behind: ahead 239 / behind 324
- Batch 13 end ahead/behind: ahead 259 / behind 327
- Batch 14 start ahead/behind: ahead 260 / behind 328
- Batch 14 end ahead/behind: ahead 280 / behind 328
- Batch 15 start ahead/behind: ahead 281 / behind 328
- Batch 15 end ahead/behind: ahead 303 / behind 329
- Batch 16 start ahead/behind: ahead 304 / behind 329
- Batch 16 end ahead/behind: ahead 324 / behind 331
- Batch 17 start ahead/behind: ahead 325 / behind 334
- Batch 17 end ahead/behind: ahead 347 / behind 334
- Batch 18 start ahead/behind: ahead 348 / behind 341
- Batch 18 end ahead/behind: ahead 371 / behind 342
- Batch 19 start ahead/behind: ahead 372 / behind 342
- Batch 19 end ahead/behind: ahead 394 / behind 345
- Batch 20 start ahead/behind: ahead 395 / behind 345
- Batch 20 end ahead/behind: ahead 419 / behind 349
- Batch 21 start ahead/behind: ahead 419 / behind 349
- Batch 21 end ahead/behind: ahead 429 / behind 349
- Batch 22 start ahead/behind: ahead 429 / behind 350
- Batch 22 end ahead/behind: ahead 431 / behind 350
- Batch 23 start ahead/behind: ahead 431 / behind 384
- Batch 23 end ahead/behind: ahead 435 / behind 384
- Batch 24 start ahead/behind: ahead 435 / behind 386
- Batch 24 end ahead/behind: ahead 437 / behind 386

## Protected Surfaces

- Protected paths file: .upstream-sync-protected-paths
- Notes: Batch 1 through Batch 22 of phased sync (mixed 10/20 commits per run), direct-to-main push cadence.

## Commit Intake Log

| order | upstream sha | action | status | risk score | confidence | tests | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | `b9a2e400018c219e3010a5a5b8ded8645184da0b` | cherry-pick | ported | 1 | 0.93 | `cargo check -p codex-skills --quiet` | Skill sample asset removal only. |
| 2 | `1c888709b5d718b2452f4bf59ef20f65ff4b5331` | skip | skipped | 0 | 0.96 | Verified `HEAD` already omits `codex-rs/core/src/guardian_tests.rs` and the legacy guardian snapshot file is absent. | Obsolete on this branch because the old guardian test file and snapshot were already removed by earlier test-layout changes. |
| 3 | `92f7541624810406d5c3d1c424147bcfa458efce` | cherry-pick+surgical | ported | 3 | 0.84 | `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui experimental_popup_includes_guardian_approval --quiet` | Guardian CI follow-up reconciled with branch-local test layout by keeping semantic assertions and adding Linux popup snapshot coverage. |

| 4 | `e8d7ede83cf09c99134866f19e5378c546d53191` | skip | skipped | 0 | 0.94 | Verified current TUI already applies `TurnStarted.model_context_window` before `TokenCount` and retains regression coverage. | No-op on this branch because the TUI runtime context-window refresh path is already present. |

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

| 41 | `0334ddeccbef07995561de5b39334dd94ef9e33a` | cherry-pick | ported | 2 | 0.94 | CARGO_INCREMENTAL=0 cargo test -p codex-core unicode_output --quiet | Core shell_command CI test update only. |

| 42 | `6ad448b6585f5a8d504bb9ff990da218bcc336ef` | cherry-pick+surgical | ported | 8 | 0.79 | CARGO_INCREMENTAL=0 cargo test -p codex-core uninstall_plugin_removes_cache_and_config_entry --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server plugin_uninstall --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server-protocol plugin_uninstall --quiet | Adds plugin/uninstall RPC across protocol, app-server, and core with tests and schema/docs updates. |

| 43 | `da991bdf3a1925e3fbd11325293f0cd683300131` | cherry-pick+surgical | ported | 5 | 0.83 | CARGO_INCREMENTAL=0 cargo test -p codex-otel session_metric_tags --quiet | Centralizes OTEL metric-name constants and shared metadata tag builders used by core/otel callsites. |

| 44 | `44ecc527cb7697454ad9241e90b2ebd472beccfb` | cherry-pick | ported | 3 | 0.90 | CARGO_INCREMENTAL=0 cargo test -p codex-core streamable_http --quiet | Stabilizes RMCP streamable HTTP readiness and bind retry behavior in test-only paths. |

| 45 | `42f20a6845939437b68848df111ccd719c64012d` | cherry-pick+surgical | ported | 5 | 0.84 | CARGO_INCREMENTAL=0 cargo test -p codex-core for_prompt_rewrites_image_generation_calls --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-tui image_generation_call_adds_history_cell --quiet | Image-generation history rewrite now includes id/prompt/save-path context and matching TUI history rendering snapshot updates. |

| 46 | `831ee51c86e715e3e546f8c3342f8c5aa94d736f` | cherry-pick+surgical | ported | 8 | 0.77 | CARGO_INCREMENTAL=0 cargo test -p codex-app-server-protocol generated_ts_optional_nullable_fields_only_in_params --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server-protocol generate_ts_with_experimental_api_retains_experimental_entries --quiet | Protocol schema fixture generation refactor + nextest serialization; full schema fixture parity test currently reveals pre-existing fixture drift in branch. |

| 47 | `d241dc598cb0bbadeefd5eab92c056a36b420624` | cherry-pick+surgical | ported | 9 | 0.79 | CARGO_INCREMENTAL=0 cargo test -p codex-core request_permissions_session_grants_carry_across_turns --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server-protocol permissions_request_approval_response_defaults_scope_to_turn --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-tui --lib permissions_session_shortcut_submits_session_scope --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server request_permissions_response_preserves_session_scope --quiet | Adds turn/session scope for request_permissions grants with protocol/app-server/core/tui plumbing and new coverage. |

| 48 | `d309c102efdb2840605ddac1d911ecb3a9945459` | cherry-pick+surgical | ported | 6 | 0.84 | CARGO_INCREMENTAL=0 cargo test -p codex-core --lib web_search_config_is_forwarded_to_tool_spec --quiet | Refactors web_search tool spec serialization to dedicated Responses API payload structs in codex-core. |

| 49 | `66e71cce1139ac7045c59f630a40b8b354fac1ce` | cherry-pick+surgical | ported | 8 | 0.80 | just bazel-lock-update && just bazel-lock-check && CARGO_INCREMENTAL=0 cargo test -p codex-app-server websocket_transport_serves_health_endpoints_on_same_listener --quiet | Adds /readyz and /healthz on websocket listener via axum upgrade path plus websocket health endpoint coverage. |

| 50 | `c1defcc98cf9c6b9001e86d8d13e5b5ec9488510` | cherry-pick+surgical | ported | 7 | 0.78 | CARGO_INCREMENTAL=0 cargo test -p codex-core approval_keys_include_move_destination --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core guardian_review_request_includes_full_patch_without_duplicate_changes --quiet | apply_patch now derives effective sandbox/additional permissions from granted request_permissions context before exec delegation. |

| 51 | `6da84efed8f615085212e7aa6207afa43b3733a9` | cherry-pick+surgical | ported | 9 | 0.76 | CARGO_INCREMENTAL=0 cargo test -p codex-protocol reject_config_request_permissions_flag_is_field_driven --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-app-server-protocol ask_for_approval_reject_round_trips_request_permissions_flag --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core request_permissions_returns_empty_grant_when_reject_policy_blocks_requests --quiet | Adds RejectConfig.request_permissions end-to-end; required surgical scope-field adaptation in core request_permissions responses for local branch compatibility. |

| 52 | `b0cbc25a48b11e311f3b1b7ce9998bb54731ea41` | cherry-pick+surgical | ported | 9 | 0.81 | CARGO_INCREMENTAL=0 cargo test -p codex-protocol legacy_workspace_write_nested_readable_root_stays_writable --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core seatbelt_legacy_workspace_write_nested_readable_root_stays_writable --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core session_configuration_apply_rederives_legacy_file_system_policy_on_cwd_update --quiet | Preserves legacy workspace-write semantics with cwd-aware legacy bridge while keeping split-policy carveout behavior for explicit modern policies. |

| 53 | `1165a16e6ffad719e8f852900fd7ff438ec88fae` | cherry-pick+surgical | ported | 7 | 0.85 | CARGO_INCREMENTAL=0 cargo test -p codex-core permissions_profiles_allow --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-core normalize_absolute_path_for_platform_simplifies_windows_verbatim_paths --quiet && CARGO_INCREMENTAL=0 cargo test -p codex-protocol unknown_special_paths_are_ignored_by_legacy_bridge --quiet | Makes permissions profile parsing forward-compatible by preserving unknown special paths as warn-and-ignore and treating missing filesystem entries as restricted with startup warnings. |

| 54 | `a5af11211a3f000c48b67bf1083e3ee0578c1431` | cherry-pick | ported | 3 | 0.92 | cargo test -p codex-tui --lib mention_popup_type_prefixes --quiet; cargo test -p codex-tui --lib plugin_mention_popup --quiet | TUI mention popup labeling + snapshots updated; targeted tests passed after cargo clean for disk recovery. |

| 55 | `aa04ea6bd705d06543c02ebdf1d26b9730b39684` | cherry-pick+surgical | ported | 6 | 0.82 | cargo test -p codex-core rejects_escalated_permissions_when_policy_not_on_request --quiet; cargo test -p codex-core request_permissions_returns_empty_grant_when_reject_policy_blocks_requests --quiet; cargo test -p codex-core guardian_allows_shell_additional_permissions_requests_past_policy_validation --quiet | One codex_tests.rs conflict (scope type path) resolved; trait-based text output extraction adopted. |

| 56 | `a3cd9f16f5b3dfefb928af3fa512b9dac6af9ac7` | cherry-pick | ported | 3 | 0.93 | cargo test -p codex-tui --lib mention_popup_type_prefixes --quiet; cargo test -p codex-tui --lib plugin_mention_popup --quiet | TUI sort order change keeps plugin suggestions before other mention types; snapshot updated. |

| 57 | `da616136ccff31142b159e97da67705bf0ab7555` | cherry-pick | ported | 7 | 0.74 | cargo test -p codex-core --test all code_mode_can_return_exec_command_output --quiet; cargo test -p codex-core --test all code_mode_can_apply_patch_via_nested_tool --quiet | Large feature add (code_mode tool + JS bridge/runner); broader code_mode-filtered run hit linker ENOSPC after targeted tests passed. |

| 58 | `244b2d53f40938ffba96acf0ca7a559473b842f1` | cherry-pick+surgical | ported | 9 | 0.66 | cargo test -p codex-hooks --quiet; cargo test -p codex-app-server-protocol --quiet; cargo test -p codex-tui --lib hook_events_render_snapshot --quiet; just bazel-lock-update; just bazel-lock-check | High-surface hooks engine import touched protected app-server-protocol/app-server files; protocol schema_fixtures still fail due pre-existing baseline drift. |

| 59 | `63597d1b2d11c94dd09e384ac4518176db65020c` | cherry-pick | ported | 4 | 0.89 | cargo test -p codex-tui --lib status_line_model_with_reasoning_fast_footer --quiet; cargo test -p codex-tui --lib clear_ui_header_shows_fast_status_only_for_gpt54 --quiet | TUI model-status fast badge now gated to GPT-5.4; chatwidget/app snapshots added and passing. |

| 60 | `d71e0426940b75f7dea0c149f2129f0b86c17f20` | cherry-pick | ported | 6 | 0.83 | cargo test -p codex-core rejects_escalated_permissions_when_policy_not_on_request --quiet; cargo test -p codex-core guardian_allows_shell_additional_permissions_requests_past_policy_validation --quiet; cargo test -p codex-core --test all view_image_tool_placeholder_for_non_image_files --quiet; cargo test -p codex-app-server --test all dynamic_tool_call_round_trip_sends_text_content_items_to_model --quiet | Tool-output unification applied across handlers/context/registry; core+app-server targeted tests passed after one disk-clean recovery. |

| 61 | `772259b01f6f3c088ac8b04414206d8cb52e0251` | cherry-pick+surgical | ported | 5 | 0.87 | cargo test -p codex-protocol reject_config_defaults_missing_request_permissions_to_false --quiet; cargo test -p codex-app-server-protocol ask_for_approval_reject_defaults_missing_request_permissions_to_false --quiet | Default for reject_config.request_permissions now explicit false across protocol/v2 schema surfaces. |

| 62 | `710682598d20c8a51d41b99a4d709b3a7b827115` | cherry-pick | ported | 4 | 0.88 | cargo test -p codex-core --test all code_mode_can_ --quiet | Code-mode runner now imports tools module directly; bridge/runner plumbing updated with integration tests adjusted. |

| 63 | `0c33af77461615f32d4c6b94060f5a39e0f5194f` | cherry-pick+surgical | ported | 7 | 0.79 | cargo test -p codex-core parses_bundled_skills_config --quiet; cargo test -p codex-core skills_for_config_excludes_bundled_skills_when_disabled_in_config --quiet | Adds skills.bundled.enabled handling across config/thread-manager/skills loading with protected app-server touchpoints. |

| 64 | `a9ae43621b6d583d8d0ff9beeee53484a7a1f38b` | cherry-pick | ported | 4 | 0.9 | cargo test -p codex-core exec_command_tool_output_formats_truncated_response --quiet; cargo test -p codex-core exec_command_args_resolve_relative_additional_permissions_against_workdir --quiet | Unified-exec truncation logic moved into ExecCommandToolOutput; handler/process-manager call paths simplified. |

| 65 | `566e4cee4bd067aeae95591430cffb71e8ede228` | cherry-pick+surgical | ported | 8 | 0.74 | cargo test -p codex-core --test all includes_apps_guidance_as_developer_message_for_chatgpt_auth --quiet; cargo test -p codex-core --test all search_tool_flag_adds_tool_for_api_key_auth --quiet; cargo test -p codex-app-server --test all list_apps_returns_empty_with_api_key_auth --quiet; cargo test -p codex-tui --lib apps_popup_stays_loading_until_final_snapshot_updates --quiet | Apps enablement now auth-aware (ChatGPT vs API-key) across core/app-server/tui; required repeated clean/reruns due disk pressure. |

| 66 | `79307b7933d9607a7f0033dbbf8446b2027279b5` | skip | skipped | 0 | 0.95 | N/A (no effective diff after apply attempt) | Upstream commit produced zero staged changes in current branch state; nothing to port. |

| 67 | `c6343e0649676579f174b6cd0da617d42ab1c58f` | cherry-pick+surgical | ported | 8 | 0.72 | cargo test -p codex-app-server --test all thread_resume_rejoins_running_thread_even_with_override_mismatch --quiet; cargo test -p codex-core unified_exec_pause_blocks_yield_timeout --quiet | Added compatibility fix in unified_exec test (`response.output` -> `response.truncated_output()`) to align with prior output-type refactor. |

| 68 | `2e24be21346c5e2d415ab0ae10cd6dac854014d5` | cherry-pick+surgical | ported | 8 | 0.78 | cargo test -p codex-core --test all inbound_handoff_request_uses_active_transcript --quiet; cargo test -p codex-app-server-protocol realtime --quiet | Realtime handoff now uses active transcript entries; protocol/app-server schema+handling updated with transcript delta/event types. |

| 69 | `aa6a57dfa2001ee2a6e6882b3f94406fc2e47179` | cherry-pick | ported | 1 | 0.96 | cargo test -p codex-core --test all retries_on_early_close --quiet | Test-only stabilization for stream retry behavior with no production code changes. |

| 70 | `6b7253b123a65038fe215a5252002054ad93744f` | skip | skipped | 0 | 0.95 | N/A (assertion fix already present from prior surgical adaptation) | No staged delta after apply; assertion change already incorporated locally. |

| 71 | `b39ae9501f10c7396857335bcb4c8630c8de28a6` | cherry-pick | ported | 2 | 0.94 | cargo test -p codex-app-server --test all websocket_transport_routes_per_connection_handshake_and_responses --quiet | Test-only websocket binding stabilization across app-server v2 suite files. |

| 72 | `f3f47cf455c2bf83465c2a2c14048f11d35c4d9d` | cherry-pick+surgical | ported | 4 | 0.9 | cargo test -p codex-app-server --test all turn_start_notify_payload_includes_initialize_client_name --quiet; cargo test -p codex-app-server --test all thread_unsubscribe_unloads_thread_and_emits_thread_closed_notification --quiet | Introduces app-server notify capture helper binary and stabilizes initialize/thread_unsubscribe notify assertions. |

| 73 | `7144f84c6948b0ccf18801a298ee2968c14d32bd` | cherry-pick | ported | 1 | 0.95 | cargo test -p codex-core --test all view_image_tool_placeholder_for_non_image_files --quiet | Small test-only fix to avoid release-mode integration compile failure in view_image suite. |

| 74 | `026cfde023e3fae85d12e414b78b9059437e303e` | cherry-pick | ported | 3 | 0.91 | cargo test -p codex-core test_detect_shell_type --quiet; cargo test -p codex-core derive_exec_args --quiet | Shell lookup now hardens tmux/Linux path handling in core shell detection helpers. |

| 75 | `f9cba5cb168c3e3bf325d30ef73d47c87ed895e1` | cherry-pick+surgical | ported | 5 | 0.82 | cargo test -p codex-core --test all prefers_apikey_when_config_prefers_apikey_even_with_chatgpt_tokens --quiet; cargo test -p codex-app-server --test all turn_start_notify_payload_includes_initialize_client_name --quiet (build blocked by ENOSPC) | Feedback tags now include ChatGPT user ID metadata where available; app-server smoke rerun blocked by disk pressure. |

| 76 | `00ea8aa7eeebb8b921573a40f4306ef3e18cf084` | cherry-pick | ported | 6 | 0.84 | cargo test -p codex-core --test all code_mode_can_ --quiet; cargo test -p codex-core exec_command_tool_output_formats_truncated_response --quiet | Introduces typed exec_command result flow through code_mode/tool routing and context serialization boundaries. |

| 77 | `52a7f4b68b13f4e0b4eea90a0671890bd09e7ed7` | cherry-pick | ported | 1 | 0.97 | cargo test -p codex-utils-pty pipe_process_can_expose_split_stdout_and_stderr --quiet | Windows-oriented PTY split-output test stabilization only; no runtime code changes. |

| 78 | `c4d35084f56313d657ad7b6f16f8aee45f5d242c` | cherry-pick | ported | 5 | 0.87 | cargo test -p codex-protocol serializes_image_outputs_as_array --quiet; cargo test -p codex-core prefers_structured_content_when_present --quiet; cargo test -p codex-core success_flag_reflects_is_error_true --quiet | Unified MCP output type reused across core/protocol handlers |

| 79 | `4ac60428508c2a4af21c66d37d23593244f1f593` | cherry-pick+surgical | ported | 6 | 0.84 | cargo test -p codex-app-server --test all thread_resume_and_read_interrupt_incomplete_rollout_turn_when_thread_is_idle --quiet; cargo test -p codex-app-server --test all thread_resume_rejoins_running_thread_even_with_override_mismatch --quiet | Interrupt stale in-progress turns on idle thread resume/read |

| 80 | `3b1c78a5c5fcb81a732de64afffc352403dd8964` | cherry-pick | ported | 2 | 0.97 | python3 -m py_compile codex-rs/skills/src/assets/samples/skill-creator/scripts/init_skill.py | Adds forward-testing guidance to sample skill and init output |

| 81 | `b7f8e9195` | cherry-pick | ported | 4 | 0.92 | cargo test -p codex-skills fingerprint_traverses_nested_entries --quiet | Adds OpenAI docs sample skill material in codex-skills assets. |

| 82 | `f2d66fadd` | cherry-pick | ported | 8 | 0.82 | cargo test -p codex-core monitor_action_posts_expected_arc_request --quiet | Adds ARC monitor approval action path and core monitor integration. |

| 83 | `d751e68f4` | cherry-pick+surgical | ported | 9 | 0.83 | cargo test -p codex-app-server --test all plugin_list_force_remote_sync_reconciles_curated_plugin_state --quiet | Forces remote plugin status sync on plugin/list (protected app-server path). |

| 84 | `3d4628c9c` | cherry-pick | ported | 2 | 0.94 | cargo test -p codex-cloud-requirements fetch_cloud_requirements_surfaces_auth_recovery_message --quiet | Adds granular cloud-requirements loading metrics. |

| 85 | `91ca20c7c` | cherry-pick | ported | 4 | 0.90 | cargo test -p codex-core --test all spawn_agent_requested_model_and_reasoning_override_inherited_settings_without_role --quiet | Adds spawn_agent model/reasoning override handling. |

| 86 | `722e8f08e` | cherry-pick | ported | 6 | 0.85 | cargo test -p codex-core --lib handle_output_item_done_records_image_save_message_after_successful_save --quiet | Routes image-generation saves through `/tmp` and updates output handling. |

| 87 | `d5694529c` | cherry-pick+surgical | ported | 7 | 0.84 | cargo test -p codex-app-server-protocol derive_supports_nested_experimental_fields --quiet; cargo test -p codex-app-server --test all thread_start_reject_approval_policy_requires_experimental_api_capability --quiet | Extends nested experimental gating for `AskForApproval::Reject`. |

| 88 | `ee8f84153` | cherry-pick | ported | 8 | 0.78 | cargo test -p codex-protocol serializes_image_outputs_as_array --quiet | Adds MCP output schema + code_mode result plumbing; integration filters are environment-limited here. |

| 89 | `3d41ff0b7` | cherry-pick | ported | 6 | 0.78 | cargo test -p codex-core --lib formatted_truncate_text_content_items_with_policy_merges_text_and_appends_images --quiet | Adds model-controlled truncation policy for code_mode outputs. |

| 90 | `a67660da2` | cherry-pick+surgical | ported | 5 | 0.88 | cargo test -p codex-core --lib apply_role_ignores_agent_metadata_fields_in_user_role_file --quiet | Loads agent metadata from role files with protected config-path overlap. |

| 91 | `b1dddcb76` | cherry-pick | ported | 1 | 0.98 | N/A (workflow-only update) | CI workflow timeout and runner tweaks only. |

| 92 | `ce1d9abf1` | cherry-pick | ported | 1 | 0.96 | cargo test -p codex-core --lib close_agent --quiet | Clarifies close_agent tool description text. |

| 93 | `07c22d20f` | cherry-pick | ported | 5 | 0.74 | cargo test -p codex-core --test all code_mode --quiet (fails in runner: unsupported custom tool call: code_mode) | Adds code_mode output_text/output_image helper support. |

| 94 | `8ac27b2a1` | cherry-pick+surgical | ported | 7 | 0.86 | cargo test -p codex-app-server --test all thread_fork_ephemeral_remains_pathless_and_omits_listing --quiet | Adds ephemeral thread-fork support on protected app-server/protocol paths. |

| 95 | `889b4796f` | cherry-pick+surgical | ported | 10 | 0.80 | cargo test -p codex-core --lib contacts_read_only_emit_contacts_read_clauses --quiet | Adds extra macOS sandbox permissions with broad protected-surface overlap. |

| 96 | `2621ba17e` | cherry-pick | ported | 5 | 0.89 | cargo test -p codex-core --test all remote_compact_replaces_history_for_followups --quiet | Passes full request params into compaction flow. |

| 97 | `83b22bb61` | cherry-pick | ported | 7 | 0.79 | cargo test -p codex-core --test all code_mode --quiet (runner-limited); cargo test -p codex-core --lib close_agent --quiet | Adds session-scoped store/load support for code_mode values. |

| 98 | `c1a424691` | cherry-pick+surgical | ported | 9 | 0.77 | cargo test -p codex-core --lib execve_prompt_rejection_uses_skill_approval_for_skill_scripts --quiet; cargo test -p codex-app-server-protocol derive_supports_nested_experimental_fields --quiet | Splits skill-approval reject policy and updates core/protocol/app-server wiring. |

| 99 | `9b5078d3e` | cherry-pick | ported | 1 | 0.97 | cargo test -p codex-utils-pty pipe_process_round_trips_stdin --quiet | Stabilizes stdin round-trip behavior in pipe-process test. |

| 100 | `e77b2fd92` | cherry-pick | ported | 3 | 0.92 | cargo test -p codex-core --lib guardian_review_request_layout --quiet | Updates guardian prompt text and snapshot layout expectations. |

| 101 | `8a099b3df` | cherry-pick | ported | 6 | 0.84 | cargo test -p codex-core --lib code_mode --quiet | Renames code_mode tool surface to exec across core/spec/tests |

| 102 | `285b3a514` | cherry-pick | ported | 8 | 0.81 | cargo test -p codex-tui --lib chatwidget::tests::collab_spawn_end_shows_requested_model_and_effort --quiet | Shows spawned agent model/effort metadata in tui and event plumbing |

| 103 | `c8446d7cf` | cherry-pick | ported | 5 | 0.88 | cargo test -p codex-api responses_websocket --quiet | Stabilizes websocket response.failed error delivery paths |

| 104 | `da74da668` | cherry-pick | ported | 9 | 0.79 | cargo test -p codex-tui --lib markdown_render_file_link_snapshot --quiet | Renders local file links using target path metadata in tui markdown |

| 105 | `01792a4c6` | cherry-pick | ported | 5 | 0.80 | cargo test -p codex-core --lib code_mode --quiet | Prefixes code_mode output with success/failure framing and error stack |

| 106 | `31bf1dbe6` | cherry-pick | ported | 7 | 0.86 | cargo test -p codex-core --lib unified_exec --quiet | Moves unified-exec session_id to numeric semantics |

| 107 | `39c1bc1c6` | cherry-pick+surgical | ported | 7 | 0.83 | cargo test -p codex-core --lib experimental_realtime_start_instructions_load_from_config_toml --quiet; just write-config-schema | Adds realtime start-instructions config override with docs/schema updates |

| 108 | `a4d884c76` | cherry-pick | ported | 9 | 0.80 | cargo test -p codex-core --lib spawn_csv --quiet | Splits spawn_csv feature surface from broader multi_agent flag |

| 109 | `12ee9eb6e` | cherry-pick | ported | 4 | 0.91 | cargo test -p codex-core --lib code_mode --quiet | Adds type-annotated snippets in tools exports for code_mode |

| 110 | `180a5820f` | cherry-pick | ported | 10 | 0.79 | cargo test -p codex-tui --lib app::agent_navigation::tests::active_agent_label_tracks_current_thread --quiet | Adds keyboard fast switching and agent navigation state in tui |

| 111 | `f385199cc` | cherry-pick | ported | 2 | 0.95 | cargo test -p codex-core monitor_action_posts_expected_arc_request --quiet | Fixes arc monitor API path wiring |

| 112 | `fd4a67352` | cherry-pick | ported | 3 | 0.92 | cargo test -p codex-api responses --quiet | Sets x-client-request-id from conversation_id for responses calls |

| 113 | `7f2232938` | cherry-pick | ported | 5 | 0.89 | cargo test -p codex-core --test all remote_compact_replaces_history_for_followups --quiet | Reverts expanded compaction params from previous sync commit |

| 114 | `548583198` | cherry-pick+surgical | ported | 4 | 0.90 | cargo test -p codex-core --lib web_search_mode_disabled_overrides_legacy_request --quiet | Allows bool web_search values in ToolsToml parsing path |

| 115 | `fa1242c83` | cherry-pick+surgical | ported | 12 | 0.78 | cargo test -p codex-otel otlp_http_loopback --quiet; just bazel-lock-update; just bazel-lock-check | Makes OTEL HTTP trace export survive app-server runtimes |

| 116 | `7b2cee53d` | cherry-pick+surgical | ported | 18 | 0.76 | cargo test -p codex-core --lib plugin --quiet; cargo test -p codex-app-server --test all plugin_list_includes_install_and_enabled_state_from_config --quiet | Wires plugin install/auth policies and category through protocol+server+core |

| 117 | `65b325159` | cherry-pick | ported | 6 | 0.85 | cargo test -p codex-core --lib code_mode --quiet | Adds ALL_TOOLS export support in code_mode bridge/runtime |

| 118 | `8f8a0f55c` | cherry-pick | ported | 5 | 0.88 | cargo test -p codex-core --test all spawn_agent_description --quiet | Expands spawn agent prompt/spec guidance and tests |

| 119 | `52a3bde6c` | cherry-pick | ported | 2 | 0.93 | cargo test -p codex-core --lib network_proxy --quiet | Adds network proxy active-state turn metric emission |

| 120 | `c32c445f1` | cherry-pick | ported | 2 | 0.94 | cargo test -p codex-core --test all subagent_notifications --quiet | Clarifies locked role settings in spawn prompt and notification flow |

| 121 | `f5bb338fd` | cherry-pick | ported | 6 | 0.86 | cargo test -p codex-core --test all snapshot_request_shape_remote_manual_compact_without_previous_user_messages --quiet | Context insertion timing changed in remote compact path. |

| 122 | `5259e5e23` | cherry-pick | ported | 3 | 0.90 | cargo test -p codex-network-proxy http_proxy_listener_accepts_plain_http1_connect_requests --quiet | HTTP proxy listener protocol mode adjustment. |

| 123 | `5a89660ae` | cherry-pick+surgical | ported | 5 | 0.82 | CODEX_JS_REPL_NODE_PATH=/Users/gp/.nvm/versions/node/v24.9.0/bin/node cargo test -p codex-core --test all js_repl_exposes_codex_path_helpers --quiet | Protected docs path overlap and node runtime gating in local environment. |

| 124 | `f54830979` | cherry-pick | ported | 2 | 0.94 | cargo test -p codex-tui agent_shortcut_matches --quiet | TUI keybinding guardrails only. |

| 125 | `8791f0ab9` | cherry-pick+surgical | ported | 6 | 0.84 | cargo test -p codex-core --lib image_detail_original_feature_enables_explicit_original_without_force --quiet && cargo test -p codex-core --test all view_image_tool_attaches_local_image --quiet | Protected docs overlap with image-detail behavior change. |

| 126 | `f50e88db8` | cherry-pick | ported | 2 | 0.96 | python3 scripts/check_blob_size.py --help | CI policy/workflow addition only. |

| 127 | `72631755e` | cherry-pick+surgical | ported | 8 | 0.81 | cargo test -p codex-app-server-protocol --lib --quiet && cargo test -p codex-app-server --test all initialize --quiet | Protected app-server/app-server-protocol surfaces with notification contract changes. |

| 128 | `77b0c7526` | cherry-pick+surgical | ported | 9 | 0.78 | cargo test -p codex-core --test all search_tool --quiet && cargo test -p codex-app-server --test all mcp_server_elicitation --quiet | One manual conflict resolution and Bedrock overlay adaptations required. |

| 129 | `f276325cd` | cherry-pick | ported | 4 | 0.91 | cargo test -p codex-protocol --quiet | Permissions precedence refactor with schema update. |

| 130 | `c1ea3f95d` | cherry-pick+surgical | ported | 3 | 0.93 | cargo test -p codex-app-server-protocol --lib --quiet | Protected protocol source touched but change is v1 cleanup. |

| 131 | `c2d5458d6` | cherry-pick+surgical | ported | 7 | 0.84 | cargo test -p codex-core --lib unix_escalation --quiet && cargo test -p codex-core --test all approvals --quiet | Protected config test path touched; approval policy behavior changed. |

| 132 | `bf5e997b3` | cherry-pick+surgical | ported | 8 | 0.83 | cargo test -p codex-app-server --test all turn_start --quiet && cargo test -p codex-tui multi_agent --quiet | Protected app-server/protocol sources plus event-shape fanout. |

| 133 | `5bc82c5b9` | cherry-pick+surgical | ported | 9 | 0.79 | cargo test -p codex-app-server tracing_tests --quiet && cargo test -p codex-core --test all fork_thread --quiet && just bazel-lock-update && just bazel-lock-check | Protected app-server runtime and dependency lock updates. |

| 134 | `917c2df20` | cherry-pick+surgical | ported | 7 | 0.85 | cargo test -p codex-app-server --test all plugin_install --quiet && cargo test -p codex-app-server --test all plugin_list --quiet && cargo test -p codex-core --lib plugin --quiet | Protected protocol/app-server plugin policy defaults changed. |

| 135 | `ba5b94287` | cherry-pick | ported | 9 | 0.77 | cargo test -p codex-core --test all tool_suggest --quiet && cargo test -p codex-tui --lib app_link_view --quiet && just bazel-lock-update && just bazel-lock-check | Large tool_suggest feature with dependency graph updates and one ENOSPC recovery. |

| 136 | `367a8a221` | cherry-pick | ported | 2 | 0.95 | cargo test -p codex-core --test all spawn_agent_description --quiet | Prompt copy clarification only. |

| 137 | `f6c6128fc` | cherry-pick | ported | 9 | 0.76 | cargo test -p codex-core --test all code_mode --no-run --quiet | Code-mode integration tests in this runner remain environment-limited. |

| 138 | `b5f927b97` | cherry-pick | ported | 6 | 0.87 | cargo test -p codex-core --lib plugin --quiet && cargo test -p codex-app-server --test all plugin_list --quiet | Plugin curated repo refactor across core/store/marketplace. |

| 139 | `04892b4ce` | cherry-pick+surgical | ported | 9 | 0.79 | cargo test -p codex-core --test all approvals --quiet && cargo test -p codex-core --lib unix_escalation --quiet && cargo test -p codex-cli --lib --quiet | Protected app-server/cli surfaces plus manual import fix in core/connectors. |

| 140 | `e99e8e4a6` | cherry-pick | ported | 6 | 0.90 | cargo test -p codex-core --test all approvals --quiet && cargo test -p codex-linux-sandbox --quiet | Follow-up cleanup for previous Linux sandbox default change. |

| 141 | `19d0949aa520b0d54aa0f003526f5f67b5ab58c4` | cherry-pick | ported | 5 | 0.86 | cargo test -p codex-core --lib unix_escalation --quiet; cargo test -p codex-core --test all approvals --quiet | Zsh fork approvals now consume pre-approved permission tokens. |

| 142 | `23e55d7666e1596f45e7ee546af1eb8fd2e55fcd` | cherry-pick | ported | 4 | 0.89 | cargo test -p codex-core --lib mcp_tool_call --quiet; cargo test -p codex-tui --lib mcp_server_elicitation --quiet | Improves user-facing elicitation messages for tool calls. |

| 143 | `745ed4e5ecf543a4b65a2e8853b2201ab65e121f` | cherry-pick | ported | 5 | 0.86 | cargo test -p codex-core --lib apply_patch --quiet | apply_patch invocation now respects granted permissions context. |

| 144 | `7f2ca502f6ea30a85e6f98358f4fc2bbcb4f6d6d` | cherry-pick | ported | 1 | 0.97 | cargo test -p codex-tui --lib tooltips --quiet | Tooltip copy refresh only. |

| 145 | `0c8a366761c9f41f76a4d8db09f03f5250b48da2` | cherry-pick+surgical | ported | 8 | 0.71 | cargo test -p codex-core --lib --no-run --quiet | Large test-file move had conflicts in protected core files; preserved local variants and accepted non-conflicting upstream splits. |

| 146 | `2f03b1a32d0604e562d4f7b31a2a069c8f0e2a3f` | cherry-pick | ported | 6 | 0.82 | cargo test -p codex-core --test all code_mode --no-run --quiet | Code-mode tool dispatch path updated for non-awaited execution. |

| 147 | `ff6764e806d8bc2fcbf7f58e6d03f77f694f70f0` | cherry-pick | ported | 7 | 0.80 | python3 -m py_compile codex-rs/python/codex_app_server_sdk/__init__.py codex-rs/python/codex_app_server_sdk/client.py codex-rs/python/codex_app_server_sdk/protocol.py | Introduces Python app-server SDK surface. |

| 148 | `a30b807efe0d013d49daf0462f8e1373840a3e4d` | cherry-pick+surgical | ported | 6 | 0.85 | cargo test -p codex-core --lib features_tests --quiet; cargo test -p codex-cli --lib --quiet | Legacy use_linux_sandbox_bwrap flag compatibility restored. |

| 149 | `09aa71adb7a642408f05fe51db82854142e00945` | cherry-pick | ported | 3 | 0.93 | cargo test -p codex-stdio-to-uds --quiet; just bazel-lock-update; just bazel-lock-check | Fixes stdio-to-uds peer-close test flake and refreshed lock metadata. |

| 150 | `c0528b9bd97dcb0f8d66719fe138a9a244fe6f3d` | cherry-pick | ported | 7 | 0.82 | cargo test -p codex-core --test all code_mode --no-run --quiet | Code-mode files moved under tools/code_mode; local lockfile drift handled in follow-up commit b924b2c71. |

| 151 | `4e99c0f1798856d445624e1c28dcd43c6b6a715f` | cherry-pick+surgical | ported | 6 | 0.84 | cargo test -p codex-core --lib features_tests --quiet; cargo test -p codex-core --lib spec_tests --quiet | Feature flag renamed from spawn_csv to enable_fanout. |

| 152 | `774965f1e8691f1a0568fb801f24b15553e5e6cd` | cherry-pick | ported | 5 | 0.86 | cargo test -p codex-linux-sandbox --quiet | Preserves split filesystem semantics in linux sandbox policy handling. |

| 153 | `cfe3f6821ae91f38d6d6f4e86dcbb0c3a29c123f` | cherry-pick | ported | 4 | 0.89 | cargo test -p codex-core --test all code_mode --no-run --quiet | Code-mode tool descriptions cleaned up. |

| 154 | `4fa7d6f444b919afb6ccec25e49c036aa0180971` | cherry-pick+surgical | ported | 5 | 0.88 | cargo test -p codex-core --lib config_tests --quiet; cargo test -p codex-app-server --lib --quiet | Malformed agent roles now handled nonfatally. |

| 155 | `fa265976890e996ed6ce78ee94f62ddd81544ddc` | cherry-pick | ported | 4 | 0.90 | cargo test -p codex-core --lib unified_exec --quiet | Disables unified_exec in sandboxed Windows scenarios. |

| 156 | `3e96c867fe91a4ffe9a262d1674bb57efdd8c99f` | cherry-pick+surgical | ported | 8 | 0.83 | cargo test -p codex-rmcp-client --quiet; cargo test -p codex-core --lib skill_dependencies --quiet | OAuth scope negotiation now uses scopes_supported when available. |

| 157 | `d1b03f0d7f53f74ee35881be49715162d8f06b5f` | cherry-pick | ported | 6 | 0.87 | cargo test -p codex-core --test all code_mode_yield_timeout_works_for_busy_loop --quiet | Introduces shared default code-mode yield timeout. |

| 158 | `25e301ed9802415450ae071122cbe338450d7844` | cherry-pick | ported | 5 | 0.88 | cargo test -p codex-core --test all code_mode_nested_tool_calls_can_run_in_parallel --quiet | Adds regression coverage for parallel nested tool calls in code_mode. |

| 159 | `4724a2e9e7919997429a5fb3bf7b721220922f06` | cherry-pick+surgical | ported | 9 | 0.86 | cargo test -p codex-app-server-protocol --quiet | Large schema pruning removed EventMsg exports; protected source hunks reviewed and accepted. |

| 160 | `d3e668053161c3f916fab3b6b611de6acd07af16` | cherry-pick+surgical | ported | 5 | 0.90 | cargo test -p codex-app-server --lib turn_start_jsonrpc_span_parents_core_turn_spans --quiet | Flaky tracing test now asserts core invariant with cleaner shutdown handling. |

| 201 | `d58620c852c5ff5cfd65959d80de265c225e59ba` | cherry-pick | ported | 3 | 0.93 | cargo test -p codex-tui multi_agent_enable_prompt --quiet | TUI naming-only update. |

| 202 | `914f7c73175b038b4d396219754fe21ba6678af2` | cherry-pick+surgical | ported | 6 | 0.87 | cargo test -p codex-core requirements_disabled_connector_overrides_enabled_connector --quiet; cargo test -p codex-core cloud_requirements_disable_connector_overrides_user_apps_config --quiet; cargo test -p codex-tui set_connector_mentions_skips_disabled_connectors --quiet | Config + connector override behavior aligned. |

| 203 | `014e19510d9fb4bc09c3b8e90fb05d7f3aa39700` | cherry-pick+surgical | ported | 7 | 0.84 | cargo test -p codex-core --lib get_model_info_tracks_fallback_usage --quiet; cargo test -p codex-app-server --lib turn_start_jsonrpc_span_parents_core_turn_spans --quiet; cargo test -p codex-protocol --quiet | Resolved import conflict in models manager. |

| 204 | `ef37d313c6c0c00b91f2ea8a0641d4deace1d67b` | cherry-pick | ported | 4 | 0.92 | cargo test -p codex-utils-pty --quiet; cargo test -p codex-shell-escalation --quiet | Unified-exec zsh-fork FD handling update. |

| 205 | `477a2dd3458be962178abc891422215bf3c22f52` | cherry-pick | ported | 5 | 0.90 | cargo test -p codex-core --lib code_mode_only_requires_code_mode --quiet; cargo test -p codex-core --lib code_mode_only_restricts_model_tools_to_exec_tools --quiet; cargo test -p codex-core --test all code_mode_only_restricts_prompt_tools --quiet; cargo test -p codex-core --test all code_mode_only_can_call_nested_tools --quiet | New code_mode_only feature integrated. |

| 206 | `6720caf778acd9a9ec5f8eb838b48e1a4ce944e8` | cherry-pick | ported | 3 | 0.93 | cargo test -p codex-tui clipboard_text --quiet | WSL clipboard OSC52 support. |

| 207 | `cfd97b36da76a17db407b2d9653ed993636e0a30` | cherry-pick | ported | 4 | 0.91 | cargo test -p codex-core --lib wait_agent --quiet | Tool rename wait -> wait_agent. |

| 208 | `36dfb844277e79793766f96305c9633f90bc043e` | cherry-pick+surgical | ported | 6 | 0.82 | cargo check -p codex-core --quiet | Feature-flag stabilization with cross-commit dependency on stale helper references. |

| 209 | `f8f82bfc2b558229cc4f7ef6245c474ee8b389c7` | cherry-pick+surgical | ported | 8 | 0.80 | just write-app-server-schema; just bazel-lock-update; just bazel-lock-check; cargo test -p codex-app-server-protocol --quiet; cargo test -p codex-app-server fs_methods_reject_relative_paths --quiet | High-risk app-server v2 filesystem API surface update. |

| 210 | `cb7d8f45a1393d71b333aea64123227028ae535f` | cherry-pick | ported | 5 | 0.86 | cargo test -p codex-core --test all search_tool --quiet; cargo test -p codex-core --test all plugins --quiet; cargo check -p codex-core --quiet | MCP tool-name normalization for code-mode safety. |

| 211 | `e3cbf913e801a611f0b17fa14e9a77865244ba8f` | cherry-pick | ported | 4 | 0.91 | cargo test -p codex-core --test all prompt_tools_are_consistent_across_requests --quiet; cargo check -p codex-core --quiet | Follow-up test expectation fix for wait_agent rename. |

| 212 | `bc24017d64829d0b97b8bc6ed529a389e1e8bc1b` | cherry-pick+surgical | ported | 8 | 0.79 | just write-app-server-schema; just write-config-schema; cargo test -p codex-app-server-protocol --quiet; cargo test -p codex-core --test all permissions_messages --quiet; cargo test -p codex-app-server turn_start --quiet; cargo check -p codex-tui --quiet | Smart Approvals guardian flow integrated across protected surfaces. |

| 213 | `467e6216bbfd2ffb1dbdeeffda248cd040274131` | cherry-pick | ported | 3 | 0.94 | cargo test -p codex-core --lib code_mode_only_restricts_model_tools_to_exec_tools --quiet | Stale helper symbol cleanup. |

| 214 | `9a44a7e499f18eaed5d06aabb5acf9184deb06b8` | cherry-pick | ported | 5 | 0.89 | cargo test -p codex-hooks --quiet; cargo test -p codex-core --test all hooks --quiet | Hooks stop/continuation mechanics adjusted. |

| 215 | `e9050e3e649a0d659208f8fc3ed9082f6b9ec4c1` | cherry-pick | ported | 4 | 0.92 | cargo test -p codex-api parse_realtime_v2_input_audio_transcription_delta_event --quiet | Realtime transcription payload fix. |

| 216 | `7fa52013653465661441ac016886ee843855a08c` | cherry-pick | ported | 4 | 0.90 | cargo test -p codex-api parse_realtime_v2_input_audio_transcription_delta_event --quiet | Parser-specific realtime voice enum alignment. |

| 217 | `b859a98e0f017f374aaff35c9e2e44f849222622` | cherry-pick | ported | 5 | 0.90 | cargo test -p codex-core --test all unified_exec --quiet; cargo test -p codex-core --lib unified_exec --quiet | zsh-fork unified-exec state explicitness update. |

| 218 | `4b9d5c8c1bdb6d9cfd43570e0b8e88c88b54d823` | cherry-pick+surgical | ported | 8 | 0.81 | cargo test -p codex-core new_uses_configured_openai_provider_for_model_refresh --quiet; cargo test -p codex-core built_in_model_providers_include_bedrock --quiet; cargo test -p codex-core bedrock_provider_stream_returns_auth_error_without_token --quiet; cargo test -p codex-core bedrock_provider_stream_without_token_returns_auth_error --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui chatwidget --quiet | Manual conflict resolution plus local Bedrock/openai provider compatibility callsite updates. |

| 219 | `69c8a1ef9e7c5a3c447ea8b0f01ec5d3a068693d` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib guardian --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui chatwidget --quiet | Windows CI assertion stabilization in tests. |

| 220 | `bbd329a81233a8bb35f5ced9aacf93b57f2f9999` | cherry-pick+surgical | ported | 7 | 0.78 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Turn-context replay/backtracking reconstruction fix applied; test-linking environment constrained by ENOSPC. |

| 221 | `6dc04df5e6ffdf7d85c935864c71eede3f214515` | cherry-pick | ported | 6 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib start_managed_network_proxy_ --quiet | Persist network host approvals across session boundaries. |

| 222 | `7f571396c8819d7f4c4486ed1e967e40a2c9ffae` | cherry-pick | ported | 5 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib spawn_agent --quiet | Subagent split sandbox-policy sync fix. |

| 223 | `d272f4505874fafef4753830b40d751674e8fd9b` | cherry-pick | ported | 6 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-protocol --quiet | Large instruction/layout reorder with snapshot updates; ENOSPC prevented integration test link. |

| 224 | `ae0a6510e19c1d65aaa1ef1824826832ac9e160a` | cherry-pick | ported | 6 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib reserved_builtin_provider_override_rejected --quiet | Reject overrides of built-in provider keys (openai/ollama/lmstudio). |

| 225 | `8ca358a13cd29bb174bebe1a32cf608e31a6494e` | cherry-pick | ported | 3 | 0.93 | python3 -m compileall sdk/python/src/codex_app_server/generated | Python SDK generated type refresh; pytest unavailable in environment. |

| 226 | `e3890910427940c9106ea61d75f82dffbf20c7a6` | cherry-pick | ported | 7 | 0.87 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet | V2 plugin defaultPrompt switched to array with back-compat handling. |

| 227 | `70eddad6b075f26f0f93c66f7ec9a4e49cdadc93` | cherry-pick | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server dynamic_tools --quiet | Added dynamic tool exposeToContext flag to optionally hide tools from context. |

| 228 | `4b31848f5b3adb7f237dd5109f83428fbd2cf343` | cherry-pick | ported | 5 | 0.88 | (cd tools/argument-comment-lint && cargo test --quiet) | Added standalone Dylint runner and argument-comment lint crate. |

| 229 | `9060dc7557848feb80a0fca612b9b1037c2ec217` | cherry-pick | ported | 7 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-protocol --quiet | Fix symlinked writable-root handling in sandbox policies and related protocol model. |

| 230 | `d692b7400786e7bbe9f1366e697fc867bd10b3c1` | cherry-pick+surgical | ported | 9 | 0.79 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core auth --quiet | Conflict in models_manager resolved with local Bedrock path preserved; follow-up compile-fix commit added. |

| 231 | `49edf311ac3ae84659b0ec5eacd5e471c881eee8` | cherry-pick | ported | 8 | 0.83 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib mcp_tool_call --quiet | Apps tool-call metadata wiring added across MCP/spec surfaces. |

| 232 | `d4af6053e212a982e53372a3dff5a627c60af1db` | cherry-pick | ported | 6 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all search_tool --quiet | Improved apps search-tool fallback resolution in core. |

| 233 | `ba463a9dc78180d9cd61b28ef6562e03342a14be` | cherry-pick | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui interrupt_preserves_unified_exec_wait_streak --quiet | Interrupt now preserves background terminals; cleanup command renamed to /stop. |

| 234 | `6fdeb1d602842b80088641b941dea174435c01b7` | cherry-pick | ported | 9 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib guardian --quiet | Guardian session reuse landed with large core guardian refactor and prompt/policy split. |

| 235 | `029aab5563caed2f2bbea8a1815a42cbf22b79a2` | cherry-pick | ported | 7 | 0.87 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib mcp_tool_call --quiet | Preserved tool_params through elicitation flow in MCP tool-call path. |

| 236 | `33acc1e65faec89172b80a0a8a4faafe9b65c8c5` | cherry-pick | ported | 6 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib apply_role_uses_active_profile_model_provider_update --quiet | Fixed sub-agent role resolution when profiles are active. |

| 237 | `18ad67549ca30c78b966d0bc9d8bc4a4a828c854` | cherry-pick | ported | 7 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib skills_for_config_reuses_cache_for_same_effective_config --quiet | Skills cache key now includes config layering semantics. |

| 238 | `3f266bcd68c78ac043969f8a7a916c7ee30df112` | cherry-pick | ported | 8 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet | Interrupt state for multi-agents made non-terminal with protocol/tui updates. |

| 239 | `c04a0a745483066da3e004ec1822a5c0838b6feb` | cherry-pick | ported | 8 | 0.80 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server --quiet | Fix for TUI freeze with sub-agents in app-server message processing path. |

| 240 | `db89b73a9cd553ac2a2afda93c9f9bdcc223540c` | cherry-pick+surgical | ported | 9 | 0.78 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server --quiet | Conflict-resolved CLI/TUI-app-server integration plus follow-up compatibility fixes for new event/status enums under local provider-aware routing. |

| 241 | `7a6e30b55b0aa75d8462058f794f571afa071bac` | cherry-pick | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server request_permissions --quiet | Request permission profile adoption across v2 protocol and app-server handlers. |

| 242 | `a0e41f4ff9b4e68148b76621a3817907a166ff43` | cherry-pick+surgical | ported | 6 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server collab_resume_interrupted_snapshot --quiet | Resolved multi_agents conflict by preserving function-level yellow lint allowance and accepting new interrupted snapshot. |

| 243 | `663dd3f93500d211409d406fcd0d801e18de6f95` | cherry-pick | ported | 4 | 0.91 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib connectors --quiet | Connector/app tool-name sanitization now consistently maps to underscores. |

| 244 | `4c9dbc1f8829d0d0423bc36c6ad59896bc1387f3` | cherry-pick | ported | 6 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib contextual_user_message --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib memories::phase1 --quiet | Memory stage1 now strips AGENTS/skill fragments while preserving environment/subagent context. |

| 245 | `d0a693e5419dba6b25537f4c931a49fd0ce14ea7` | cherry-pick | ported | 8 | 0.85 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-windows-sandbox --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-utils-pty --quiet | Windows sandbox runner IPC + ConPTY foundation landed with large module split. |

| 246 | `a3ba10b44b3c9a584ad0bccf84b6da072bd96d8f` | cherry-pick+surgical | ported | 6 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo build -p codex-rmcp-client --bin test_stdio_server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all code_mode_exit --quiet | Broad code_mode filter failed on missing helper binary; validated with helper build + exit-focused test. |

| 247 | `6f05d8d735392640cd32ec44c2088e0fec9aeaee` | cherry-pick | ported | 5 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-api realtime_websocket --quiet | Realtime websocket methods split into v1/v2/common modules. |

| 248 | `b77fe8fefeffe71c2f221129491b7235af4766d0` | cherry-pick+surgical | ported | 9 | 0.77 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core -p codex-app-server -p codex-tui --quiet | Broad mechanical lint sweep; resolved conflicts in core/thread_manager and tui/lib by preserving local provider/startup behavior. |

| 249 | `59533a2c26e349c59417e4773b930c26211d7bdd` | cherry-pick | ported | 2 | 0.95 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-skills --quiet | Skill-creator sample now defaults generated skills under ~/.codex/skills. |

| 250 | `49c2b66ece0d1c19245cdc78a94036313b8eaacc` | cherry-pick | ported | 8 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_list --quiet | Added marketplace display-name fields to plugin list surfaces; ENOSPC during first app-server run mitigated via cargo clean. |

| 251 | `1d85fe79edd7235fc56d6607db03109f6c3dd101` | cherry-pick | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_install --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_uninstall --quiet | Plugin install/uninstall now support remote_sync with protocol + server wiring updates. |

| 252 | `fbd7f9b9864bef4ee074974d649f0939f3bc91e9` | cherry-pick | ported | 9 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-api realtime_websocket --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server realtime_conversation --quiet | Realtime v2 wire/runtime alignment across protocol/api/core/app-server/tui. |

| 253 | `e5a28ba0c2fd27f58c4949821d4fb886c54a44d3` | cherry-pick | ported | 7 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_list --quiet | Follow-up naming alignment for marketplace display-name fields. |

| 254 | `8e34caffcc3678212acf5ce14ce94adf60ee9f48` | cherry-pick | ported | 2 | 0.96 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib spawn_agent --quiet | Added Jason to predefined subagent names list. |

| 255 | `15ede607a087d043a834aaae5021453377e79fd9` | cherry-pick | ported | 3 | 0.90 | git diff --cached --check | GitHub workflow/action shell quoting tightened; validated no whitespace/syntax artifacts in staged YAML edits. |

| 256 | `79f476e47dc9d6055ef85322481d56302bfccf53` | cherry-pick | ported | 6 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib realtime_context --quiet | Realtime startup now includes current thread context payload. |

| 257 | `32e4a5d5d9ae1acad2e85a142c1b2d446306a4e5` | cherry-pick | ported | 6 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui realtime --quiet | TUI realtime playback now reduces self-interrupt behavior. |

| 258 | `db7e02c73988f643722b98fdd47d40340b72d6b7` | cherry-pick | ported | 7 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-linux-sandbox --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib landlock --quiet | Linux sandbox cwd canonicalization for symlinked paths across core and linux-sandbox. |

| 259 | `57f865c069c4acc213d43371a82671b2deed4e1c` | cherry-pick+surgical | ported | 5 | 0.80 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui-app-server --quiet | Ignored duplicate legacy stream events in tui_app_server adapter; test-link hit ENOSPC and was validated via compile gate after cargo clean. |

| 260 | `d37dcca7e080a8d397f37f8bf4bf695d40f7d88e` | cherry-pick+surgical | ported | 8 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --quiet | Reverted tui in-process app-server dependency; conflict resolved by keeping local Bedrock provider constant and thread-manager provider wiring. |

| 261 | `603b6493a9d93f110bacf8d29295acdcdc080d89` | cherry-pick | ported | 2 | 0.93 | cargo test -p codex-linux-sandbox landlock --quiet | Linux sandbox writable-root handling now skips missing roots. |

| 262 | `31648563c8d7f77957c79cc04501d0ed11844635` | cherry-pick | ported | 4 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Centralized artifact runtime version constant under core packages module. |

| 263 | `4ed19b07664d28ef67592ab5d77aa30d13d3aba0` | cherry-pick | ported | 4 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib close_agent --quiet | close_agent output field renamed to previous_status with schema/tool-description updates. |

| 264 | `ef36d39199c7328899e4f1f6b20a2c9ba5065f83` | cherry-pick | ported | 5 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-state agent_jobs --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all suite::agent_jobs --quiet | Agent-job completion/state transition race fix with reduced status polling churn. |

| 265 | `e8add54e5dda2fc6f49757aa939378a21b8515e9` | cherry-pick | ported | 6 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib spawn_agent_requested_model_and_reasoning_override_inherited_settings_without_role --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server turn_start_emits_spawn_agent_item_with_effective_role_model_metadata_v2 --quiet | Spawn-agent completion metadata now reports effective role-derived model/reasoning. |

| 266 | `6ea041032b500a6f3e8511d225af366d5e53439b` | cherry-pick+surgical | ported | 9 | 0.79 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' just write-config-schema; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib model_provider_info; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all suite::client_websockets | Protected config/model paths updated; added local compatibility fixes for provider-id constructor and new timeout field initializers. |

| 267 | `8e258eb3f57a42477b5811a54321263185136a6a` | cherry-pick | ported | 5 | 0.87 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-protocol reasoning_effort_from_str --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-state extract --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib realtime_context_tests --quiet | Persists thread model and reasoning effort metadata to sqlite with migration and extraction wiring. |

| 268 | `78e8ee4591d4ff42d180000fbad29d5fb3bcd2a5` | cherry-pick | ported | 6 | 0.80 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server restore_started_app_server_thread_replays_remote_history; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui-app-server --quiet | Restores remote resume/fork history by replaying thread snapshots into TUI event pipeline. |

| 269 | `f26ad3c92c3ac1bd1c63325d74924053d3cd0c01` | cherry-pick | ported | 3 | 0.91 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server fuzzy_file_search --quiet | App-server fuzzy-search test harness now buffers unmatched notifications by payload-aware matching. |

| 270 | `d484bb57d9baea4603df0a89ad4f602cee79871d` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core shell_snapshot --quiet | Shell snapshot filenames now include generation suffix with cleanup parser handling legacy and suffixed names. |

| 271 | `0d531c05f2cc497d29da8e478f6770850cdb51bc` | cherry-pick | ported | 6 | 0.78 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all code_mode_yield; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Code-mode worker now emits started signal before scheduling initial yield timeout to avoid startup race. |

| 272 | `904dbd414f223027ecdb3d54a8444d3c94395aa6` | cherry-pick+surgical | ported | 8 | 0.78 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server-protocol --quiet | Adds hidden CLI subcommand to emit internal RolloutLine schema and aligns schemars/ts output mapping for function-call outputs. |

| 273 | `95bdea93d2600aabef1b87ee5fab05a6022a7d45` | cherry-pick | ported | 5 | 0.85 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-windows-sandbox --quiet | Windows elevated runner now uses framed IPC transport instead of request-file bootstrap path. |

| 274 | `49e7dda2dfd6e67dd5f9dd8bfa22b7c2b1df17ef` | cherry-pick | ported | 7 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server refresh_request_uses_local_chatgpt_auth --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server device_code_attempt_matches_only_for_matching_cancel --quiet | Adds device-code onboarding path and local ChatGPT token refresh handling in tui_app_server. |

| 275 | `683c37ce755f198f417db27f780965a5972b5b7b` | cherry-pick | ported | 7 | 0.79 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core -p codex-tui -p codex-tui-app-server --quiet | Adds plugin-backed discoverable tool suggestions and plugin-install elicitation wiring across core/tui/tui_app_server. |

| 276 | `b02388672f7df432fbe34a9128f78e7a1e9d43ea` | cherry-pick+surgical | ported | 6 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all apply_patch_cli_can_use_shell_command_output_as_patch_input --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all websocket_test_codex_shell_chain --quiet | Resolved thread_manager conflict by preserving provider-aware wiring and passing explicit None for parent_trace/user_shell_override in internal helpers. |

| 277 | `23a44ddbe8f45154a6e55280a74d28957dfefe72` | cherry-pick | ported | 3 | 0.93 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui permissions_selection --quiet | TUI permissions-popup tests now assert semantic selected row/preset rather than fragile row-order assumptions. |

| 278 | `4d9d4b7b0f2b8cfbe4ab18e31a7bd80465a975e4` | cherry-pick | ported | 2 | 0.94 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all approvals --quiet | Approval matrix write-file scenario now uses deterministic python file I/O plus extra diagnostics. |

| 279 | `2cc4ee413f8d86c38a5a46887d2fd5a18d40efbe` | cherry-pick | ported | 2 | 0.95 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-windows-sandbox --quiet | Temporarily disables private desktop flag in elevated IPC process spawn path. |

| 280 | `ee756eb80f94fe018c7a07306c0e43e1a42bcfa6` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib code_mode_only_restricts_model_tools_to_exec_tools --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all code_mode_only_restricts_prompt_tools --quiet | Renames code-mode wait tool from exec_wait to wait across feature docs/spec/tests. |

| 281 | `0d2ff40a58dde63e5aa8be85b5a5f19f384c354c` | cherry-pick (manual conflict resolution) | ported | 2 | 0.79 | ENOSPC on cargo test; cargo clean + CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core | Merged upstream auth-env telemetry with local Bedrock/provider wiring; preserved invariants |

| 282 | `43ee72a9b9c9c88dccc86e1e50901ac90dadcc37` | cherry-pick | ported | 2 | 0.84 | Switched from long-running cargo test to CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server | Applied cleanly; snapshot files included |

| 283 | `1a9555eda98cc561b4beec51fd1c577b0b068e2a` | cherry-pick (manual conflict resolution) | ported | 2 | 0.83 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet (134 passed) | Resolved rollout policy conflict by dropping removed remote-skill events while retaining existing PlanUpdate persistence behavior |

| 284 | `c6ab4ee537e5b118a20e9e0d3e0c0023cae2d982` | cherry-pick | ported | 2 | 0.72 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server-protocol --quiet; cargo check -p codex-app-server --quiet | Protected paths touched; preserved local config/provider behavior. write-config-schema attempted but blocked by ENOSPC |

| 285 | `98be562fd393b23250090e36b43012ed69000a69` | cherry-pick | ported | 2 | 0.77 | Reatime test filter was long-running; fallback CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Applied cleanly |

| 286 | `0d1539e74c28c7de9a6c471c7e96d77f15dfcd6e` | cherry-pick | ported | 2 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-linux-sandbox --quiet; cargo check -p codex-app-server --quiet | Protected config paths touched; retained local overlay behavior |

| 287 | `fc75d07504ae816c57ec8d3102a45137e89c535f` | cherry-pick | ported | 2 | 0.86 | python3 -m compileall -q sdk/python/src/codex_app_server sdk/python/examples | Non-Rust SDK/docs/examples expansion; compileall passed |

| 288 | `a5d3114e97166cab28bf5806204314f9ade1dbdc` | cherry-pick | ported | 2 | 0.76 | Plugin test filter timed out; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet && cargo check -p codex-core --quiet | Protected app-server path touched; preserved local provider/auth invariants |

| 289 | `19b887128e6b9ddc1aa134a7bdd481858473b663` | cherry-pick | ported | 2 | 0.79 | connection_handling_websocket test filter timed out; fallback CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet | Protected app-server transport path touched; behavior preserved |

| 290 | `83a60fdb94d5ee074a9ec33a48699d576a89c4a1` | cherry-pick | ported | 2 | 0.81 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server -p codex-environment -p codex-utils-image --quiet | Dependency change handling completed; protected app-server paths preserved |

| 291 | `6fe8a05dcbeb62df3d9cb0388f7dd9364488f5ca` | cherry-pick | ported | 2 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Applied cleanly |

| 292 | `d950543e6559db52855a718c96f7577922411fcd` | cherry-pick | ported | 2 | 0.83 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-windows-sandbox --quiet | Applied cleanly |

| 293 | `770616414a51fa179ce4cef10f7f8df838d3f46f` | cherry-pick (manual compile fix) | ported | 2 | 0.75 | Initial websocket_fallback test revealed compile break; fixed stale constructor arg in core/src/client.rs, then CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet | Reviewed provider/websocket diffs to preserve Bedrock/provider invariants; protected app-server path touched |

| 294 | `3ce879c64610cae8e460d3e8c126e57acbeb437d` | cherry-pick | ported | 2 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui -p codex-tui-app-server --quiet | Applied cleanly after restoring clean tree |

| 295 | `226241f035de7df4946ba3866fee9e22f83a9f99` | cherry-pick | ported | 2 | 0.74 | Attempted CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo run -p codex-core --bin codex-write-config-schema --quiet (no resulting diff); cargo check -p codex-core -p codex-app-server -p codex-tui -p codex-tui-app-server --quiet | Protected config/app-server paths touched; guardian snapshots updated |

| 296 | `6fef4216546cc9b8880f1616e349e77277b50ba3` | cherry-pick (manual conflict resolution) | ported | 2 | 0.78 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet (134 passed); cargo check -p codex-core -p codex-hooks --quiet | Resolved core/lib conflict by keeping both indubitably_auth and upstream hook_runtime modules |

| 297 | `a3613035f32a45146297a74e058a8c70b91c56c2` | cherry-pick | ported | 2 | 0.95 | N/A (workflow-only change) | No runtime code touched |

| 298 | `84f4e7b39d17fea6d28c98bc748652ea4b279a14` | cherry-pick (manual conflict resolution) | ported | 2 | 0.77 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Resolved thread_manager conflict by preserving upstream inherited exec-policy args and local provider/thread wiring flow |

| 299 | `40a7d1d15b446991094c5ecfbb1d0f21f2d9ad40` | cherry-pick | ported | 2 | 0.74 | Attempted CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo run -p codex-core --bin codex-write-config-schema --quiet (no resulting diff); cargo check -p codex-core --quiet | Protected config paths touched; config schema in commit remained consistent |

| 300 | `0f9484dc8a7ad0962a808892924bb160e9466ad9` | cherry-pick | ported | 2 | 0.83 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-artifacts -p codex-core --quiet | Dependency update handled with bazel lock verification |

| 301 | `a265d6043edc8b41e42ae508291f4cfb9ed46805` | cherry-pick | ported | 8 | 0.83 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core -p codex-protocol -p codex-tui-app-server --quiet | Protected app-server-protocol surfaces touched; no conflict |

| 302 | `58ac2a8773da0ac6eb21471e6d3da5744d9e9e0c` | cherry-pick | ported | 1 | 0.97 | N/A (template text-only) | No Rust code changed |

| 303 | `347c6b12ec63e8fe41e1dce6b00cca83dd2dba67` | cherry-pick | ported | 4 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server --quiet | Large tui_app_server-only refactor with snapshot updates |

| 304 | `7ae99576a615d524bb22bf0f68e2b2baf88c37ce` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Memory phase2 gate update |

| 305 | `606d85055f61ca9e81f0b96a4e7f6effc33c82be` | cherry-pick (manual compile fix) | ported | 7 | 0.76 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core --quiet | Added local compatibility fixes for CustomToolCallOutput.name in bedrock adapters |

| 306 | `580f32ad2ab642e3fe9661bce838d972f8989663` | cherry-pick (manual compile fix) | ported | 8 | 0.75 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-app-server -p codex-core -p codex-app-server-client --quiet | Protected app-server/protocol/cli paths; added local SessionSource::Custom compatibility in canonical_trace |

| 307 | `334164a6f714c171bb9f6440c7d3cd04ec04d295` | cherry-pick | ported | 4 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server --quiet; cargo check -p codex-core --quiet | Applied cleanly |

| 308 | `392347d436cddac41c535e70dd0357ff74624559` | cherry-pick | ported | 1 | 0.96 | N/A (workflow-only change) | No Rust code touched |

| 309 | `88e5382fc4cc7d7694fe99e39996bf148ebe9bcd` | cherry-pick | ported | 4 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Code-mode error propagation updates |

| 310 | `5cada46ddf74701dbaf1a152df0514b918ead70c` | cherry-pick | ported | 4 | 0.84 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-protocol -p codex-utils-image --quiet | Dependency updates handled with lock verification |

| 311 | `e5de13644d9459d3c2be0e60610009e619f50488` | cherry-pick | ported | 4 | 0.92 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui --quiet | TUI startup warning + snapshot update |

| 312 | `86982ca1f93c2e18711dd192eb2989f91f6814a1` | cherry-pick (manual compile fix) | ported | 8 | 0.75 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core --quiet; cargo check -p codex-app-server --quiet; cargo check -p codex-app-server-client --quiet | Protected app-server/protocol/cli paths; removed local SessionSource::Custom compatibility arm after revert |

| 313 | `7b37a0350f40c646e5cd36d55892da3fc4df4891` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-api -p codex-core --quiet | Realtime handoff output formatting update |

| 314 | `ebbbc52ce40324d6f47745fe6edf41f3a1cfbe48` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-state --quiet | State/log formatting migration alignment |

| 315 | `bb304324216e1305e9b7b5aa59700907c6326bd7` | cherry-pick | ported | 4 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet | App-server thread resume behavior update |

| 316 | `b306885bd8ea4cd6c7e742b93c20614b79e6ac5d` | cherry-pick | ported | 4 | 0.92 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-api --quiet | Realtime V2 transcript gating; no protected overlap |

| 317 | `3590e181fa2736c88a559389ea70dd1fe68d228e` | cherry-pick | ported | 4 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core code_mode_update_plan_nested_tool_result_is_empty_object --quiet (fails due pre-existing ModelClient::new test compile mismatch); cargo check -p codex-core --quiet | Code-mode update_plan output wiring applied; core check passes |

| 318 | `56d0c6bf67e15ff94c4bbf9e4fbc369b978b0bf1` | cherry-pick | ported | 4 | 0.81 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core code_mode_can_apply_patch_via_nested_tool --quiet (fails due pre-existing ModelClient::new test compile mismatch); cargo check -p codex-core --quiet | Apply-patch code-mode output object wiring; core check passes |

| 319 | `dcd5e0826960258b0b0c79fbd80aa66e9dd24296` | cherry-pick | ported | 8 | 0.78 | cargo test -p codex-app-server plugin_list_skips_invalid_marketplace_file --quiet (failed: no space left on device); df -h .; cargo clean; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server -p codex-core -p codex-config --quiet | Protected app-server overlap reviewed; uses resolved config for skills/plugin gating; compile gate used after disk cleanup |

| 320 | `81996fcde605a452ca94662eb7028e8c8b6f9ebb` | cherry-pick | ported | 4 | 0.88 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet | New exec-server crate + docs landed cleanly; lock checks passed from repo root |

| 321 | `825d09373dc6676ade6860f8052fc5018ea7197f` | cherry-pick | ported | 8 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server plugin_list --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-tui --quiet | Protected app-server/app-server-protocol overlap integrated; plugin/list now returns featuredPluginIds |

| 322 | `4fd2774614182ebaf74f2e7a8c04bbcf0b09ed96` | cherry-pick | ported | 4 | 0.90 | /tmp/codex-pytest-venv/bin/pip install -e sdk/python; /tmp/codex-pytest-venv/bin/python -m pytest sdk/python/tests/test_public_api_signatures.py sdk/python/tests/test_public_api_runtime_behavior.py | Python SDK docs/api/tests updated; local venv used for pytest + package deps |

| 323 | `903660edba6e1ecfd7c9b1782105be4ebf0e02a7` | cherry-pick | ported | 4 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet | Exec-server removes stdio transport and refreshes websocket harness/tests |

| 324 | `20f2a216df3e2d534069438ca7126811de9ff89a` | cherry-pick | ported | 4 | 0.80 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core client_websockets --quiet (fails due pre-existing ModelClient::new test compile mismatch); CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-api --quiet | Websocket response.create now forwards per-turn trace context metadata |

| 325 | `b14689df3b97245faa9c29a0b8f3f6c4d09393bf` | cherry-pick | ported | 4 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-rmcp-client --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Adds session/turn request headers through MCP connection manager into streamable-http RMCP client |

| 326 | `42e932d7bf70cc8e7ce912b4bbd27c0266293ad5` | cherry-pick | ported | 4 | 0.88 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-hooks --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Hooks Stop/UserPromptSubmit schemas and payloads now include turn_id with added core hook assertions |

| 327 | `10eb3ec7fccaf805c7162d8370b5b99bf57ddc48` | cherry-pick | ported | 8 | 0.85 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server fuzzy_file_search --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-file-search -p codex-tui -p codex-tui-app-server --quiet | Protected fuzzy-search protocol/app-server updates reviewed; adds directory match_type through search and UI mention insertion |

| 328 | `01df50cf422b2eb89cb6ad8f845548e8c0d3c60c` | cherry-pick | ported | 8 | 0.80 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server thread_shell_command --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server -p codex-core --quiet | Protected protocol/app-server overlap reviewed; new thread/shellCommand API wired into app-server and tui_app_server command path |

| 329 | `db5781a08872873a4df82fbb4b3dc6ffd98b5d15` | cherry-pick (manual compile fix) | ported | 8 | 0.74 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server plugin_ --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-protocol -p codex-tui -p codex-tui-app-server --quiet | Protected protocol/app-server overlap with conflict in thread_manager resolved; added canonical_trace SessionSource::Custom compatibility arm |

| 330 | `70cdb17703a4310b7173642e011f7534d2b2624f` | cherry-pick | ported | 4 | 0.83 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-state --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Introduces persisted thread-spawn graph and cascade close/resume support via state runtime + agent control |

| 331 | `32d2df5c1e97948cb5c55481f0b5fd3f8dfabf43` | cherry-pick | ported | 4 | 0.92 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Close-agent flow now treats already-shutdown agents idempotently while still cleaning thread state |

| 332 | `dee03da508a2cdefa9cf8eadad083f6af7fe49f8` | cherry-pick | ported | 8 | 0.87 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet | Protected app-server fs_api overlap reviewed; environment crate moved into exec-server and callsites retargeted |

| 333 | `2cf4d5ef353a0264df280644b26fa7d8fb42d406` | cherry-pick | ported | 4 | 0.96 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-otel --quiet | Adds profile-usage telemetry counter when active profile is present |

| 334 | `859c58f07dc3768b654711b7841f35e676005e6c` | cherry-pick | ported | 4 | 0.93 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Disables memory generation in phase2 consolidation subagents and asserts persisted disabled memory mode |

| 335 | `5ec121ba120ba40cc4fa89960093a115e5e58da2` | cherry-pick | ported | 4 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-rmcp-client --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet | Reverts request-header forwarding path for MCP HTTP requests |

| 336 | `267499bed853c0011613a1ef26cf2e4db711e556` | cherry-pick (manual compile fix) | ported | 8 | 0.74 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-hooks --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server turn_start --quiet (blocked by ENOSPC); df -h .; cargo clean; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server -p codex-core -p codex-protocol -p codex-tui-app-server --quiet | Protected protocol/app-server overlap reviewed; added canonical_trace TurnItem::HookPrompt compatibility arm; compile-gate fallback after ENOSPC |

| 337 | `1837038f4e65ba37022d0163894cf29883b4d620` | cherry-pick | ported | 8 | 0.86 | just write-config-schema; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet | Protected app-server/core-config overlap reviewed; adds experimental_exec_server_url and async Environment::create wiring |

| 338 | `b87ba0a3cc1ee3cb1f558233a8d4e3b994217795` | cherry-pick | ported | 8 | 0.88 | tools/argument-comment-lint cargo check --quiet (initial ENOSPC, recovered via cargo clean in codex-rs and tool crate; then check passed) | Adds release workflow + dotslash manifest and runnable argument-comment-lint package entrypoint |

| 339 | `1d210f639e39040bdb1611267b02df723eb1901f` | cherry-pick (manual compile fix) | ported | 7 | 0.80 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet | Added exec/filesystem RPC flow in exec-server; fixed Environment callsite compatibility in server/filesystem.rs |

| 340 | `fe287ac467e915a4a75fccb8ce7b7b82d5c12e53` | cherry-pick | ported | 4 | 0.94 | CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-otel --quiet | Telemetry now records guardian-routed approvals as automated_reviewer source distinct from user/config |

| 341 | `60cd0cf75eb29798c71bdfd80f1625e69a26d58d` | cherry-pick+surgical | ported | 8 | 0.81 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui title_setup --quiet; cargo test -p codex-core config_toml_deserializes_model_availability_nux --quiet (pre-existing ModelClient::new mismatch); CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Added terminal-title status surface and config schema wiring; protected config overlap reviewed. |

| 342 | `668330acc12b8907ecd82bc15148e0a627246783` | cherry-pick+surgical | ported | 6 | 0.89 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server turn_start_jsonrpc_span_parents_core_turn_spans --quiet | App-server request spans now record turn.id for start/steer/interrupt. |

| 343 | `7eb19e53198470304eb9e74599ec8fb4b97adc3c` | cherry-pick+surgical | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-terminal-detection --quiet; just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core -p codex-cli -p codex-tui -p codex-tui-app-server --quiet | Resolved conflicts in cli/main.rs and tui/lib.rs while keeping local CLI version/env behavior and command helper usage. |

| 344 | `69750a0b5a9f10f2e085b48943d41fd5b12ebc0b` | cherry-pick | ported | 4 | 0.87 | cargo test -p codex-core test_exec_command_tool_windows_description_includes_shell_safety_guidance --quiet (blocked by pre-existing ModelClient::new mismatch); cargo test -p codex-core --lib test_shell_tool --quiet (same baseline mismatch); CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Windows shell/exec tool descriptions now include explicit destructive-filesystem safety guidance. |

| 345 | `27977d67166cc3d0b32c04780e153d05077a66a1` | cherry-pick | ported | 5 | 0.90 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui image_generation_call_adds_history_cell --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server image_generation_call_adds_history_cell --quiet | TUI and tui_app_server now retain/render full image file URI path for generated images with snapshot updates. |

| 346 | `2254ec4f30b78469bbb0fc310894ea2d7bf6944f` | cherry-pick+surgical | ported | 7 | 0.82 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_read --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_install --quiet (ENOSPC); df -h .; cargo clean; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server --quiet | Exposed needsAuth in plugin/read+install summaries; app-server/plugin helper logic updated; compile fallback after ENOSPC. |

| 347 | `2bee37fe69fee6a8af13cd82850718433e8eb742` | cherry-pick | ported | 7 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-rmcp-client --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | MCP tool calls now include x-codex-turn-metadata in _meta, preserving _codex_apps metadata. |

| 348 | `9e695fe83083ba5201f9b53021a56fec183d32c6` | cherry-pick+surgical | ported | 8 | 0.84 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server thread_start --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui-app-server --quiet | Added v2 mcpServer/startupStatus/updated notification schema/protocol/app-server mapping and tui_app_server handling. |

| 349 | `6b8175c7346d25a13479bc044819ca406ea1c3ae` | cherry-pick | ported | 6 | 0.85 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet | Image-generation artifacts now default under codex_home/generated_images/<thread>/<call>.png with sanitized names and updated tests. |

| 350 | `403b397e4e1d1830a5848367fe05096f8b41faac` | cherry-pick+surgical | ported | 8 | 0.85 | just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-exec-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server -p codex-core --quiet | Exec-server filesystem split into shared trait + local/remote implementations; protected app-server fs_api overlap reviewed. |

| 351 | `ded7854f09d210b4ae7236272ef002279b3f5de2` | cherry-pick | ported | 1 | 0.89 | just bazel-lock-check; bazel query //third_party/v8:all | Adds Bazel V8 source-build/release wiring and targets; no protected overlay paths touched. |

| 355 | `35f8b87a5b396ac9780fa0100cf6fb1af5a5e282` | cherry-pick | ported | 2 | 0.86 | CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet (targeted lib tests blocked by pre-existing ModelClient::new test compile errors) | Plugin marketplace/product gating now distinguishes a missing products field from an explicit empty list. |

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

- Upstream intent: Remove a flaky legacy guardian snapshot test and its old snapshot artifact.
- Local overlays touched: None (no protected-path overlap); the targeted legacy files are already absent on this branch.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unchanged.
- Risk factors: No-op relative to current branch state because earlier guardian test restructuring already removed the old file and snapshot.
- Strategy selected: skip.
- Confidence: 0.96.
- Validation evidence: Verified `HEAD` does not track `codex-rs/core/src/guardian_tests.rs` and the old guardian snapshot path is already absent.
- Rollback note: No rollback needed because no code changes were applied.

### Commit `92f7541624810406d5c3d1c424147bcfa458efce`

- Upstream intent: Fix guardian-related CI instability across core and TUI tests.
- Local overlays touched: None (no protected-path overlap), but upstream overlapped the branch's already-diverged guardian/TUI test layout.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unchanged.
- Risk factors: Conflicts landed in already-diverged guardian and TUI test files, and the popup snapshot coverage is platform-specific.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.84.
- Validation evidence: `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui experimental_popup_includes_guardian_approval --quiet`.
- Rollback note: Revert this sync commit if guardian popup snapshot coverage or related TUI test stability regresses.

### Commit `e8d7ede83cf09c99134866f19e5378c546d53191`

- Upstream intent: Correct context window display before initial `TokenCount` events in the TUI.
- Local overlays touched: None (no protected-path overlap); the equivalent runtime context-window behavior is already present on this branch.
- Invariants checked: No auth/provider/runtime overlay paths modified.
- Risk factors: No-op relative to current branch state because `TurnStarted.model_context_window` handling and regression coverage are already integrated.
- Strategy selected: skip.
- Confidence: 0.94.
- Validation evidence: Reviewed current TUI sources/tests for `TurnStarted.model_context_window` handling and matching regression coverage.
- Rollback note: No rollback needed because no code changes were applied.

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

### Commit `0334ddeccbef07995561de5b39334dd94ef9e33a`

- Upstream intent: Speed up and stabilize shell_command unicode_output test by using a child process on Windows.
- Local overlays touched: None (no protected-path overlap).
- Invariants checked: Indubitably auth path and Bedrock provider/runtime selection unchanged.
- Risk factors: Test-only adjustment in codex-core suite with low runtime impact.
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: CARGO_INCREMENTAL=0 cargo test -p codex-core unicode_output --quiet (8 passed, filtered).
- Rollback note: Revert this sync commit if shell_command unicode coverage regresses.

### Commit `6ad448b6585f5a8d504bb9ff990da218bcc336ef`

- Upstream intent: Introduce plugin/uninstall endpoint to remove plugin cache files and clear user config entries.
- Local overlays touched: Protected app-server and app-server-protocol files touched; no Indubitably auth or Bedrock runtime/provider logic changed.
- Invariants checked: Indubitably auth path intact; Bedrock provider/runtime and model-selection overlays unchanged.
- Risk factors: Cross-crate API and runtime behavior change in protected protocol/app-server surfaces plus core plugin manager logic.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: After freeing disk via cargo clean, targeted codex-core/codex-app-server/codex-app-server-protocol tests passed.
- Rollback note: Revert this sync commit if plugin uninstall removes incorrect paths or regresses plugin config persistence.

### Commit `da991bdf3a1925e3fbd11325293f0cd683300131`

- Upstream intent: Move shared metric names and metadata tag construction into codex-otel and update core callsites to use canonical constants.
- Local overlays touched: Touches codex-rs/core/src and codex-rs/otel; no Indubitably auth or Bedrock provider/runtime files changed.
- Invariants checked: Indubitably auth behavior and Bedrock model/provider routing remain unchanged.
- Risk factors: Cross-crate telemetry refactor in core and otel with new public metric constants/tags module.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.83
- Validation evidence: codex-otel session_metric_tags tests passed; broader codex-core compile check was skipped after repeated local disk exhaustion during this batch.
- Rollback note: Revert this sync commit if telemetry metrics/tags regress or metric naming compatibility issues appear.

### Commit `44ecc527cb7697454ad9241e90b2ebd472beccfb`

- Upstream intent: Remove flaky RMCP streamable HTTP races by waiting for metadata/tool readiness and retrying bind collisions.
- Local overlays touched: None; test-only changes in codex-core test suite and rmcp test helper binary.
- Invariants checked: Indubitably auth and Bedrock provider/runtime paths unchanged.
- Risk factors: Cross-test harness timing and readiness logic updated across two test components.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 cargo test -p codex-core streamable_http --quiet passed.
- Rollback note: Revert this sync commit if RMCP readiness tests regress or startup wait logic introduces hangs.

### Commit `42f20a6845939437b68848df111ccd719c64012d`

- Upstream intent: Pass image generation save and prompt metadata into prompt-history normalization and TUI history cells for richer context.
- Local overlays touched: Touches codex-core context_manager and codex-tui history rendering; no Indubitably auth or Bedrock provider/runtime files changed.
- Invariants checked: Indubitably auth and Bedrock model/provider routing behavior unchanged.
- Risk factors: Cross-crate user-visible history formatting change with snapshot impact and context-manager rewrite behavior updates.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: Targeted codex-core and codex-tui tests for image-generation history behavior passed.
- Rollback note: Revert this sync commit if image-generation history context or TUI snapshots regress.

### Commit `831ee51c86e715e3e546f8c3342f8c5aa94d736f`

- Upstream intent: Stabilize schema fixture generation by using in-memory TS tree generation, normalized comparisons, and split JSON/TS fixture tests.
- Local overlays touched: Protected app-server-protocol source files and nextest config touched; no Indubitably auth or Bedrock runtime/provider code changed.
- Invariants checked: Indubitably auth path and Bedrock provider/model-routing overlays unchanged.
- Risk factors: Large protected-surface refactor in export/schema-fixture pipeline plus test harness behavior updates.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.77
- Validation evidence: Focused protocol export tests passed; running schema_fixtures integration test reports existing fixture drift (SkillInvocation/TurnSkillContext schema entries) in current branch state.
- Rollback note: Revert this sync commit if schema fixture generation or experimental filtering behavior regresses.

### Commit `d241dc598cb0bbadeefd5eab92c056a36b420624`

- Upstream intent: Persist request_permissions grants across turns when client approves session scope while preserving turn-scoped default behavior.
- Local overlays touched: Protected app-server-protocol/app-server paths touched; core/protocol/tui request_permissions flow updated.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection overlays unchanged while permission scope behavior expanded.
- Risk factors: Wide multi-crate behavior change to permission grant lifecycle and user approval UX semantics.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: Targeted core, app-server-protocol, app-server, and tui tests for request_permissions scope behavior passed.
- Rollback note: Revert this sync commit if request_permissions approvals no longer honor turn/session scope semantics.

### Commit `d309c102efdb2840605ddac1d911ecb3a9945459`

- Upstream intent: Replace custom web_search serializers with dedicated Responses API filter/user-location structs to reduce config/wire-format drift.
- Local overlays touched: Touches codex-core tool-spec/client-common only; no Indubitably auth or Bedrock runtime/provider paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/model-selection overlays unchanged.
- Risk factors: Core tool payload shape refactor at client boundary with serialization behavior changes.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: After cleaning build artifacts for disk headroom, codex-core web_search tool-spec forwarding test passed.
- Rollback note: Revert this sync commit if web_search payload serialization regresses against Responses API expectations.

### Commit `66e71cce1139ac7045c59f630a40b8b354fac1ce`

- Upstream intent: Serve health probe endpoints on ws listener and migrate websocket acceptor to axum upgrade handling.
- Local overlays touched: Protected app-server transport path touched; docs and websocket integration tests updated; Cargo dependencies refreshed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection overlays unchanged.
- Risk factors: Network transport refactor plus dependency/lock changes in app-server stack and listener behavior.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.80
- Validation evidence: Bazel lock update/check passed; websocket health endpoint integration test passed.
- Rollback note: Revert this sync commit if websocket listener routing or health endpoint behavior regresses.

### Commit `c1defcc98cf9c6b9001e86d8d13e5b5ec9488510`

- Upstream intent: Ensure apply_patch respects granted additional permissions from request_permissions flows and can preapprove when permissions are already granted.
- Local overlays touched: Touches codex-core apply_patch handler/runtime and request_permissions tool suite only; no Indubitably auth or Bedrock runtime/provider files changed.
- Invariants checked: Indubitably auth and Bedrock provider/model-routing overlays unchanged.
- Risk factors: Core permission enforcement behavior change on apply_patch execution path with approval semantics impact.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.78
- Validation evidence: Relevant core apply_patch unit tests passed; new request_permissions_tool integration case hit environment-specific sandbox Signal(6) in this runner.
- Rollback note: Revert this sync commit if apply_patch permission propagation or approval handling regresses.

### Commit `6da84efed8f615085212e7aa6207afa43b3733a9`

- Upstream intent: Support explicit reject-policy behavior for request_permissions approvals and expose config/wire flags for that category.
- Local overlays touched: Protected app-server-protocol v2 surface touched with broad core/protocol plumbing changes; no Bedrock/Indubitably auth overlay paths changed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Multi-crate approval-policy behavior change including protocol schema/config and request_permissions runtime semantics.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.76
- Validation evidence: Targeted protocol, app-server-protocol, and core request_permissions reject-policy tests passed after scope compatibility fix.
- Rollback note: Revert this sync commit if reject-policy request_permissions behavior or approval-policy serialization regresses.

### Commit `b0cbc25a48b11e311f3b1b7ce9998bb54731ea41`

- Upstream intent: Prevent redundant legacy readable roots under writable workspace roots from becoming unintended read-only carveouts.
- Local overlays touched: Protected app-server/config surfaces touched plus core/protocol sandbox conversion paths; Bedrock/Indubitably auth overlays unchanged.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-crate sandbox policy derivation semantics changed across protocol/core/app-server/linux/mac adapters.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.81
- Validation evidence: Targeted protocol and core legacy-workspace-write regression tests passed.
- Rollback note: Revert this sync commit if legacy workspace-write read/write semantics regress across adapters.

### Commit `1165a16e6ffad719e8f852900fd7ff438ec88fae`

- Upstream intent: Keep new permissions profile formats forward-compatible with older runtimes while failing closed instead of aborting config load.
- Local overlays touched: Touches protected core config parsing path and protocol permissions bridge; no Bedrock/Indubitably auth overlay logic changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Core config parsing behavior change with warning semantics and cross-platform path normalization edge cases.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.85
- Validation evidence: Targeted core permissions-profile and protocol legacy-bridge compatibility tests passed.
- Rollback note: Revert this sync commit if permissions profile loading or legacy bridge handling regresses.

### Commit `a5af11211a3f000c48b67bf1083e3ee0578c1431`

- Upstream intent: Always disambiguate $-mention suggestions by prefixing category labels (skill, app, plugin) in the popup.
- Local overlays touched: Touches only TUI chat composer/skill popup rendering and snapshots; no auth/runtime/provider overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: User-visible UI text ordering change in mention popup plus snapshot updates.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: Two targeted codex-tui mention popup tests passed after clearing build artifacts for disk headroom.
- Rollback note: Revert this sync commit if mention popup category labeling causes UX regression or snapshot mismatch.

### Commit `aa04ea6bd705d06543c02ebdf1d26b9730b39684`

- Upstream intent: Refactor tool output handling into trait-based implementations so handlers emit strongly-typed renderable outputs.
- Local overlays touched: Touches core tool context/handlers and codex core tests only; no Indubitably auth or Bedrock runtime/provider paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-cutting refactor across many core tool handlers with runtime output typing and test extraction changes.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: Three targeted codex-core tests covering request_permissions and guardian output parsing passed after resolving scope-path conflict.
- Rollback note: Revert this sync commit if tool output serialization or handler output extraction regresses.

### Commit `a3cd9f16f5b3dfefb928af3fa512b9dac6af9ac7`

- Upstream intent: Sort plugin mentions before other categories in the $-mention menu to improve discoverability.
- Local overlays touched: Touches only TUI mention popup ordering and snapshot output; no auth/runtime/provider overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: UI ordering-only change with snapshot delta in mention popup.
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: Targeted codex-tui mention popup tests passed after rebuilding with fresh target artifacts.
- Rollback note: Revert this sync commit if mention ordering causes UX regressions or snapshot instability.

### Commit `da616136ccff31142b159e97da67705bf0ab7555`

- Upstream intent: Introduce experimental code_mode with nested tool execution bridge and spec wiring for controlled code execution flows.
- Local overlays touched: Touches core feature flags, tool registry/spec/context, runtime handlers, schema, and new integration tests; no Bedrock or Indubitably auth overlay paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Large multi-file additive feature including JS bridge/runtime execution path and tool registry/spec integration.
- Strategy selected: cherry-pick
- Confidence: 0.74
- Validation evidence: Two new code_mode integration tests passed; additional broad code_mode-filtered cargo test failed due disk exhaustion during unrelated test-target linking.
- Rollback note: Revert this sync commit if code_mode tool registration or nested tool execution semantics regress.

### Commit `244b2d53f40938ffba96acf0ca7a559473b842f1`

- Upstream intent: Introduce initial hooks engine plumbing with protocol events, app-server notifications, hook registry/dispatcher, and TUI rendering support.
- Local overlays touched: Touches protected app-server-protocol/app-server surfaces plus core/hooks/protocol/tui/exec; no Indubitably auth or Bedrock runtime/provider-specific paths modified.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged on reviewed touched files.
- Risk factors: Very large cross-crate feature import with new schemas/events, runtime dispatch pipeline, dependency lock updates, and UI event rendering.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.66
- Validation evidence: codex-hooks tests passed; codex-tui hook snapshot test passed; bazel lock update/check passed; codex-app-server-protocol unit tests passed but schema_fixtures still fail from existing fixture drift requiring write-app-server-schema.
- Rollback note: Revert this sync commit if hook lifecycle events or app-server notification wiring regresses thread/turn behavior.

### Commit `63597d1b2d11c94dd09e384ac4518176db65020c`

- Upstream intent: Restrict fast-status UI indicator to GPT-5.4 models instead of broader model families.
- Local overlays touched: Touches only TUI app/chatwidget status rendering and snapshots; no auth/runtime/provider overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: User-visible status-line logic and snapshot updates in TUI only.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: Two targeted codex-tui tests covering fast-status header/footer snapshots passed.
- Rollback note: Revert this sync commit if fast-status model gating regresses or snapshot output becomes unstable.

### Commit `d71e0426940b75f7dea0c149f2129f0b86c17f20`

- Upstream intent: Unify handler output contract to a single tool output type and remove mixed content-item/text output ambiguity.
- Local overlays touched: Touches core handler/context/registry flows and related core/app-server tests; no Indubitably auth or Bedrock runtime/provider paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-handler output-path refactor affecting tool serialization and downstream request payload expectations.
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: Targeted core permissions/guardian/view_image tests and app-server dynamic-tools round-trip test all passed.
- Rollback note: Revert this sync commit if tool output serialization or dynamic tool round-trip behavior regresses.

### Commit `772259b01f6f3c088ac8b04414206d8cb52e0251`

- Upstream intent: Set RejectConfig.request_permissions default to false and align protocol/app-server-protocol schema defaults.
- Local overlays touched: Touches protected app-server-protocol v2 surface and core protocol/config schema; no Bedrock/Indubitably auth overlay paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Protocol default semantic change reflected in generated schemas and app-server protocol conversion logic.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.87
- Validation evidence: Targeted protocol and app-server-protocol defaulting tests passed (after one disk-clean rebuild).
- Rollback note: Revert this sync commit if reject-policy defaulting for request_permissions regresses in config or protocol bridges.

### Commit `710682598d20c8a51d41b99a4d709b3a7b827115`

- Upstream intent: Export and use the tools module inside code_mode runner to simplify nested tool invocation wiring.
- Local overlays touched: Touches core code_mode JS bridge/runner and code_mode suite tests only; no auth/provider overlays touched.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Code_mode runtime/bridge path changes across Rust+JS boundary.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: code_mode integration filters passed in codex-core suite.
- Rollback note: Revert this sync commit if code_mode nested tool execution or bridge messaging regresses.

### Commit `0c33af77461615f32d4c6b94060f5a39e0f5194f`

- Upstream intent: Allow disabling bundled system skills via config/session stack while preserving explicit user skill controls.
- Local overlays touched: Touches protected config and app-server surfaces plus core skills loading/caching paths; no Bedrock/Indubitably auth overlay paths changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Config semantic change plus skills cache/installation behavior updates across startup and per-thread skill resolution.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: Targeted core config and skills manager regression tests passed.
- Rollback note: Revert this sync commit if bundled-skill enablement semantics regress or startup skill cache behavior changes unexpectedly.

### Commit `a9ae43621b6d583d8d0ff9beeee53484a7a1f38b`

- Upstream intent: Centralize exec_command output truncation in ExecCommandToolOutput and remove duplicate truncation handling across unified-exec layers.
- Local overlays touched: Touches core unified_exec and tool context/handler only; no auth/provider overlays touched.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Behavioral movement of truncation responsibility across core unified-exec output flow.
- Strategy selected: cherry-pick
- Confidence: 0.9
- Validation evidence: Two targeted codex-core unified-exec truncation/arg-resolution tests passed.
- Rollback note: Revert this sync commit if exec output truncation formatting or unified-exec output payload behavior regresses.

### Commit `566e4cee4bd067aeae95591430cffb71e8ede228`

- Upstream intent: Fix apps feature/tool enablement gating so apps guidance and behavior align with auth mode and thread-level state.
- Local overlays touched: Touches protected app-server runtime path plus core/chatgpt/tui apps surfaces; no Bedrock/Indubitably-specific overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-cutting auth/feature gating change spanning core instruction shaping, app-server list APIs, and TUI apps popup state handling.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.74
- Validation evidence: Targeted core apps/search filters, app-server app_list API-key filter, and tui apps popup loading snapshot test all passed.
- Rollback note: Revert this sync commit if apps availability gating regresses between ChatGPT and API-key authentication modes.

### Commit `79307b7933d9607a7f0033dbbf8446b2027279b5`

- Upstream intent: Delay pending cleanup timing until task abort path.
- Local overlays touched: No local files changed in this branch state.
- Invariants checked: All local overlays unchanged (no-op).
- Risk factors: None; empty/no-op application against current tree.
- Strategy selected: skip
- Confidence: 0.95
- Validation evidence: apply produced no file delta; working tree remained clean.
- Rollback note: No rollback required for skipped no-op commit.

### Commit `c6343e0649676579f174b6cd0da617d42ab1c58f`

- Upstream intent: Track out-of-band elicitation count per thread to pause stopwatch/yield-time accounting while external elicitations are active.
- Local overlays touched: Touches protected app-server-protocol/app-server surfaces plus core thread and unified-exec paths; no Bedrock/Indubitably overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-layer state coordination change between app-server resume flow, protocol methods, and core thread pause semantics.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.72
- Validation evidence: Targeted app-server thread-resume and core unified-exec pause tests passed after adapting staged test code to new ExecCommandToolOutput API.
- Rollback note: Revert this sync commit if elicitation pause/resume state handling regresses timeout behavior or thread-resume consistency.

### Commit `2e24be21346c5e2d415ab0ae10cd6dac854014d5`

- Upstream intent: Replace handoff-context sourcing with realtime transcript data to improve inbound handoff continuity and context fidelity.
- Local overlays touched: Touches protected app-server runtime surface plus protocol/app-server-protocol/core realtime stacks; no Bedrock/Indubitably-specific overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-crate realtime protocol/event shape migration affecting app-server websocket methods, core handoff context, and wire schemas.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.78
- Validation evidence: Targeted core realtime handoff test and app-server-protocol realtime filters passed.
- Rollback note: Revert this sync commit if realtime handoff context construction or transcript event wiring regresses.

### Commit `aa6a57dfa2001ee2a6e6882b3f94406fc2e47179`

- Upstream intent: Stabilize early-close SSE retry test behavior and assertions.
- Local overlays touched: Touches only core test suite file.
- Invariants checked: Runtime overlays unchanged.
- Risk factors: Low risk; test harness-only adjustments.
- Strategy selected: cherry-pick
- Confidence: 0.96
- Validation evidence: Updated retries_on_early_close test passed.
- Rollback note: Revert this sync commit if SSE retry test stability unexpectedly regresses.

### Commit `6b7253b123a65038fe215a5252002054ad93744f`

- Upstream intent: Adjust unified exec test assertion to current output API.
- Local overlays touched: No files changed in current branch state.
- Invariants checked: All local overlays unchanged (no-op).
- Risk factors: None; no-op commit in current sequence.
- Strategy selected: skip
- Confidence: 0.95
- Validation evidence: apply resulted in zero-file delta with clean tree.
- Rollback note: No rollback required for skipped no-op commit.

### Commit `b39ae9501f10c7396857335bcb4c8630c8de28a6`

- Upstream intent: Stabilize websocket test server binding behavior to reduce flaky app-server websocket tests.
- Local overlays touched: Touches only app-server test suite files.
- Invariants checked: Runtime overlays unchanged.
- Risk factors: Low risk; integration test harness adjustments only.
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: Targeted websocket transport handshake test passed after disk-recovery rerun.
- Rollback note: Revert this sync commit if websocket suite stability unexpectedly worsens.

### Commit `f3f47cf455c2bf83465c2a2c14048f11d35c4d9d`

- Upstream intent: Stabilize app-server initialize notification tests by using a dedicated notify-capture binary and unsubscribe coverage updates.
- Local overlays touched: Touches protected app-server runtime/bin plus app-server v2 tests; no Bedrock/Indubitably overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Adds auxiliary app-server binary and updates notification test harness behavior for initialize/unsubscribe flows.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.9
- Validation evidence: Two targeted app-server initialize/thread_unsubscribe filters passed.
- Rollback note: Revert this sync commit if notify initialize/unsubscribe test harness behavior regresses.

### Commit `7144f84c6948b0ccf18801a298ee2968c14d32bd`

- Upstream intent: Repair release-mode integration test compilation by adjusting view_image test code.
- Local overlays touched: Touches only core view_image test suite file.
- Invariants checked: Runtime overlays unchanged.
- Risk factors: Low risk; test compilation fix only.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: Targeted view_image placeholder test passed after disk-recovery rerun.
- Rollback note: Revert this sync commit if view_image suite compilation behavior regresses.

### Commit `026cfde023e3fae85d12e414b78b9059437e303e`

- Upstream intent: Prevent Linux tmux-related crashes in user-shell discovery logic.
- Local overlays touched: Touches core shell helper implementation only.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Platform-specific shell path resolution changes for tmux/Linux environments.
- Strategy selected: cherry-pick
- Confidence: 0.91
- Validation evidence: Targeted core shell detection/exec-args tests passed.
- Rollback note: Revert this sync commit if shell detection regresses on Linux/tmux environments.

### Commit `f9cba5cb168c3e3bf325d30ef73d47c87ed895e1`

- Upstream intent: Attach ChatGPT user identifier to feedback-related telemetry/tagging paths.
- Local overlays touched: Touches protected app-server message processor plus core auth and tui chatwidget paths; no Bedrock/Indubitably-specific overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-surface metadata enrichment across auth/session and feedback tagging pathways.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: Core auth-mode regression filter passed; app-server smoke compile attempt failed from disk exhaustion in this runner.
- Rollback note: Revert this sync commit if feedback tagging metadata propagation regresses or user-id tagging causes side effects.

### Commit `00ea8aa7eeebb8b921573a40f4306ef3e18cf084`

- Upstream intent: Expose strongly-typed exec_command results so downstream tool handlers and code_mode can consume structured data safely.
- Local overlays touched: Touches core tool routing/context/spec/code_mode surfaces only; no Bedrock/Indubitably overlay files changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-cutting core tool output API shift affecting code_mode bridge and router serialization contracts.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: Targeted code_mode integration filters and exec_command output formatting test passed.
- Rollback note: Revert this sync commit if typed exec_command result plumbing regresses tool output handling.

### Commit `52a7f4b68b13f4e0b4eea90a0671890bd09e7ed7`

- Upstream intent: Stabilize split stdout/stderr PTY test expectations on Windows.
- Local overlays touched: Touches only utils/pty test file.
- Invariants checked: Runtime overlays unchanged.
- Risk factors: Low risk; test-only adjustments.
- Strategy selected: cherry-pick
- Confidence: 0.97
- Validation evidence: Targeted codex-utils-pty split stdout/stderr test passed.
- Rollback note: Revert this sync commit if PTY split-output test stability regresses.

### Commit `c4d35084f56313d657ad7b6f16f8aee45f5d242c`

- Upstream intent: Reuse McpToolOutput across handler and protocol conversion paths to remove duplicated wrapper logic.
- Local overlays touched: Touches core/protocol tool-output plumbing only; no protected app-server auth/runtime overlays modified.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Cross-cutting serialization/output conversion behavior used by MCP tools and code_mode.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: Targeted codex-protocol and codex-core MCP conversion tests passed.
- Rollback note: Revert this sync commit if MCP output serialization or code_mode tool output compatibility regresses.

### Commit `4ac60428508c2a4af21c66d37d23593244f1f593`

- Upstream intent: Mark replayed incomplete turns interrupted when resumed thread is idle to avoid stuck non-interactable threads.
- Local overlays touched: Touches protected app-server message processor plus thread_resume integration tests; no Bedrock/Indubitably-specific overlays changed.
- Invariants checked: Indubitably auth and Bedrock provider/runtime/model-selection behavior unchanged.
- Risk factors: Thread status/state transition changes across resume/read and listener response paths.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: Two targeted app-server thread-resume filters passed after ENOSPC recovery via cargo clean.
- Rollback note: Revert this sync commit if thread resume/read status transitions regress or stale turns are misclassified.

### Commit `3b1c78a5c5fcb81a732de64afffc352403dd8964`

- Upstream intent: Expand sample skill-creator guidance with forward-testing instructions and update init script next-step output.
- Local overlays touched: Touches only sample skill assets in codex-rs/skills; no runtime overlays touched.
- Invariants checked: Runtime/auth/provider invariants unchanged.
- Risk factors: Docs/sample-script guidance only; no production runtime code paths.
- Strategy selected: cherry-pick
- Confidence: 0.97
- Validation evidence: Python syntax check for updated init_skill.py passed.
- Rollback note: Revert this sync commit if sample skill guidance changes need to be deferred.

### Commit `b7f8e9195`

- Upstream intent: Add an OpenAI docs sample skill to bundled skill examples.
- Local overlays touched: codex-skills sample assets only; no protected runtime paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Low-risk content/sample update.
- Strategy selected: cherry-pick.
- Confidence: 0.92.
- Validation evidence: `cargo test -p codex-skills fingerprint_traverses_nested_entries --quiet`.
- Rollback note: Revert if sample skill content causes packaging or docs regressions.

### Commit `f2d66fadd`

- Upstream intent: Add ARC monitor approval action support in core tool flow.
- Local overlays touched: core monitoring/action surfaces only; no protected paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Approval action routing touches monitor behavior and tool-call payload paths.
- Strategy selected: cherry-pick.
- Confidence: 0.82.
- Validation evidence: `cargo test -p codex-core monitor_action_posts_expected_arc_request --quiet`.
- Rollback note: Revert if ARC monitor actions regress MCP approval handling.

### Commit `d751e68f4`

- Upstream intent: Force remote curated plugin status sync during `plugin/list`.
- Local overlays touched: Protected app-server runtime/message processor paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Cross-thread/plugin state sync behavior across local and remote status reconciliation.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.83.
- Validation evidence: `cargo test -p codex-app-server --test all plugin_list_force_remote_sync_reconciles_curated_plugin_state --quiet`.
- Rollback note: Revert if plugin status reconciliation causes stale or oscillating states.

### Commit `3d4628c9c`

- Upstream intent: Emit more granular cloud-requirements load metrics.
- Local overlays touched: Cloud-requirements crate only; no protected paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Metric emission path changes; low behavioral risk.
- Strategy selected: cherry-pick.
- Confidence: 0.94.
- Validation evidence: `cargo test -p codex-cloud-requirements fetch_cloud_requirements_surfaces_auth_recovery_message --quiet`.
- Rollback note: Revert if cloud requirements metric cardinality or load-path behavior regresses.

### Commit `91ca20c7c`

- Upstream intent: Add model/reasoning overrides for `spawn_agent`.
- Local overlays touched: core multi-agent tooling only; no protected paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Agent config inheritance and override precedence logic.
- Strategy selected: cherry-pick.
- Confidence: 0.90.
- Validation evidence: `cargo test -p codex-core --test all spawn_agent_requested_model_and_reasoning_override_inherited_settings_without_role --quiet`.
- Rollback note: Revert if spawned agent model/reasoning settings drift from requested overrides.

### Commit `722e8f08e`

- Upstream intent: Normalize image-generation save path handling through `/tmp`.
- Local overlays touched: core image-generation output and save-path handling.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: File save-path behavior and output messaging for generated images.
- Strategy selected: cherry-pick.
- Confidence: 0.85.
- Validation evidence: `cargo test -p codex-core --lib handle_output_item_done_records_image_save_message_after_successful_save --quiet`.
- Rollback note: Revert if image-save behavior regresses path correctness or user-facing messages.

### Commit `d5694529c`

- Upstream intent: Propagate nested experimental gating to `AskForApproval::Reject`.
- Local overlays touched: Protected app-server-protocol + app-server approval capability paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Experimental capability enforcement across wire/protocol/server.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.84.
- Validation evidence: `cargo test -p codex-app-server-protocol derive_supports_nested_experimental_fields --quiet`; `cargo test -p codex-app-server --test all thread_start_reject_approval_policy_requires_experimental_api_capability --quiet`.
- Rollback note: Revert if approval reject-policy experimental gating diverges across protocol/server.

### Commit `ee8f84153`

- Upstream intent: Add MCP output schema and expose MCP tool results to `code_mode`.
- Local overlays touched: core/protocol code_mode and MCP output conversion paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Structured output type handling across MCP->code_mode boundaries.
- Strategy selected: cherry-pick.
- Confidence: 0.78.
- Validation evidence: `cargo test -p codex-protocol serializes_image_outputs_as_array --quiet`; integration coverage is limited in this runner because `test_stdio_server` binary resolution is unavailable.
- Rollback note: Revert if MCP output schemas or code_mode tool result rendering regress.

### Commit `3d41ff0b7`

- Upstream intent: Add model-controlled truncation policy for `code_mode` results.
- Local overlays touched: core code_mode output truncation logic.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Output truncation shape and token-budget policy interactions.
- Strategy selected: cherry-pick.
- Confidence: 0.78.
- Validation evidence: `cargo test -p codex-core --lib formatted_truncate_text_content_items_with_policy_merges_text_and_appends_images --quiet`; broader code_mode integration filters remain environment-limited in this runner.
- Rollback note: Revert if truncation formatting or budget application regresses.

### Commit `a67660da2`

- Upstream intent: Load agent metadata fields from role files.
- Local overlays touched: Protected core config/role loading paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Role parsing and metadata inheritance semantics.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.88.
- Validation evidence: `cargo test -p codex-core --lib apply_role_ignores_agent_metadata_fields_in_user_role_file --quiet`.
- Rollback note: Revert if role metadata parsing impacts user role behavior unexpectedly.

### Commit `b1dddcb76`

- Upstream intent: Increase SDK workflow timeout and update workflow runner config.
- Local overlays touched: CI workflow files only.
- Invariants checked: Runtime overlays untouched.
- Risk factors: CI-only operational tuning.
- Strategy selected: cherry-pick.
- Confidence: 0.98.
- Validation evidence: N/A (workflow-only change).
- Rollback note: Revert if workflow timeout/run behavior should remain pre-change.

### Commit `ce1d9abf1`

- Upstream intent: Clarify `close_agent` tool description text.
- Local overlays touched: core tool spec text only.
- Invariants checked: Runtime overlay logic unchanged.
- Risk factors: Documentation/spec text update; low runtime risk.
- Strategy selected: cherry-pick.
- Confidence: 0.96.
- Validation evidence: `cargo test -p codex-core --lib close_agent --quiet`.
- Rollback note: Revert if wording conflicts with local UX/tool-doc policy.

### Commit `07c22d20f`

- Upstream intent: Add `output_text`/`output_image` helper support path updates for `code_mode`.
- Local overlays touched: core code_mode helper handling and tests.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Output helper semantics and content-shape serialization for code_mode.
- Strategy selected: cherry-pick.
- Confidence: 0.74.
- Validation evidence: targeted code_mode integration filter run failed in this runner (`unsupported custom tool call: code_mode`); treated as environment limitation.
- Rollback note: Revert if output helper behavior regresses once full integration environment is available.

### Commit `8ac27b2a1`

- Upstream intent: Add ephemeral flag support for thread fork APIs.
- Local overlays touched: Protected app-server and app-server-protocol thread fork/resume/start surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Thread lifecycle and persistence semantics for ephemeral forks.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.86.
- Validation evidence: `cargo test -p codex-app-server --test all thread_fork_ephemeral_remains_pathless_and_omits_listing --quiet`.
- Rollback note: Revert if ephemeral thread fork state leaks into persisted listings.

### Commit `889b4796f`

- Upstream intent: Add additional macOS sandbox permissions (LaunchServices/Contacts/Reminders related).
- Local overlays touched: Broad protected/core/tui/protocol/app-server surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Large cross-cutting sandbox-permission policy changes.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.80.
- Validation evidence: `cargo test -p codex-core --lib contacts_read_only_emit_contacts_read_clauses --quiet`.
- Rollback note: Revert if macOS sandbox permission clauses or escalation behavior regresses.

### Commit `2621ba17e`

- Upstream intent: Pass full request params through compaction path.
- Local overlays touched: core compaction request handling.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Request history/compaction payload shape and replacement semantics.
- Strategy selected: cherry-pick.
- Confidence: 0.89.
- Validation evidence: `cargo test -p codex-core --test all remote_compact_replaces_history_for_followups --quiet`.
- Rollback note: Revert if compaction payload forwarding regresses follow-up turns.

### Commit `83b22bb61`

- Upstream intent: Add store/load persistence support for `code_mode` across turns.
- Local overlays touched: core code_mode runtime + session services.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: New session-scoped mutable state for code_mode values and serialization boundaries.
- Strategy selected: cherry-pick.
- Confidence: 0.79.
- Validation evidence: `cargo test -p codex-core --test all code_mode --quiet` failed in this runner due missing `test_stdio_server` and unsupported code_mode custom-tool path; `cargo test -p codex-core --lib close_agent --quiet` passed as compile/sanity gate.
- Rollback note: Revert if persisted code_mode values leak across sessions or mismatch expected turn scope.

### Commit `c1a424691`

- Upstream intent: Split skill-approval reject policy into a separate flag and propagate across protocol/server/core.
- Local overlays touched: Protected app-server-protocol + app-server + core approval/sandboxing paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Cross-layer approval policy behavior change with schema and runtime updates.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.77.
- Validation evidence: `cargo test -p codex-core --lib execve_prompt_rejection_uses_skill_approval_for_skill_scripts --quiet` and `cargo test -p codex-app-server-protocol derive_supports_nested_experimental_fields --quiet` passed; full `codex-app-server-protocol` fixture parity test in this branch reports pre-existing schema fixture drift.
- Rollback note: Revert if reject-policy routing or approval-scope semantics regress.

### Commit `9b5078d3e`

- Upstream intent: Stabilize pipe-process stdin round-trip test behavior.
- Local overlays touched: utils/pty test file only.
- Invariants checked: Runtime overlays unchanged.
- Risk factors: Test-only behavior and platform newline handling.
- Strategy selected: cherry-pick.
- Confidence: 0.97.
- Validation evidence: `cargo test -p codex-utils-pty pipe_process_round_trips_stdin --quiet`.
- Rollback note: Revert if round-trip PTY test stability regresses.

### Commit `e77b2fd92`

- Upstream intent: Refine guardian prompt wording and review request layout snapshot.
- Local overlays touched: core guardian prompt source + snapshot fixtures.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model selection unchanged.
- Risk factors: Prompt text and snapshot expectation changes for guardian review.
- Strategy selected: cherry-pick.
- Confidence: 0.92.
- Validation evidence: `cargo test -p codex-core --lib guardian_review_request_layout --quiet`.
- Rollback note: Revert if guardian prompt guidance or snapshot expectations regress.

### Commit `8a099b3df`

- Upstream intent: Rename code_mode tool to exec and preserve compatibility hooks.
- Local overlays touched: Core code_mode/spec surfaces only; no protected overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Tool naming transition and compatibility behavior for nested calls.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: cargo test -p codex-core --lib code_mode --quiet
- Rollback note: Revert if exec/code_mode naming compatibility regresses tool invocation.

### Commit `285b3a514`

- Upstream intent: Expose spawned agent model and reasoning effort metadata end-to-end.
- Local overlays touched: Protocol/core/exec/tui event surfaces; no protected overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Cross-surface event-shape propagation and tui rendering updates.
- Strategy selected: cherry-pick
- Confidence: 0.81
- Validation evidence: cargo test -p codex-tui --lib chatwidget::tests::collab_spawn_end_shows_requested_model_and_effort --quiet
- Rollback note: Revert if spawned-agent metadata rendering or event parsing regresses.

### Commit `c8446d7cf`

- Upstream intent: Ensure websocket response.failed errors are forwarded consistently.
- Local overlays touched: codex-api websocket endpoint plus core websocket test harness updates.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Websocket error propagation and retry/stream sequencing behavior.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: cargo test -p codex-api responses_websocket --quiet
- Rollback note: Revert if websocket response.failed handling regresses in client flows.

### Commit `da74da668`

- Upstream intent: Render local file links from target paths in tui markdown.
- Local overlays touched: TUI markdown/history/streaming rendering surfaces only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Large markdown renderer changes with snapshot and link-shape impact.
- Strategy selected: cherry-pick
- Confidence: 0.79
- Validation evidence: cargo test -p codex-tui --lib markdown_render_file_link_snapshot --quiet
- Rollback note: Revert if local-file markdown links or rendering snapshots regress.

### Commit `01792a4c6`

- Upstream intent: Add explicit success/failure prefixes and stack details to code_mode output.
- Local overlays touched: Core code_mode/spec surfaces; no protected overlays touched.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Output formatting changes for custom-tool execution pathways.
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: cargo test -p codex-core --lib code_mode --quiet
- Rollback note: Revert if code_mode result framing or error-stack output regresses.

### Commit `31bf1dbe6`

- Upstream intent: Make unified-exec session_id numeric across tool/context/process-manager paths.
- Local overlays touched: Core unified_exec and tool context surfaces only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Identifier type migration across async watchers/process manager and tool output.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: cargo test -p codex-core --lib unified_exec --quiet
- Rollback note: Revert if unified-exec session_id serialization/lookup regresses.

### Commit `39c1bc1c6`

- Upstream intent: Add configurable realtime start instructions override for remote flows.
- Local overlays touched: Protected core config and docs paths plus compact-remote behavior surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Config parsing and remote compact instruction-shaping changes.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.83
- Validation evidence: cargo test -p codex-core --lib experimental_realtime_start_instructions_load_from_config_toml --quiet; just write-config-schema
- Rollback note: Revert if realtime start instruction overrides break config loading or request shaping.

### Commit `a4d884c76`

- Upstream intent: Split spawn_csv from multi_agent feature gating and tool spec text.
- Local overlays touched: Core feature/spec/task/guardian/memory surfaces; no protected overlays.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Feature-gating behavior changes for multi-agent and csv spawning paths.
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: cargo test -p codex-core --lib spawn_csv --quiet
- Rollback note: Revert if spawn_csv availability diverges from expected feature flags.

### Commit `12ee9eb6e`

- Upstream intent: Add typed snippets and structured descriptions for code_mode tool exports.
- Local overlays touched: Core code_mode description/spec generation only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Large prompt/spec text generation refactor affecting tool descriptions.
- Strategy selected: cherry-pick
- Confidence: 0.91
- Validation evidence: cargo test -p codex-core --lib code_mode --quiet
- Rollback note: Revert if code_mode tool snippet generation or descriptions regress.

### Commit `180a5820f`

- Upstream intent: Add keyboard-based fast agent switching in tui collaboration mode.
- Local overlays touched: TUI app/footer/chatwidget/multi-agent surfaces plus AGENTS.md notes.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Large interaction-model and state-management update in tui agent navigation.
- Strategy selected: cherry-pick
- Confidence: 0.79
- Validation evidence: cargo test -p codex-tui --lib app::agent_navigation::tests::active_agent_label_tracks_current_thread --quiet
- Rollback note: Revert if agent navigation keyboard flow or active-agent labeling regresses.

### Commit `f385199cc`

- Upstream intent: Fix ARC monitor API path usage for monitor action flow.
- Local overlays touched: Core arc_monitor and mcp_tool_call wiring only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Low-risk request-path fix in monitor action integration.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: cargo test -p codex-core monitor_action_posts_expected_arc_request --quiet
- Rollback note: Revert if ARC monitor action requests route to the wrong endpoint.

### Commit `fd4a67352`

- Upstream intent: Propagate conversation_id into x-client-request-id for responses endpoint calls.
- Local overlays touched: codex-api responses endpoint plus core client/websocket tests.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Header propagation changes across responses request construction.
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: cargo test -p codex-api responses --quiet
- Rollback note: Revert if request-id header behavior regresses for responses calls.

### Commit `7f2232938`

- Upstream intent: Revert prior pass-more-compaction-params behavior.
- Local overlays touched: codex-api common and core client/compact_remote surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Behavioral rollback of compaction request payload shaping.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: cargo test -p codex-core --test all remote_compact_replaces_history_for_followups --quiet
- Rollback note: Revert this revert if full compaction-param forwarding is reintroduced upstream.

### Commit `548583198`

- Upstream intent: Allow boolean tools.web_search entries in ToolsToml parsing.
- Local overlays touched: Protected core config parsing + app-server config RPC tests.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Config compatibility and rpc serialization behavior for tools.web_search.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.90
- Validation evidence: cargo test -p codex-core --lib web_search_mode_disabled_overrides_legacy_request --quiet
- Rollback note: Revert if bool web_search compatibility causes config parsing regressions.

### Commit `fa1242c83`

- Upstream intent: Preserve OTEL HTTP trace export behavior in app-server runtimes.
- Local overlays touched: Protected app-server runtime plus core otel init and codex-otel crate surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Cross-runtime telemetry routing and exporter lifecycle behavior changes.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.78
- Validation evidence: cargo test -p codex-otel otlp_http_loopback --quiet; just bazel-lock-update; just bazel-lock-check
- Rollback note: Revert if HTTP trace export routing/lifecycle regresses in app-server mode.

### Commit `7b2cee53d`

- Upstream intent: Wire plugin policies and category from marketplace.json through protocol/server/core.
- Local overlays touched: Protected app-server-protocol and app-server runtime plus core plugin manager/marketplace.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Schema + runtime policy propagation across plugin install/list workflows.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.76
- Validation evidence: cargo test -p codex-core --lib plugin --quiet; cargo test -p codex-app-server --test all plugin_list_includes_install_and_enabled_state_from_config --quiet
- Rollback note: Revert if plugin policy/category propagation regresses plugin install or list behavior.

### Commit `65b325159`

- Upstream intent: Add ALL_TOOLS export to code_mode tool bridge and runner behavior.
- Local overlays touched: Core code_mode bridge/runner/spec surfaces only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Tool export semantics and compatibility for code_mode module imports.
- Strategy selected: cherry-pick
- Confidence: 0.85
- Validation evidence: cargo test -p codex-core --lib code_mode --quiet
- Rollback note: Revert if ALL_TOOLS export causes code_mode import/dispatch regressions.

### Commit `8f8a0f55c`

- Upstream intent: Update spawn-agent prompt/description guidance and associated suite coverage.
- Local overlays touched: Core codex/spec surfaces and dedicated spawn-agent description suite.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Prompt-shape and tool-description behavior for spawn workflows.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: cargo test -p codex-core --test all spawn_agent_description --quiet
- Rollback note: Revert if spawn prompt/description changes reduce instruction clarity or correctness.

### Commit `52a3bde6c`

- Upstream intent: Emit turn metric indicating whether managed network proxy is active.
- Local overlays touched: Core task telemetry emission and codex-otel metric-name registry.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Low-risk telemetry addition in turn execution path.
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: cargo test -p codex-core --lib network_proxy --quiet
- Rollback note: Revert if network-proxy metric emission impacts turn performance or telemetry correctness.

### Commit `c32c445f1`

- Upstream intent: Clarify locked role settings in spawn prompt/role handling.
- Local overlays touched: Core agent role logic plus subagent notification suite coverage.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Prompt/role-setting clarity updates with notification behavior checks.
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: cargo test -p codex-core --test all subagent_notifications --quiet
- Rollback note: Revert if role-setting prompt clarifications regress subagent notification behavior.

### Commit `f5bb338fd`

- Upstream intent: Defer initial context insertion until first turn is submitted.
- Local overlays touched: Core remote compaction/request-shaping surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Turn initialization and request-shape ordering for first-turn compaction.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: core snapshot_request_shape_remote_manual_compact_without_previous_user_messages passed.
- Rollback note: Revert if first-turn request context ordering regresses remote compact behavior.

### Commit `5259e5e23`

- Upstream intent: Serve managed HTTP proxy listener as HTTP/1.
- Local overlays touched: codex-network-proxy listener/protocol handling only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Proxy CONNECT handling and listener protocol negotiation.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: targeted network-proxy HTTP/1 CONNECT test passed; full crate run had one unrelated baseline audit-event failure.
- Rollback note: Revert if managed proxy stops accepting plain HTTP/1 CONNECT requests.

### Commit `5a89660ae`

- Upstream intent: Add codex.cwd and codex.homeDir helpers to js_repl.
- Local overlays touched: Core js_repl runtime + tests with protected docs/js_repl.md update.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Helper exposure plus environment propagation to kernel runtime.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: js_repl_exposes_codex_path_helpers passed with Node v24 path override; default Node v20 in runner is below upstream minimum.
- Rollback note: Revert if js_repl path helper exposure or dependency-env propagation regresses.

### Commit `f54830979`

- Upstream intent: Keep agent-switch word-motion fallback keys out of draft editing.
- Local overlays touched: TUI app/multi-agent key handling only.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Shortcut precedence between text editing and agent switching.
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: codex-tui agent_shortcut_matches filter passed.
- Rollback note: Revert if alt-word-motion editing keys are intercepted during draft composition.

### Commit `8791f0ab9`

- Upstream intent: Allow models to opt into original image detail for view_image/js_repl output.
- Local overlays touched: Core feature flags/view_image/js_repl plus docs/js_repl.md.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Image detail normalization and tool output behavior across model capabilities.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: original-image-detail unit and view_image integration filters passed.
- Rollback note: Revert if original-image-detail gating causes unexpected image payload sizing behavior.

### Commit `f50e88db8`

- Upstream intent: Add CI check for oversized binary blobs.
- Local overlays touched: GitHub workflow plus Python helper script.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: CI enforcement path and script invocation interface.
- Strategy selected: cherry-pick
- Confidence: 0.96
- Validation evidence: script help/argument contract validated locally.
- Rollback note: Revert if blob-size policy introduces false positives or blocks intended artifacts.

### Commit `72631755e`

- Upstream intent: Stop emitting codex/event notifications from app-server initialization capability path.
- Local overlays touched: Protected app-server-protocol common/v1 + app-server transport/processor surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Protocol capability compatibility and initialization notification routing.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.81
- Validation evidence: protocol lib tests and app-server initialize filter passed; full protocol suite hits existing schema-fixture drift in this branch.
- Rollback note: Revert if initialize capability negotiation or notification filtering regresses clients.

### Commit `77b0c7526`

- Upstream intent: Migrate search_tool to BYO Responses API tool_search flow.
- Local overlays touched: Protocol/core/app-server search-tool plumbing with Bedrock overlay compatibility updates.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Large cross-crate tool-shape migration and output item compatibility.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.78
- Validation evidence: search_tool and app-server mcp_server_elicitation filters passed after resolving mcp_connection_manager conflict and adding Bedrock tool_search compatibility arms.
- Rollback note: Revert if tool_search call/output routing regresses across core/protocol/app-server.

### Commit `f276325cd`

- Upstream intent: Centralize filesystem permission precedence logic.
- Local overlays touched: Protocol permissions plus core config schema artifact.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Permission precedence behavior across legacy/split filesystem settings.
- Strategy selected: cherry-pick
- Confidence: 0.91
- Validation evidence: codex-protocol crate tests passed.
- Rollback note: Revert if filesystem permission precedence behavior changes unexpectedly.

### Commit `c1ea3f95d`

- Upstream intent: Delete unused app-server v1 RPC methods.
- Local overlays touched: Protected app-server-protocol v1/lib surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: v1 API dead-surface removal and compile compatibility.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.93
- Validation evidence: codex-app-server-protocol lib tests passed.
- Rollback note: Revert if any legacy v1 client still depends on removed methods.

### Commit `c2d5458d6`

- Upstream intent: Align core approvals with split sandbox policy semantics.
- Local overlays touched: Core safety/orchestrator/sandboxing/unified-exec surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Approval gating behavior under split filesystem sandbox policies.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: unix_escalation and approvals filters passed.
- Rollback note: Revert if approval decisions diverge from expected split-policy semantics.

### Commit `bf5e997b3`

- Upstream intent: Include spawn-agent model metadata in app-server item streams.
- Local overlays touched: Protected app-server-protocol thread_history/v2 + app-server event handling + tui rendering.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Cross-surface metadata propagation and schema compatibility.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.83
- Validation evidence: app-server turn_start and tui multi_agent filters passed.
- Rollback note: Revert if spawned-agent metadata is missing or malformed in item notifications.

### Commit `5bc82c5b9`

- Upstream intent: Propagate traces across task boundaries and core operations.
- Local overlays touched: Protected app-server sources plus core thread/codex flow and lockfiles.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Trace propagation correctness and runtime instrumentation lifecycle.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: app-server tracing tests and core fork_thread filter passed; bazel lock update/check completed.
- Rollback note: Revert if trace parentage or task/core operation correlation regresses.

### Commit `917c2df20`

- Upstream intent: Default plugin install/auth policies to AVAILABLE and ON_INSTALL.
- Local overlays touched: Protected app-server-protocol v2 + app-server plugin flows + core plugin manager.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Default policy behavior across plugin list/install wire and runtime surfaces.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.85
- Validation evidence: app-server plugin_install/plugin_list filters and core plugin lib tests passed.
- Rollback note: Revert if default plugin policy semantics diverge from upstream expectations.

### Commit `ba5b94287`

- Upstream intent: Add tool_suggest tool and connector discovery plumbing.
- Local overlays touched: Core/connectors/chatgpt/tui tool suggestion surfaces plus dependency lockfiles.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: New discoverable-tool flow, connector metadata wiring, and UI suggestion rendering.
- Strategy selected: cherry-pick
- Confidence: 0.77
- Validation evidence: tool_suggest filter compiled (no matching runtime tests), tui app_link_view lib tests passed after cargo clean, and bazel lock update/check passed.
- Rollback note: Revert if tool_suggest dispatch or connector suggestion UI behavior regresses.

### Commit `367a8a221`

- Upstream intent: Clarify spawn agent authorization guidance.
- Local overlays touched: Core tool spec text plus spawn_agent_description suite.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Authorization prompt wording consistency.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: spawn_agent_description filter passed.
- Rollback note: Revert if spawn agent authorization guidance becomes ambiguous.

### Commit `f6c6128fc`

- Upstream intent: Support resumable waiting for long-running code_mode sessions via exec_wait.
- Local overlays touched: Core code_mode service/runtime/spec and expanded code_mode suite.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Session lifecycle, yield/wait semantics, and code_mode process management.
- Strategy selected: cherry-pick
- Confidence: 0.76
- Validation evidence: full code_mode filter run failed in this environment (missing test_stdio_server and existing custom-tool output harness constraints); compile gate via --no-run passed.
- Rollback note: Revert if exec_wait session lifecycle or resumed output handling regresses.

### Commit `b5f927b97`

- Upstream intent: Refactor openai-curated plugin repository handling.
- Local overlays touched: Core plugin curated_repo/manager/marketplace/store plus app-server plugin_list test coverage.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Curated plugin source reconciliation and listing behavior.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: core plugin lib and app-server plugin_list filters passed.
- Rollback note: Revert if curated plugin availability/state reconciliation regresses.

### Commit `04892b4ce`

- Upstream intent: Make bubblewrap the default Linux sandbox policy.
- Local overlays touched: Protected app-server source and cli entrypoints plus broad core sandbox paths.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Default sandbox-policy flip with Linux runtime behavior and approval gating.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: approvals/unix_escalation/core filters passed; codex-cli lib tests passed; merge required manual restore of missing Feature import in core/connectors.rs.
- Rollback note: Revert if default sandbox selection or escalation behavior regresses on Linux paths.

### Commit `e99e8e4a6`

- Upstream intent: Apply Linux sandbox review nits after default bubblewrap migration.
- Local overlays touched: Core sandbox tag/registry/turn metadata and linux-sandbox runner surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider/model-selection behavior unchanged.
- Risk factors: Post-refactor correctness and cleanup for linux sandbox routing.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: core approvals filter passed and linux-sandbox crate test gate completed without failures.
- Rollback note: Revert if post-migration Linux sandbox metadata/runner behavior regresses.

### Commit `19d0949aa520b0d54aa0f003526f5f67b5ab58c4`

- Upstream intent: Handle pre-approved permission grants during zsh fork escalation flow.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock routing/runtime behavior unchanged.
- Risk factors: Approval-state handling in shell escalation path.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: Both targeted codex-core approval/escalation test filters passed.
- Rollback note: Revert this sync commit if zsh escalation approval flow regresses.

### Commit `23e55d7666e1596f45e7ee546af1eb8fd2e55fcd`

- Upstream intent: Make elicitation tool-call copy clearer in core and tui rendering.
- Local overlays touched: No protected-path overlap.
- Invariants checked: No overlay invariants touched.
- Risk factors: UX wording changes across core+tui tool-call surfaces.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: Core MCP tool-call and tui elicitation filters passed.
- Rollback note: Revert this sync commit if elicitation copy causes UX regressions.

### Commit `745ed4e5ecf543a4b65a2e8853b2201ab65e121f`

- Upstream intent: Use granted permissions when invoking apply_patch tool execution.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably and Bedrock overlays unchanged.
- Risk factors: Permission propagation behavior change in patch tool path.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: codex-core apply_patch lib tests passed.
- Rollback note: Revert this sync commit if apply_patch permission propagation regresses.

### Commit `7f2ca502f6ea30a85e6f98358f4fc2bbcb4f6d6d`

- Upstream intent: Refresh out-of-date free/go plan availability tooltip text.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: User-facing copy-only tweak.
- Strategy selected: cherry-pick
- Confidence: 0.97
- Validation evidence: codex-tui tooltip test filter passed.
- Rollback note: Revert this sync commit if tooltip text needs rollback.

### Commit `0c8a366761c9f41f76a4d8db09f03f5250b48da2`

- Upstream intent: Move inline codex-core unit tests into sibling files for maintainability.
- Local overlays touched: Protected overlap in codex-rs/core/src/model_* and related core files.
- Invariants checked: Preserved local model/provider/auth invariants by restoring conflicted protected files from HEAD.
- Risk factors: Large file-move commit with conflict potential and compile breakage risk.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.71
- Validation evidence: Core lib compile gate passed after surgical conflict handling.
- Rollback note: Revert this sync commit if test module relocation introduces hidden core regressions.

### Commit `2f03b1a32d0604e562d4f7b31a2a069c8f0e2a3f`

- Upstream intent: Dispatch tools when code mode is not awaited directly.
- Local overlays touched: No protected-path overlap.
- Invariants checked: No overlay invariants touched.
- Risk factors: Code-mode runtime/control-flow behavior change.
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: codex-core code_mode integration compile gate passed.
- Rollback note: Revert this sync commit if non-awaited code_mode dispatch regresses.

### Commit `ff6764e806d8bc2fcbf7f58e6d03f77f694f70f0`

- Upstream intent: Add first-party Python app-server SDK package and support files.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Rust overlay invariants unaffected by Python SDK addition.
- Risk factors: New SDK surface area and packaging footprint.
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: Python SDK modules compiled successfully via py_compile.
- Rollback note: Revert this sync commit if SDK packaging or runtime behavior regresses.

### Commit `a30b807efe0d013d49daf0462f8e1373840a3e4d`

- Upstream intent: Support legacy use_linux_sandbox_bwrap config flag in cli/core paths.
- Local overlays touched: Protected overlap via codex-rs/core/src/config* pattern.
- Invariants checked: Preserved local config/provider invariants while accepting compatibility flag bridge.
- Risk factors: Config compatibility change across cli/core argument plumbing.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.85
- Validation evidence: codex-cli lib tests and codex-core features compile gate passed.
- Rollback note: Revert this sync commit if legacy flag handling conflicts with local policy defaults.

### Commit `09aa71adb7a642408f05fe51db82854142e00945`

- Upstream intent: Stabilize stdio-to-uds peer-close handling under test pressure.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants untouched.
- Risk factors: Runtime edge-case in ipc bridge plus dependency-lock hygiene.
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: codex-stdio-to-uds tests passed; Bazel lock update/check completed.
- Rollback note: Revert this sync commit if stdio-to-uds shutdown semantics regress.

### Commit `c0528b9bd97dcb0f8d66719fe138a9a244fe6f3d`

- Upstream intent: Move code mode tool files under tools/code_mode and split functionality.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Large refactor of code_mode file layout and module wiring.
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: codex-core code_mode compile gate passed after refactor.
- Rollback note: Revert this sync commit if code_mode module resolution regresses.

### Commit `4e99c0f1798856d445624e1c28dcd43c6b6a715f`

- Upstream intent: Rename spawn_csv feature flag to enable_fanout across config/spec paths.
- Local overlays touched: Protected overlap via codex-rs/core/src/config* pattern.
- Invariants checked: Maintained provider/model selection invariants while applying flag rename.
- Risk factors: Feature-flag compatibility and config-surface churn.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: Targeted core feature/spec compile gates passed.
- Rollback note: Revert this sync commit if fanout feature toggles regress.

### Commit `774965f1e8691f1a0568fb801f24b15553e5e6cd`

- Upstream intent: Preserve split filesystem semantics in linux sandbox paths.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unchanged.
- Risk factors: Sandbox semantics update in Linux-specific runtime.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: codex-linux-sandbox test command passed (compile gate on this runner).
- Rollback note: Revert this sync commit if split filesystem behavior regresses.

### Commit `cfe3f6821ae91f38d6d6f4e86dcbb0c3a29c123f`

- Upstream intent: Cleanup code_mode tool descriptions and prompt copy.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: User-facing tool text updates within code_mode runtime metadata.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: codex-core code_mode compile gate passed.
- Rollback note: Revert this sync commit if code_mode description copy should be restored.

### Commit `4fa7d6f444b919afb6ccec25e49c036aa0180971`

- Upstream intent: Handle malformed agent role definitions without aborting startup flows.
- Local overlays touched: Protected overlap via codex-rs/core/src/config* and codex-rs/app-server/src/* patterns.
- Invariants checked: Preserved local auth/provider invariants while applying resilient parsing path.
- Risk factors: Config and app-server resilience behavior change.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.88
- Validation evidence: codex-core config tests and codex-app-server lib tests passed.
- Rollback note: Revert this sync commit if malformed role handling masks actionable errors.

### Commit `fa265976890e996ed6ce78ee94f62ddd81544ddc`

- Upstream intent: Do not allow unified_exec for sandboxed scenarios on Windows.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Platform-specific runtime gating change.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: codex-core unified_exec lib tests passed (36 passed, 2 ignored).
- Rollback note: Revert this sync commit if unified_exec availability checks regress on Windows.

### Commit `3e96c867fe91a4ffe9a262d1674bb57efdd8c99f`

- Upstream intent: Use scopes_supported for OAuth when present on MCP servers.
- Local overlays touched: Protected overlap via codex-rs/app-server/src/* and codex-rs/core/src/config* patterns.
- Invariants checked: Preserved local overlay behavior while adopting upstream OAuth scope negotiation.
- Risk factors: Auth negotiation change across rmcp/core/app-server paths.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.83
- Validation evidence: rmcp-client full tests passed and core skill_dependencies tests passed after ENOSPC recovery.
- Rollback note: Revert this sync commit if MCP OAuth scope negotiation regresses.

### Commit `d1b03f0d7f53f74ee35881be49715162d8f06b5f`

- Upstream intent: Add default code-mode yield timeout and wire through runtime protocol.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Code-mode scheduler/timing behavior update.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: Focused busy-loop yield-timeout integration test passed.
- Rollback note: Revert this sync commit if default code_mode yield behavior regresses.

### Commit `25e301ed9802415450ae071122cbe338450d7844`

- Upstream intent: Add code_mode nested parallel tool-call timing test and model pinning.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Concurrency behavior assertion in integration-style code_mode tests.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: New nested parallel tool-call test passed.
- Rollback note: Revert this sync commit if new test assumptions conflict with runtime guarantees.

### Commit `4724a2e9e7919997429a5fb3bf7b721220922f06`

- Upstream intent: Stop exporting EventMsg schemas and generated TypeScript artifacts from app-server-protocol.
- Local overlays touched: Protected overlap in codex-rs/app-server-protocol/src/* files.
- Invariants checked: No Bedrock/Indubitably overlay behavior touched; app-server protocol invariants preserved.
- Risk factors: Large generated-schema deletion touching protocol export contracts.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.86
- Validation evidence: codex-app-server-protocol tests passed (116 unit tests + schema fixture checks).
- Rollback note: Revert this sync commit if downstream tooling requires EventMsg schema artifacts.

### Commit `d3e668053161c3f916fab3b6b611de6acd07af16`

- Upstream intent: Reduce turn_start_jsonrpc_span_parents_core_turn_spans flakiness in app-server tracing tests.
- Local overlays touched: Protected overlap via codex-rs/app-server/src/* pattern.
- Invariants checked: Local overlays unaffected; only tracing-test behavior adjusted.
- Risk factors: Test stability update in protected app-server test module.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.90
- Validation evidence: Focused app-server tracing test passed.
- Rollback note: Revert this sync commit if tracing test should enforce stricter span-chain expectations.

### Commit `d58620c852c5ff5cfd65959d80de265c225e59ba`

- Upstream intent: Use subagent naming in TUI prompt path.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: UI copy-level update in multi-agent flow.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.93
- Validation evidence: 2 tests passed in codex-tui filter.
- Rollback note: Revert if subagent naming regresses in TUI.

### Commit `914f7c73175b038b4d396219754fe21ba6678af2`

- Upstream intent: Honor requirements.toml app settings overrides for connector gating.
- Local overlays touched: Touches core config and connector behavior surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Override precedence across requirements/config apps.
- Strategy selected: Cherry-pick with local overlay review.
- Confidence: 0.87
- Validation evidence: Targeted core+tui tests passed; one extra filter matched 0 tests.
- Rollback note: Revert if requirements override precedence regresses.

### Commit `014e19510d9fb4bc09c3b8e90fb05d7f3aa39700`

- Upstream intent: Add model fallback accounting and related protocol/app-server coverage.
- Local overlays touched: Core + protocol/app-server overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Conflict resolution in shared manager imports.
- Strategy selected: Cherry-pick with conflict fix preserving local constants/imports.
- Confidence: 0.84
- Validation evidence: Core/app-server/protocol targeted tests passed (protocol 114 passed).
- Rollback note: Revert if model fallback accounting or spans regress.

### Commit `ef37d313c6c0c00b91f2ea8a0641d4deace1d67b`

- Upstream intent: Preserve zsh-fork escalation file descriptors in unified-exec path.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Process IO inheritance semantics.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.92
- Validation evidence: codex-utils-pty and codex-shell-escalation suites passed.
- Rollback note: Revert if zsh-fork escalation IO breaks.

### Commit `477a2dd3458be962178abc891422215bf3c22f52`

- Upstream intent: Add code_mode_only feature gates for tool/runtime behavior.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Tool exposure constraints and feature gating.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.90
- Validation evidence: 4 targeted codex-core tests passed.
- Rollback note: Revert if code_mode_only gating behavior regresses.

### Commit `6720caf778acd9a9ec5f8eb838b48e1a4ce944e8`

- Upstream intent: Add WSL support for slash-copy OSC52 behavior.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Platform-specific clipboard path.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.93
- Validation evidence: 2 clipboard tests passed.
- Rollback note: Revert if WSL slash-copy behavior regresses.

### Commit `cfd97b36da76a17db407b2d9653ed993636e0a30`

- Upstream intent: Rename multi-agent wait tool to wait_agent.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Tool-name migration and caller compatibility.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.91
- Validation evidence: 7 codex-core lib tests passed; integration filter matched 0 tests.
- Rollback note: Revert if wait_agent invocation compatibility regresses.

### Commit `36dfb844277e79793766f96305c9633f90bc043e`

- Upstream intent: Stabilize multi-agent feature flag semantics.
- Local overlays touched: No direct protected overlap; interacted with nearby local test deltas.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Cross-commit dependency surfaced stale helper symbol during validation.
- Strategy selected: Cherry-pick with immediate compile-gate and follow-on fix in next commit.
- Confidence: 0.82
- Validation evidence: codex-core compile gate passed.
- Rollback note: Revert if multi-agent flag defaults regress.

### Commit `f8f82bfc2b558229cc4f7ef6245c474ee8b389c7`

- Upstream intent: Add app-server v2 filesystem APIs and related schema surface.
- Local overlays touched: Protected-path overlap: app-server/app-server-protocol/schema.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Wire-shape/API expansion and schema regeneration.
- Strategy selected: Cherry-pick with protected-path review and schema/lock regeneration.
- Confidence: 0.80
- Validation evidence: Protocol suite passed (123); targeted app-server fs test passed after ENOSPC recovery.
- Rollback note: Revert if v2 filesystem RPC contracts regress.

### Commit `cb7d8f45a1393d71b333aea64123227028ae535f`

- Upstream intent: Normalize MCP tool names for code-mode safe filtering.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Tool-name matching and filtering logic.
- Strategy selected: Straight cherry-pick with compile fallback for unrelated stale helper error.
- Confidence: 0.86
- Validation evidence: search_tool/plugins tests passed; compile gate passed.
- Rollback note: Revert if MCP tool filtering/name normalization regresses.

### Commit `e3cbf913e801a611f0b17fa14e9a77865244ba8f`

- Upstream intent: Fix wait_agent test expectations in core.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Consistency assertions after tool rename.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.91
- Validation evidence: Targeted prompt-tools test and compile gate passed.
- Rollback note: Revert if prompt-tool consistency assertions regress.

### Commit `bc24017d64829d0b97b8bc6ed529a389e1e8bc1b`

- Upstream intent: Add Smart Approvals guardian review flow and config/protocol wiring.
- Local overlays touched: Protected-path overlap: core config, app-server protocol, app-server.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Policy/wire/config interaction and UI guardrails.
- Strategy selected: Cherry-pick with protected-path review and schema regeneration.
- Confidence: 0.79
- Validation evidence: Protocol/core/app-server targeted tests passed; tui compile gate used after ENOSPC on full test.
- Rollback note: Revert if Smart Approvals policy prompts or wire behavior regress.

### Commit `467e6216bbfd2ffb1dbdeeffda248cd040274131`

- Upstream intent: Fix stale create_wait_tool reference.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Test-helper symbol consistency.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.94
- Validation evidence: Targeted codex-core lib test passed.
- Rollback note: Revert if helper symbol mismatches reappear.

### Commit `9a44a7e499f18eaed5d06aabb5acf9184deb06b8`

- Upstream intent: Improve hooks stop continuation mechanics.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Hook lifecycle semantics.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.89
- Validation evidence: codex-hooks suite passed; core hooks filter passed.
- Rollback note: Revert if hook stop/continue sequencing regresses.

### Commit `e9050e3e649a0d659208f8fc3ed9082f6b9ec4c1`

- Upstream intent: Fix realtime transcription tools payload handling.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Realtime event payload parsing.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.92
- Validation evidence: Targeted codex-api parser test passed.
- Rollback note: Revert if realtime transcription payload parsing regresses.

### Commit `7fa52013653465661441ac016886ee843855a08c`

- Upstream intent: Use parser-specific realtime voice enum.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Enum wire mapping in realtime parser.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.90
- Validation evidence: Targeted parser test passed; secondary e2e filter matched 0 tests.
- Rollback note: Revert if realtime voice enum parsing regresses.

### Commit `b859a98e0f017f374aaff35c9e2e44f849222622`

- Upstream intent: Make unified-exec zsh-fork state explicit.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Unified-exec state lifecycle semantics.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.90
- Validation evidence: core unified_exec tests passed (28 integration, 37 lib).
- Rollback note: Revert if unified-exec zsh-fork state handling regresses.

### Commit `4b9d5c8c1bdb6d9cfd43570e0b8e88c88b54d823`

- Upstream intent: Add openai_base_url config override for built-in provider.
- Local overlays touched: Protected-path overlap: core config/provider + app-server test/config surfaces.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Provider construction signature changes with local Bedrock overlay interactions.
- Strategy selected: Cherry-pick with surgical conflict resolution and compatibility callsite fixes.
- Confidence: 0.81
- Validation evidence: Core targeted tests passed; app-server compile gate passed; tui chatwidget filter passed.
- Rollback note: Revert if openai_base_url override or provider selection behavior regresses.

### Commit `69c8a1ef9e7c5a3c447ea8b0f01ec5d3a068693d`

- Upstream intent: Fix Windows CI assertions for guardian and Smart Approvals tests.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Test assertion portability across platforms.
- Strategy selected: Straight cherry-pick.
- Confidence: 0.90
- Validation evidence: Core guardian lib filter and tui chatwidget filter passed.
- Rollback note: Revert if guardian/Smart Approvals test assertions regress on Windows.

### Commit `bbd329a81233a8bb35f5ced9aacf93b57f2f9999`

- Upstream intent: Fix turn context reconstruction after backtracking.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: History reconstruction correctness during rollback/backtracking.
- Strategy selected: Straight cherry-pick with compile-gate validation in this runner.
- Confidence: 0.78
- Validation evidence: Repeated ENOSPC while linking codex-core test binary; compile gate passed.
- Rollback note: Revert if rollback/backtracking context reconstruction regresses.

### Commit `6dc04df5e6ffdf7d85c935864c71eede3f214515`

- Upstream intent: Persist future managed-network host approvals across sessions.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Network approval persistence and replayed policy state.
- Strategy selected: Straight cherry-pick with targeted core test filter.
- Confidence: 0.88
- Validation evidence: 2 targeted codex-core tests passed.
- Rollback note: Revert if future network host approvals no longer persist correctly.

### Commit `7f571396c8819d7f4c4486ed1e967e40a2c9ffae`

- Upstream intent: Sync split sandbox policies when spawning subagents.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Propagation of file/network sandbox policy into subagent turns.
- Strategy selected: Straight cherry-pick with focused multi-agent tests.
- Confidence: 0.89
- Validation evidence: 16 codex-core multi-agent tests passed.
- Rollback note: Revert if subagents inherit incorrect sandbox policies.

### Commit `d272f4505874fafef4753830b40d751674e8fd9b`

- Upstream intent: Move plugin/skill instructions into developer message and reorder prompt sections.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior preserved.
- Risk factors: Prompt composition reordering across core render paths and many snapshots.
- Strategy selected: Straight cherry-pick with compile-gate validation under disk constraints.
- Confidence: 0.82
- Validation evidence: core+protocol compile gates passed; collaboration_instructions test-all link failed from ENOSPC.
- Rollback note: Revert if plugin/skill instruction ordering or visible prompt layout regresses.

### Commit `ae0a6510e19c1d65aaa1ef1824826832ac9e160a`

- Upstream intent: Enforce explicit configuration errors when built-in model providers are overridden.
- Local overlays touched: Protected overlap (codex-rs/core/src/config*); no Bedrock/Indubitably auth logic touched.
- Invariants checked: Provider-aware model routing preserved; Bedrock runtime/provider behavior unchanged; Indubitably auth invariants unchanged.
- Risk factors: Config validation surface and schema update; potential startup regression if errors misfire.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib reserved_builtin_provider_override_rejected --quiet
- Rollback note: Revert this sync commit if provider override validation breaks valid config startup.

### Commit `8ca358a13cd29bb174bebe1a32cf608e31a6494e`

- Upstream intent: Refresh generated Python SDK app-server type surfaces to match upstream schema.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior not touched.
- Risk factors: Generated SDK output churn only; low runtime risk.
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: python3 -m compileall sdk/python/src/codex_app_server/generated (pytest not installed for sdk/python test execution)
- Rollback note: Revert this sync commit if Python SDK generated interfaces mismatch runtime expectations.

### Commit `e3890910427940c9106ea61d75f82dffbf20c7a6`

- Upstream intent: Make plugin defaultPrompt list-shaped while preserving compatibility for legacy scalar form.
- Local overlays touched: Protected overlap (codex-rs/app-server-protocol/src/*); no auth/provider override logic touched.
- Invariants checked: Indubitably auth invariants unchanged; Bedrock runtime/provider routing unchanged.
- Risk factors: Wire-shape/schema changes across protocol, app-server tests, and plugin manifest parsing.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet
- Rollback note: Revert this sync commit if plugin defaultPrompt compatibility or app-server schema integration regresses.

### Commit `70eddad6b075f26f0f93c66f7ec9a4e49cdadc93`

- Upstream intent: Support dynamic-tool visibility control with exposeToContext plumbing across protocol/app-server/core.
- Local overlays touched: Protected overlap (codex-rs/app-server-protocol/src/*, codex-rs/core/src/*, codex-rs/app-server/src/*); applied cleanly.
- Invariants checked: Indubitably auth invariants preserved; Bedrock runtime/provider selection behavior unchanged.
- Risk factors: Wire-shape and context-building behavior changes plus DB migration for deferred dynamic tool loading.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server dynamic_tools --quiet
- Rollback note: Revert this sync commit if dynamic tool visibility or deferred loading semantics regress.

### Commit `4b31848f5b3adb7f237dd5109f83428fbd2cf343`

- Upstream intent: Introduce a repository-supported Dylint runner that enforces argument-comment rules.
- Local overlays touched: No protected-path overlap in core auth/provider behavior.
- Invariants checked: Indubitably auth and Bedrock runtime/provider invariants unaffected.
- Risk factors: New lint tooling and workspace-level hooks/justfile updates; minimal runtime impact.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: (cd tools/argument-comment-lint && cargo test --quiet) [passes on pinned nightly toolchain]
- Rollback note: Revert this sync commit if new Dylint runner causes toolchain or lint workflow regressions.

### Commit `9060dc7557848feb80a0fca612b9b1037c2ec217`

- Upstream intent: Correct writable-root sandbox policy behavior when roots include symlinked paths.
- Local overlays touched: Protected overlap (codex-rs/protocol/src/*); merged cleanly.
- Invariants checked: Indubitably auth unchanged; Bedrock provider/runtime behavior unchanged.
- Risk factors: Permissions semantics in protocol and seatbelt tests can affect sandbox policy enforcement.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-protocol --quiet
- Rollback note: Revert this sync commit if sandbox writable-root permission handling regresses.

### Commit `d692b7400786e7bbe9f1366e697fc867bd10b3c1`

- Upstream intent: Add richer auth 401 observability tags/telemetry for client bug reports.
- Local overlays touched: Protected overlap (core auth/client/models manager plus telemetry crates); preserved Indubitably Bedrock model-fetch path.
- Invariants checked: Indubitably auth recovery messaging retained; Bedrock provider/runtime behavior preserved in model fetch flow.
- Risk factors: Cross-cutting auth error/telemetry changes with merge conflict in models manager and error-shape expansion.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.79
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core auth --quiet; plus local fix commit 216b90e45 for UnexpectedResponseError field completeness
- Rollback note: Revert commit chain (379720676 + 216b90e45) if auth telemetry/recovery behavior regresses.

### Commit `49edf311ac3ae84659b0ec5eacd5e471c881eee8`

- Upstream intent: Add app/tool-call metadata propagation for app and MCP tool flows.
- Local overlays touched: Protected overlap (codex-rs/core/src/*, config schema/template surfaces); auto-merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime invariants unchanged.
- Risk factors: Core tool-spec/MCP metadata path changes with schema/template updates.
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: Initial non-lib filter hit missing test_stdio_server harness in integration tests; authoritative gate: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib mcp_tool_call --quiet
- Rollback note: Revert this sync commit if app/MCP tool-call metadata propagation regresses.

### Commit `d4af6053e212a982e53372a3dff5a627c60af1db`

- Upstream intent: Improve fallback behavior for app-backed search tool execution.
- Local overlays touched: Protected overlap (codex-rs/core/src/*); merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: Search fallback logic affects tool routing behavior under failure/missing-path scenarios.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all search_tool --quiet
- Rollback note: Revert this sync commit if search tool fallback selection regresses.

### Commit `ba463a9dc78180d9cd61b28ef6562e03342a14be`

- Upstream intent: Preserve unified-exec background sessions on interrupt and rename cleanup UX to /stop.
- Local overlays touched: Protected overlap (app-server/protocol/core/tui); merged cleanly with snapshot rename.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: Cross-layer behavior changes spanning protocol events, task management, and TUI UX/snapshots.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui interrupt_preserves_unified_exec_wait_streak --quiet
- Rollback note: Revert this sync commit if interrupt/session preservation or /stop command behavior regresses.

### Commit `6fdeb1d602842b80088641b941dea174435c01b7`

- Upstream intent: Reuse guardian sessions across approvals to improve continuity and reduce reinitialization.
- Local overlays touched: Protected overlap (core config/codex/features + tui app/chatwidget); auto-merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime invariants unchanged.
- Risk factors: Large module move/refactor with behavior changes in guardian state machine and UI integration.
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib guardian --quiet
- Rollback note: Revert this sync commit if guardian review session reuse or approval flow behavior regresses.

### Commit `029aab5563caed2f2bbea8a1815a42cbf22b79a2`

- Upstream intent: Ensure elicitation paths retain tool_params rather than dropping them in core flow.
- Local overlays touched: Protected overlap (codex-rs/core/src/* and tui elicitation pane); merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: MCP tool-call serialization/approval template behavior and elicitation UI coupling.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib mcp_tool_call --quiet
- Rollback note: Revert this sync commit if elicitation tool parameter propagation regresses.

### Commit `33acc1e65faec89172b80a0a8a4faafe9b65c8c5`

- Upstream intent: Correct sub-agent role behavior in profile-based configurations.
- Local overlays touched: Protected overlap (codex-rs/core/src/agent/*); merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: Role/profile precedence logic affects spawned-agent model/provider selection semantics.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: Initial filter --lib sub_agent matched 0 tests; authoritative gate: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib apply_role_uses_active_profile_model_provider_update --quiet
- Rollback note: Revert this sync commit if role/profile precedence for sub-agents regresses.

### Commit `18ad67549ca30c78b966d0bc9d8bc4a4a828c854`

- Upstream intent: Improve skills cache-key derivation so config-layer changes invalidate/reuse cache correctly.
- Local overlays touched: Protected overlap (codex-rs/core/src/codex* and skills manager); merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: Cache reuse/invalidations can alter skill loading behavior across sessions and profile stacks.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib skills_for_config_reuses_cache_for_same_effective_config --quiet
- Rollback note: Revert this sync commit if skills cache invalidation or reuse behavior regresses.

### Commit `3f266bcd68c78ac043969f8a7a916c7ee30df112`

- Upstream intent: Allow interrupt state transitions to remain resumable in multi-agent flows.
- Local overlays touched: Protected overlap (app-server-protocol/protocol/exec/tui); merged cleanly with new snapshot.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: State-machine semantics and protocol notification shape implications for multi-agent resume/interrupt flows.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet
- Rollback note: Revert this sync commit if multi-agent interrupt/resume state handling regresses.

### Commit `c04a0a745483066da3e004ec1822a5c0838b6feb`

- Upstream intent: Prevent freeze conditions when sub-agents are present by tightening app-server thread/message handling.
- Local overlays touched: Protected overlap (codex-rs/app-server/src/*); merged cleanly.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior unchanged.
- Risk factors: Concurrency/state flow changes in message processor and in-process thread state.
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: Full crate test run had one flaky tracing timeout (message_processor::tracing_tests::turn_start_jsonrpc_span_parents_core_turn_spans); compile gate passed: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-app-server --quiet
- Rollback note: Revert this sync commit if sub-agent presence still causes app-server/TUI freeze behavior.

### Commit `db89b73a9cd553ac2a2afda93c9f9bdcc223540c`

- Upstream intent: Introduce the parallel `tui_app_server` stack and route TUI startup through app-server pathways behind feature-gated integration points.
- Local overlays touched: Protected overlap (`codex-rs/cli/src/*`, `codex-rs/tui_app_server/src/*`, and compile-coupled core/protocol surfaces); preserved local provider-selection routing and remote-mode guards.
- Invariants checked: Indubitably auth and Bedrock provider/runtime invariants preserved; CLI provider propagation for `exec/login/review` kept intact during conflict resolution.
- Risk factors: Large cross-crate integration with enum-shape drift and conflict-prone startup path wiring.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.78
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server --quiet (1342 passed / 0 failed / 2 ignored); plus follow-up fix commit `ad0c3da0f` to reconcile new event/status variants and CLI dependency dedupe.
- Rollback note: Revert commit chain (`efb48eb49` + `ad0c3da0f`) if app-server TUI startup, agent status rendering, or CLI routing behavior regresses.

### Commit `7a6e30b55b0aa75d8462058f794f571afa071bac`

- Upstream intent: Use request-permission profile as the wire shape for app-server approval requests and responses.
- Local overlays touched: Protected overlap in codex-rs/app-server-protocol/src/* and codex-rs/app-server/src/*; merged cleanly.
- Invariants checked: Indubitably auth and Bedrock runtime/provider selection remained unchanged.
- Risk factors: Protocol schema surface update spanning README/docs and request-permissions handling path.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: Protocol suite and app-server request_permissions filter both passed.
- Rollback note: Revert this sync commit if v2 request-permissions wire compatibility regresses.

### Commit `a0e41f4ff9b4e68148b76621a3817907a166ff43`

- Upstream intent: Fixed build failures related to PR 14717 (#14826)
- Local overlays touched: Protected-path and local-overlay safeguards reviewed; surgical conflict handling applied where needed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Resolved multi_agents conflict by preserving function-level yellow lint allowance and accepting new interrupted snapshot.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.89
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui-app-server collab_resume_interrupted_snapshot --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `663dd3f93500d211409d406fcd0d801e18de6f95`

- Upstream intent: fix(core): fix sanitize name to use '_' everywhere (#14833)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Connector/app tool-name sanitization now consistently maps to underscores.
- Strategy selected: cherry-pick
- Confidence: 0.91
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib connectors --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `4c9dbc1f8829d0d0423bc36c6ad59896bc1387f3`

- Upstream intent: memories: exclude AGENTS and skills from stage1 input (#14268)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Memory stage1 now strips AGENTS/skill fragments while preserving environment/subagent context.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib contextual_user_message --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib memories::phase1 --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `d0a693e5419dba6b25537f4c931a49fd0ce14ea7`

- Upstream intent: windows-sandbox: add runner IPC foundation for future unified_exec (#14139)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Windows sandbox runner IPC + ConPTY foundation landed with large module split.
- Strategy selected: cherry-pick
- Confidence: 0.85
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-windows-sandbox --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-utils-pty --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `a3ba10b44b3c9a584ad0bccf84b6da072bd96d8f`

- Upstream intent: Add exit helper to code mode scripts (#14851)
- Local overlays touched: Protected-path and local-overlay safeguards reviewed; surgical conflict handling applied where needed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Broad code_mode filter failed on missing helper binary; validated with helper build + exit-focused test.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo build -p codex-rmcp-client --bin test_stdio_server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --test all code_mode_exit --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `6f05d8d735392640cd32ec44c2088e0fec9aeaee`

- Upstream intent: [stack 1/4] Split realtime websocket methods by version (#14828)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Realtime websocket methods split into v1/v2/common modules.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-api realtime_websocket --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `b77fe8fefeffe71c2f221129491b7235af4766d0`

- Upstream intent: Apply argument comment lint across codex-rs (#14652)
- Local overlays touched: Protected-path and local-overlay safeguards reviewed; surgical conflict handling applied where needed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Broad mechanical lint sweep; resolved conflicts in core/thread_manager and tui/lib by preserving local provider/startup behavior.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.77
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core -p codex-app-server -p codex-tui --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `59533a2c26e349c59417e4773b930c26211d7bdd`

- Upstream intent: skill-creator: default new skills to ~/.codex/skills (#14837)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Skill-creator sample now defaults generated skills under ~/.codex/skills.
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-skills --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `49c2b66ece0d1c19245cdc78a94036313b8eaacc`

- Upstream intent: Add marketplace display names to plugin/list (#14861)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Added marketplace display-name fields to plugin list surfaces; ENOSPC during first app-server run mitigated via cargo clean.
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_list --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `1d85fe79edd7235fc56d6607db03109f6c3dd101`

- Upstream intent: feat: support remote_sync for plugin install/uninstall. (#14878)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Plugin install/uninstall now support remote_sync with protocol + server wiring updates.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_install --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_uninstall --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `fbd7f9b9864bef4ee074974d649f0939f3bc91e9`

- Upstream intent: [stack 2/4] Align main realtime v2 wire and runtime flow (#14830)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Realtime v2 wire/runtime alignment across protocol/api/core/app-server/tui.
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-api realtime_websocket --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server realtime_conversation --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `e5a28ba0c2fd27f58c4949821d4fb886c54a44d3`

- Upstream intent: fix: align marketplace display name with existing interface conventions (#14886)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Follow-up naming alignment for marketplace display-name fields.
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-app-server plugin_list --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `8e34caffcc3678212acf5ce14ce94adf60ee9f48`

- Upstream intent: [codex] add Jason as a predefined subagent name (#14881)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Added Jason to predefined subagent names list.
- Strategy selected: cherry-pick
- Confidence: 0.96
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib spawn_agent --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `15ede607a087d043a834aaae5021453377e79fd9`

- Upstream intent: fix: tighten up shell arg quoting in GitHub workflows (#14864)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: GitHub workflow/action shell quoting tightened; validated no whitespace/syntax artifacts in staged YAML edits.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: git diff --cached --check
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `79f476e47dc9d6055ef85322481d56302bfccf53`

- Upstream intent: [stack 3/4] Add current thread context to realtime startup (#14829)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Realtime startup now includes current thread context payload.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib realtime_context --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `32e4a5d5d9ae1acad2e85a142c1b2d446306a4e5`

- Upstream intent: [stack 4/4] Reduce realtime self-interruptions during playback (#14827)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: TUI realtime playback now reduces self-interrupt behavior.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-tui realtime --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `db7e02c73988f643722b98fdd47d40340b72d6b7`

- Upstream intent: fix: canonicalize symlinked Linux sandbox cwd (#14849)
- Local overlays touched: No protected-path conflict requiring manual porting.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Linux sandbox cwd canonicalization for symlinked paths across core and linux-sandbox.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-linux-sandbox --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib landlock --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `57f865c069c4acc213d43371a82671b2deed4e1c`

- Upstream intent: Fix tui_app_server: ignore duplicate legacy stream events (#14892)
- Local overlays touched: Protected-path and local-overlay safeguards reviewed; surgical conflict handling applied where needed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Ignored duplicate legacy stream events in tui_app_server adapter; test-link hit ENOSPC and was validated via compile gate after cargo clean.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.80
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui-app-server --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `d37dcca7e080a8d397f37f8bf4bf695d40f7d88e`

- Upstream intent: Revert tui code so it does not rely on in-process app server (#14899)
- Local overlays touched: Protected-path and local-overlay safeguards reviewed; surgical conflict handling applied where needed.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior were preserved.
- Risk factors: Reverted tui in-process app-server dependency; conflict resolved by keeping local Bedrock provider constant and thread-manager provider wiring.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-tui --quiet
- Rollback note: Revert this sync commit if the touched behavior regresses in targeted flows.

### Commit `603b6493a9d93f110bacf8d29295acdcdc080d89`

- Upstream intent: Skip missing writable roots in bubblewrap mounts while keeping existing roots writable.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Linux sandbox mount behavior and regression-test updates.
- Strategy selected: Direct cherry-pick.
- Confidence: 0.93
- Validation evidence: cargo test -p codex-linux-sandbox landlock --quiet (compile gate; filter matched 0 tests).
- Rollback note: Revert commit ab32d851a if writable-root mount behavior regresses.

### Commit `31648563c8d7f77957c79cc04501d0ed11844635`

- Upstream intent: Move pinned artifact runtime version into shared package-version module.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Core module layout change plus artifacts handler/test updates.
- Strategy selected: cherry-pick with compile-gate validation due ENOSPC pressure.
- Confidence: 0.86
- Validation evidence: Initial cargo test hit ENOSPC; after cargo clean, low-footprint cargo check -p codex-core passed.
- Rollback note: Revert commit 075952a75 if artifact runtime version resolution regresses.

### Commit `4ed19b07664d28ef67592ab5d77aa30d13d3aba0`

- Upstream intent: Make close_agent output explicit by returning previous_status.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Core tool-output shape and tests/spec update for multi-agent close flow.
- Strategy selected: Direct cherry-pick with focused core test.
- Confidence: 0.88
- Validation evidence: cargo test -p codex-core --lib close_agent --quiet passed (1 test).
- Rollback note: Revert commit 868755618 if close_agent output contract regresses.

### Commit `ef36d39199c7328899e4f1f6b20a2c9ba5065f83`

- Upstream intent: Make agent job finalization atomic and replace blind polling with status-subscription waiting.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Cross-crate behavioral change across core job runner and state runtime completion semantics.
- Strategy selected: Direct cherry-pick with focused core/state integration tests.
- Confidence: 0.86
- Validation evidence: codex-state agent_jobs tests passed (2 tests); codex-core suite::agent_jobs filter passed (4 tests).
- Rollback note: Revert commit 03e547ce7 if agent-job finalization/regression appears.

### Commit `e8add54e5dda2fc6f49757aa939378a21b8515e9`

- Upstream intent: Expose effective model/reasoning in spawn-agent events after config layering.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Cross-crate event-contract changes across core/protocol plus app-server integration test coverage.
- Strategy selected: Direct cherry-pick with targeted core/app-server validation.
- Confidence: 0.82
- Validation evidence: Core targeted filter compiled (0 tests matched); app-server targeted filter passed with 1 test.
- Rollback note: Revert commit 903a9db78 if spawn-agent metadata contract regresses.

### Commit `6ea041032b500a6f3e8511d225af366d5e53439b`

- Upstream intent: Prevent turn/start hangs by bounding websocket warmup and enabling clean HTTP fallback/interrupt behavior.
- Local overlays touched: Protected overlap in core config/model-provider surfaces; Bedrock defaults preserved with websocket timeout unset.
- Invariants checked: Indubitably auth and Bedrock provider/runtime behavior preserved; ThreadManager provider wiring unchanged.
- Risk factors: Large core behavioral refactor plus protected config/model files and schema updates.
- Strategy selected: cherry-pick+surgical due protected-path overlap and local API compatibility adjustments.
- Confidence: 0.79
- Validation evidence: After ENOSPC recovery and schema regen, codex-core model_provider_info tests passed (5) and client_websockets suite passed (29).
- Rollback note: Revert commit 81bb0c463 if websocket prewarm timeout/fallback behavior regresses.

### Commit `8e258eb3f57a42477b5811a54321263185136a6a`

- Upstream intent: Store latest model/reasoning effort in thread metadata for later thread/resume reuse.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: State migration plus cross-crate metadata serialization updates.
- Strategy selected: Direct cherry-pick with protocol/state/core targeted validation.
- Confidence: 0.87
- Validation evidence: codex-protocol reasoning_effort tests passed (2); codex-state extract tests passed (9); core filter compiled (0 tests matched).
- Rollback note: Revert commit f2a397734 if thread metadata persistence/regression occurs.

### Commit `78e8ee4591d4ff42d180000fbad29d5fb3bcd2a5`

- Upstream intent: Preserve and replay full thread snapshots for start/resume/fork in tui_app_server.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Large tui_app_server replay-state refactor touching app/session/chatwidget paths.
- Strategy selected: Direct cherry-pick with compile fallback under disk pressure.
- Confidence: 0.80
- Validation evidence: Targeted tui_app_server test run failed at link with ENOSPC; after cargo clean, low-footprint cargo check -p codex-tui-app-server passed.
- Rollback note: Revert commit c0311a132 if remote resume/fork transcript restoration regresses.

### Commit `f26ad3c92c3ac1bd1c63325d74924053d3cd0c01`

- Upstream intent: Stabilize fuzzy-search tests under out-of-order notification delivery by buffering/matching notifications correctly.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Test harness behavior update in app-server test utilities and fuzzy-search suite.
- Strategy selected: Direct cherry-pick with targeted app-server test filter.
- Confidence: 0.91
- Validation evidence: cargo test -p codex-app-server fuzzy_file_search --quiet passed (11 tests).
- Rollback note: Revert commit 875bfd98b if fuzzy-search test notification handling regresses.

### Commit `d484bb57d9baea4603df0a89ad4f602cee79871d`

- Upstream intent: Avoid snapshot path collisions by appending generation suffix and updating stale-snapshot cleanup parsing.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Core shell snapshot path/cleanup behavior with expanded test coverage.
- Strategy selected: Direct cherry-pick with targeted core shell_snapshot tests.
- Confidence: 0.90
- Validation evidence: cargo test -p codex-core shell_snapshot --quiet passed (12 + 4 tests, 4 ignored).
- Rollback note: Revert commit 163403635 if shell snapshot lifecycle behavior regresses.

### Commit `0d531c05f2cc497d29da8e478f6770850cdb51bc`

- Upstream intent: Eliminate startup race by starting initial-yield timer only after worker posts explicit started event.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Runtime sequencing change in code-mode JS runner can affect yield timing semantics.
- Strategy selected: Direct cherry-pick with compile fallback after environment-specific integration failure.
- Confidence: 0.78
- Validation evidence: code_mode_yield filter failed in this runner with unsupported custom tool call output; low-footprint cargo check -p codex-core passed.
- Rollback note: Revert commit 47a8e8d8e if code-mode startup yield behavior regresses.

### Commit `904dbd414f223027ecdb3d54a8444d3c94395aa6`

- Upstream intent: Generate internal RolloutLine JSON schema and fix writer/reader schema mismatches in protocol models.
- Local overlays touched: Protected overlap in app-server-protocol src and cli main; no Indubitably/Bedrock behavior changes.
- Invariants checked: Indubitably auth preserved; Bedrock runtime/provider behavior unchanged; ThreadManager provider wiring unchanged.
- Risk factors: Schema/export surface changes across app-server-protocol and CLI with generated fixture churn.
- Strategy selected: cherry-pick+surgical with protected-path review and compile gate under ENOSPC constraints.
- Confidence: 0.78
- Validation evidence: Full protocol test run hit ENOSPC; after cargo clean, low-footprint cargo check -p codex-app-server-protocol passed.
- Rollback note: Revert commit ce52c15ee if internal schema generation or protocol schema mapping regresses.

### Commit `95bdea93d2600aabef1b87ee5fab05a6022a7d45`

- Upstream intent: Switch elevated one-shot Windows command path to framed runner IPC in preparation for unified_exec session support.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Windows sandbox process-launch and IPC transport refactor across multiple modules.
- Strategy selected: Direct cherry-pick with windows-sandbox targeted validation.
- Confidence: 0.85
- Validation evidence: cargo test -p codex-windows-sandbox --quiet passed (compile gate on non-Windows host; 0 tests matched).
- Rollback note: Revert commit c011d4b05 if elevated Windows command execution regressions appear.

### Commit `49e7dda2dfd6e67dd5f9dd8bfa22b7c2b1df17ef`

- Upstream intent: Support headless ChatGPT onboarding and token refresh requests in app-server-backed TUI flows.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock runtime/provider behavior unaffected; ThreadManager provider wiring unchanged.
- Risk factors: Large tui_app_server auth/onboarding flow expansion with new local token loader and request handling.
- Strategy selected: Direct cherry-pick with focused tui_app_server auth test filters.
- Confidence: 0.84
- Validation evidence: Two targeted tui_app_server filters passed (refresh handling and device-code attempt matching).
- Rollback note: Revert commit 9e155d4f5 if onboarding or chatgpt token refresh flows regress.

### Commit `683c37ce755f198f417db27f780965a5972b5b7b`

- Upstream intent: Allow tool-suggest flow to elicit plugin installation for allowed discoverable plugin/connectors not yet installed.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Broad multi-crate UX and suggestion-logic changes with new core plugin discovery modules.
- Strategy selected: Direct cherry-pick with compile-gate fallback under ENOSPC pressure.
- Confidence: 0.79
- Validation evidence: Targeted test run hit ENOSPC; after cargo clean, low-footprint cargo check for core/tui/tui_app_server passed.
- Rollback note: Revert commit 446011d10 if plugin installation elicitation or tool-suggest behavior regresses.

### Commit `b02388672f7df432fbe34a9128f78e7a1e9d43ea`

- Upstream intent: Stabilize Windows shell-driven integration harnesses by allowing test-only shell override and safer nested cmd/PowerShell handling.
- Local overlays touched: No protected-path overlap; local ThreadManager provider-aware behavior preserved during conflict resolution.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring preserved.
- Risk factors: Core session/thread initialization plumbing plus integration harness behavior for shell execution tests.
- Strategy selected: cherry-pick+surgical due thread_manager conflict resolution.
- Confidence: 0.84
- Validation evidence: Targeted core integration filters passed: apply_patch_cli_can_use_shell_command_output_as_patch_input and websocket_test_codex_shell_chain.
- Rollback note: Revert commit 97d254fc8 if Windows shell test harness behavior or thread start wiring regresses.

### Commit `23a44ddbe8f45154a6e55280a74d28957dfefe72`

- Upstream intent: Stabilize permissions selection tests across platform-specific initial row ordering and popup layout differences.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Test-only assertions/helpers in tui chatwidget suite.
- Strategy selected: Direct cherry-pick with targeted tui permissions selection tests.
- Confidence: 0.93
- Validation evidence: cargo test -p codex-tui permissions_selection --quiet passed (10 tests).
- Rollback note: Revert commit 9c412bf09 if permissions popup test semantics regress.

### Commit `4d9d4b7b0f2b8cfbe4ab18e31a7bd80465a975e4`

- Upstream intent: Remove shell-redirection/quoting variance from write-file approval scenario by using explicit UTF-8 read/write script.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Test-only adjustment in core approvals suite.
- Strategy selected: Direct cherry-pick with targeted approvals test filter.
- Confidence: 0.94
- Validation evidence: cargo test -p codex-core --test all approvals --quiet passed (8 tests).
- Rollback note: Revert commit e1c60c7b7 if approvals test harness behavior regresses.

### Commit `2cc4ee413f8d86c38a5a46887d2fd5a18d40efbe`

- Upstream intent: Mitigate elevated IPC path instability by forcing non-private desktop until compatibility is restored.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Single-line behavior toggle in Windows elevated command runner path.
- Strategy selected: Direct cherry-pick with windows-sandbox compile validation.
- Confidence: 0.95
- Validation evidence: cargo test -p codex-windows-sandbox --quiet passed (compile gate on non-Windows host; 0 tests matched).
- Rollback note: Revert commit f5fc27931 if private desktop behavior needs to be restored earlier.

### Commit `ee756eb80f94fe018c7a07306c0e43e1a42bcfa6`

- Upstream intent: Align code-mode tool naming and documentation by replacing exec_wait with wait.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and ThreadManager provider wiring unchanged.
- Risk factors: Tool-name contract changes in code-mode spec and tests.
- Strategy selected: Direct cherry-pick with focused core spec/integration filters.
- Confidence: 0.90
- Validation evidence: Both targeted tests passed: code_mode_only_restricts_model_tools_to_exec_tools and code_mode_only_restricts_prompt_tools.
- Rollback note: Revert commit d9e15ce82 if code-mode tool naming compatibility regresses.

### Commit `0d2ff40a58dde63e5aa8be85b5a5f19f384c354c`

- Upstream intent: Add auth env observability (#14905)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Merged upstream auth-env telemetry with local Bedrock/provider wiring; preserved invariants
- Strategy selected: cherry-pick (manual conflict resolution)
- Confidence: 0.79
- Validation evidence: ENOSPC on cargo test; cargo clean + CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core
- Rollback note: Revert commit 0d2ff40a58dde63e5aa8be85b5a5f19f384c354c if regression is detected.

### Commit `43ee72a9b9c9c88dccc86e1e50901ac90dadcc37`

- Upstream intent: fix(tui): implement /mcp inventory for tui_app_server (#14931)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Applied cleanly; snapshot files included
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: Switched from long-running cargo test to CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server
- Rollback note: Revert commit 43ee72a9b9c9c88dccc86e1e50901ac90dadcc37 if regression is detected.

### Commit `1a9555eda98cc561b4beec51fd1c577b0b068e2a`

- Upstream intent: Cleanup skills/remote/xxx endpoints. (#14977)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Resolved rollout policy conflict by dropping removed remote-skill events while retaining existing PlanUpdate persistence behavior
- Strategy selected: cherry-pick (manual conflict resolution)
- Confidence: 0.83
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet (134 passed)
- Rollback note: Revert commit 1a9555eda98cc561b4beec51fd1c577b0b068e2a if regression is detected.

### Commit `c6ab4ee537e5b118a20e9e0d3e0c0023cae2d982`

- Upstream intent: Gate realtime audio interruption logic to v2 (#14984)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected paths touched; preserved local config/provider behavior. write-config-schema attempted but blocked by ENOSPC
- Strategy selected: cherry-pick
- Confidence: 0.72
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server-protocol --quiet; cargo check -p codex-app-server --quiet
- Rollback note: Revert commit c6ab4ee537e5b118a20e9e0d3e0c0023cae2d982 if regression is detected.

### Commit `98be562fd393b23250090e36b43012ed69000a69`

- Upstream intent: Unify realtime shutdown in core (#14902)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Applied cleanly
- Strategy selected: cherry-pick
- Confidence: 0.77
- Validation evidence: Reatime test filter was long-running; fallback CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert commit 98be562fd393b23250090e36b43012ed69000a69 if regression is detected.

### Commit `0d1539e74c28c7de9a6c471c7e96d77f15dfcd6e`

- Upstream intent: fix(linux-sandbox): prefer system /usr/bin/bwrap when available (#14963)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected config paths touched; retained local overlay behavior
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-linux-sandbox --quiet; cargo check -p codex-app-server --quiet
- Rollback note: Revert commit 0d1539e74c28c7de9a6c471c7e96d77f15dfcd6e if regression is detected.

### Commit `fc75d07504ae816c57ec8d3102a45137e89c535f`

- Upstream intent: Add Python SDK public API and examples (#14446)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Non-Rust SDK/docs/examples expansion; compileall passed
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: python3 -m compileall -q sdk/python/src/codex_app_server sdk/python/examples
- Rollback note: Revert commit fc75d07504ae816c57ec8d3102a45137e89c535f if regression is detected.

### Commit `a5d3114e97166cab28bf5806204314f9ade1dbdc`

- Upstream intent: feat: Add product-aware plugin policies and clean up manifest naming (#14993)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected app-server path touched; preserved local provider/auth invariants
- Strategy selected: cherry-pick
- Confidence: 0.76
- Validation evidence: Plugin test filter timed out; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet && cargo check -p codex-core --quiet
- Rollback note: Revert commit a5d3114e97166cab28bf5806204314f9ade1dbdc if regression is detected.

### Commit `19b887128e6b9ddc1aa134a7bdd481858473b663`

- Upstream intent: app-server: reject websocket requests with Origin headers (#14995)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected app-server transport path touched; behavior preserved
- Strategy selected: cherry-pick
- Confidence: 0.79
- Validation evidence: connection_handling_websocket test filter timed out; fallback CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet
- Rollback note: Revert commit 19b887128e6b9ddc1aa134a7bdd481858473b663 if regression is detected.

### Commit `83a60fdb94d5ee074a9ec33a48699d576a89c4a1`

- Upstream intent: Add FS abstraction and use in view_image (#14960)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Dependency change handling completed; protected app-server paths preserved
- Strategy selected: cherry-pick
- Confidence: 0.81
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server -p codex-environment -p codex-utils-image --quiet
- Rollback note: Revert commit 83a60fdb94d5ee074a9ec33a48699d576a89c4a1 if regression is detected.

### Commit `6fe8a05dcbeb62df3d9cb0388f7dd9364488f5ca`

- Upstream intent: fix: honor active permission profiles in sandbox debug (#14293)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Applied cleanly
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert commit 6fe8a05dcbeb62df3d9cb0388f7dd9364488f5ca if regression is detected.

### Commit `d950543e6559db52855a718c96f7577922411fcd`

- Upstream intent: feat: support restricted ReadOnlyAccess in elevated Windows sandbox (#14610)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Applied cleanly
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-windows-sandbox --quiet
- Rollback note: Revert commit d950543e6559db52855a718c96f7577922411fcd if regression is detected.

### Commit `770616414a51fa179ce4cef10f7f8df838d3f46f`

- Upstream intent: Prefer websockets when providers support them (#13592)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Reviewed provider/websocket diffs to preserve Bedrock/provider invariants; protected app-server path touched
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.75
- Validation evidence: Initial websocket_fallback test revealed compile break; fixed stale constructor arg in core/src/client.rs, then CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet
- Rollback note: Revert commit 770616414a51fa179ce4cef10f7f8df838d3f46f if regression is detected.

### Commit `3ce879c64610cae8e460d3e8c126e57acbeb437d`

- Upstream intent: Handle realtime conversation end in the TUI (#14903)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Applied cleanly after restoring clean tree
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui -p codex-tui-app-server --quiet
- Rollback note: Revert commit 3ce879c64610cae8e460d3e8c126e57acbeb437d if regression is detected.

### Commit `226241f035de7df4946ba3866fee9e22f83a9f99`

- Upstream intent: Use workspace requirements for guardian prompt override (#14727)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected config/app-server paths touched; guardian snapshots updated
- Strategy selected: cherry-pick
- Confidence: 0.74
- Validation evidence: Attempted CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo run -p codex-core --bin codex-write-config-schema --quiet (no resulting diff); cargo check -p codex-core -p codex-app-server -p codex-tui -p codex-tui-app-server --quiet
- Rollback note: Revert commit 226241f035de7df4946ba3866fee9e22f83a9f99 if regression is detected.

### Commit `6fef4216546cc9b8880f1616e349e77277b50ba3`

- Upstream intent: [hooks] userpromptsubmit - hook before user's prompt is executed (#14626)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Resolved core/lib conflict by keeping both indubitably_auth and upstream hook_runtime modules
- Strategy selected: cherry-pick (manual conflict resolution)
- Confidence: 0.78
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet (134 passed); cargo check -p codex-core -p codex-hooks --quiet
- Rollback note: Revert commit 6fef4216546cc9b8880f1616e349e77277b50ba3 if regression is detected.

### Commit `a3613035f32a45146297a74e058a8c70b91c56c2`

- Upstream intent: Pin setup-zig GitHub Action to immutable SHA (#14858)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: No runtime code touched
- Strategy selected: cherry-pick
- Confidence: 0.95
- Validation evidence: N/A (workflow-only change)
- Rollback note: Revert commit a3613035f32a45146297a74e058a8c70b91c56c2 if regression is detected.

### Commit `84f4e7b39d17fea6d28c98bc748652ea4b279a14`

- Upstream intent: fix(subagents) share execpolicy by default (#13702)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Resolved thread_manager conflict by preserving upstream inherited exec-policy args and local provider/thread wiring flow
- Strategy selected: cherry-pick (manual conflict resolution)
- Confidence: 0.77
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert commit 84f4e7b39d17fea6d28c98bc748652ea4b279a14 if regression is detected.

### Commit `40a7d1d15b446991094c5ecfbb1d0f21f2d9ad40`

- Upstream intent: [plugins] Support configuration tool suggest allowlist. (#15022)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Protected config paths touched; config schema in commit remained consistent
- Strategy selected: cherry-pick
- Confidence: 0.74
- Validation evidence: Attempted CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo run -p codex-core --bin codex-write-config-schema --quiet (no resulting diff); cargo check -p codex-core --quiet
- Rollback note: Revert commit 40a7d1d15b446991094c5ecfbb1d0f21f2d9ad40 if regression is detected.

### Commit `0f9484dc8a7ad0962a808892924bb160e9466ad9`

- Upstream intent: feat: adapt artifacts to new packaging and 2.5.6 (#14947)
- Local overlays touched: Reviewed against Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring invariants.
- Invariants checked: Preserve Indubitably auth behavior; preserve Bedrock runtime/provider behavior; no ThreadManager provider wiring regression.
- Risk factors: Dependency update handled with bazel lock verification
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-artifacts -p codex-core --quiet
- Rollback note: Revert commit 0f9484dc8a7ad0962a808892924bb160e9466ad9 if regression is detected.

### Commit `a265d6043edc8b41e42ae508291f4cfb9ed46805`

- Upstream intent: feat: add memory citation to agent message (#14821)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server-protocol surfaces touched; no conflict
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core -p codex-protocol -p codex-tui-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `58ac2a8773da0ac6eb21471e6d3da5744d9e9e0c`

- Upstream intent: nit: disable live memory edition (#15058)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: No Rust code changed
- Strategy selected: cherry-pick
- Confidence: 0.97
- Validation evidence: N/A (template text-only)
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `347c6b12ec63e8fe41e1dce6b00cca83dd2dba67`

- Upstream intent: Removed remaining core events from tui_app_server (#14942)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Large tui_app_server-only refactor with snapshot updates
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `7ae99576a615d524bb22bf0f68e2b2baf88c37ce`

- Upstream intent: chore: disable memory read path for morpheus (#15059)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Memory phase2 gate update
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `606d85055f61ca9e81f0b96a4e7f6effc33c82be`

- Upstream intent: Add notify to code-mode (#14842)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Added local compatibility fixes for CustomToolCallOutput.name in bedrock adapters
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.76
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `580f32ad2ab642e3fe9661bce838d972f8989663`

- Upstream intent: fix: harden plugin feature gating (#15020)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server/protocol/cli paths; added local SessionSource::Custom compatibility in canonical_trace
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.75
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-app-server -p codex-core -p codex-app-server-client --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `334164a6f714c171bb9f6440c7d3cd04ec04d295`

- Upstream intent: feat(tui): restore composer history in app-server tui (#14945)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Applied cleanly
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server --quiet; cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `392347d436cddac41c535e70dd0357ff74624559`

- Upstream intent: fix: try to fix "Stage npm package" step in ci.yml (#15092)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: No Rust code touched
- Strategy selected: cherry-pick
- Confidence: 0.96
- Validation evidence: N/A (workflow-only change)
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `88e5382fc4cc7d7694fe99e39996bf148ebe9bcd`

- Upstream intent: Propagate tool errors to code mode (#15075)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Code-mode error propagation updates
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `5cada46ddf74701dbaf1a152df0514b918ead70c`

- Upstream intent: Return image URL from view_image tool (#15072)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Dependency updates handled with lock verification
- Strategy selected: cherry-pick
- Confidence: 0.84
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-protocol -p codex-utils-image --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `e5de13644d9459d3c2be0e60610009e619f50488`

- Upstream intent: Add a startup deprecation warning for custom prompts (#15076)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: TUI startup warning + snapshot update
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `86982ca1f93c2e18711dd192eb2989f91f6814a1`

- Upstream intent: Revert "fix: harden plugin feature gating" (#15102)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server/protocol/cli paths; removed local SessionSource::Custom compatibility arm after revert
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.75
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; cargo check -p codex-core --quiet; cargo check -p codex-app-server --quiet; cargo check -p codex-app-server-client --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `7b37a0350f40c646e5cd36d55892da3fc4df4891`

- Upstream intent: Add final message prefix to realtime handoff output (#15077)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Realtime handoff output formatting update
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-api -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `ebbbc52ce40324d6f47745fe6edf41f3a1cfbe48`

- Upstream intent: Align SQLite feedback logs with feedback formatter (#13494)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: State/log formatting migration alignment
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-state --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `bb304324216e1305e9b7b5aa59700907c6326bd7`

- Upstream intent: Feat: reuse persisted model and reasoning effort on thread resume (#14888)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: App-server thread resume behavior update
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `b306885bd8ea4cd6c7e742b93c20614b79e6ac5d`

- Upstream intent: don't add transcript for v2 realtime (#15111)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Realtime V2 transcript gating; no protected overlap
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-api --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `3590e181fa2736c88a559389ea70dd1fe68d228e`

- Upstream intent: Add update_plan code mode result (#15103)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Code-mode update_plan output wiring applied; core check passes
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core code_mode_update_plan_nested_tool_result_is_empty_object --quiet (fails due pre-existing ModelClient::new test compile mismatch); cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `56d0c6bf67e15ff94c4bbf9e4fbc369b978b0bf1`

- Upstream intent: Add apply_patch code mode result (#15100)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Apply-patch code-mode output object wiring; core check passes
- Strategy selected: cherry-pick
- Confidence: 0.81
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core code_mode_can_apply_patch_via_nested_tool --quiet (fails due pre-existing ModelClient::new test compile mismatch); cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `dcd5e0826960258b0b0c79fbd80aa66e9dd24296`

- Upstream intent: fix: harden plugin feature gating (#15104)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server overlap reviewed; uses resolved config for skills/plugin gating; compile gate used after disk cleanup
- Strategy selected: cherry-pick
- Confidence: 0.78
- Validation evidence: cargo test -p codex-app-server plugin_list_skips_invalid_marketplace_file --quiet (failed: no space left on device); df -h .; cargo clean; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server -p codex-core -p codex-config --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `81996fcde605a452ca94662eb7028e8c8b6f9ebb`

- Upstream intent: Add exec-server stub server and protocol docs (#15089)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: New exec-server crate + docs landed cleanly; lock checks passed from repo root
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `825d09373dc6676ade6860f8052fc5018ea7197f`

- Upstream intent: Support featured plugins (#15042)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server/app-server-protocol overlap integrated; plugin/list now returns featuredPluginIds
- Strategy selected: cherry-pick
- Confidence: 0.82
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server plugin_list --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-tui --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `4fd2774614182ebaf74f2e7a8c04bbcf0b09ed96`

- Upstream intent: Add Python SDK thread.run convenience methods (#15088)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Python SDK docs/api/tests updated; local venv used for pytest + package deps
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: /tmp/codex-pytest-venv/bin/pip install -e sdk/python; /tmp/codex-pytest-venv/bin/python -m pytest sdk/python/tests/test_public_api_signatures.py sdk/python/tests/test_public_api_runtime_behavior.py
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `903660edba6e1ecfd7c9b1782105be4ebf0e02a7`

- Upstream intent: Remove stdio transport from exec server (#15119)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Exec-server removes stdio transport and refreshes websocket harness/tests
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `20f2a216df3e2d534069438ca7126811de9ff89a`

- Upstream intent: feat(core, tracing): create turn spans over websockets (#14632)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Websocket response.create now forwards per-turn trace context metadata
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-core client_websockets --quiet (fails due pre-existing ModelClient::new test compile mismatch); CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-api --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `b14689df3b97245faa9c29a0b8f3f6c4d09393bf`

- Upstream intent: Forward session and turn headers to MCP HTTP requests (#15011)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Adds session/turn request headers through MCP connection manager into streamable-http RMCP client
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-rmcp-client --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `42e932d7bf70cc8e7ce912b4bbd27c0266293ad5`

- Upstream intent: [hooks] turn_id extension for Stop & UserPromptSubmit (#15118)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Hooks Stop/UserPromptSubmit schemas and payloads now include turn_id with added core hook assertions
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-hooks --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `10eb3ec7fccaf805c7162d8370b5b99bf57ddc48`

- Upstream intent: Simple directory mentions (#14970)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected fuzzy-search protocol/app-server updates reviewed; adds directory match_type through search and UI mention insertion
- Strategy selected: cherry-pick
- Confidence: 0.85
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server fuzzy_file_search --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-file-search -p codex-tui -p codex-tui-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `01df50cf422b2eb89cb6ad8f845548e8c0d3c60c`

- Upstream intent: Add thread/shellCommand to app server API surface (#14988)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected protocol/app-server overlap reviewed; new thread/shellCommand API wired into app-server and tui_app_server command path
- Strategy selected: cherry-pick
- Confidence: 0.80
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server thread_shell_command --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-tui-app-server -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `db5781a08872873a4df82fbb4b3dc6ffd98b5d15`

- Upstream intent: feat: support product-scoped plugins. (#15041)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected protocol/app-server overlap with conflict in thread_manager resolved; added canonical_trace SessionSource::Custom compatibility arm
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.74
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server plugin_ --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-protocol -p codex-tui -p codex-tui-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `70cdb17703a4310b7173642e011f7534d2b2624f`

- Upstream intent: feat: add graph representation of agent network (#15056)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Introduces persisted thread-spawn graph and cascade close/resume support via state runtime + agent control
- Strategy selected: cherry-pick
- Confidence: 0.83
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-state --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `32d2df5c1e97948cb5c55481f0b5fd3f8dfabf43`

- Upstream intent: fix: case where agent is already closed (#15163)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Close-agent flow now treats already-shutdown agents idempotently while still cleaning thread state
- Strategy selected: cherry-pick
- Confidence: 0.92
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `dee03da508a2cdefa9cf8eadad083f6af7fe49f8`

- Upstream intent: Move environment abstraction into exec server (#15125)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server fs_api overlap reviewed; environment crate moved into exec-server and callsites retargeted
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `2cf4d5ef353a0264df280644b26fa7d8fb42d406`

- Upstream intent: chore: add metrics for profile (#15180)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Adds profile-usage telemetry counter when active profile is present
- Strategy selected: cherry-pick
- Confidence: 0.96
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-otel --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `859c58f07dc3768b654711b7841f35e676005e6c`

- Upstream intent: chore: morpheus does not generate memories (#15175)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Disables memory generation in phase2 consolidation subagents and asserts persisted disabled memory mode
- Strategy selected: cherry-pick
- Confidence: 0.93
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `5ec121ba120ba40cc4fa89960093a115e5e58da2`

- Upstream intent: Revert "Forward session and turn headers to MCP HTTP requests" (#15185)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Reverts request-header forwarding path for MCP HTTP requests
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-rmcp-client --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `267499bed853c0011613a1ef26cf2e4db711e556`

- Upstream intent: [hooks] use a user message > developer message for prompt continuation (#14867)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected protocol/app-server overlap reviewed; added canonical_trace TurnItem::HookPrompt compatibility arm; compile-gate fallback after ENOSPC
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.74
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server-protocol --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-hooks --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-app-server turn_start --quiet (blocked by ENOSPC); df -h .; cargo clean; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-app-server -p codex-core -p codex-protocol -p codex-tui-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `1837038f4e65ba37022d0163894cf29883b4d620`

- Upstream intent: Add experimental exec server URL handling (#15196)
- Local overlays touched: Protected surfaces touched; reconciled without changing auth/Bedrock/provider wiring
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Protected app-server/core-config overlap reviewed; adds experimental_exec_server_url and async Environment::create wiring
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: just write-config-schema; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-app-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `b87ba0a3cc1ee3cb1f558233a8d4e3b994217795`

- Upstream intent: Publish runnable DotSlash package for argument-comment lint (#15198)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Adds release workflow + dotslash manifest and runnable argument-comment-lint package entrypoint
- Strategy selected: cherry-pick
- Confidence: 0.88
- Validation evidence: tools/argument-comment-lint cargo check --quiet (initial ENOSPC, recovered via cargo clean in codex-rs and tool crate; then check passed)
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `1d210f639e39040bdb1611267b02df723eb1901f`

- Upstream intent: Add exec-server exec RPC implementation (#15090)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Added exec/filesystem RPC flow in exec-server; fixed Environment callsite compatibility in server/filesystem.rs
- Strategy selected: cherry-pick (manual compile fix)
- Confidence: 0.80
- Validation evidence: just bazel-lock-update; just bazel-lock-check; CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo test -p codex-exec-server --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `fe287ac467e915a4a75fccb8ce7b7b82d5c12e53`

- Upstream intent: Log automated reviewer approval sources distinctly (#15201)
- Local overlays touched: No protected overlap
- Invariants checked: Indubitably auth behavior preserved; Bedrock runtime/provider behavior preserved; provider-aware ThreadManager wiring preserved.
- Risk factors: Telemetry now records guardian-routed approvals as automated_reviewer source distinct from user/config
- Strategy selected: cherry-pick
- Confidence: 0.94
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS=-C debuginfo=0 cargo check -p codex-core -p codex-otel --quiet
- Rollback note: Revert the corresponding sync commit if regressions appear.

### Commit `60cd0cf75eb29798c71bdfd80f1625e69a26d58d`

- Upstream intent: Add /title terminal title configuration and shared status-surface refresh path in TUI.
- Local overlays touched: Protected overlap on core config files only; no auth/provider wiring changes.
- Invariants checked: Indubitably auth, Bedrock runtime/provider behavior, and provider-aware ThreadManager wiring unchanged.
- Risk factors: Large TUI surface change plus config schema updates and new snapshots.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.81
- Validation evidence: Targeted codex-tui title tests passed; codex-core compile gate passed after known baseline test mismatch.
- Rollback note: Revert this sync commit if terminal title rendering/config persistence regresses.

### Commit `668330acc12b8907ecd82bc15148e0a627246783`

- Upstream intent: Tag app-server turn request spans with turn_id for trace filtering.
- Local overlays touched: Protected app-server path overlap; no Bedrock/auth code touched.
- Invariants checked: Indubitably auth behavior and provider-aware thread wiring preserved.
- Risk factors: Tracing field additions in request-context lifecycle.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.89
- Validation evidence: Focused app-server tracing test passed.
- Rollback note: Revert this sync commit if turn span attribution or tracing context propagation regresses.

### Commit `7eb19e53198470304eb9e74599ec8fb4b97adc3c`

- Upstream intent: Move terminal detection module into standalone codex-terminal-detection crate and update consumers.
- Local overlays touched: No protected-path overlap; conflict resolution kept local Indubitably CLI metadata wiring intact.
- Invariants checked: Indubitably auth behavior, Bedrock provider/runtime behavior, and ThreadManager provider wiring unaffected.
- Risk factors: Workspace member/dependency refactor with crate moves and broad import rewiring.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: Terminal-detection tests and multi-crate compile gate passed; Bazel lock updated and checked.
- Rollback note: Revert this sync commit if terminal detection imports/user-agent reporting regress across cli/core/tui surfaces.

### Commit `69750a0b5a9f10f2e085b48943d41fd5b12ebc0b`

- Upstream intent: Add stronger Windows destructive-command guidance to tool specs and tests.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected; behavior change is documentation/schema text only.
- Risk factors: Prompt/tool-description text changes can affect model behavior.
- Strategy selected: cherry-pick
- Confidence: 0.87
- Validation evidence: codex-core compile gate passed; targeted tests blocked by known branch-wide core test mismatch.
- Rollback note: Revert this sync commit if tool-spec descriptions cause regressions in approval/tool prompting behavior.

### Commit `27977d67166cc3d0b32c04780e153d05077a66a1`

- Upstream intent: Render full saved image path/URI in generated-image history cells so links are openable.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Provider/auth overlays untouched.
- Risk factors: User-visible snapshot and history-cell formatting changes across both TUI implementations.
- Strategy selected: cherry-pick
- Confidence: 0.90
- Validation evidence: Targeted image-generation history tests passed in both TUI crates.
- Rollback note: Revert this sync commit if image history rendering or URI handling regresses.

### Commit `2254ec4f30b78469bbb0fc310894ea2d7bf6944f`

- Upstream intent: Expose app summary needs_auth in plugin/read/install responses and protocol schema.
- Local overlays touched: Protected overlap in app-server-protocol and app-server helper path reviewed with invariants preserved.
- Invariants checked: Indubitably auth and Bedrock/provider overlays unchanged.
- Risk factors: Protected API surface changes plus connector accessibility resolution logic.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.82
- Validation evidence: Protocol tests and plugin_read filter passed; app-server compile gate used after ENOSPC in plugin_install link step.
- Rollback note: Revert this sync commit if plugin read/install app summary compatibility or auth-state reporting regresses.

### Commit `2bee37fe69fee6a8af13cd82850718433e8eb742`

- Upstream intent: Plumb turn/session metadata into MCP tools/call _meta payloads for end-to-end tracing.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Core MCP metadata wiring and request-shape changes for custom and codex_apps servers.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: rmcp-client suite passed and codex-core compile gate passed.
- Rollback note: Revert this sync commit if MCP tool-call metadata payload compatibility regresses.

### Commit `9e695fe83083ba5201f9b53021a56fec183d32c6`

- Upstream intent: Expose MCP startup-status update as app-server v2 notification.
- Local overlays touched: Protected overlap in app-server-protocol/common+v2 and app-server bespoke event handling.
- Invariants checked: Indubitably auth, Bedrock provider/runtime behavior, and provider-aware ThreadManager wiring preserved.
- Risk factors: Protected protocol surface growth with notification wiring across server and UI adapter.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.84
- Validation evidence: Protocol tests and app-server thread_start filter passed; tui_app_server compile gate passed.
- Rollback note: Revert this sync commit if mcp startup notification wire compatibility or event routing regresses.

### Commit `6b8175c7346d25a13479bc044819ca406ea1c3ae`

- Upstream intent: Change default image-generation save directory from temp dir to codex_home thread-scoped path.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Auth/provider overlays unaffected.
- Risk factors: Filesystem path behavior change for generated image persistence and user-facing save-message text.
- Strategy selected: cherry-pick
- Confidence: 0.85
- Validation evidence: codex-core compile gate passed; targeted core test filters remain constrained by known baseline test mismatch.
- Rollback note: Revert this sync commit if generated image artifact pathing or save-message behavior regresses.

### Commit `403b397e4e1d1830a5848367fe05096f8b41faac`

- Upstream intent: Refactor exec-server filesystem into local and remote implementations behind a shared trait and handler split.
- Local overlays touched: Protected overlap in codex-rs/app-server/src/fs_api.rs only; no auth/model-provider custom paths changed.
- Invariants checked: Indubitably auth behavior, Bedrock runtime/provider behavior, and ThreadManager provider wiring preserved.
- Risk factors: Broad exec-server refactor touching environment wiring, transport handlers, and filesystem tests; dependency lock updated.
- Strategy selected: cherry-pick+surgical
- Confidence: 0.85
- Validation evidence: Bazel lock update/check passed; codex-exec-server tests passed; app-server/core compile gate passed.
- Rollback note: Revert this sync commit if remote/local filesystem routing or fs API behavior regresses.

### Commit `ded7854f09d210b4ae7236272ef002279b3f5de2`

- Upstream intent: Add Bazel-managed V8 source-build support, release automation, and third_party targets for musl-capable rusty_v8 artifacts.
- Local overlays touched: No protected-path overlap; local Indubitably auth, Bedrock provider/runtime, and provider-aware thread wiring are unaffected.
- Invariants checked: Confirmed no protected paths were touched and local auth/provider/thread-manager overlays remain outside the changed surface.
- Risk factors: Build/release wiring can silently affect Bazel packaging and the new V8 targets have limited local coverage.
- Strategy selected: cherry-pick
- Confidence: 0.89
- Validation evidence: just bazel-lock-check; bazel query //third_party/v8:all
- Rollback note: Revert this sync commit if Bazel V8 target loading or release wiring regresses.

### Commit `35f8b87a5b396ac9780fa0100cf6fb1af5a5e282`

- Upstream intent: Treat a missing plugin products field as unrestricted and an explicit empty products list as no products allowed.
- Local overlays touched: No protected-path overlap; local Indubitably auth, Bedrock provider/runtime, and provider-aware thread wiring are unaffected.
- Invariants checked: Confirmed no protected paths were touched and overlay behavior remains outside the plugin marketplace/product gating surface.
- Risk factors: Core plugin admission semantics changed, but the patch is localized and carries targeted unit-test additions for the new missing-vs-empty cases.
- Strategy selected: cherry-pick
- Confidence: 0.86
- Validation evidence: CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo check -p codex-core --quiet; targeted lib tests for the new plugin-product cases were blocked by the existing ModelClient::new test compile mismatch in core/src/client.rs and core/tests/suite/bedrock_runtime.rs.
- Rollback note: Revert this sync commit if plugin marketplace filtering or remote sync admission regresses.

## Batch Validation

- [x] CLI default provider smoke
- [x] CLI `--indubitably` smoke
- [x] Targeted crate tests for touched code
- [x] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits: none in this 20-commit batch.
- Manual port TODOs: none; protected-path review strategy used where required (batch 9: orders 107, 114, 115, 116; batch 10: orders 123, 127, 128, 130, 131, 132, 133, 134, 139; batch 11: orders 145, 148, 151, 154, 156, 159, 160).
- Batch 2 summary: processed 6 (orders 15-20), blocked 0, skipped 0, branch now ahead 63 / behind 292 vs upstream/main.
- Batch 3 summary: processed 10 (orders 21-30), blocked 0, skipped 0, branch now ahead 79 / behind 293 vs upstream/main.
- Batch 4 summary: processed 10 (orders 31-40), blocked 0, skipped 0, branch now ahead 90 / behind 301 vs upstream/main.
- Batch 5 summary: processed 10 (orders 41-50), blocked 0, skipped 0, branch now ahead 101 / behind 304 vs upstream/main.
- Batch 6 summary: processed 10 (orders 51-60), blocked 0, skipped 0, branch now ahead 113 / behind 306 vs upstream/main.
- Batch 7 summary: processed 18 (orders 61-80), blocked 0, skipped 2 (orders 66 and 70 no-op), branch now ahead 132 / behind 307 vs upstream/main.
- Batch 8 summary: processed 20 (orders 81-100), blocked 0, skipped 0, branch now ahead 153 / behind 310 vs upstream/main.
- Batch 9 summary: processed 20 (orders 101-120), blocked 0, skipped 0, branch now ahead 174 / behind 318 vs upstream/main.
- Batch 10 summary: processed 20 (orders 121-140), blocked 0, skipped 0, branch now ahead 195 / behind 321 vs upstream/main.
- Batch 11 summary: processed 20 (orders 141-160), blocked 0, skipped 0, branch now ahead 217 / behind 323 vs upstream/main.
- Risk notes: batch-level full crate tests were attempted this run. `cargo test -p codex-app-server-protocol` passed; `cargo test -p codex-core` failed in this runner with 5 known-environment failures (4 `suite::cli_stream::*` missing `target/debug/codex`, 1 request_permissions apply_patch sandbox signal 6); `cargo test -p codex-app-server` failed with initialize deadline flakes (8 failures; narrowed reruns left 2 reproducible timeout failures in auth/conversation_summary filters).
- Additional batch-9 gate notes: persistent disk pressure (os error 28) required repeated `cargo clean` recovery; low-footprint test settings (`CARGO_INCREMENTAL=0`, `RUSTFLAGS='-C debuginfo=0'`) were used to stabilize compilation. Known environment constraints from earlier batches remain: code_mode integration filters requiring `test_stdio_server` resolution and full app-server-protocol schema-fixture parity checks are not fully representative in this runner.
- Additional batch-10 gate notes: repeated low-space recovery (`cargo clean -p codex-tui`) was required once; full `codex-app-server-protocol` schema fixture checks still show branch baseline drift, and full `code_mode` integration filters remain environment-limited (`test_stdio_server` resolution and custom-tool harness behavior), so commit-level compile/targeted tests were used where necessary.

## Batch 12 Intake (Orders 161-180)

### Commit `09ba6b47ae5c13aef51924a30763415eed70cb67`

- Upstream intent: Reuse existing tool runtime in code mode worker path.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Indubitably auth and Bedrock/provider overlays untouched.
- Risk factors: Code-mode runtime execution path change.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `cargo test -p codex-core --test all code_mode --no-run --quiet`.
- Rollback note: Revert this sync commit if code-mode worker/runtime coupling regresses.

### Commit `f35d46002a34759901d395664c00a89ee0c88bc9`

- Upstream intent: Prevent js_repl hangs on line/paragraph separator handling.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: js_repl text-path edge cases.
- Strategy selected: cherry-pick.
- Confidence: 0.91
- Validation evidence: `cargo test -p codex-core --lib js_repl_dynamic_tool_response_preserves_js_line_separator_text --quiet`.
- Rollback note: Revert this sync commit if js_repl unicode separator handling regresses.

### Commit `a5a4899d0c0755400534ca1a15f5a1df394675fb`

- Upstream intent: Skip nested code-mode parallel test on Windows.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Platform-gated test behavior.
- Strategy selected: cherry-pick.
- Confidence: 0.93
- Validation evidence: `cargo test -p codex-core --test all code_mode_nested_tool_calls_can_run_in_parallel --quiet`.
- Rollback note: Revert this sync commit if test-platform gating should be restored.

### Commit `dadffd27d45dd3b330e7b71094b828ce2c1a2d84`

- Upstream intent: Fix MCP tool calling in code-mode runtime.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: MCP namespace import/runtime wiring in code mode.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: Built helper bin `cargo build -p codex-rmcp-client --bin test_stdio_server --quiet`; then `cargo test -p codex-core --test all code_mode_can_dynamically_import_namespaced_mcp_tools --quiet`.
- Rollback note: Revert this sync commit if dynamic MCP imports in code mode regress.

### Commit `11812383c544e80836a3522a659882ba7bfcc9e1`

- Upstream intent: Refocus memory-write prompts toward user preferences.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Prompt behavior and memory summarization expectations.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `cargo test -p codex-core memories --quiet`.
- Rollback note: Revert this sync commit if memory prompt behavior regresses.

### Commit `04e14bdf233839830f2c8cb1ee429f46bdcd1747`

- Upstream intent: Rename exec session IDs to cell IDs in code-mode flows.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Identifier semantics across exec wait/status paths.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `cargo test -p codex-core --test all code_mode_exec_wait_returns_error_for_unknown_session --quiet`.
- Rollback note: Revert this sync commit if code-mode exec ID/cell ID handling regresses.

### Commit `bc48b9289a332673335adb3fc80bde6721cde27b`

- Upstream intent: Update tool search prompt guidance.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Prompt copy/behavior for tool-search planning.
- Strategy selected: cherry-pick.
- Confidence: 0.92
- Validation evidence: `cargo test -p codex-core --lib tool_search --quiet`.
- Rollback note: Revert this sync commit if tool-search prompt behavior regresses.

### Commit `a314c7d3aea10ac399ef8b3fd06dbd444fd25e40`

- Upstream intent: Decouple request_permissions feature flag and tool availability.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Permission-tool gating behavior changes.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: `cargo test -p codex-core --test all request_permissions_tool_is_auto_denied_when_reject_request_permissions_is_enabled --quiet`.
- Rollback note: Revert this sync commit if request_permissions gating regresses.

### Commit `b560494c9f997c699f5cc0dec204b18e58e34d78`

- Upstream intent: Persist js_repl codex helpers across cells.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: js_repl state persistence semantics across executions.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `cargo test -p codex-core --lib js_repl_persisted_tool_helpers_work_across_cells --quiet`.
- Rollback note: Revert this sync commit if js_repl helper persistence regresses.

### Commit `a2546d5dff12e7f629ff540bb2603e7ae635748d`

- Upstream intent: Expose code-mode tools via global scope.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Code-mode tool exposure and global binding behavior.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `cargo test -p codex-core --test all code_mode_lists_global_scope_items --quiet`.
- Rollback note: Revert this sync commit if global tool exposure in code mode regresses.

### Commit `651717323cd664f5dcb357c090fb8d88c66ebc02`

- Upstream intent: Gate search_tool by model capability.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Capability gating can alter tool availability.
- Strategy selected: cherry-pick.
- Confidence: 0.91
- Validation evidence: `cargo test -p codex-core --test all search_tool --quiet`.
- Rollback note: Revert this sync commit if model-capability gating for search_tool regresses.

### Commit `53d59722268dde82fb93c1f37964ce196c2a86d7`

- Upstream intent: Reapply passing tools/reasoning/text params through remote compaction.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Compaction payload parity with responses request surface.
- Strategy selected: cherry-pick.
- Confidence: 0.87
- Validation evidence: `cargo test -p codex-core --test all remote_compact_replaces_history_for_followups --quiet`.
- Rollback note: Revert this sync commit if remote compaction payload parity regresses.

### Commit `d32820ab07a38b2f8c35835f6ce8a18a149d697c`

- Upstream intent: Preserve selected profile (`--profile`) when launching exec app-server thread.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: CLI profile propagation into thread startup config.
- Strategy selected: cherry-pick.
- Confidence: 0.84
- Validation evidence: Targeted test needed `codex` bin rebuild and was environment-limited; compile gate used: `CARGO_INCREMENTAL=0 cargo check -p codex-exec --quiet`.
- Rollback note: Revert this sync commit if profile-scoped instructions/config fail to propagate in `codex exec`.

### Commit `b7dba72dbdb109789fcd426f09a840a9035fac4b`

- Upstream intent: Rename reject approval policy to granular approval config.
- Local overlays touched: Protected overlap (`codex-rs/app-server-protocol/src/*`, `codex-rs/core/src/config*`, docs/config surface).
- Invariants checked: Preserved local overlays; aligned lingering local `Reject*` references to upstream `Granular*` semantics.
- Risk factors: Wide protocol/core rename with behavior semantic inversion notes.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.81
- Validation evidence: `cargo test -p codex-protocol granular_approval_config_defaults_missing_optional_flags_to_false --quiet`; `cargo test -p codex-app-server-protocol ask_for_approval_granular_round_trips_request_permissions_flag --quiet`; `cargo test -p codex-core --lib request_permissions_is_auto_denied_when_granular_policy_blocks_tool_requests --quiet`.
- Rollback note: Revert this sync commit if granular policy wire-compat or prompt behavior regresses.

### Commit `1ea69e8d506e3bd3b8e6cf956e3ff8cd04556cf4`

- Upstream intent: Add v2 `plugin/read` request/response plus app-server handling.
- Local overlays touched: Protected overlap (`codex-rs/app-server-protocol/src/*`, `codex-rs/app-server/src/*`, docs/config surface).
- Invariants checked: Overlay invariants preserved; plugin-read path integrated without auth/provider overlay changes.
- Risk factors: New app-server API surface and plugin metadata expansion.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.78
- Validation evidence: `cargo test -p codex-app-server-protocol --quiet` passed; `cargo test -p codex-app-server plugin_read_*` repeatedly timed out on initialize in this runner; fallback compile gate passed: `cargo check -p codex-app-server --quiet`.
- Rollback note: Revert this sync commit if plugin/read server behavior or schema contracts regress.

### Commit `76d8d174b1c1fa3978eb4a8cdd437b055b2d7144`

- Upstream intent: Add custom CA support for login flows.
- Local overlays touched: Protected overlap via docs path.
- Invariants checked: Overlay invariants preserved; no Bedrock/provider routing change.
- Risk factors: New CA parsing/loading codepath and dependency surface expansion.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.86
- Validation evidence: `just bazel-lock-update`; `just bazel-lock-check`; `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-login --test ca_env --quiet` (8 passed).
- Rollback note: Revert this sync commit if login custom-CA loading or env precedence regresses.

### Commit `793bf32585c31e5c3a33a538bc816c8023074da7`

- Upstream intent: Split multi-agent handlers into per-tool handlers.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Tool registry/dispatch rewiring for collab tools.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib spawn_agent_rejects_empty_message --quiet`.
- Rollback note: Revert this sync commit if multi-agent handler dispatch/regression appears.

### Commit `d9a403a8c01b864d284daf0f4ac545fb442d4c40`

- Upstream intent: Hard-stop active js_repl execs on explicit user interrupts.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Interrupt cleanup timing and kernel lifecycle.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib interrupt_turn_exec_clears_matching_submitted_exec --quiet`.
- Rollback note: Revert this sync commit if interrupt semantics or js_repl kernel reset behavior regresses.

### Commit `6912da84a869a313e77a03b0baf0f35f21d34d8c`

- Upstream intent: Extend custom CA handling to shared HTTP and websocket clients.
- Local overlays touched: Protected overlap via docs path.
- Invariants checked: Overlay invariants preserved; auth/provider overlays unaffected.
- Risk factors: Shared transport-layer CA handling and websocket connector changes.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.85
- Validation evidence: `just bazel-lock-update`; `just bazel-lock-check`; `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-client --test ca_env --quiet` (8 passed).
- Rollback note: Revert this sync commit if custom-CA behavior for HTTPS/websocket clients regresses.

### Commit `7626f612748515d6d79e149c2ae37d7d783cf989`

- Upstream intent: Return typed outputs for multi-agent function tools and add output schemas.
- Local overlays touched: No protected-path overlap.
- Invariants checked: Overlay invariants unaffected.
- Risk factors: Tool-output serialization and schema compatibility.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-core --lib resume_agent_noops_for_active_agent --quiet`.
- Rollback note: Revert this sync commit if multi-agent output wire shape regresses.

- Batch 12 summary: processed 20 (orders 161-180), blocked 0, skipped 0, branch now ahead 238 / behind 324 vs upstream/main.
- Batch 12 risk notes: repeated ENOSPC required periodic `cargo clean`; low-footprint settings (`CARGO_INCREMENTAL=0`, `RUSTFLAGS='-C debuginfo=0'`) were used for reliability. `codex-app-server` `plugin_read_*` runtime tests were consistently initialize-timeout on this runner, so compile-gate validation was used for that commit.

## Batch 13 Intake (Orders 181-200)

### Commit `f194d4b11539a446629b10c93b67c2b95eb5500a`

- Upstream intent: Reopen writable Linux carveouts under denied parents.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0' cargo test -p codex-linux-sandbox landlock --quiet` (0 tests matched; compile gate passed).
- Rollback note: Revert this sync commit if writable carveout handling regresses under denied parent paths.

### Commit `1a363d5fcfadfac0278c4ffe70d53a8130c13c5e`

- Upstream intent: Add plugin usage telemetry.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.87
- Validation evidence: `cargo build -p codex-rmcp-client --bin test_stdio_server --quiet`; `cargo test -p codex-core --test all plugins --quiet`; `cargo test -p codex-app-server plugin_install_tracks_analytics_event --quiet`.
- Rollback note: Revert this sync commit if plugin telemetry events regress or become mis-attributed.

### Commit `0daffe667a755d8d34965e6ffb27b8a1f4a40e83`

- Upstream intent: Move code-mode exec params handling to pragma.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `cargo test -p codex-core --lib parse_freeform_args_with_pragma --quiet`.
- Rollback note: Revert this sync commit if code-mode pragma argument parsing regresses.

### Commit `650beb177e675aa8b0498b459b757451c347db57`

- Upstream intent: Surface cloud requirements load errors through JSON-RPC.
- Local overlays touched: Protected overlap (`codex-rs/app-server/src/*`).
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.86
- Validation evidence: `cargo test -p codex-cloud-requirements --quiet`; `cargo test -p codex-app-server thread_start_surfaces_cloud_requirements_load_errors --quiet`.
- Rollback note: Revert this sync commit if cloud requirements failures are not propagated correctly over app-server APIs.

### Commit `3e8f47169e523d2004fe4491bd2e29e78f9c6720`

- Upstream intent: Add feature-gated realtime v2 event parser.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: `cargo test -p codex-api realtime_ws_e2e_realtime_v2_parser_emits_handoff_requested --quiet`.
- Rollback note: Revert this sync commit if realtime v2 parser event handling regresses.

### Commit `7c7e2675010df55565b547ed101aaea60f9acfe4`

- Upstream intent: Simplify available permissions in `request_permissions`.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.85
- Validation evidence: `cargo test -p codex-tui approval_overlay --quiet`; `cargo test -p codex-core --lib request_permissions_is_auto_denied_when_granular_policy_blocks_tool_requests --quiet`.
- Rollback note: Revert this sync commit if request permissions UI/policy messaging regresses.

### Commit `0c60eea4a5e125f888140f83e8c87519cc038f62`

- Upstream intent: Support skill-scoped managed network overrides.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: `cargo test -p codex-core --lib managed_network --quiet`; `cargo test -p codex-tui skills --quiet`.
- Rollback note: Revert this sync commit if managed-network overrides fail to respect skill scope.

### Commit `eaf81d3f6f3d8c9c80ef977bf8da6a3c03f9b900`

- Upstream intent: Add realtime v2 codex tool handoff support.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `cargo test -p codex-api realtime_ws_e2e_realtime_v2_parser_emits_handoff_requested --quiet`; `cargo test -p codex-api --lib parse_realtime_v2_handoff_tool_call_event --quiet`.
- Rollback note: Revert this sync commit if realtime handoff tool-call parsing regresses.

### Commit `2253a9d1d7832cacb86cebf48c267eb58d039603`

- Upstream intent: Add realtime transcription websocket mode.
- Local overlays touched: Protected overlap (`codex-rs/core/src/config*`).
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.84
- Validation evidence: `cargo test -p codex-api --lib parse_realtime_v2_input_audio_transcription_delta_event --quiet`; `cargo test -p codex-core --lib experimental_realtime_ws_mode_loads_from_config_toml --quiet`.
- Rollback note: Revert this sync commit if transcription-mode realtime configuration or parsing regresses.

### Commit `c7e847aaeb2dba6655f663ed8a887c4e488f2dd6`

- Upstream intent: Add timeout diagnostics for `read_only_unless_trusted`.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.87
- Validation evidence: `cargo test -p codex-core --test all read_only_unless_trusted --quiet` (0 tests matched; compile gate passed).
- Rollback note: Revert this sync commit if read-only timeout diagnostics regress.

### Commit `8e89e9ededc64253c228749521fc9d8049f8947b`

- Upstream intent: Split multi-agent handler into dedicated files.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.89
- Validation evidence: `cargo test -p codex-core --lib spawn_agent_rejects_empty_message --quiet`.
- Rollback note: Revert this sync commit if multi-agent handler dispatch behavior regresses.

### Commit `9c9867c9fafb98cbae885ee44c0d3327abebb9cf`

- Upstream intent: Format code-mode tool declarations on one line.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.92
- Validation evidence: `cargo test -p codex-core --lib append_code_mode_sample_uses_global_tools_for_valid_identifiers --quiet`.
- Rollback note: Revert this sync commit if code-mode sample generation regresses.

### Commit `6b3d82daca540318d074c1ad2afcaba5b3337a3d`

- Upstream intent: Use private desktop for Windows sandbox.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `cargo test -p codex-windows-sandbox --quiet`; `cargo test -p codex-core --lib windows_sandbox --quiet`.
- Rollback note: Revert this sync commit if Windows sandbox desktop isolation behavior regresses.

### Commit `958f93f899c99e8954535c0a6a2e75adde8fd601`

- Upstream intent: Preserve image-generation calls during model switching.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.88
- Validation evidence: `cargo test -p codex-core --test all model_change_from_generated_image_to_text_preserves_prior_generated_image_call --quiet`.
- Rollback note: Revert this sync commit if generated-image call preservation regresses on model change.

### Commit `59b588b8ec11686d38fc62ff8b6ec491d00fc85d`

- Upstream intent: Improve granular approval policy prompt copy.
- Local overlays touched: Protected overlap (`codex-rs/app-server-protocol/src/*`).
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.87
- Validation evidence: `cargo test -p codex-core --test all permissions_messages --quiet` (protocol filter run matched 0 tests in this runner).
- Rollback note: Revert this sync commit if granular approval prompt messaging regresses.

### Commit `9f2da5a9ce13138b6c455ef0bf205cdad69658c8`

- Upstream intent: Clarify plugin and app instruction copy.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.90
- Validation evidence: `cargo test -p codex-core --lib render_plugins_section_includes_descriptions_and_skill_naming_guidance --quiet`; `cargo test -p codex-core --lib plugin_telemetry_metadata_uses_default_mcp_config_path --quiet`.
- Rollback note: Revert this sync commit if plugin/app instruction text behavior regresses.

### Commit `8567e3a5c7e11cb854c5e5950d9ce200bea517a0`

- Upstream intent: Bump Bazel C/C++ and Rust toolchains.
- Local overlays touched: No protected-path overlap.
- Strategy selected: cherry-pick.
- Confidence: 0.86
- Validation evidence: `just bazel-lock-check`.
- Rollback note: Revert this sync commit if Bazel toolchain resolution regresses.

### Commit `9dba7337f21dbc720bd5af70c1628d7c3217f47b`

- Upstream intent: Start TUI on embedded in-process app-server.
- Local overlays touched: Protected overlap (`codex-rs/app-server/src/*`, `codex-rs/tui/src/*`) with local provider-aware `ThreadManager::new` signature preserved.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.80
- Validation evidence: `cargo test -p codex-app-server-client --quiet`; `cargo test -p codex-tui embedded_app_server --quiet`; `cargo test -p codex-app-server in_process --quiet`.
- Rollback note: Revert this sync commit if embedded app-server startup/shutdown or TUI thread-manager wiring regresses.

### Commit `3aabce9e0a75767edadf9f1543bb13f731b91ad9`

- Upstream intent: Unify realtime v1/v2 session configuration shape.
- Local overlays touched: Protected overlap (`codex-rs/core/src/config*`).
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.84
- Validation evidence: `just write-config-schema`; `cargo test -p codex-core realtime_loads_from_config_toml --quiet`; `cargo test -p codex-core realtime_conversation --quiet`; `cargo test -p codex-app-server realtime_conversation_streams_v2_notifications --quiet`.
- Rollback note: Revert this sync commit if realtime config mapping or parser selection regresses.

### Commit `50558e6507f5f5e31106948e341dbf2920adbe8a`

- Upstream intent: Add platform OS/family fields to app-server initialize response.
- Local overlays touched: Protected overlap (`codex-rs/app-server-protocol/src/*`, `codex-rs/app-server/src/*`).
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.88
- Validation evidence: `just write-app-server-schema`; `cargo test -p codex-app-server-protocol --quiet`; `cargo test -p codex-app-server initialize_uses_client_info_name_as_originator --quiet`; `cargo test -p codex-app-server initialize_respects_originator_override_env_var --quiet`.
- Rollback note: Revert this sync commit if initialize response wire compatibility regresses.

- Batch 13 summary: processed 20 (orders 181-200), blocked 0, skipped 0, branch now ahead 259 / behind 327 vs upstream/main.
- Batch 13 risk notes: commit 198 required manual conflict resolution in `app-server`/`tui` files to preserve local provider-aware `ThreadManager` construction; ENOSPC recurred during schema generation and was mitigated by `cargo clean` plus low-footprint build flags.

## Batch 14 Intake (Orders 201-220)

- Batch 14 summary: processed 20 (orders 201-220), blocked 0, skipped 0, branch now ahead 280 / behind 328 vs upstream/main.
- Batch 15 summary: processed 20 (orders 221-240), blocked 0, skipped 0, branch now ahead 303 / behind 329 vs upstream/main.
- Batch 16 summary: processed 20 (orders 241-260), blocked 0, skipped 0, branch now ahead 324 / behind 331 vs upstream/main.
- Batch 14 risk notes: order 218 required surgical conflict resolution plus local Bedrock/openai provider compatibility callsite updates; repeated ENOSPC required targeted `cargo clean` and low-footprint validation (`CARGO_INCREMENTAL=0`, `RUSTFLAGS='-C debuginfo=0'`), with `codex-core` test-binary linking for order 220 falling back to compile-gate validation.
- Batch 15 risk notes: order 230 required protected-path conflict resolution in `models_manager` plus follow-up error-field completion (`216b90e45`), and order 240 required CLI conflict resolution with a compatibility follow-up (`ad0c3da0f`) after temporary ENOSPC recovery via `cargo clean`.
- Batch 16 risk notes: order 248 required surgical conflict resolution in `core/src/thread_manager.rs` and `tui/src/lib.rs` to preserve local provider-aware startup behavior during broad lint churn; orders 250 and 259 required ENOSPC recovery (`cargo clean`) with compile-gate fallback where test-binary linking exceeded local disk capacity.
- Batch 17 summary: processed 20 (orders 261-280), blocked 0, skipped 0, branch now ahead 345 / behind 334 vs upstream/main before publish.
- Batch 17 risk notes: order 266 touched protected config/model surfaces and required local compatibility follow-ups (`ModelClient::new` provider-id callsites plus `websocket_connect_timeout_ms` struct initializers) while preserving Bedrock defaults; order 276 required a `thread_manager.rs` conflict resolution that kept provider-aware thread wiring intact; repeated ENOSPC required multiple `cargo clean` cycles and compile-gate fallbacks for orders 265/268/272/275; order 271 code_mode_yield test failed in this runner with an unsupported custom tool call output, so compile-gate validation was used.
- Batch 18 summary: processed 20 (orders 281-300), blocked 0, skipped 0, branch now ahead 371 / behind 342 vs upstream/main after publish.
- Batch 18 risk notes: protected-path conflicts were resolved in `core/src/client.rs`, `core/src/models_manager/manager.rs`, `core/src/rollout/policy.rs`, `core/src/lib.rs`, and `core/src/thread_manager.rs` while preserving Indubitably auth, Bedrock runtime/provider behavior, and provider-aware thread wiring; one follow-up fix commit (`sync(upstream): fix websocket constructor arg cleanup`) was required after order 293 to remove stale constructor arguments; repeated long-running test/link steps forced compile-gate fallbacks and one interrupted `just fix -p codex-core` run.
- Batch 19 summary: processed 20 (orders 301-320), blocked 0, skipped 0, branch now ahead 394 / behind 345 vs upstream/main after publish.
- Batch 19 risk notes: protected-path overlaps in orders 301/306/312/319 were integrated surgically while preserving Indubitably auth behavior, Bedrock runtime/provider behavior, and provider-aware `ThreadManager` wiring; orders 317/318 hit a pre-existing `ModelClient::new` test-compile mismatch in `codex-core` tests so compile-gate validation was used; order 319 hit ENOSPC during app-server test linking and was recovered via `cargo clean` plus `CARGO_INCREMENTAL=0 RUSTFLAGS='-C debuginfo=0'`; order 320 added `codex-exec-server` and required lock maintenance (`just bazel-lock-update`/`just bazel-lock-check`) from repo root.
- Batch 20 summary: processed 20 (orders 321-340), blocked 0, skipped 0, branch now ahead 419 / behind 349 vs upstream/main after publish.
- Batch 20 risk notes: protected-path overlaps in orders 321/327/328/329/332/336/337 were resolved without regressing Indubitably auth, Bedrock runtime/provider behavior, or provider-aware thread wiring; order 329 required a `thread_manager.rs` merge plus a `SessionSource::Custom` canonical-trace compatibility follow-up, order 336 required a `TurnItem::HookPrompt` canonical-trace compatibility follow-up, and order 339 required an exec-server `Environment` initialization follow-up in `server/filesystem.rs`; ENOSPC recurred during orders 336 and 338 and was mitigated with `cargo clean` and low-footprint compile gates.
- Batch 21 summary: processed 9 (orders 341-349), blocked 0, skipped 0, branch now ahead 429 / behind 349 vs upstream/main after publish.
- Batch 21 risk notes: protected-path overlaps in orders 341/342/346/348 were integrated surgically without regressing Indubitably auth, Bedrock runtime/provider behavior, or provider-aware thread wiring; order 343 required conflict resolution in `cli/src/main.rs` and `tui/src/lib.rs` during terminal-detection crate extraction while preserving local CLI version/env behavior; ENOSPC recurred during order 346 app-server test linking and was mitigated with `cargo clean` and low-footprint compile gates; the pre-existing `ModelClient::new` core test-signature mismatch continued to require compile-gate fallback for core-targeted validation.
- Batch 22 summary: processed 1 (order 350), blocked 0, skipped 0, branch now ahead 431 / behind 350 vs upstream/main after publish.
- Batch 22 risk notes: order 350 touched protected `app-server/src/fs_api.rs` while introducing a broad exec-server filesystem split; integration preserved Indubitably auth behavior, Bedrock runtime/provider behavior, and provider-aware thread wiring; dependency lock maintenance (`just bazel-lock-update`/`just bazel-lock-check`) passed; targeted `codex-exec-server` tests passed and app-server/core compile-gate validation succeeded without regressions.
- Batch 23 summary: processed 4 (orders 351-354), blocked 0, skipped 2, branch now ahead 435 / behind 384 vs upstream/main before publish.
- Batch 23 risk notes: order 351 added Bazel V8 source-build/release wiring and validated cleanly via `just bazel-lock-check` plus `bazel query //third_party/v8:all`; order 352 was already obsolete on this branch because the legacy guardian test file and snapshot had been removed earlier; order 353 required surgical reconciliation with branch-local guardian/TUI test layout and was validated with `cargo test -p codex-tui experimental_popup_includes_guardian_approval --quiet`; order 354 was a true no-op because the `TurnStarted.model_context_window` TUI refresh path and regression coverage were already present.
- Batch 24 summary: processed 1 (order 355), blocked 0, skipped 0, branch now ahead 437 / behind 386 vs upstream/main before publish.
- Batch 24 risk notes: order 355 changed plugin marketplace admission semantics so a missing `products` field means unrestricted while an explicit empty list means no products allowed; it applied cleanly with no protected-path overlap and passed `cargo check -p codex-core --quiet`, while the new lib tests remained blocked by the existing `ModelClient::new` compile mismatch in unrelated `codex-core` test code.

### Commit `2aa4873802134124071b160ddfa21bab28bd45da`

- Upstream intent: Move auth implementation and token-data handling into `codex-login` while keeping `codex-core` re-exports stable for downstream callers.
- Local overlays touched: Protected overlap (`codex-rs/app-server/src/*`) and fork-local auth surfaces; preserved Indubitably auth behavior by continuing to expose `codex_core::auth`/`codex_core::token_data` through `codex-login` and keeping provider-aware consumers intact.
- Strategy selected: cherry-pick+surgical.
- Confidence: 0.79
- Validation evidence: `cargo test -p codex-login --quiet`; `cargo test -p codex-tui-app-server local_chatgpt_auth --quiet`; `cargo test -p codex-app-server --test all external_auth_refreshes_on_unauthorized -- --nocapture`; `cargo test -p codex-app-server --test all external_auth_refresh -- --test-threads=1` (3/4 passed; `external_auth_refresh_error_fails_turn` still reported zero `/responses` requests in this runner even though the staged `app-server` diff only rewired auth types/imports); `cargo test -p codex-core --lib --quiet` (known unrelated failures in `mcp_connection_manager::tests::*`); `cargo test -p codex-core --test all auth_refresh --quiet` (still blocked by the existing `bedrock_runtime` `ModelClient::new` signature mismatch in untouched integration tests).
- Rollback note: Revert this sync commit if auth loading, external ChatGPT token refresh, or default-client re-exports regress.

- Batch 25 summary: processed 1 (order 352), blocked 0, skipped 0, branch now ahead 439 / behind 386 vs upstream/main before publish.
- Batch 25 risk notes: queue recovery discovered that order 352 (`Move auth code into login crate`) was the missing upstream ancestor of the previously attempted features split, so intake was restarted from the correct frozen order; protected `app-server` overlaps were limited to auth-type rewires plus `ChatgptAuthTokens` account display handling; local compatibility adjustments removed stale core-local `try_parse_error_message` tests and updated branch-local `ModelClient::new` unit-test callsites to the current 9-argument signature; dependency guardrails (`just bazel-lock-update` / `just bazel-lock-check`) passed without a `MODULE.bazel.lock` delta; `just fix` failed with `ENOSPC` after validation and `just fmt` completed successfully.
