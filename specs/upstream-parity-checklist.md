# Upstream Parity Checklist

## Run Metadata

- Date: 2026-03-17
- Fork branch: main
- Upstream ref: upstream/main
- Start ahead/behind: ahead 40 / behind 273
- End ahead/behind:

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

## Batch Validation

- [ ] CLI default provider smoke
- [ ] CLI `--indubitably` smoke
- [ ] Targeted crate tests for touched code
- [ ] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits:
- Manual port TODOs:
- Risk notes:
