# Security policy

This source preview has no production security certification or support SLA.
Only the reviewed SPEC-001 branch is being assessed; older prototype commits
have unsafe network defaults and must not be exposed.

Report suspected vulnerabilities privately through GitHub's private vulnerability
reporting, **if enabled**, or contact the repository owner through an already
trusted private channel. Do not post keys, prompts, datasets, database exports,
raw responses or exploit details in public issues. No private reporting mailbox
or enabled GitHub setting has been verified or created by this refactor.

Include the source SHA, affected command, minimal synthetic reproduction,
expected/actual result and redacted logs. Disclosure timing is agreed with the
owner; no response deadline is promised. Rotate exposed credentials with the
provider and review account activity. Deleting a file does not revoke a key.
History rewriting requires a separate owner decision.

Run complete-history Gitleaks, current RustSec audit (including unsoundness),
license inventory and source-archive gates before publishing source changes.
Never suppress a scanner result to obtain green CI. New advisories can invalidate
older evidence; report the actual tool/database revision and scan time.

Live safety testing requires permission for each exact target. Refusal-marker
signals are not vulnerability confirmations. Do not run unsolicited external
scans, real inference or paid requests during development or CI.

See [threat model](docs/THREAT_MODEL.md) and [self-hosting](docs/SELF_HOSTING.md)
for trust boundaries, privacy and residual risks.
