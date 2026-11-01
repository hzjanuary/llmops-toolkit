# Design

Keep one binary with existing modules. Small new `config`, `pricing`, and
`http_client` modules own parsed policy, exact model estimates and bounded
transport. Reuse the proxy transport in opt-in judge calls. No provider egress
is reachable at startup, health, dashboard or offline evaluation/scanning.

Bearer auth is owner-controlled and separate from provider credentials. Public
binding requires a flag; TLS termination is the operator's responsibility.
Typed text-only non-streaming requests deliberately reject unsupported fields.
All external HTTP disables environment proxies and redirects; only production
OpenAI endpoint is used by proxy/judge. Test injection stays under cfg(test).

Legacy `llm_request_logs` stays intact. New versioned receipts distinguish
pending, success, upstream error and indeterminate response; missing usage and
unknown cost are null, never invented zero. Pending records make crash windows
visible. Pre-egress persistence failure prevents forwarding; final persistence
failure reports potentially billable work and discourages blind retries.

Scanner live policy requires exact target and explicit authorization, literal
IP (no DNS rebinding), fixed chat path, no query/userinfo/redirects, HTTPS for
non-loopback targets, fixed serial five-case maximum and bounded body/time/token
budgets. No target is scanned during implementation.

MIT remains a proposal pending ownership review, especially inherited harness
material. Publishing a source branch is not a license grant or release approval.
