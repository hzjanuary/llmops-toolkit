# LLMOps Toolkit maintenance

These are product follow-ups, not implemented guarantees or approval to perform
live work. Extracted from the former development backlog during REBRAND-001.

- Reconcile estimates, pending/indeterminate receipts and actual provider
  invoices without inventing missing usage or zero cost.
- Evaluate durable budget windows if per-process reservations and reset on
  restart are insufficient. Keep provider hard spend caps mandatory; design any
  persistence/migration separately with preservation tests.
- Verify Windows/macOS and any lower MSRV with actual platform CI before
  claiming support. Current proof is Linux x86_64 and Rust 1.99.0.
- Obtain independent security review and assess trusted-workflow enforcement.
  Do not weaken source CI or suppress scanners to satisfy a check.
- Preserve exact source/run/artifact digests and raw failure evidence beyond
  GitHub artifact retention; export proof to durable operator storage. Old
  results keep their original SHA and never become proof of a new revision.
- Improve evaluation with reviewed datasets and explicit abstentions; compare
  heuristics with human judgments. Any real judge run needs separate privacy
  consent, reviewed model/pricing and bounded request/token/cost authorization.
- Review metadata retention/privacy, backup/restore and crash/disk-failure
  recovery without destructive automated cleanup of operator files.
- Owner decision remains outstanding on the three disabled main ruleset
  proposals in governance/. Do not apply or change settings automatically.

Resolved history: owner-confirmed MIT is installed; the unused nested Hello
World crate was retired; v0.1.0 was published as a source preview. Generic process
templates and obsolete installer/task attachments are retired in REBRAND-001
with blob hashes in legal/RETIREMENT_RECORD.json, recoverable through Git.
