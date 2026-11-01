# Harness Backlog

Use this file when an agent discovers a missing harness capability but should
not change the operating model immediately.

## Template

```md
## Missing Harness Capability

### Title

Short name.

### Discovered While

Task or story that exposed the gap.

### Current Pain

What was hard, repeated, ambiguous, or unsafe?

### Suggested Improvement

What should be added or changed?

### Risk

Tiny, normal, or high-risk.

### Status

proposed | accepted | implemented | rejected
```

## Items

No backlog items yet.


## SPEC-001 follow-ups and owner decisions

- Verify inherited harness provenance/license and project ownership; decide on
  LICENSE.proposed. No grant has been activated.
- Decide separately whether to remove the inspected unused rag-harness/ Hello
  World crate and stale AGENT_REQUIREMENT.md; preserve Git/story evidence.
- Decide on the three proposed main-branch rulesets; do not apply automatically.
- Review platform support beyond Linux x86_64 and establish a lower MSRV only
  with actual CI proof. No speculative compatibility claims.
- Consider provider invoice reconciliation and durable budget windows if the
  operator needs guarantees beyond bounded per-process reservations.
- Improve independent security review and trusted-workflow enforcement before
  source-only v0.1.0; no scanner suppression or replacement of missing proof.
- Evidence artifacts expire under GitHub retention. Owner release review should
  export and verify exact run/commit/digest records to durable storage.
- Preserve original failures in append-only evidence; never relabel a baseline
  or stale failure as a current PASS. Current product truth and historical proof
  need explicit headings when a secure refactor supersedes earlier behavior.
