# Documentation map

RAGOps Harness has an implemented Rust prototype. SPEC-001 is the active secure
local-first refactor; the MIT grant uses the explicitly confirmed copyright identity.

- [Product contracts](product/README.md): proxy/FinOps, terminal monitor,
  evaluation and bounded safety heuristics.
- [Architecture](ARCHITECTURE.md) and [decisions](decisions/README.md): current
  module boundaries and preserved historical choices.
- [SPEC-001 plan](stories/initiatives/SPEC-001/execplan.md),
  [acceptance matrix](validation/SPEC-001-acceptance.md),
  [baseline audit](validation/SPEC-001-baseline.md),
  [Vietnamese handoff](validation/SPEC-001-handoff-vi.md).
- [Threat model](THREAT_MODEL.md), [self-hosting/privacy](SELF_HOSTING.md),
  [security policy](../SECURITY.md), [contributing/verification](../CONTRIBUTING.md),
  [NOTICE](../NOTICE) and [MIT License](../LICENSE).
- [Governance proposals](governance/README.md): disabled main-branch rulesets,
  requiring a separate owner decision.
- [HARNESS](HARNESS.md), [feature intake](FEATURE_INTAKE.md),
  [test matrix](TEST_MATRIX.md), [harness backlog](HARNESS_BACKLOG.md),
  [glossary](GLOSSARY.md), [story packets](stories/README.md) and templates retain
  the human-agent operating model.

Historical Harness v0 text describes the original bootstrap. Current code,
regression tests and source CI exist; there is no release or deployment. Treat
past story evidence as historical snapshots, not current security proof.
