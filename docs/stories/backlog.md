# Story Backlog

This backlog will be populated after a user provides a project spec or selects a
specific initiative.

Do not create every possible story packet up front. Create story packets when
the work is selected or when a product decision needs a durable place to land.

## Candidate Epics

| Epic | Description | Status |
| --- | --- | --- |
| E01 Proxy and FinOps | Local proxy, provider forwarding, usage logging, and cost visibility. | active |
| E02 Security | Red teaming and prompt-injection checks for OpenAI-compatible endpoints. | active |
| E03 Evaluation | RAG answer quality evaluation, scoring, and reporting workflows. | active |
| E04 Dashboard | Local CLI reporting surfaces for FinOps, quality, and security metrics. | active |

## SPEC-001 maintenance initiative

Active: [secure OSS refactor](initiatives/SPEC-001/overview.md), high-risk lane.
Supersedes unsafe runtime details in US-011/021/031/043 without erasing their
historical evidence. Plan, design, current validation and owner blockers live in
that packet and docs/validation/SPEC-001-acceptance.md. Do not treat old Safe /
Vulnerable scanner or public unauthenticated proxy behavior as current contracts.
