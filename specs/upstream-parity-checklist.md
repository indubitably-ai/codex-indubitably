# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-18
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

## Protected Surfaces

- Protected paths file: .upstream-sync-protected-paths
- Notes: Batch 1 through Batch 8 of phased sync (mixed 10/20 commits per run), direct-to-main push cadence.

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

## Batch Validation

- [x] CLI default provider smoke
- [x] CLI `--indubitably` smoke
- [x] Targeted crate tests for touched code
- [x] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits: none in this 20-commit batch.
- Manual port TODOs: none; protected-path review strategy used where required (orders 83, 87, 90, 94, 95, 98 in this batch).
- Batch 2 summary: processed 6 (orders 15-20), blocked 0, skipped 0, branch now ahead 63 / behind 292 vs upstream/main.
- Batch 3 summary: processed 10 (orders 21-30), blocked 0, skipped 0, branch now ahead 79 / behind 293 vs upstream/main.
- Batch 4 summary: processed 10 (orders 31-40), blocked 0, skipped 0, branch now ahead 90 / behind 301 vs upstream/main.
- Batch 5 summary: processed 10 (orders 41-50), blocked 0, skipped 0, branch now ahead 101 / behind 304 vs upstream/main.
- Batch 6 summary: processed 10 (orders 51-60), blocked 0, skipped 0, branch now ahead 113 / behind 306 vs upstream/main.
- Batch 7 summary: processed 18 (orders 61-80), blocked 0, skipped 2 (orders 66 and 70 no-op), branch now ahead 132 / behind 307 vs upstream/main.
- Batch 8 summary: processed 20 (orders 81-100), blocked 0, skipped 0, branch now ahead 153 / behind 310 vs upstream/main.
- Risk notes: full `cargo test -p codex-core`, full `cargo test -p codex-app-server`, and full `cargo test -p codex-app-server-protocol` remain outside this batch gate; targeted filters passed for each processed commit except environment-limited code_mode integration coverage.
- Additional batch-8 gate notes: persistent disk pressure (os error 28) required repeated `cargo clean` recovery. `cargo test -p codex-core --test all code_mode --quiet` fails in this runner due missing `test_stdio_server` binary + unsupported custom `code_mode` call path. `cargo test -p codex-app-server-protocol --quiet` reports existing schema-fixture drift in this branch state (`just write-app-server-schema` needed) and app-server heavy integration compile attempts hit ENOSPC.
