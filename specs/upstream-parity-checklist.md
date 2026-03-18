# Upstream Parity Checklist

## Run Metadata

- Date:
- Fork branch:
- Upstream ref:
- Start ahead/behind:
- End ahead/behind:

## Protected Surfaces

- Protected paths file:
- Notes:

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
