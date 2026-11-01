# SPEC-001 Secure OSS refactor

Input: user-supplied SPEC-001, 2026-11-01. Baseline d73849824cdf4eddaeb7898d375818f259450dec.
Lane: **high-risk maintenance initiative** (auth, authorization, migrations,
privacy, external systems, public contracts, existing behavior, weak proof,
multi-domain). User explicitly authorizes implementation and source CI, feature
branch push and Draft PR after privacy gates; no release or settings changes.

Retain the root Rust/Tokio/Axum/Reqwest/SQLite CLI prototype. A RAG engine,
general AI gateway and agent orchestration are outside scope. Preserve the
pre-existing untracked `docs/specs/RAGOPS_SPEC001_HEADLESS_OSS_REFACTOR.md`;
it is input context, not an agent-authored publication artifact.

Current contracts: [proxy](../../../product/finops-proxy.md),
[evaluation](../../../product/rag-evaluation-engine.md),
[scanner](../../../product/red-team-engine.md),
[monitor](../../../product/dashboard.md).

Hard limits: zero real LLM calls, zero paid calls, zero external target scans,
no model downloads, operator-data deletion, history rewriting, tags, releases,
package publication, deployment, merge or main push.
