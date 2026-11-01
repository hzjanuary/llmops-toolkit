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
