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

## Batch Validation

- [ ] CLI default provider smoke
- [ ] CLI `--indubitably` smoke
- [ ] Targeted crate tests for touched code
- [ ] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits:
- Manual port TODOs:
- Risk notes:
