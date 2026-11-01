# LLMOps Toolkit architecture

SPEC-001 keeps the root Rust binary, Tokio, Axum, Reqwest and bundled SQLite.
Its public surfaces are CLI/terminal and two HTTP routes. There is no browser
UI, RAG indexing/storage engine, general AI gateway or agent orchestrator.

```text
CLI -> parsed policy -> local Axum proxy -> fixed OpenAI Chat Completions
                          |                   (explicit enablement only)
                          +-> SQLite pending/outcome receipts -> CLI/TUI
CLI eval -> strict dataset -> offline heuristic OR consented, bounded judge
CLI scan -> offline fixtures OR exactly authorized literal-IP chat endpoint
```

| Module | Responsibility |
| --- | --- |
| main | CLI dispatch, signal supervision, TTY guard, graceful shutdown |
| environment | Current/deprecated environment aliases, conflict rejection, compatible DB default |
| config | Parsed bounds, network exposure policy, bounded regular-file inputs |
| proxy | Independent bearer auth, typed text requests, model/budget gates, receipts |
| http_client | Rust TLS, no redirects/proxies, deadlines, bounded body reads |
| pricing | Versioned exact-model rates, source/date/hash, nullable estimates |
| db | Blocking-thread SQLite access, additive migration, attempts/outcomes, backup |
| dashboard | Read-only metadata presentation; no HTTP or provider dependencies |
| eval | Validated question/context/answer data, deterministic heuristic/opt-in judge |
| red_team | Refusal-marker heuristics, fixture path and exact authorized live scope |

The fixed production upstream is deliberately not operator-configurable. Unit
tests inject loopback upstreams under `cfg(test)` only. Scanner destinations use
a different scoped policy and token. Provider credentials are process-local and
never taken from caller headers. No automatic .env discovery or startup calls.

Requests are parsed before forwarding. Unsupported fields are rejected rather
than passed through. Runtime budgets are process-local; there is no distributed
service, pool, cache, retry framework or migration framework. The pricing model
covers configured text-token estimates only, not tools, regional premiums,
provider adjustments, invoice truth or future pricing changes.

SQLite operations run in `spawn_blocking` behind a connection mutex. WAL,
FULL synchronous writes and a two-second busy timeout support concurrent readers
and connections. An IMMEDIATE transaction serializes version inspection and
schema migration. `user_version=1` creates the unchanged legacy schema if absent
plus `request_receipts`; version 2 adds a bounded upstream correlation ID column.
Both migrate in the same transaction on a fresh database; existing version-1
receipts survive the second migration. It never copies, updates or drops legacy
rows. Reports retain timestamps, observed latency and correlation IDs. A higher
version fails closed. A durable pending receipt precedes egress; final writes
record observed metadata or an indeterminate/error outcome. A final-write
failure leaves pending work, returns a fixed warning and never retries.

The snapshot dashboard opens existing files read-only and does not migrate them.
Serve opens/migrates the selected file. Backup uses SQLite's snapshot API and an
atomically reserved new destination, preserving existing files and failed partial
backups. See [SQLite's backup documentation](https://www.sqlite.org/backup.html).

The server task is supervised and drained on SIGINT/SIGTERM. A terminal guard
restores cursor/screen on normal return, error or Rust unwinding. No restoration
is possible after SIGKILL, abort, power loss, or a failed terminal device write.
There is no claim of bounded total TCP connections; use an OS/reverse-proxy limit
if exposing the service. Application body and provider concurrency are bounded.

[ADR 0010](decisions/0010-secure-local-oss-refactor.md) supersedes the unsafe
runtime details of ADRs 0004/0005/0006/0009. Product ADRs 0004 onward and technical stories remain
for provenance. REBRAND-001 retires generic process templates and obsolete
installer/task attachments from the current tree; their Git blobs/hashes remain
in the retirement record. Root runtime never depended on the previously retired
Hello World crate. CI audits the retained root lockfile; the complete former
dependency inventory remains archived. No external installer is fetched.

[ADR 0011](decisions/0011-product-rebrand-and-environment-compatibility.md)
records LLMOps Toolkit identity and environment compatibility. Aliases resolve
once before dispatch/DB/network I/O; conflict errors are fixed and redacted.
The explicit --db option otherwise precedes environment and the unchanged
ragops_harness.sqlite3 default. Branding changes no DB schema version or rows.
