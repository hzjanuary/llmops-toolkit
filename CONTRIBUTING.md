# Contributing

Read AGENTS.md, README.md, docs/HARNESS.md and docs/FEATURE_INTAKE.md before work.
The owner declaration is recorded in docs/legal/OWNER_DECLARATION.md. Exact
public copyright-holder text and the official MIT grant are still pending; do
not represent this branch as a released OSS package or an activated license.

Use a feature branch and reviewable commits. Keep current product docs, story
packets and docs/TEST_MATRIX.md synchronized. Retain historical evidence, local
operator files and migrations. Never put credentials, real datasets or SQLite
files in commits. Do not auto-load .env, add live CI tests, rewrite history or
publish packages/images/releases as part of a normal contribution.

```bash
cargo fetch --locked
cargo fetch --locked --manifest-path rag-harness/Cargo.toml
python3 scripts/license_inventory.py
python3 scripts/verify.py --output /tmp/ragops-evidence-UNIQUE
```

The output directory must not already exist. `verify.py` requires a committed,
clean tracked tree and runs format, strict Clippy, offline tests/build, the real
TTY/headless smoke, declared-license inventory, archive/tree comparison and a
fresh offline build from the archive. The dedicated ignored fixture-setup test
is invoked explicitly by `runtime_smoke.py`; it is never a hidden skipped PASS.
Linux x86_64 and Python 3.12+ are the tested validation platform. Rust 1.99.0 is
pinned; a lower MSRV has not been established.

```bash
python3 scripts/install_security_tools.py /tmp/ragops-tools-UNIQUE
/tmp/ragops-tools-UNIQUE/gitleaks git . --log-opts='--all --full-history' --redact
/tmp/ragops-tools-UNIQUE/gitleaks dir . --redact
/tmp/ragops-tools-UNIQUE/bin/cargo-audit audit --deny unsound --file Cargo.lock
/tmp/ragops-tools-UNIQUE/bin/cargo-audit audit --deny unsound --file rag-harness/Cargo.lock
```

Tool installation, initial crate fetching and advisory updates use network access;
application validation performs no real provider calls or external scans. Do not
run the nested Hello World crate as a product. It remains for a separate removal
decision; its lockfile is still audited and included in license inventory.

A Draft PR should state the problem, resulting behavior, exact tested source SHA,
commands/artifacts, material limits and remaining owner decisions. A green
`verified-source` check is a source gate, not license approval or release approval.
Ruleset proposals in docs/governance are never applied automatically.
