# SPEC-001 acceptance matrix

Statuses are PASS, FAIL, BLOCKED or NOT_RUN. No live LLM inference, paid request,
external target scan or model download is authorized. A mock PASS never means a
live provider PASS. Project ownership and source CI are separate gates.

Local implementation proof is for **5f3a2d5b57465dafdf1b18c105783b537e30f601**. Run:
`python3 scripts/verify.py --output /tmp/ragops-spec001-retention-evidence` (exit 0).
Its [immutable manifest](evidence/5f3a2d5b57465dafdf1b18c105783b537e30f601/evidence.json) contains the exact command
arrays, exits and log SHA-256 for every offline check. Normal suite: **28 passed,
0 failed, 1 ignored fixture helper**; runtime smoke explicitly runs that helper
(1 passed) in a new synthetic database and verifies the real PTY flow.

[Security manifest](evidence/5f3a2d5b57465dafdf1b18c105783b537e30f601/security-evidence.json) binds full-history/tree
scans and both dependency audits to the same source SHA and report digests.
Gitleaks 8.30.0, cargo-audit 0.22.2; RustSec database
`2295466c4e9df26647fd67d8fa1c705ac2fc7f81` (1395 advisories).
[Baseline failures](evidence/baseline-d738498/provenance.json) stay FAIL for the
baseline; later passes do not rewrite them. New evidence-only commits do not
retrospectively change the tested SHA. GitHub CI verifies each subsequent HEAD.

[GitHub run snapshot](evidence/github-run-41272218134.json) records the first
remote green source SHA, job IDs, artifact IDs/digests and expiry. Its downloaded
offline ZIP matched the recorded SHA-256 and every internal log digest. That run
predates migration 2; newer local proof above includes migration/metadata
regressions. Final reviewed HEAD and its latest GitHub CI run are recorded in the
[Draft PR description](https://github.com/hzjanuary/ragops-harness/pull/1), since an
evidence-only commit cannot contain its own commit hash. Earlier local evidence
for 7b2349a remains immutable in its separate directory.

Evidence references below use **L** = the local immutable manifest/logs, **S** =
the security manifest/reports, **G** = actual GitHub run/artifact records once
available. The exact suite command shown per row ran all the named tests; no
unexecuted individual filter command is claimed.

| ID | Concrete acceptance test | Status | Exact command / evidence | Source SHA |
| --- | --- | --- | --- | --- |
| S01 | Local HEAD/refs, operator state, untracked inputs and history audited | PASS | git status --short --branch; git rev-parse HEAD; git branch -avv; git log -8 --oneline; git status --ignored --short; Gitleaks commands in baseline report | d73849824cdf4eddaeb7898d375818f259450dec |
| S02 | Default bind loopback and provider egress disabled | PASS | cargo test --locked --offline; config::tests::default_loopback_and_egress_disabled (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S03 | Non-loopback bind needs deliberate opt-in | PASS | cargo test --locked --offline; config::tests::public_binding_requires_explicit_opt_in (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S04 | Unauthorized/malformed bearer access and provider-key-as-caller denied | PASS | cargo test --locked --offline; proxy::tests::auth_health_validation_and_credential_isolation (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S05 | Owner/provider credentials isolated and distinct; exact reviewed model policy required | PASS | cargo test --locked --offline; proxy::tests::enablement_requires_separate_credentials_and_reviewed_models (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S06 | Typed text-only JSON/content-type/body-size limits enforced | PASS | cargo test --locked --offline; auth_health_validation_and_credential_isolation and typed_request_rejects_unsupported_or_unbounded_fields (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S07 | Streaming rejected, output cap and model allowlist enforced | PASS | cargo test --locked --offline; proxy::tests::typed_request_rejects_unsupported_or_unbounded_fields (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S08 | Concurrency/request/cost reservations block excess egress | PASS | cargo test --locked --offline; proxy::tests::request_concurrency_cost_and_persistence_limits_prevent_egress (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S09 | Upstream size/deadline/shape failures recorded as indeterminate | PASS | cargo test --locked --offline; oversized_and_malformed_success_responses_are_indeterminate and upstream_deadline_and_transport_failures_record_uncertain_work (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S10 | Provider error/known credential strings not disclosed | PASS | cargo test --locked --offline; failures_and_missing_usage_are_truthful_and_redacted and credential_echo_is_redacted_recursively (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S11 | Startup/health/offline commands make no provider calls | PASS | cargo test --locked --offline; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S12 | Exact price matching, cached-token estimate and version/source/hash provenance | PASS | cargo test --locked --offline; pricing::tests::unknown_price_is_none_and_matching_is_exact (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S13 | Unknown price with observed usage remains null, separately counted | PASS | cargo test --locked --offline; proxy::tests::unknown_response_price_with_observed_usage_stays_null (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S14 | Failed upstream/missing usage do not invent cost or tokens | PASS | cargo test --locked --offline; proxy::tests::failures_and_missing_usage_are_truthful_and_redacted (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S15 | Failed pre-egress log blocks forwarding; final log failure preserves pending | PASS | cargo test --locked --offline; request_concurrency_cost_and_persistence_limits_prevent_egress and final_log_failure_preserves_pending_and_warns_against_retries (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S16 | Legacy migration preserves values through restart and backup/restore | PASS | cargo test --locked --offline; db::tests::migration_preserves_legacy_restart_and_backup (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S17 | Future schema version rejected unchanged | PASS | cargo test --locked --offline; db::tests::future_version_is_rejected_without_changing_data (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S18 | Two concurrent SQLite connections/readers retain 32 attempts | PASS | cargo test --locked --offline; db::tests::concurrent_connections_and_readers_preserve_receipts (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S19 | Real headless and TTY monitor, SIGINT/SIGTERM cleanup, bind-error behavior | PASS | python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S20 | Authenticated HTTP proxy -> local mock -> SQLite -> real TUI | PASS | python3 scripts/runtime_smoke.py (explicit ignored setup test) (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S21 | Offline scan outcomes deterministic and honestly named | PASS | cargo test --locked --offline; red_team::tests::fixture_outcomes_are_heuristics_not_findings; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S22 | Scanner rejects missing auth/scope, DNS, metadata and unsafe destinations | PASS | cargo test --locked --offline; red_team::tests::authorization_and_destination_policy_fail_closed; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S23 | Authorized local mock scan stays within request scope and never follows redirect | PASS | cargo test --locked --offline; red_team::tests::authorized_synthetic_mock_does_not_follow_redirects_or_expand_requests (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S24 | Eval rejects malformed/empty/unknown/oversized data and unsafe file types | PASS | cargo test --locked --offline; eval::tests::malformed_oversized_empty_and_unknown_fields_are_rejected and input_paths_are_regular_and_bounded (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S25 | Offline eval JSON/hash/aggregation reproducible and labeled heuristic | PASS | cargo test --locked --offline; eval::tests::offline_contract_and_aggregation_are_deterministic; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S26 | Judge consent/config gate and synthetic adapter work; unknown accounting halts | PASS | cargo test --locked --offline; eval::tests::llm_judge_adapter_uses_only_synthetic_local_mock and judge_stops_after_unknown_accounting_without_fabricating_scores; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S27 | Current full-history and working-tree secret scans without suppression | PASS | gitleaks git . --log-opts='--all --full-history' --redact; gitleaks dir . --redact (S) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S28 | Both retained lockfiles pass vulnerability/unsoundness gates | PASS | cargo-audit audit --deny unsound --file Cargo.lock; cargo-audit audit --deny unsound --file rag-harness/Cargo.lock (S) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S29 | Declared dependency-license inventory matches both locks | PASS | python3 scripts/license_inventory.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S30 | Project copyright/inherited-source rights permit MIT grant | BLOCKED | Owner review of NOTICE and LICENSE.proposed required; no grant activated | 5f3a2d5b57465dafdf1b18c105783b537e30f601 (source review; no live/ownership proof) |
| S31 | Pinned Rust format, strict Clippy, locked offline tests/build | PASS | python3 scripts/verify.py --output /tmp/ragops-spec001-retention-evidence (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S32 | Source archive matches every Git blob/mode, excludes operator state, reproduces bytewise | PASS | python3 scripts/source_gate.py --revision HEAD --output NEW_DIRECTORY (called by verify.py) (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S33 | Clean build from reviewed source archive with empty target | PASS | cargo build --locked --offline --target-dir NEW_DIRECTORY (called by verify.py) (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S34 | GitHub verified-source passed the exact recorded source; final HEAD run is linked from Draft PR | PASS | Source CI run 41272218134: offline-source/security/verified-source all success; downloaded artifact ZIP and inner log digests verified (G) | 794f7495f0a6de9af72d121fff0705d4abced45a (prior head; see final PR body for latest HEAD) |
| S35 | Live provider/judge compatibility and external target security assessment | NOT_RUN | Prohibited by SPEC-001 hard limits; no execution command run | 5f3a2d5b57465dafdf1b18c105783b537e30f601 (source review; no live/ownership proof) |
| S36 | Immutable provenance, SHA/digests, accurate docs and unapplied ruleset proposals | PASS | verify.py evidence.json; exact GitHub run/artifact records; source review (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S37 | Version-1 receipts migrate to version 2; timestamps, latency and correlation IDs retained | PASS | cargo test --locked --offline; db::tests::published_v1_receipts_survive_additive_v2_correlation_migration; python3 scripts/runtime_smoke.py (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |
| S38 | Provider metadata cannot echo known owner/provider credentials into SQLite | PASS | cargo test --locked --offline; proxy::tests::credential_echo_in_response_metadata_never_reaches_sqlite (L) | 5f3a2d5b57465dafdf1b18c105783b537e30f601 |

S19/S20/S31 cover the explicitly invoked fixture setup test separately from the
normal suite's one ignored setup helper. Archive reproducibility is bytewise
source tar reproducibility, not a bit-identical compiled-binary claim. S23 proves
one narrowed local request and redirect refusal; remote authorization remains an
operator assertion and no external target is scanned. Source CI cannot establish
ownership or independently certify safety.
