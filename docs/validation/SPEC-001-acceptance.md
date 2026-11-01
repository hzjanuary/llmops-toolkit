# SPEC-001 acceptance matrix — source publication candidate

Exactly 38 criteria are retained. Statuses: PASS / FAIL / BLOCKED / NOT_RUN.
The [original matrix](SPEC-001-acceptance-baseline-782dfab.md) and all prior
SHA-bound evidence remain unchanged. Its baseline was 36 PASS / 1 BLOCKED /
1 NOT_RUN. Owner-confirmed MIT installation resolves S30; S35 remains NOT_RUN.

Current local source: **7c798c51b80dfc8d03055a2a8cd28da135cbbfa7**. Command:
`python3 scripts/verify.py --output /tmp/ragops-oss001-publication-local-3` (exit 0).
[Local manifest](evidence/7c798c51b80dfc8d03055a2a8cd28da135cbbfa7/evidence.json) records every exact command array,
source SHA, exit and log hash. Results: **28 ordinary tests passed, one ignored
fixture helper explicitly run by runtime smoke and passed**; strict Clippy,
format/build, real authenticated mock -> SQLite -> PTY/headless flow, installed
MIT/retained dependency inventory, exact Git archive, different-prefix ZIP/tar
verification with tamper rejection, and fresh archive build all PASS.

[Security manifest](evidence/62540ba5dadf7ea3914c78baebd6532fd960ae7d/security-evidence.json): Gitleaks 8.30.0 full
reachable history/tree PASS for the recorded 62540ba checkpoint; newer source is
rescanned before push and in fresh CI; cargo-audit 0.22.2 --deny unsound PASS for Cargo.lock,
the only retained lockfile. No findings or suppression. RustSec database
2295466c4e9df26647fd67d8fa1c705ac2fc7f81. Root runtime/tests/migrations/dependency
lock are identical to the previously reviewed implementation; only project
license, cleanup, source-verification tooling and documentation changed.

**L** means the linked exact local manifest/logs; **S** means the security record.
Suite commands ran all named cases; no unexecuted individual filter is claimed.
The current legal result is PASS; remote verification of new publication source
is still NOT_RUN until actually observed (36 PASS / 0 BLOCKED / 2 NOT_RUN at this
local checkpoint). Once that fresh CI passes, the same matrix becomes 37 PASS /
0 BLOCKED / 1 NOT_RUN. No mock proof establishes live compatibility/certification.

| ID | Concrete acceptance test | Status | Exact command / evidence | Source SHA |
| --- | --- | --- | --- | --- |
| S01 | Local HEAD/refs, operator state, untracked inputs and history audited | PASS | Original baseline preflight plus publication file audit (docs/legal/PROVENANCE_REVIEW.md) and exact archive manifest (L) | d73849824cdf4eddaeb7898d375818f259450dec |
| S02 | Default bind loopback and provider egress disabled | PASS | cargo test --locked --offline; config::tests::default_loopback_and_egress_disabled (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S03 | Non-loopback bind needs deliberate opt-in | PASS | cargo test --locked --offline; config::tests::public_binding_requires_explicit_opt_in (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S04 | Unauthorized/malformed bearer access and provider-key-as-caller denied | PASS | cargo test --locked --offline; proxy::tests::auth_health_validation_and_credential_isolation (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S05 | Owner/provider credentials isolated and distinct; exact reviewed model policy required | PASS | cargo test --locked --offline; proxy::tests::enablement_requires_separate_credentials_and_reviewed_models (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S06 | Typed text-only JSON/content-type/body-size limits enforced | PASS | cargo test --locked --offline; auth_health_validation_and_credential_isolation and typed_request_rejects_unsupported_or_unbounded_fields (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S07 | Streaming rejected, output cap and model allowlist enforced | PASS | cargo test --locked --offline; proxy::tests::typed_request_rejects_unsupported_or_unbounded_fields (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S08 | Concurrency/request/cost reservations block excess egress | PASS | cargo test --locked --offline; proxy::tests::request_concurrency_cost_and_persistence_limits_prevent_egress (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S09 | Upstream size/deadline/shape failures recorded as indeterminate | PASS | cargo test --locked --offline; oversized_and_malformed_success_responses_are_indeterminate and upstream_deadline_and_transport_failures_record_uncertain_work (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S10 | Provider error/known credential strings not disclosed | PASS | cargo test --locked --offline; failures_and_missing_usage_are_truthful_and_redacted and credential_echo_is_redacted_recursively (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S11 | Startup/health/offline commands make no provider calls | PASS | cargo test --locked --offline; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S12 | Exact price matching, cached-token estimate and version/source/hash provenance | PASS | cargo test --locked --offline; pricing::tests::unknown_price_is_none_and_matching_is_exact (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S13 | Unknown price with observed usage remains null, separately counted | PASS | cargo test --locked --offline; proxy::tests::unknown_response_price_with_observed_usage_stays_null (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S14 | Failed upstream/missing usage do not invent cost or tokens | PASS | cargo test --locked --offline; proxy::tests::failures_and_missing_usage_are_truthful_and_redacted (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S15 | Failed pre-egress log blocks forwarding; final log failure preserves pending | PASS | cargo test --locked --offline; request_concurrency_cost_and_persistence_limits_prevent_egress and final_log_failure_preserves_pending_and_warns_against_retries (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S16 | Legacy migration preserves values through restart and backup/restore | PASS | cargo test --locked --offline; db::tests::migration_preserves_legacy_restart_and_backup (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S17 | Future schema version rejected unchanged | PASS | cargo test --locked --offline; db::tests::future_version_is_rejected_without_changing_data (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S18 | Two concurrent SQLite connections/readers retain 32 attempts | PASS | cargo test --locked --offline; db::tests::concurrent_connections_and_readers_preserve_receipts (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S19 | Real headless and TTY monitor, SIGINT/SIGTERM cleanup, bind-error behavior | PASS | python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S20 | Authenticated HTTP proxy -> local mock -> SQLite -> real TUI | PASS | python3 scripts/runtime_smoke.py (explicit ignored setup test) (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S21 | Offline scan outcomes deterministic and honestly named | PASS | cargo test --locked --offline; red_team::tests::fixture_outcomes_are_heuristics_not_findings; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S22 | Scanner rejects missing auth/scope, DNS, metadata and unsafe destinations | PASS | cargo test --locked --offline; red_team::tests::authorization_and_destination_policy_fail_closed; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S23 | Authorized local mock scan stays within request scope and never follows redirect | PASS | cargo test --locked --offline; red_team::tests::authorized_synthetic_mock_does_not_follow_redirects_or_expand_requests (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S24 | Eval rejects malformed/empty/unknown/oversized data and unsafe file types | PASS | cargo test --locked --offline; eval::tests::malformed_oversized_empty_and_unknown_fields_are_rejected and input_paths_are_regular_and_bounded (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S25 | Offline eval JSON/hash/aggregation reproducible and labeled heuristic | PASS | cargo test --locked --offline; eval::tests::offline_contract_and_aggregation_are_deterministic; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S26 | Judge consent/config gate and synthetic adapter work; unknown accounting halts | PASS | cargo test --locked --offline; eval::tests::llm_judge_adapter_uses_only_synthetic_local_mock and judge_stops_after_unknown_accounting_without_fabricating_scores; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S27 | Current full-history and working-tree secret scans without suppression | PASS | gitleaks git . --log-opts='--all --full-history' --redact; gitleaks dir . --redact (S) | 62540ba5dadf7ea3914c78baebd6532fd960ae7d (recorded security checkpoint; fresh CI pending) |
| S28 | All retained lockfiles pass vulnerability/unsoundness gates | PASS | /tmp/ragops-spec001-tools/bin/cargo-audit audit --deny unsound --file Cargo.lock --json (S); retired nested graph preserved in historical inventory | 62540ba5dadf7ea3914c78baebd6532fd960ae7d (recorded security checkpoint; fresh CI pending) |
| S29 | Installed MIT and declared inventory match every retained dependency | PASS | python3 scripts/license_inventory.py (L); exact confirmed grant hash, Cargo MIT and root graph; former full inventory archived | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S30 | Project copyright/provenance and installed MIT grant | PASS | python3 scripts/license_inventory.py (L); Copyright (c) 2026 Hoang Nguyen explicitly confirmed in OWNER_DECLARATION.md; LICENSE installed; proposal removed; recorded provenance decision | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S31 | Pinned Rust format, strict Clippy, locked offline tests/build | PASS | python3 scripts/verify.py --output /tmp/ragops-oss001-publication-local-3 (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S32 | Source archive matches every Git blob/mode, excludes operator state, reproduces bytewise | PASS | python3 scripts/source_gate.py --revision 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 --output /tmp/ragops-oss001-publication-local-3/archive (L); ZIP/tar verifier self-test also passes | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S33 | Clean build from reviewed source archive with empty target | PASS | cargo build --locked --offline --target-dir <fresh archive target> (L: exact temporary command in evidence.json) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S34 | GitHub verified-source passed the exact recorded source; final HEAD run is linked from Draft PR | NOT_RUN | Await fresh GitHub Source CI for publication changes; previous run 41272959002 applies only to 782dfab | Publication HEAD pending remote verification |
| S35 | Live provider/judge compatibility and external target security assessment | NOT_RUN | Prohibited by SPEC-001 hard limits; no execution command run | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S36 | Immutable provenance, SHA/digests, accurate docs and unapplied ruleset proposals | PASS | verify.py evidence.json; exact GitHub run/artifact records; source review (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S37 | Version-1 receipts migrate to version 2; timestamps, latency and correlation IDs retained | PASS | cargo test --locked --offline; db::tests::published_v1_receipts_survive_additive_v2_correlation_migration; python3 scripts/runtime_smoke.py (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |
| S38 | Provider metadata cannot echo known owner/provider credentials into SQLite | PASS | cargo test --locked --offline; proxy::tests::credential_echo_in_response_metadata_never_reaches_sqlite (L) | 7c798c51b80dfc8d03055a2a8cd28da135cbbfa7 |

Final reviewed source and fresh remote CI will be bound in PR #1 and its recorded
run/artifact snapshots. Evidence-only commits do not retroactively change tested
source SHA. Actual main merge/tag/public archives are publication observations,
not substituted for prior source proof. S35 requires separately authorized actual
live validation and remains NOT_RUN under this task's hard limits.

Ownership/provenance: ../legal/OWNER_DECLARATION.md and PROVENANCE_REVIEW.md.
Focused cleanup and archive hashes: ../legal/RETIREMENT_RECORD.json. Complete
former dependency metadata, old task/installer text, historical stories/decisions,
migrations and all prior acceptance evidence are retained. Current CI audits
every retained lockfile and does not fetch an external Harness environment.
