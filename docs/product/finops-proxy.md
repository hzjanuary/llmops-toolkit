# Proxy and FinOps contract — SPEC-001

`serve` defaults to 127.0.0.1:8000 and no provider forwarding. Health is public,
minimal and offline. Billable forwarding requires explicit enablement, separate
owner/provider credentials, exact allowed models and pricing policy. Non-loopback
binding requires an additional flag. No incoming authentication fallback exists.
See README for all supported text-only request fields and fixed bounds.

Production endpoint: OpenAI HTTPS Chat Completions only. Caller headers are never
forwarded. Redirects, automatic retries, environment proxies, startup inference,
streaming, tools, images and arbitrary passthrough fields are disabled.

HTTP errors contain only fixed error code/message and a local request ID. No raw
provider body, request text, serde detail or network error is exposed. Provider
error status is metadata in SQLite; public upstream errors map to 502. Known
operator credential strings are also redacted recursively from successful JSON.
Owner token digest comparisons use constant-time equality. Invalid/missing auth
is 401; unsupported media is 415; oversized input 413; invalid fields 400;
concurrency/request/reservation exhaustion 429; disabled provider/pre-egress log
failure 503. A final telemetry failure is 502 with an explicit potentially-billable
code. Local validation/auth rejections are not provider-attempt receipts.

Schema version 1 adds request_receipts transactionally. Legacy llm_request_logs
and indexes remain unchanged, with all historical values intact. New pending
receipts commit before egress. Outcomes: pending, success, upstream_error,
indeterminate. Usage, response model, upstream status and estimate are nullable;
provider errors, bad bodies, deadlines and missing usage never invent usage or
zero price. A final-log failure can leave pending work. Potentially-billable
remains conservatively true for all attempted egress, even an upstream rejection.

Version-1 pricing configuration supplies source/date/exact-model rates and a
SHA-256 of its original bytes. Known means an operator-configured estimate, not
an independently verified provider invoice. Unknown price or missing/invalid
usage produces null. Aliases do not inherit rates. Legacy estimates lack reviewed
provenance and are excluded from the new subtotal. Unknown count and failed/pending
count are separate; neither can be inferred from a zero subtotal.

SQLite access runs on blocking threads with serialized connections, WAL/FULL and
busy timeout. No request/response text or credentials is stored. Models are bounded
ASCII identifiers. Backup uses a consistent snapshot to a new file; restart,
legacy migration and concurrent connections use synthetic test data. Account
billing, crash/disk-loss reconciliation, encryption, retention and provider hard
spending limits remain operator responsibilities. Process budgets reset at
restart. Unknown-price opt-in explicitly cannot provide a USD guarantee.
