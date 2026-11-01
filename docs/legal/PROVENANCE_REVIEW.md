# Ownership and provenance review — publication Phase A

Review source: **782dfab4c7c0883a633ac39fb26fa9c7154c2a06**. Local and remote
feature HEAD match; main remains d73849824cdf4eddaeb7898d375818f259450dec. Full
local history has 18 reachable commits and no shallow boundary. Existing PR #1
is open, Draft and unmerged. GitHub Source CI 41272959002 succeeded for that SHA;
that historical result is not assigned to any upcoming publication changes.

## Authority and correction

[Owner declaration](OWNER_DECLARATION.md) is the authority for the declared
RAGOps-specific ownership. The previous NOTICE associated an old workflow URL
with a presumed external ownership clearance requirement. The URL establishes a
historical development/workflow reference, not RAGOps project copyright,
third-party runtime integration, or ownership by a named GitHub user. That
unsupported implication is withdrawn. Git author metadata is not copyright proof.

This correction does not claim every historical sentence was independently
written. Local Git evidence cannot establish the original authorship of generic
workflow text or prove whether it was copied before its first RAGOps commit.
No actual third-party source copyright notice or externally sourced runtime
component was identified in the RAGOps-owned source inspected here. If specific
copied third-party material is later identified, its rights and notices must be
reviewed on that evidence; this record does not erase such obligations.

## File-level inspection

[file-provenance-782dfab.tsv](file-provenance-782dfab.tsv) classifies **every tracked
file** in the reviewed source and records its first local appearance, SHA-256,
worktree state and evidence basis. [Summary](file-provenance-782dfab.json) records
category totals. Application/test/fixture ownership and current project-specific
documentation/scripts/configuration are accepted on the owner's declaration.
The inventory does not silently turn that declaration into independent forensic
proof of every retained phrase's origin.

Generic templates under docs/templates/ were present in the first local commit,
180a6b4afa6f37cb93525e7d7058b9d13270c8fc. They are retained historical workflow
materials; independent generic-template authorship is not confirmed by local
history. Current project-specific adaptations in AGENTS, HARNESS, intake,
glossary, backlog and scripts documentation are distinguished from their earlier
generic wording. No file is classified third-party merely because it mentions
Harness. Standard MIT proposal text is a license form, not project authorship.

All historical stories, decisions and immutable acceptance evidence are retained.
No historical record is rewritten to make earlier ownership uncertainty disappear.
The current owner's clarification supersedes that earlier uncertainty where the
owner expressly asserts RAGOps rights.

## Third-party obligations

Both lockfiles remain present. The existing dependency inventory covers 249 root
and 210 nested-crate registry dependency entries (all lockfile platforms).
Names, versions, checksums, declared licenses and upstream repositories remain
unchanged. No registry source is vendored in the RAGOps Git source archive.
Crates and their bundled native/data components retain their own licenses;
project MIT cannot replace those obligations. The CI actions and audit tools are
external development dependencies, not RAGOps-owned runtime source. Their pinned
references are retained. No unsupported project ownership attribution is added.

## Development-scaffolding inventory (no Phase-B removal yet)

| Candidate | Actual use | Disposition at Phase-A boundary |
| --- | --- | --- |
| rag-harness/ | Independent Hello World, not linked by root Cargo/runtime; explicitly fetched/audited by CI and license_inventory.py; included in archive and historical evidence | Removal candidate only with coordinated CI/inventory/docs updates; not removed now |
| AGENT_REQUIREMENT.md | Historical US-043.1 task, not runtime; already deleted in worktree before this task | HEAD text inspected through Git; external deletion preserved unstaged, no restoration or adoption in audit commit |
| docs/templates/ and protocol docs | Reusable Codex intake/story/decision workflow | Retain useful workflow and provenance; no blanket removal |
| scripts/README.md installer examples | Historical external URL references; no installer script exists in this checkout and runtime/CI never execute them | Obsolete executable examples are a later focused documentation cleanup candidate; no remote installer is fetched |
| scripts/verify.py, runtime_smoke.py, source_gate.py, license_inventory.py, install_security_tools.py | Active source/security/runtime/archive tooling | Retain; never retire as generic clutter |
| docs/stories/, docs/decisions/, docs/validation/evidence/ | Historical contracts and SHA-bound acceptance records | Preserve; old evidence is not a removal justification |

## Commands and limits

Read-only inspection used git status/rev-parse/branch/remote/log/ls-tree/show,
ls-remote --heads --tags, all tracked blob reads and rg reference searches.
`gitleaks git . --log-opts='--all --full-history' --redact --no-banner` scanned all
18 reachable commits with no leaks; the existing license inventory check passed
for the unchanged dependencies. No operator credential variables or local
credential/database candidates were found; values were never printed. The
pre-existing untracked spec hash matches the previous handoff and stays outside
commits. Operator state is preserved.

## A3 resolution

The owner explicitly confirmed `Copyright (c) 2026 Hoang Nguyen` and authorized
installation of MIT. Root LICENSE now installs that exact public identity and
full grant; the obsolete LICENSE.proposed is removed. The declaration and file
review remain the ownership/provenance record. This resolves the expected A3
identity blocker without inventing generic workflow authorship or removing
third-party terms. Fresh publication gates are still required. Phase D has not
begun and remains gated on actual publication/public archive verification.
