# LLMOps Toolkit current source provenance

REBRAND-001 baseline: `a3df205abc5d9bd9d665f0a9072032bbd2145978`.
The repository's stable ID is 1245373914; current canonical name is
hzjanuary/llmops-toolkit. Published v0.1.0 retains its original product name,
annotated tag, source tree and release notes. Git history is not rewritten.

The [owner declaration](OWNER_DECLARATION.md) records the explicit independent
project ownership and confirmed Copyright (c) 2026 Hoang Nguyen. LICENSE remains
byte-identical MIT. Git author/username/URL metadata does not establish rights.
No external ownership attribution is inferred from old development material.

Project-owned runtime is src/: CLI/signals/TUI, environment compatibility,
config/resource policy, fixed-upstream auth proxy, SQLite receipts/backup,
pricing, dashboard, offline evaluation and bounded scanner. REBRAND-001 changes
identity and environment resolution; it preserves DB default, schema, history
and security controls. No vendored runtime source/model/dataset is introduced.

Active project-owned validation tools are scripts/verify.py, runtime_smoke.py,
verify_public_archives.py, source_gate.py, license_inventory.py,
install_security_tools.py and rebrand_gate.py. The root mock_script.py remains
an optional offline evaluation wrapper. CI invokes only local tooling; no
external workflow repository, installer or hidden update mechanism is needed.

Third-party dependencies retain their licenses, registry checksums and upstream
references in [dependency-inventory.json](dependency-inventory.json): 249 root
registry packages, all lockfile platforms. Root rename changes only the package
record and lock digest, not the registry graph. Full former two-manifest data is
retained in [the historical inventory](archive/dependency-inventory-before-crate-retirement.json).
NOTICE retains applicable obligations; source archives do not vendor registry
code. Any future binary/vendored redistribution needs a separate complete notice
review for bundled native/data components; project MIT cannot replace terms.

The obsolete generic process documents/templates, decisions 0001–0003, external
installer attachment and redundant US-043 task copy are retired from the current
tree. Product-relevant backlog items move to [maintenance](../MAINTENANCE.md);
actual commands live in [development](../DEVELOPMENT.md). [RETIREMENT_RECORD.json](RETIREMENT_RECORD.json)
records each original path/Git blob/SHA-256/source/removal commit and recovery
from history. It retains earlier nested-crate retirement evidence. No copied
installer command is retained as an operational instruction.

Product-specific stories, decisions 0004 onward, SPEC-001/OSS-001 initiatives,
SHA-bound validation and failures, original file-provenance tables and former
dependency inventory remain unchanged. Local Git establishes lineage, not proof
of independently authored historical generic wording. Retiring obsolete current
copies does not erase their origin, legitimate third-party rights or evidence.
The original 146-file review and previous uncertainty remain recoverable at the
recorded source, rather than being relabeled as new forensic proof.

The complete reference audit covers every tracked baseline file and unique
path/blob version in all reachable history; matched line hashes/categories are
recorded in REBRAND-001 evidence. Categories distinguish runtime/current docs,
active validation, product history, generic scaffolding and installer attachment.
Only the last two are retired. No outside repository is modified or requested.
