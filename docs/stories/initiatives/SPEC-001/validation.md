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
