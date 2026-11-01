# Contributing to LLMOps Toolkit

Read [AGENTS](AGENTS.md), [development](docs/DEVELOPMENT.md),
[architecture](docs/ARCHITECTURE.md) and the affected product contract. Work on a
feature branch and open a PR with focused commits. Keep product docs, tests and
the test matrix current. Preserve historical evidence, migrations, operator
data and untracked inputs; stage explicit paths only.

```bash
cargo fetch --locked
python3 scripts/license_inventory.py
python3 scripts/verify.py --output /tmp/llmops-evidence-UNIQUE
```

verify.py requires a committed clean tracked tree and a new evidence directory.
It runs pinned format, strict Clippy, locked offline tests/build, synthetic
loopback/SQLite/PTY/headless smoke, license/identity/reference checks, exact Git
archive verification and a fresh offline archive build. Runtime smoke explicitly
runs the ignored fixture helper. It clears LLMOPS_* and deprecated RAGOPS_*
credential/database variables; never provide real credentials to tests.

```bash
python3 scripts/install_security_tools.py /tmp/llmops-tools-UNIQUE
/tmp/llmops-tools-UNIQUE/gitleaks git . --log-opts='--all --full-history' --redact
/tmp/llmops-tools-UNIQUE/gitleaks dir . --redact
/tmp/llmops-tools-UNIQUE/bin/cargo-audit audit --deny unsound --file Cargo.lock
git diff --check
```

Tool installation, registry fetch and advisory updates use network access, not
inference. Never run real/paid LLM requests, external scans or model downloads
in tests/CI. Keep secure proxy defaults and all resource/privacy gates. No .env
auto-loading, destructive DB handling, secret printing or scanner suppression.
The only retained dependency lockfile is root Cargo.lock; do not restore the toy
crate. MIT and every third-party obligation remain separate; see LICENSE,
NOTICE and docs/legal/dependency-inventory.json.

Record PASS/FAIL/BLOCKED/NOT_RUN with exact commands, source SHA, exit codes and
log/artifact digests. Retain failed attempts and byte-exact raw logs where
presentation differs. Verify offline-source, security and verified-source for
the actual PR HEAD, then download/hash their artifacts. Historical SPEC-001
results and v0.1.0 identity must not be rewritten or assigned to new source.

For an explicitly authorized archive review:

```text
python3 scripts/verify_public_archives.py --revision REVIEWED_SHA \
  --zip DOWNLOADED_SOURCE.zip --tar-gz DOWNLOADED_SOURCE.tar.gz \
  --output NEW_VERIFICATION_RECORD.json
```

No external installer or workflow checkout is needed. Ordinary contributions
do not authorize merge, release/tag, deployment, package/binary/image publication
or repository settings changes. REBRAND-001 stops at one Draft PR.
