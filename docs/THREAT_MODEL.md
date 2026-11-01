# LLMOps Toolkit threat model and operator privacy

Scope: one trusted local operator, a local proxy caller, an untrusted provider
response, an explicitly authorized scanner target, local files, and source CI.
Protect provider/caller credentials, account spend, dataset confidentiality,
telemetry integrity, operator state and honest claims. The host OS, root user,
local filesystem owner, provider billing system and CI platform are outside the
application's ability to secure.

| Boundary / threat | Enforcement | Residual risk / operator control |
| --- | --- | --- |
| LAN caller causes spend | Loopback by default; non-loopback flag; separate bearer auth; explicit provider enablement | HTTP has no TLS. Use trusted TLS termination, firewall and OS connection/rate limits. Loopback is not user isolation. |
| Stolen keys / provider redirect | Constant-time digest comparison; no caller header forwarding, redirects or env proxies; fixed upstream; redacted error classes | Environment/process memory and local admins can reveal keys. Use separate low-privilege provider keys and rotate outside this tool. No zeroization guarantee. |
| Resource/spend exhaustion | Body/time/concurrency/model/output/request/reservation limits | Process restart resets budgets. Reservations are estimates, unknown-price opt-in has no USD guarantee. Provider account caps are required. TCP connection count is not globally bounded. |
| Provider failure after billing | Pending receipt before egress; no retries; nullable usage/cost; indeterminate states | Crash/disk failure after egress cannot prove whether billing happened. Reconcile with provider invoice; avoid blind client retries. |
| Local state corruption | Additive transactional migration, WAL/FULL, busy timeout, backup tests, reject future schema | Disk failure, malicious local changes and host compromise remain possible. Back up before opening existing data. |
| Sensitive retained metadata | No request/response text in SQLite; bounded exact model IDs, numeric usage, provenance | Models, timing and costs reveal activity. Protect DB/WAL/SHM, reports and backups; no automatic retention deletion or encryption at rest. |
| Evaluation data exfiltration | Offline default; judge consent, reviewed provider/model/pricing and bounds | Live judge sends full context/answer to provider. Hashes may reveal low-entropy input. Question is validated but not sent/scored. Review every dataset and provider policy. |
| Accidental target scan / SSRF | Offline fixtures default; exact authorization URL, literal IP, fixed path, no redirect, no link-local/mapped hosts; HTTPS + flag for non-loopback | Operator authorization is an assertion, not independently verified. Deliberate private-network targets are allowed only with explicit opt-in. Target may bill requests. |
| False security/quality confidence | Named heuristics, abstentions/errors, separate known/unknown price | Refusal text is not exploit proof; lexical overlap is not semantic entailment. LLM judging is fallible and prompt-injectable despite instructions. |
| Malicious file / build supply chain | Bounded regular-file reads, no symlink inputs; lockfiles; pinned Rust/actions/tools; complete-history secrets and archive gates | Local file-swap races are not sandboxed. Registry/build scripts and mutable advisory data remain trusted dependencies. No bit-identical binary-build claim. |
| Source publication / licenses | Redacted history/tree scans, exact archive/tree manifest, declared SPDX inventory, owner-confirmed MIT grant | Detection is incomplete; generic historical wording is not claimed independently authored. CI green does not authorize new license grants or bypass release rules. |

No external LLM request, paid call or external target scan was used as proof.
Offline tests use loopback mocks and synthetic temporary databases. No production
security assessment, penetration-test certification, streaming accounting or
cross-platform support claim is made.

LLMOPS_* and deprecated RAGOPS_* aliases are checked before operator I/O;
conflicting pairs fail closed with no values in diagnostics. No .env auto-load,
credential CLI flag or branding-driven SQLite file/schema migration is added.
