# Baseline audit — 2026-11-01

Source: d73849824cdf4eddaeb7898d375818f259450dec, main tracking origin/main.
Local full repository: 8 reachable commits, 122 historical blobs; no shallow
boundary. No additional branches. Initial tracked tree clean; one pre-existing
untracked spec file was preserved and excluded from agent commits. No ignored
operator database, .env, credentials or sessions found in the checkout. No
running ragops-harness process observed. No operator file was opened as a DB.

Read README, AGENTS, HARNESS, FEATURE_INTAKE, Cargo manifests/locks, all src,
product/story/decision docs, architecture and test matrix before editing.

## Read-only results

- Gitleaks 8.30.0 archive SHA-256:
  `79a3ab579b53f71efd634f3aaf7e04a0fa0cf206b7ed434638d1547a2470a66e`,
  checked against upstream release checksums.
- `gitleaks git . --log-opts='--all --full-history' --redact --no-banner
  --report-format json --report-path /tmp/ragops-spec001-audit/history-secrets.json`:
  PASS, 8 commits, no leaks. `gitleaks dir . --redact --no-banner`:
  PASS, no leaks. Additional read-only provider/token/private-key pattern scan
  inspected all 122 blobs, no candidates. Detection is not proof of absence.
- Root lock: 249 packages; nested lock: 211. Registry crates only; no Git
  dependencies. Root has no declared license. Existing commits identify one
  GitHub author, but generic harness material references an upstream author;
  authorship metadata alone cannot establish ownership. MIT grant BLOCKED.
- Baseline test commands and failures are retained in the initiative validation.
  Raw logs remain private under /tmp/ragops-spec001-audit.
- `rag-harness/` is an independent unused Hello World crate, three tracked
  files, no product behavior. Preserve it pending separate removal decision.
- `mock_script.py` is a stale real proxy caller, not an offline mock. Replace
  its behavior with an explicit offline example; retain history.
- `data.json` contains one synthetic Harness Engineering example, no observed
  private data; keep dataset contract.

## Risk register

| Risk | Baseline | Planned mitigation |
| --- | --- | --- |
| Unauthorized spend | Public bind, no caller auth | Loopback and owner bearer + opt-in |
| Secret/privacy disclosure | Raw upstream errors, eval/scan previews | Fixed error classes, metadata-only output |
| Resource exhaustion | Weak input bounds, unbounded bodies | Typed limits, serial/limited calls, deadlines |
| False FinOps confidence | Unknown price = 0; failures absent | Nullable estimates, provenance, pending receipts |
| Data loss | Unversioned schema | Additive transactional migration, backup tests |
| False security claims | SAFE/VULNERABLE | Heuristic outcomes only |
| Accidental egress | eval always live; misleading mock | Offline defaults and explicit independent consent |
| Unreliable terminal | unwrap + unsupervised server | RAII restoration, TTY detection, graceful join |
| OSS rights | No license; inherited harness | MIT proposal, attribution and owner review |

No secrets found requiring rotation in this audit. If a later scan finds genuine
credentials, report only redacted identifiers privately and require owner review;
never publish values or rewrite history automatically.

## Later read-only inventory confirmation

After implementation, presence-only checks found no OPENAI_API_KEY,
RAGOPS_PROXY_TOKEN, RAGOPS_SCAN_TOKEN or RAGOPS_DB_PATH in the task environment,
and no parent-directory .env files. No credential values were printed or loaded.
Remote head/tag inventory contains only main and the newly authorized feature
branch, with no tags; main remains the original baseline. The pre-existing
untracked spec remains outside commits (SHA-256
`c677bf8c46d65cb90baf71f7c5dc5793995c0cab9d9074c0eac08e0b6825656a`).
This follow-up is not retroactively labeled as a preflight command.
