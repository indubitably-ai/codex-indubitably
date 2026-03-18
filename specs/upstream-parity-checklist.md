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
| 1 |  | cherry-pick / manual-port / skip | ported / blocked / skipped |  |  |  |  |

## Decision Briefs

### Commit <sha>

- Upstream intent:
- Local overlays touched:
- Invariants checked:
- Risk factors:
- Strategy selected:
- Confidence:
- Validation evidence:
- Rollback note:

## Batch Validation

- [ ] CLI default provider smoke
- [ ] CLI `--indubitably` smoke
- [ ] Targeted crate tests for touched code
- [ ] App-server protocol smoke (if app-server/protocol files changed)

## Follow-ups

- Blocked commits:
- Manual port TODOs:
- Risk notes:
