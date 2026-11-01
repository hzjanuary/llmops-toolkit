# Local operation and self-hosting

The default deployment is one Linux user's local machine. Read SECURITY.md and
the threat model first. No cloud backend, container image or deployment is
required or published. Tested platform: Linux x86_64, Rust 1.99.0, Python 3.12+.

1. Fetch locked dependencies, build offline, and run the offline checks in README.
2. Use a private working directory and `umask 077`. Keep existing .env, session,
   database, WAL/SHM and backup files outside source commits. No command auto-loads
   .env. `LLMOPS_DB_PATH` or global `--db` selects the file.
3. Back up existing SQLite data with SQLite's backup API (Python sqlite3 backup
   or the `backup` command) to a **new** file before starting the refactored server.
   Do not copy only the main file while WAL writes are active.
4. Start locked local service with `serve`. Health and dashboard are offline.
   `dashboard` is read-only and needs an existing telemetry DB; `serve` applies
   additive schema versions 1 and 2 (latest). Future versions are rejected, not reset.

## Enabling billable forwarding (separate deliberate operator action)

This documentation is not a development test command. No live request was made
for SPEC-001. Obtain independently generated, high-entropy owner and provider
credentials. Supply `LLMOPS_PROXY_TOKEN` and `OPENAI_API_KEY` through a protected
environment, not CLI arguments or shared shell-history commands. Both must be
32–512 non-space ASCII characters and distinct. Restrict the provider account's
spending and permissions outside the application.

Prepare a pricing file using the schema in `examples/pricing.synthetic.json`.
Replace every synthetic model/rate with reviewed exact-model text-token rates,
source and date; do not use that fixture as live pricing. Review available model
compatibility with non-streaming text Chat Completions. Use a small output cap,
attempt cap and reservation budget, for example this **server-only** shape:

```text
llmops-toolkit --db /private/operator/telemetry.sqlite3 serve \
  --enable-provider --model REVIEWED_EXACT_MODEL --pricing /private/operator/prices.json \
  --max-output-tokens 128 --max-requests 10 --max-estimated-usd 0.25 --no-tui
```

It starts no inference. The caller must send its owner bearer credential and a
valid bounded text request. Provider keys never belong in caller headers. Unknown
pricing requires proxy-only `--allow-unknown-price`; that explicitly waives any
USD bound for unknown traffic. Reservations include conservative input-byte and
framing estimates and capped output, are not refunded, and reset at restart.
They are not an invoice guarantee. No automatic retries are performed.

## Network exposure

Loopback serves local callers from any process with the token; it is not an OS
user boundary. To serve another machine, deliberately choose `--listen` and
`--allow-non-loopback`, place a trusted TLS terminator in front, firewall direct
access, and enforce connection/rate limits. The Axum listener has no built-in
TLS, per-user billing partition, global connection cap or token rotation API.
Never expose raw HTTP bearer credentials on an untrusted network.

Environment HTTP proxies and redirects are disabled. Proxy and judge production
egress is fixed to OpenAI HTTPS; no configurable arbitrary upstream is available.
This intentionally favors credential isolation over gateway generality.

## Persistence, recovery and privacy

New receipts distinguish pending, success, upstream_error and indeterminate.
Success with unavailable usage has null tokens/cost. Non-success responses,
transport/body/parse/deadline errors and final-write failure can be billable.
Final-write failure returns `potentially_billable_telemetry_incomplete`, leaving
a pending receipt if its initial write survived. Pre-egress failure returns
`telemetry_unavailable_no_forward` and makes no provider call. Never blindly
retry. Review pending entries and reconcile separately with provider statements.

Legacy rows are never rewritten. They lack reviewed provenance and are counted
separately, excluded from the new subtotal. Inspect them directly if needed;
the latest table shows new receipts only. Metadata can still disclose project
activity; protect the DB, sidecars, pricing file, reports and backups. Content is
not stored by the proxy, but caller/provider/terminal capture can retain it.
No encryption at rest, retention deletion or credential zeroization is claimed.

`backup --destination NEW_FILE` uses a consistent SQLite snapshot and refuses an
existing file. Restore by checking `PRAGMA integrity_check`, selecting the backup
under a new `--db` path, and comparing dashboard JSON. Preserve the original.
A failed backup can leave a partial new file; it is not automatically deleted.
No automatic schema rollback or destructive restore is implemented.

TTY mode refreshes every two seconds. `--no-tui` and non-TTY output avoid screen
control. SIGINT/SIGTERM drain requests and restore the cursor/screen. SIGKILL,
power loss, abort or broken terminal writes can prevent restoration (`reset` is
an operator recovery action). Do not log real dataset or scan response text.

Live judge/scanner flags are documented in their product contracts. Explicit
privacy consent and exact target authorization are required independently; the
offline examples need neither credentials nor inference.

## Source preview identity

For v0.1.0, verify the annotated tag's peeled commit against the published review
record and download only GitHub-generated source ZIP/tar.gz. The project license
is MIT, Copyright (c) 2026 Hoang Nguyen; retain LICENSE and NOTICE. A full Git clone
is needed for history/provenance audits; source archives contain the selected
snapshot, not the full Git history. scripts/verify_public_archives.py compares
both archive formats against an exact local Git commit without extracting them.
No external workflow installer or hidden update mechanism is required.

Current LLMOPS_* environment names retain deprecated RAGOPS_* aliases: use the
current value when present, legacy fallback when absent, accept equal pairs and
reject conflicting pairs without logging values. See README environment table.
The default ragops_harness.sqlite3 filename remains unchanged intentionally; no
branding-driven database migration, move or rename occurs.
