# REBRAND-001 acceptance

Separate from immutable SPEC-001 (historical 37 PASS / 1 NOT_RUN). Statuses are
PASS / FAIL / BLOCKED / NOT_RUN; no mock establishes actual live compatibility.
Baseline: a3df205abc5d9bd9d665f0a9072032bbd2145978. Local verification and real CI
are still pending for the current work; no previous run is assigned to new code.
Full commands/exits/source/log hashes will be bound to actual tested commits.

| ID | Concrete criterion | Status | Exact command / evidence route | Tested source |
| --- | --- | --- | --- | --- |
| R01 | Repository ID/name, About/topics and old-name redirect preserved | NOT_RUN | GitHub API ID1245373914 and baseline metadata, final read-only API comparison | Pending actual execution |
| R02 | README current product identity | NOT_RUN | scripts/rebrand_gate.py | Pending actual execution |
| R03 | Cargo package identity | NOT_RUN | cargo metadata --locked --offline; scripts/rebrand_gate.py | Pending actual execution |
| R04 | CLI help identity | NOT_RUN | python3 scripts/runtime_smoke.py | Pending actual execution |
| R05 | Dashboard heading identity | NOT_RUN | python3 scripts/runtime_smoke.py | Pending actual execution |
| R06 | Lockfile matches manifest with unchanged registry graph | NOT_RUN | cargo test --locked --offline; scripts/rebrand_gate.py | Pending actual execution |
| R07 | Current LLMOPS environment names work | NOT_RUN | cargo test --locked --offline environment::tests; runtime_smoke.py | Pending actual execution |
| R08 | Deprecated RAGOPS aliases remain compatible | NOT_RUN | cargo test --locked --offline environment::tests; runtime_smoke.py | Pending actual execution |
| R09 | Conflicting aliases fail closed without value disclosure | NOT_RUN | cargo test --locked --offline environment::tests; runtime_smoke.py | Pending actual execution |
| R10 | Legacy default SQLite filename remains compatible | NOT_RUN | runtime_smoke.py: default-path synthetic DB; environment unit test | Pending actual execution |
| R11 | No existing/operator database moved or deleted | NOT_RUN | baseline private inventory; runtime_smoke.py byte preservation; src/db.rs unchanged | Pending actual execution |
| R12 | No obsolete external installer commands in operational files | NOT_RUN | scripts/rebrand_gate.py; classified complete reference audit | Pending actual execution |
| R13 | Generic templates removed from current Git tree | NOT_RUN | scripts/rebrand_gate.py; retirement original blob/removal commit verification | Pending actual execution |
| R14 | Product story/evidence bytes remain intact | NOT_RUN | scripts/rebrand_gate.py protected files and original historical matrix | Pending actual execution |
| R15 | Active validation/security scripts retained and executable | NOT_RUN | scripts/rebrand_gate.py; Git100755 modes | Pending actual execution |
| R16 | Nested toy crate/root task not restored | NOT_RUN | scripts/rebrand_gate.py; source/private-path archive gate | Pending actual execution |
| R17 | Current and historical Markdown destinations resolve | NOT_RUN | scripts/rebrand_gate.py tracked Markdown link check | Pending actual execution |
| R18 | NOTICE/owner declaration and unchanged official MIT correct | NOT_RUN | scripts/license_inventory.py; scripts/rebrand_gate.py; provenance review | Pending actual execution |
| R19 | Complete retained third-party inventory and licenses | NOT_RUN | python3 scripts/license_inventory.py; root lock registry tuples unchanged | Pending actual execution |
| R20 | Published v0.1.0 tag/release/source provenance unchanged | NOT_RUN | scripts/rebrand_gate.py; GitHub final API comparison to exact baseline release-body hash | Pending actual execution |
| R21 | Complete reachable-history/tree Gitleaks no suppression | NOT_RUN | pinned gitleaks git . --log-opts=--all --full-history --redact; gitleaks dir . --redact | Pending actual execution |
| R22 | Fmt/strict Clippy/locked offline tests and build | NOT_RUN | python3 scripts/verify.py --output /tmp/llmops-rebrand-UNIQUE | Pending actual execution |
| R23 | Synthetic offline process/TTY/runtime flow | NOT_RUN | python3 scripts/runtime_smoke.py; zero real provider or external target | Pending actual execution |
| R24 | Exact Git source archive and private-path integrity | NOT_RUN | python3 scripts/source_gate.py --revision TESTED_SHA --output NEW_DIRECTORY; verify_public_archives.py --self-test | Pending actual execution |
| R25 | Fresh target build from exact reviewed source archive | NOT_RUN | verify.py clean-archive-build exact command array | Pending actual execution |
| R26 | Genuine GitHub verified-source CI on exact PR HEAD | NOT_RUN | Real run/jobs + downloaded artifact digests; final exact HEAD bound in PR/handoff | Pending actual execution |
