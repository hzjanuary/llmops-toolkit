# 0011 Product rebrand and environment compatibility

Date: 2026-11-01. Status: owner-authorized REBRAND-001 implementation; review in Draft PR.

The canonical product is LLMOps Toolkit, repository/package/CLI llmops-toolkit.
Preserve repository ID 1245373914 and the published v0.1.0 identity. Runtime
architecture, security policy, root dependency graph and SQLite schema stay small
and unchanged. Rename current help/dashboard/health/user agent only as relevant.

Resolve LLMOPS_DB_PATH/PROXY_TOKEN/SCAN_TOKEN centrally once, before operator I/O.
Support deprecated RAGOPS aliases when the current name is absent; equal pairs
are allowed and any conflicting pair fails closed, including with explicit --db.
Do not expose values or auto-load .env. OPENAI_API_KEY remains the provider name.
An explicit DB path otherwise takes precedence. Keep ragops_harness.sqlite3 as
the default to avoid silently hiding an existing user's telemetry; no move,
rename, schema migration or row rewrite is authorized by branding.

Retire generic development workflow scaffolding and copied installer/task
attachments with original blob/hash/source/removal records and intact Git
history. Keep product decisions 0004 onward, technical stories, immutable source
proof, dependency data, active local validation tools and the MIT grant. Replace
process templates with concise actual development commands and maintenance.
No outside workflow repository is a runtime or build dependency.

A mechanical global rename would corrupt historical evidence and DB compatibility;
leaving active old names would mislead users. Scoped edits and offline alias/
credential/SQLite process tests verify the chosen compatibility boundary. See
REBRAND-001 acceptance and preservation gates. No merge/tag/release in this task.
