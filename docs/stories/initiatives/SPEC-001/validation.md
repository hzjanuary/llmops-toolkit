# Validation

[Acceptance matrix](../../../validation/SPEC-001-acceptance.md) records actual
commands, source SHA and artifacts. Required statuses: PASS, FAIL, BLOCKED,
NOT_RUN. Tests use only ephemeral synthetic databases and loopback mock servers.
Never run the original mock_script.py: despite its name it forwards real work.

Evidence must identify the tested commit, tool versions, command, exit code and
SHA-256 of logs. A later evidence-only commit does not imply its parent was
retested; CI verifies each new head. Historical story evidence remains historical.

Baseline: initial `cargo test --locked --offline --target-dir
/tmp/ragops-spec001-baseline-target` FAIL (empty axum cache). Retrying locked
with registry access FAIL (missing openssl.pc for openssl-sys). These are build
environment failures, not successful regression proof. Gitleaks 8.30.0 full local
history/tree scans PASS; current dependency/security review is still underway.

## Local committed-source result

Source `7b2349a7168a254b46b65a021863b57970380bee`: verify.py PASS (format, strict Clippy, 26 ordinary
tests + explicitly invoked fixture helper, build, full synthetic HTTP/SQLite/PTY
flow, license inventory, archive/tree/mode checks and clean archive build).
Complete-history/tree Gitleaks PASS; both cargo-audit --deny unsound gates PASS.
Immutable logs/manifests are in docs/validation/evidence/7b2349a7168a254b46b65a021863b57970380bee/.

Live provider/judge/target validation remains NOT_RUN by instruction. Project
license/ownership remains BLOCKED. GitHub proof and Draft PR are pending actual
execution; see acceptance matrix for updates.

## Final retention review

A subsequent review restored timestamp/latency presentation and upstream response
correlation metadata. Additive migration 2 preserves any version-1 receipts.
Provider metadata echoing known credentials is excluded before persistence.
Focused regressions passed 28 tests plus one intentionally ignored setup helper;
new committed-source and real runtime evidence will be attached separately.
The earlier SHA-bound evidence remains unchanged.

Latest local implementation proof: `5f3a2d5b57465dafdf1b18c105783b537e30f601`.
`python3 scripts/verify.py --output /tmp/ragops-spec001-retention-evidence` PASS:
28 ordinary tests, explicit helper, real runtime flow including latency/correlation,
strict Clippy/build/license/archive and fresh archive build. Added migration 2
preserves published version-1 receipts. Immutable second evidence directory is
separate from all earlier results. Full history/dependency gates also PASS.

First GitHub Source CI run 41272218134 PASS for source 794f749; its artifact ZIP
and contained log digests were independently downloaded and verified. It predates
the retention fix. Final PR body binds final reviewed HEAD to its actual latest CI.

GitHub Source CI run 41272626554 PASS all jobs for
f253ddda41d6a06e116e67ba288145ae3b646e5d, including the complete implementation.
Both artifact ZIPs were downloaded and checked against API digests; offline log
hashes, 28 tests/helper, source SHA and zero security findings were verified.
See immutable github-run-41272626554.json snapshot. Final documentation index
clarifications preserve the old task as explicitly historical; no runtime code
changed after the recorded implementation proof.
