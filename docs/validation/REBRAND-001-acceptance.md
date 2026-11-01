# REBRAND-001 acceptance

Separate from immutable SPEC-001 (historical 37 PASS / 1 NOT_RUN). Statuses are
PASS / FAIL / BLOCKED / NOT_RUN; no mock establishes actual live compatibility.
Baseline: a3df205abc5d9bd9d665f0a9072032bbd2145978. Actual local suite source:
**0178ea96657b4f2c3765abea6cfd15f0dc9628f3**. Command:
`python3 scripts/verify.py --output /tmp/llmops-rebrand-local-0178ea9` (exit 0).
[Local command/exit/log manifest](rebrand-001/local-0178ea9/evidence.json) records
all ten checks; 32 ordinary tests PASS plus one ignored helper explicitly run and
PASS in smoke. Current and deprecated aliases, conflicts before I/O, missing/
invalid credentials, redaction, default DB bytes and .env behavior are exercised.
All 121 protected files and 106 Markdown destinations verify; 17 exact retired
paths have original blob/hash/source and actual removal commit.

[Security manifest](rebrand-001/security-0178ea9/security-evidence.json): actual
full-history/tree Gitleaks 8.30.0 PASS without suppression; cargo-audit 0.22.2
PASS, zero vulnerabilities/warnings, no ignored advisories. RustSec DB
2295466c4e9df26647fd67d8fa1c705ac2fc7f81, 1395 advisories. 249 registry package
records are unchanged. [Post-rename v0.1.0 verification](rebrand-001/v010-post-rename-verification.json)
confirms original 192-file content/modes at the original tag; GitHub's rename can
change archive prefixes/compression bytes without changing source. Release
metadata/body, About/topics and refs are compared read-only before push/final.

Local statuses below are source-bound proof, not a prior run assigned to new code.
[Genuine GitHub Source CI 41295320982](rebrand-001/github-run-41295320982.json)
PASS for **bec04674eb26a94feabc140c03c59d132bab2c49**: offline-source,
security and verified-source all succeeded. Both uploaded ZIP digests and every
normal/raw log hash, source SHA, exact Git archive, 32 tests + helper, alias/DB
smoke and zero security findings were downloaded and verified. The recorded
26 criteria now reconcile to **26 PASS / 0 FAIL / 0 BLOCKED / 0 NOT_RUN**.
Evidence-only follow-up commits still need their own exact-head remote CI; the
final PR/handoff binds the actual latest run, SHA and verified artifact digests.
No result is assigned to a later commit before genuine execution. Original SPEC-001 matrix,
its source SHA and every historical failure remain byte-identical.

| ID | Concrete criterion | Status | Exact command / evidence route | Tested source |
| --- | --- | --- | --- | --- |
| R01 | Repository ID/name, About/topics and old-name redirect preserved | PASS | GitHub API ID1245373914 and baseline metadata, final read-only API comparison | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R02 | README current product identity | PASS | scripts/rebrand_gate.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R03 | Cargo package identity | PASS | cargo metadata --locked --offline; scripts/rebrand_gate.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R04 | CLI help identity | PASS | python3 scripts/runtime_smoke.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R05 | Dashboard heading identity | PASS | python3 scripts/runtime_smoke.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R06 | Lockfile matches manifest with unchanged registry graph | PASS | cargo test --locked --offline; scripts/rebrand_gate.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R07 | Current LLMOPS environment names work | PASS | cargo test --locked --offline environment::tests; runtime_smoke.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R08 | Deprecated RAGOPS aliases remain compatible | PASS | cargo test --locked --offline environment::tests; runtime_smoke.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R09 | Conflicting aliases fail closed without value disclosure | PASS | cargo test --locked --offline environment::tests; runtime_smoke.py | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R10 | Legacy default SQLite filename remains compatible | PASS | runtime_smoke.py: default-path synthetic DB; environment unit test | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R11 | No existing/operator database moved or deleted | PASS | baseline private inventory; runtime_smoke.py byte preservation; src/db.rs unchanged | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R12 | No obsolete external installer commands in operational files | PASS | scripts/rebrand_gate.py; classified complete reference audit | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R13 | Generic templates removed from current Git tree | PASS | scripts/rebrand_gate.py; retirement original blob/removal commit verification | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R14 | Product story/evidence bytes remain intact | PASS | scripts/rebrand_gate.py protected files and original historical matrix | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R15 | Active validation/security scripts retained and executable | PASS | scripts/rebrand_gate.py; Git100755 modes | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R16 | Nested toy crate/root task not restored | PASS | scripts/rebrand_gate.py; source/private-path archive gate | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R17 | Current and historical Markdown destinations resolve | PASS | scripts/rebrand_gate.py tracked Markdown link check | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R18 | NOTICE/owner declaration and unchanged official MIT correct | PASS | scripts/license_inventory.py; scripts/rebrand_gate.py; provenance review | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R19 | Complete retained third-party inventory and licenses | PASS | python3 scripts/license_inventory.py; root lock registry tuples unchanged | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R20 | Published v0.1.0 tag/release/source provenance unchanged | PASS | scripts/rebrand_gate.py; GitHub final API comparison to exact baseline release-body hash | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R21 | Complete reachable-history/tree Gitleaks no suppression | PASS | Exact Gitleaks command arrays/report hashes in security-0178ea9/security-evidence.json; no suppression | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R22 | Fmt/strict Clippy/locked offline tests and build | PASS | python3 scripts/verify.py --output /tmp/llmops-rebrand-UNIQUE | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R23 | Synthetic offline process/TTY/runtime flow | PASS | python3 scripts/runtime_smoke.py; zero real provider or external target | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R24 | Exact Git source archive and private-path integrity | PASS | python3 scripts/source_gate.py --revision TESTED_SHA --output NEW_DIRECTORY; verify_public_archives.py --self-test | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R25 | Fresh target build from exact reviewed source archive | PASS | verify.py clean-archive-build exact command array | 0178ea96657b4f2c3765abea6cfd15f0dc9628f3 (actual local suite; a3df205 baseline/metadata where applicable) |
| R26 | Genuine GitHub verified-source CI on exact PR HEAD | PASS | Real Source CI41295320982 all three jobs PASS; rebrand-001/github-run-41295320982.json verifies downloaded ZIP/payload digests; later final HEAD CI must be bound in PR/handoff | bec04674eb26a94feabc140c03c59d132bab2c49 (actual recorded CI source) |
