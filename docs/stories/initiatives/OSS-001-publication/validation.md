# Validation ledger

Phase-A audit source: 782dfab4c7c0883a633ac39fb26fa9c7154c2a06.
- git status/rev-parse/branch/remote/log/ls-tree/show/ls-remote and all tracked
  blob inspection: complete; see docs/legal/file-provenance-782dfab.tsv/json.
- Gitleaks 8.30.0 full reachable history: PASS, 18 commits, no leaks.
- Existing dependency inventory: PASS for unchanged 249 root + 210 nested entries.
- Existing GitHub Source CI 41272959002: success for the old reviewed SHA only.
- No runtime files or lockfiles changed during this Phase-A review.
- Pre-existing AGENT_REQUIREMENT.md deletion and untracked spec preserved.

A3: BLOCKED pending exact public copyright-holder text. Official LICENSE absent;
no grant activated, no S30 PASS, no merge/tag/release/public archives. Phase D
NOT_RUN because publication and public archive verification have not occurred.

Full Phase-C gates are NOT_RUN for new publication changes. Prior results are
retained, not reassigned to new commits. No live LLM/paid/external scan is run.

Phase-A formatting result: the initial staged git diff --check FAILED because
the TSV writer emitted CRLF line endings. The audit fields/source hashes were
unchanged. A follow-up commit normalizes LF; history is not rewritten. Subsequent
worktree/staged/base-to-HEAD diff checks PASS after that correction. No publication
or runtime verification is inferred from these documentation-only checks.

A3 resolution: owner explicitly confirmed Copyright (c) 2026 Hoang Nguyen.
Official MIT installed, Cargo SPDX set, obsolete proposal removed. Legal check
verifies exact license SHA-256 and declaration; S30 PASS. Earlier blocked notes
above are historical gate observations, not the current legal status. Final
publication checks are still pending and old CI is not assigned to these edits.

Focused cleanup complete: nested crate removed, current CI/audit/inventory scoped
to all retained root dependencies; exact task text, old installer README and full
former inventory archived with hashes. No runtime/test/migration/root lock change.
Public archive verifier adds a different-prefix ZIP/tar and tamper self-test to
source gates. The first rules read returned no rulesets and main unprotected;
no setting was applied. Final local/remote checks and publication remain pending.

Local publication candidate 62540ba5dadf7ea3914c78baebd6532fd960ae7d: verify.py PASS (28 ordinary tests + explicit
helper, runtime flow, installed legal inventory, exact Git archive, public-format
verifier/tamper self-test and clean archive build). Full reachable-history/tree
Gitleaks and root vulnerability/unsoundness audit PASS. Immutable manifest/logs
are under docs/validation/evidence/62540ba5dadf7ea3914c78baebd6532fd960ae7d/. Fresh remote CI is not yet attributed.

Final verifier review adds directory-entry validation (including root prefixes,
extra directories, duplicate directories and unsafe paths) and an explicit
synthetic unsafe-directory rejection regression. It reads archives without
extracting them. This is project verification tooling, not a runtime redesign;
new exact-source checks follow before any merge/publication.

Publication-delta git diff --check FAILED on a blank final Cargo-log line in the
new evidence copy. Original manifest and byte-exact untrimmed redacted log are
preserved (base64 payload with original digest); only the presentation copy is
normalized and both digests are explicit. verify.py now records this presentation
policy at generation. No test/security outcome changed; previous historical
proof remains untouched. Fresh exact-source checks verify this correction.
