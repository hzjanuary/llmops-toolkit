# Agent Operating Guide

The repository has an implemented Rust prototype. SPEC-001 explicitly authorizes
its secure local-first refactor, regression tests and source CI. Preserve useful
features, historical stories/ADRs, and all operator data. Do not introduce a RAG
engine, general gateway, UI framework or agent orchestration.

Default development proof is entirely offline: zero real LLM/paid calls, external
target scans or model downloads. Never auto-load .env, delete databases, rewrite
history, suppress scanners, publish packages/images, tag, release, deploy or merge
under this task. Use a feature branch; a Draft PR is the stop boundary. Main-branch
rulesets/settings require a separate owner decision. Check ownership before
activating LICENSE.proposed; green source CI is not legal/release approval.

## Source Of Truth

Read in this order:

1. `README.md` for project status.
2. `docs/HARNESS.md` for the human-agent operating model.
3. `docs/FEATURE_INTAKE.md` before turning any prompt into work.
4. The user-provided spec or prompt, when one exists.
5. `docs/product/` for current product contracts.
6. `docs/ARCHITECTURE.md` before proposing implementation shape.
7. `docs/stories/` for story packets and backlog.
8. `docs/TEST_MATRIX.md` for proof status.
9. `docs/decisions/` for why important choices were made.

This harness does not ship with a project-specific `SPEC.md`. When the human
provides a spec for a new project, treat that spec as input material for the
first buildout. Derive product docs, story packets, architecture decisions, and
validation expectations from it. Product docs, stories, tests, and decisions
then become the living contract that agents should update as the system evolves.

## Task Loop

For every task:

1. Classify the request with `docs/FEATURE_INTAKE.md`.
2. Identify whether the input is a new spec, spec slice, change request, new
   initiative, maintenance request, or harness improvement.
3. Locate the affected product docs and story files.
4. Check `docs/TEST_MATRIX.md` for existing proof and gaps.
5. Work only inside the selected lane: tiny, normal, or high-risk.
6. Before finishing, ask:
   - Did product truth change?
   - Did validation expectations change?
   - Did architecture rules change?
   - Did we discover a repeated failure pattern?
   - Did the next agent need a clearer instruction?
7. Update routine harness files directly, or add a proposal to
   `docs/HARNESS_BACKLOG.md` when the change is structural.

## Harness Change Policy

Agents may update directly:

- Story status and evidence.
- `docs/TEST_MATRIX.md` rows.
- Links from story packets to product docs.
- Validation notes and reports.
- Small clarifications tied to the current task.

Agents should ask for human confirmation before:

- Changing architecture direction.
- Removing validation requirements.
- Changing the source-of-truth hierarchy.
- Changing risk classification rules.
- Replacing the feature workflow.

## Done Definition

A task is done only when:

- The requested change is completed or the blocker is documented.
- Relevant docs, stories, and test matrix entries remain current.
- Validation commands were run when they exist.
- Missing harness capabilities were added to `docs/HARNESS_BACKLOG.md`.
- The final response says what changed and what was not attempted.

## Current verification

Read CONTRIBUTING.md for exact pinned tool, offline source and security commands.
`python3 scripts/verify.py --output /tmp/UNIQUE-EVIDENCE-DIRECTORY` requires a
committed clean tracked tree; it records the actual tested SHA and log hashes.
Fetch both lockfile dependencies first. Keep the pre-existing untracked spec
input outside agent commits unless the owner specifically selects it for
publication. Never stage everything indiscriminately.

Use PASS/FAIL/BLOCKED/NOT_RUN in SPEC-001 acceptance evidence. Preserve failures;
record source SHA, exact command, exit code and immutable log/artifact digest.
Local secrets findings must be redacted; genuine credentials require private
owner rotation/review, not printed values or automatic history rewriting.
