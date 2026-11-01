# 0010 Secure local-first source refactor

Date: 2026-11-01. Status: accepted by user-supplied SPEC-001 for implementation.

## Context

Local d738498 matched the reported baseline. Prototype features were useful but
public unauthenticated binding, raw errors, unknown-as-zero pricing, implicit
live evaluation, overconfident scanner labels and weak terminal lifecycle were
unsafe. AGENTS still described an unimplemented bootstrap.

## Decision

Keep the existing small Rust/Tokio/Axum/Reqwest/SQLite CLI architecture and modules.
Use Rust TLS, fixed no-redirect/no-proxy upstream, loopback/owner auth and explicit
provider enablement; typed text-only input and bounded resources. Add small policy,
pricing and transport modules instead of platform/services/framework scaffolding.
New additive SQLite receipts precede egress and express nullable observations,
price provenance and indeterminate failures; retain legacy values unchanged. Version 2 adds correlation IDs and preserves
the already-reviewed version-1 receipts; reports retain observed latency and
timestamps. Credential echoes in provider metadata are excluded before storage.
Default eval/scanner to offline heuristics. Retain opt-in judge and scoped
literal-IP scanner with independent privacy/authorization gates. Supervise the
server and guard terminal state. Add focused offline proofs and source CI.

This supersedes runtime details in 0004/0005/0006/0009; prior files and stories stay
as historical evidence. Source hierarchy/risk lanes/validation requirements are
not weakened. MIT remains a proposal pending verified ownership. Rulesets, release,
merge, tags, deployment and package publication require separate owner decisions.

## Tradeoffs

Text-only strict requests narrow prior passthrough compatibility. No automatic
.env loading or no-argument serve; no-argument prints help. No streaming accounting.
Budgets reset on restart and estimates cannot guarantee invoices. Exact configured
prices can age; aliases stay unknown. Legacy estimates are visible in SQLite but
excluded from new priced totals. Scanner DNS destinations are intentionally
unsupported to prevent rebinding. Terminal guarantees exclude uncatchable exits.

## Evidence

See ../validation/SPEC-001-acceptance.md and the SPEC-001 initiative. Baseline
build and audit failures remain recorded. No real inference or external scan was
run. An independent security assessment and OSS rights review remain outstanding.
