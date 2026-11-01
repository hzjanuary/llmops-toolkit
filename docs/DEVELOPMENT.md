# Developing LLMOps Toolkit

The root Cargo package and CLI are `llmops-toolkit`. Rust 1.99.0 is pinned in
rust-toolchain.toml; Linux x86_64, Python 3.12+ and a C compiler for bundled SQLite
are the tested development environment. No external workflow repository,
installer, template package, model download or cloud service is required.

```bash
cargo fetch --locked
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
cargo build --locked --offline
python3 scripts/verify.py --output /tmp/llmops-evidence-UNIQUE
```

Initial dependency/tool/advisory fetches use the network. Application tests stay
offline: synthetic temporary SQLite files, bounded loopback HTTP fixtures and
PTY/headless processes. verify.py needs a committed clean tracked tree and a
new evidence directory; it records exact SHA/commands/exits/log hashes, runs the
runtime/legal/source checks and builds an exact Git archive in a fresh target.
It clears current and legacy credential/DB environment names. The ignored Rust
fixture helper is explicitly invoked by runtime_smoke.py, not counted as a
normal executed test. `python3 mock_script.py` is an optional product-owned
wrapper for the same offline evaluation example, verified by runtime smoke.

Read [architecture](ARCHITECTURE.md), [product contracts](product/README.md),
[contributing/security commands](../CONTRIBUTING.md) and
[test matrix](TEST_MATRIX.md). Preserve the no-streaming contract and honest
unknown pricing/usage; heuristics do not establish semantic faithfulness or
security certification. Live workflows require separate owner authorization.

Environment aliases are resolved once before operator I/O. `LLMOPS_DB_PATH`,
`LLMOPS_PROXY_TOKEN` and `LLMOPS_SCAN_TOKEN` prefer the current prefix; deprecated
RAGOPS equivalents work if the current name is absent. Equal pairs are allowed;
conflicting pairs fail closed even with an explicit --db. Missing credentials
never enable forwarding. Non-Unicode credentials fail with fixed redacted
errors. --help/--version only print command metadata. No command auto-loads .env.
Explicit --db otherwise overrides environment, then the intentional compatible
default `ragops_harness.sqlite3` is used. No branding-driven file/schema change.

Make focused changes on a feature branch; inspect every staged path and keep
operator state/untracked inputs out of commits. Update the affected product
contract and add source-bound proof to docs/validation/. Use a numbered product
decision only for meaningful product/architecture choices; current maintenance
is tracked in [MAINTENANCE](MAINTENANCE.md). Historical stories and evidence are
read-only snapshots, not setup instructions or results for a new SHA.

Open a Draft PR after local source/security/license checks. Verify real GitHub
offline-source/security/verified-source checks and downloaded artifact digests
for the exact PR HEAD. Never assign an earlier run to a new commit. A green check
does not authorize merging, changing branch settings, tags, releases, binaries,
packages, containers or deployment. REBRAND-001 ends at the Draft PR.
