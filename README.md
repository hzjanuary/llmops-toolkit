# RAGOps Harness

A small, local-first Rust toolkit for non-streaming OpenAI Chat Completions
proxy telemetry, SQLite FinOps, terminal monitoring, reproducible RAG evaluation
and bounded safety-response heuristics. It is a prototype under secure OSS
refactor, not security-certified software. It is not a RAG engine, general AI
gateway or multi-agent operating system.

SPEC-001 retains the existing Rust/Tokio/Axum/Reqwest/SQLite CLI. Default commands
perform no inference. Streaming is rejected; streamed usage accounting is not
implemented. Live functionality requires deliberate authorization and may cost
money or disclose supplied text to a provider.

## Offline quick start

Tested on Linux x86_64 with Rust **1.99.0** (pinned), a C compiler for bundled
SQLite, Python 3.12+, and Cargo registry access for the initial fetch. Dependencies
are not vendored; an empty Cargo cache cannot build offline. No models download.
Windows/macOS behavior has not been verified.

```bash
cargo fetch --locked
cargo build --locked --offline
cargo test --locked --offline
cargo run --locked --offline -- eval --dataset examples/dataset.json
cargo run --locked --offline -- scan
cargo run --locked --offline -- --help
```

`eval` defaults to a deterministic **lexical subset heuristic**, not ground-truth
faithfulness. `scan` defaults to three built-in synthetic fixtures and reports
`refusal_observed`, `no_refusal_marker`, or `error_or_inconclusive`; none confirms
whether a target is secure. Both commands emit content-free JSON to stdout.

## Proxy and terminal monitor

```bash
umask 077
cargo run --locked --offline -- --db ./ragops_harness.sqlite3 serve
# In a second terminal; health never calls a provider:
curl --noproxy '*' http://127.0.0.1:8000/health
cargo run --locked --offline -- --db ./ragops_harness.sqlite3 dashboard
cargo run --locked --offline -- --db ./ragops_harness.sqlite3 dashboard --json
```

The server binds `127.0.0.1:8000`. Forwarding is disabled until `--enable-provider`
is supplied with independent `RAGOPS_PROXY_TOKEN` and `OPENAI_API_KEY` credentials,
at least one exact `--model`, and reviewed pricing (or proxy-only explicit
`--allow-unknown-price`). Tokens must be 32–512 non-space ASCII characters;
choose high-entropy secrets. Keys are read only from the process environment;
`.env` files are not automatically loaded. No-argument invocation prints help.

The proxy accepts `Authorization: Bearer <owner-token>`, never the provider key.
The only production upstream is `https://api.openai.com/v1/chat/completions`;
redirects, environment HTTP proxies, retries and startup calls are disabled.
`--listen` on a non-loopback address requires `--allow-non-loopback`. The server
itself has no TLS; see [self-hosting](docs/SELF_HOSTING.md) before exposing it.

Supported request fields: exact allowed `model`, 1–64 text `messages` (role and
nonempty content), `stream: false`, optional `temperature` in 0–2, and either
`max_tokens` or `max_completion_tokens`. Other fields, tools, multimodal content,
multiple completions and streaming are rejected. Limits: 64 KiB input, 2 MiB
upstream body, four concurrent provider calls, 16 concurrent authorized body
readers, five-second body-read deadline, 30-second provider deadline. Output
cap defaults to 256 tokens (owner maximum 4096). Per-process attempt cap defaults
to 100 (maximum 1000); no retries. These limits do not replace a provider account
spending cap or an OS/network service limit.

TTY serve mode uses an alternate screen and refreshes every two seconds. Non-TTY
and `--no-tui` mode run headless without terminal escapes. SIGINT/SIGTERM initiate
graceful shutdown and restoration; SIGKILL/power loss cannot restore a terminal.

## Truthful FinOps

SQLite stores metadata only: request IDs, exact requested/observed models,
nullable usage and estimated costs, outcome, status, latency, error class and
price provenance. It records a pending attempt before egress, so crashes and
failed final writes remain potentially billable work. Pre-egress logging failure
prevents forwarding. Provider errors are redacted. Do not blindly retry a failed
request: it may already have been billed.

A version-1 JSON pricing catalog supplies exact-model text-token rates, source,
date and SHA-256. [Synthetic pricing](examples/pricing.synthetic.json) is a schema
example, **not real prices**. There are no built-in current-price claims or alias
matching. Unknown price/usage is null, never a verified zero. The dashboard
separates the estimated subtotal from unknown and legacy counts. Existing
`llm_request_logs` rows stay intact; unverified legacy estimates are excluded
from the new subtotal. Additive schema migrations finish at `PRAGMA user_version=2`: version 1 adds
receipts, version 2 adds upstream correlation IDs while preserving version-1 data.
Average latency, timestamps and per-request latency remain visible in reports.

The USD policy is a conservative **per-process estimate reservation**, not a
billing guarantee. Reservations are not refunded, and restart resets process
budgets. `--allow-unknown-price` explicitly waives a USD bound for unknown-model
traffic while retaining request/token limits. Use a provider account hard cap.

```bash
cargo run --locked --offline -- --db ./ragops_harness.sqlite3 backup \
  --destination ./new-backup.sqlite3
```

Backup never replaces an existing destination. Restore by selecting a verified
backup as a new DB path, not overwriting an operator database.

## Explicit live workflows

Live inference and live scans were **not executed** for SPEC-001. Live judge
support is separately gated by `--judge`, `--consent-send-dataset`,
`--reviewed-config`, `--provider openai`, an exact `--model`, reviewed `--pricing`,
`--max-live-requests`, `--max-judge-tokens`, `--max-estimated-usd` and both independent
credentials. Question/context/answer remains the dataset contract. See
[evaluation contract](docs/product/rag-evaluation-engine.md).

Live scans require identical `--target` and `--authorize-target` URLs and an exact
`--model`. The authorization flag asserts actual owner permission. Literal IPs
only, fixed chat path, no credentials/query/fragment, no redirects, no DNS,
no link-local/metadata/mapped destinations. Non-loopback targets also require
HTTPS and `--allow-non-loopback-target`. At most five serial requests, 64 output
tokens by default, 64 KiB response and ten seconds per case. Optional
`RAGOPS_SCAN_TOKEN` is target-specific; provider credentials are never reused.
See [scanner contract](docs/product/red-team-engine.md).

## Evidence and project docs

- [SPEC-001 acceptance matrix](docs/validation/SPEC-001-acceptance.md) and
  [baseline audit](docs/validation/SPEC-001-baseline.md).
- [Architecture](docs/ARCHITECTURE.md), [threat model](docs/THREAT_MODEL.md),
  [privacy/operator guide](docs/SELF_HOSTING.md), [security policy](SECURITY.md).
- [Contributing and verification](CONTRIBUTING.md), [test matrix](docs/TEST_MATRIX.md),
  [Vietnamese handoff](docs/validation/SPEC-001-handoff-vi.md).
- [High-risk plan](docs/stories/initiatives/SPEC-001/execplan.md) and historical
  story/decision packets remain accessible. Historical evidence is not current proof.

## License and release status

Ownership review is pending. [LICENSE.proposed](LICENSE.proposed) proposes MIT
without activating a grant. [NOTICE](NOTICE) identifies inherited harness material
and dependency attribution. Source CI checks dependencies' declared license
metadata; it does not establish project ownership. This repository is **not yet
ready for an OSS release**. No tag, release, binary, package, image, deployment,
merge or ruleset/settings change is authorized by this refactor.
