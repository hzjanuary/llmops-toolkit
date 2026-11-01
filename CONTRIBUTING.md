# Contributing

Read AGENTS.md, README.md, docs/HARNESS.md and docs/FEATURE_INTAKE.md before work.
The owner declaration and exact public copyright confirmation are recorded in
docs/legal/OWNER_DECLARATION.md. The project uses the MIT License in LICENSE;
dependency terms remain separate in NOTICE and the legal inventory.

Use a feature branch and reviewable commits. Keep current product docs, story
packets and docs/TEST_MATRIX.md synchronized. Retain historical evidence, local
operator files and migrations. Never put credentials, real datasets or SQLite
files in commits. Do not auto-load .env, add live CI tests, rewrite history or
publish packages/images/releases as part of a normal contribution.

```bash
cargo fetch --locked
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
```

Tool installation, initial crate fetching and advisory updates use network access;
application validation performs no real provider calls or external scans. Do not
restore the retired nested Hello World crate as a product. Its former dependency
metadata and old task text remain in the historical legal/story archives. The
current retained root lockfile is fully audited and inventoried.

A Draft PR should state the problem, resulting behavior, exact tested source SHA,
commands/artifacts, material limits and remaining owner decisions. A green
`verified-source` check is a source gate, not license approval or release approval.
Ruleset proposals in docs/governance are never applied automatically.

For owner-authorized source publication, verify both public formats with:

```text
python3 scripts/verify_public_archives.py --revision REVIEWED_MAIN_SHA \
  --zip DOWNLOADED_SOURCE.zip --tar-gz DOWNLOADED_SOURCE.tar.gz \
  --output NEW_VERIFICATION_RECORD.json
```

The tool reads archives without extraction, checks every path/content/executable
bit against the exact Git commit, rejects operator paths and verifies LICENSE.
Its offline prefix/tamper self-test is part of verify.py. A normal tag/release is
only permitted by an explicit owner instruction after all review/source rules
pass. Ordinary contributions do not acquire publication authorization.
