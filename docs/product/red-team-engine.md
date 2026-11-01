# Bounded safety-response heuristics — SPEC-001

`scan` defaults to three offline synthetic fixtures. Optional --fixtures reads
1..5 strict status/body fixtures from a bounded regular JSON file. Files cap at
1 MiB and each body at 64 KiB. Reports retain case index, HTTP status, response
SHA-256 and fixed error class, not raw prompt/response/target content.

Successful chat responses with nonempty choices[0].message.content are classified:
refusal_observed if a refusal marker matches, no_refusal_marker otherwise.
HTTP failures, empty/malformed/non-chat bodies and transport/body/deadline errors
are error_or_inconclusive. A marker can be quoted or negated and absence can be
benign. Neither category confirms safety, exploitability or a security finding.
These are not comprehensive red-team assessments.

Live mode requires --target URL and identical --authorize-target URL asserting
actual owner permission, plus explicit --model. Literal IP only, exact
/v1/chat/completions path, HTTP(S), no userinfo, query, fragment, redirect or DNS.
Link-local, metadata, unspecified, multicast, broadcast and IPv4-mapped IPv6
addresses are rejected. Non-loopback IPs additionally need HTTPS and explicit
--allow-non-loopback-target. No TLS verification bypass exists. Optional
RAGOPS_SCAN_TOKEN is used only for that exact target; provider credentials and
owner-proxy credentials are not silently reused.

At most five serial fixed inherited probes, no retries, 100 ms between cases,
10 seconds for each full response, 64 KiB response cap, 64 output tokens by default
(owner may choose 1..256). --max-requests only narrows 1..5. Redirects are reported
as inconclusive and never followed. Authorization is a user assertion, not proof
that the target owner consented; operator must possess actual permission and
review billing/privacy first. No external scan was executed for SPEC-001.
