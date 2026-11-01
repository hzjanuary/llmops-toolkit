# LLMOps Toolkit contributor instructions

Read README.md, docs/DEVELOPMENT.md, docs/ARCHITECTURE.md and the affected
docs/product/ contract before editing. Check docs/TEST_MATRIX.md and record new
proof separately from historical SPEC-001/OSS-001 evidence.

- Keep the small Rust/Tokio/Axum/Reqwest/SQLite CLI/TUI architecture. main owns
  dispatch/signals/terminal restoration; environment resolves aliases; config
  owns bounds; proxy owns auth/forwarding/receipts; db owns persistence/backup;
  pricing owns nullable estimates; eval/red_team own bounded heuristic workflows.
- Preserve loopback-first, disabled forwarding, independent owner/provider
  credentials, model allowlists, request/token/time/concurrency/cost limits,
  redacted errors and no retries/redirects. Never add startup provider requests,
  automatic .env loading or implicit live evaluation/scanning.
- Prefer LLMOPS_* variables; keep RAGOPS_* deprecated aliases and fail closed on
  conflicting pairs without printing values. OPENAI_API_KEY is unchanged.
- Preserve ragops_harness.sqlite3 as the default filename. Never rename/move,
  delete or overwrite operator DBs, sidecars, backups, credentials or local
  evidence. Branding does not authorize schema/data migration. Keep untracked
  docs/specs/ inputs outside commits.
- Run locked offline Rust tests, strict Clippy, runtime smoke, license inventory,
  full-history/tree secret scans, dependency audit and exact source/archive gates
  as documented in CONTRIBUTING.md. Use only synthetic secrets, loopback mocks
  and temporary data; zero real/paid inference, external scans or model downloads.
- Keep active project scripts and MIT/dependency obligations. Record exact source
  SHA, commands, exits and log/artifact digests; preserve failures and prior proof.
  Never weaken gates, suppress findings or turn mock results into live claims.
- Use a feature branch, explicit staging, focused commits and a PR. Do not push
  to main, force-push, rewrite history, change repository settings or modify an
  external workflow repository. No release/tag/merge, package/image/binary
  publication or deployment without a separate explicit owner instruction.

REBRAND-001 stops at one Draft PR with exact-head verified source CI. Preserve
the published v0.1.0 tag, release and historical evidence unchanged. Current
maintenance belongs in docs/MAINTENANCE.md; no external installer is required.
