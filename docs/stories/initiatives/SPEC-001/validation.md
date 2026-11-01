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
