# Proposed main-branch rulesets — not applied

Three independent disabled JSON proposals target refs/heads/main:

1. **Preserve main**: prevent deletion and non-fast-forward updates.
2. **Require PR**: all changes enter via PR, resolve review threads, dismiss stale
   reviews. Proposed approval count is zero for a solo developer; the owner may
   choose an independent reviewer policy. This does not claim external review.
3. **Verified source**: require the exact `verified-source` status from the GitHub
   Actions app (15368), with the branch up to date. That aggregate requires both
   offline-source and security to succeed and refuses skipped/cancelled/failing
   dependencies. Verify the actual emitted check name/app before activation.

The source workflow is pinned and has read-only permissions; PR code runs without
provider credentials and checkout does not persist Git credentials. Owner review
must protect workflow/gate changes; a status check alone cannot prevent a malicious
PR from weakening its own workflow. Evaluate stronger trusted workflow/ruleset
support for the owner's GitHub plan separately. No bypass is proposed.

These are review artifacts only. Do not import, enable or change settings without
a separate explicit owner decision. See [GitHub's ruleset rules](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets).
License approval, branch protection and release approval remain distinct decisions.
