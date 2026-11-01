# Execution plan

1. Read-only baseline inventory: refs/history, tracked/untracked/ignored files,
   operator state, source/dependencies/licenses/secrets and architecture.
2. Preserve evidence and classify risk; create feature branch and this packet.
3. Secure proxy/runtime: loopback, explicit provider enablement, independent
   owner bearer credential, model allowlist, typed bounded non-streaming input,
   concurrency/deadline/request/output/cost limits, redacted errors, supervised
   graceful shutdown and terminal guard. Add local mock regressions.
4. Add transactional versioned SQLite tables without changing legacy records;
   durable pre-egress attempts, nullable observed usage/cost, exact versioned
   operator price provenance, honest failed/indeterminate accounting. Test old
   data, concurrent connections, restart and SQLite backup/restore.
5. Bound dataset parsing and produce deterministic offline heuristic JSON.
   Retain a separately opt-in bounded LLM judge with privacy/cost gates, tested
   only with local synthetic mocks. Scanner defaults to fixtures and requires
   exact authorized literal-IP target policy for live operation.
6. Update product/architecture/agent/OSS/operator docs; inspect and propose
   redundant-crate cleanup without deleting historical evidence. Add source CI,
   dependency/license/history-secret/archive gates and ruleset proposals.
7. Run local offline verification and publication privacy gates on exact source
   commits; push feature branch and open Draft PR only after local gates pass.
   Record actual GitHub CI results and immutable SHA-linked evidence. Finish
   with acceptance matrix, remaining release blockers and Vietnamese handoff.

Each phase has a focused commit. Failures stay in evidence; no optimistic PASS.
