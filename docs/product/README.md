# LLMOps Toolkit product contracts

The implementation retains the secure SPEC-001 contracts. Read
[proxy/FinOps](finops-proxy.md), [terminal monitor](dashboard.md),
[RAG evaluation](rag-evaluation-engine.md), [safety heuristics](red-team-engine.md)
and the [acceptance evidence](../validation/SPEC-001-acceptance.md).

Keep this toolkit distinct from a RAG engine, general gateway and agent OS.
Historical story/ADR packets explain previous behavior but do not override
these current contracts. Change product docs, the relevant story packet and
TEST_MATRIX together; record meaningful decisions separately.

REBRAND-001 changes current identity and environment names only; compatible
RAGOPS_* aliases and the original default SQLite filename are retained. See
[development](../DEVELOPMENT.md) and [rebrand acceptance](../validation/REBRAND-001-acceptance.md).
