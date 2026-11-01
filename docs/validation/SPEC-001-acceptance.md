# SPEC-001 acceptance matrix

Statuses are PASS, FAIL, BLOCKED or NOT_RUN. No live LLM inference, paid request,
external target scan or model download is authorized. A mock PASS never means a
live provider PASS. Project ownership and source CI are separate gates.

Initial matrix is NOT_RUN until the exact committed source verification record
is attached. Baseline source/build/audit failures remain in the baseline report;
future PASS results do not rewrite them. Detailed results will replace these
initial statuses after execution.

| ID | Concrete acceptance test | Status | Exact command / evidence | Source SHA |
| --- | --- | --- | --- | --- |
| S01 | Local HEAD/refs, operator state, untracked inputs and history audited | PASS | Read-only commands in SPEC-001-baseline.md | d73849824cdf4eddaeb7898d375818f259450dec |
| S02 | Default bind loopback and provider egress disabled | NOT_RUN | cargo test --locked --offline; config::tests::default_loopback_and_egress_disabled | pending committed source |
| S03 | Non-loopback bind needs deliberate opt-in | NOT_RUN | cargo test --locked --offline; config::tests::public_binding_requires_explicit_opt_in | pending committed source |
| S04 | Unauthorized/malformed bearer access and provider-key-as-caller denied | NOT_RUN | cargo test --locked --offline; proxy::tests::auth_health_validation_and_credential_isolation | pending committed source |
| S05 | Owner/provider credentials isolated and distinct; exact reviewed model policy required | NOT_RUN | cargo test --locked --offline; proxy::tests::enablement_requires_separate_credentials_and_reviewed_models | pending committed source |
| S06 | Typed text-only JSON/content-type/body-size limits enforced | NOT_RUN | cargo test --locked --offline; auth_health_validation_and_credential_isolation and typed_request_rejects_unsupported_or_unbounded_fields | pending committed source |
| S07 | Streaming rejected, output cap and model allowlist enforced | NOT_RUN | cargo test --locked --offline; proxy::tests::typed_request_rejects_unsupported_or_unbounded_fields | pending committed source |
| S08 | Concurrency/request/cost reservations block excess egress | NOT_RUN | cargo test --locked --offline; proxy::tests::request_concurrency_cost_and_persistence_limits_prevent_egress | pending committed source |
| S09 | Upstream size/deadline/shape failures recorded as indeterminate | NOT_RUN | cargo test --locked --offline; oversized_and_malformed_success_responses_are_indeterminate and upstream_deadline_and_transport_failures_record_uncertain_work | pending committed source |
| S10 | Provider error/known credential strings not disclosed | NOT_RUN | cargo test --locked --offline; failures_and_missing_usage_are_truthful_and_redacted and credential_echo_is_redacted_recursively | pending committed source |
| S11 | Startup/health/offline commands make no provider calls | NOT_RUN | cargo test --locked --offline; python3 scripts/runtime_smoke.py | pending committed source |
| S12 | Exact price matching, cached-token estimate and version/source/hash provenance | NOT_RUN | cargo test --locked --offline; pricing::tests::unknown_price_is_none_and_matching_is_exact | pending committed source |
| S13 | Unknown price with observed usage remains null, separately counted | NOT_RUN | cargo test --locked --offline; proxy::tests::unknown_response_price_with_observed_usage_stays_null | pending committed source |
| S14 | Failed upstream/missing usage do not invent cost or tokens | NOT_RUN | cargo test --locked --offline; proxy::tests::failures_and_missing_usage_are_truthful_and_redacted | pending committed source |
| S15 | Failed pre-egress log blocks forwarding; final log failure preserves pending | NOT_RUN | cargo test --locked --offline; request_concurrency_cost_and_persistence_limits_prevent_egress and final_log_failure_preserves_pending_and_warns_against_retries | pending committed source |
| S16 | Legacy migration preserves values through restart and backup/restore | NOT_RUN | cargo test --locked --offline; db::tests::migration_preserves_legacy_restart_and_backup | pending committed source |
| S17 | Future schema version rejected unchanged | NOT_RUN | cargo test --locked --offline; db::tests::future_version_is_rejected_without_changing_data | pending committed source |
| S18 | Two concurrent SQLite connections/readers retain 32 attempts | NOT_RUN | cargo test --locked --offline; db::tests::concurrent_connections_and_readers_preserve_receipts | pending committed source |
| S19 | Real headless and TTY monitor, SIGINT/SIGTERM cleanup, bind-error behavior | NOT_RUN | python3 scripts/runtime_smoke.py | pending committed source |
| S20 | Authenticated HTTP proxy -> local mock -> SQLite -> real TUI | NOT_RUN | python3 scripts/runtime_smoke.py (explicit ignored setup test) | pending committed source |
| S21 | Offline scan outcomes deterministic and honestly named | NOT_RUN | cargo test --locked --offline; red_team::tests::fixture_outcomes_are_heuristics_not_findings; python3 scripts/runtime_smoke.py | pending committed source |
| S22 | Scanner rejects missing auth/scope, DNS, metadata and unsafe destinations | NOT_RUN | cargo test --locked --offline; red_team::tests::authorization_and_destination_policy_fail_closed; python3 scripts/runtime_smoke.py | pending committed source |
| S23 | Authorized local mock scan stays within request scope and never follows redirect | NOT_RUN | cargo test --locked --offline; red_team::tests::authorized_synthetic_mock_does_not_follow_redirects_or_expand_requests | pending committed source |
| S24 | Eval rejects malformed/empty/unknown/oversized data and unsafe file types | NOT_RUN | cargo test --locked --offline; eval::tests::malformed_oversized_empty_and_unknown_fields_are_rejected and input_paths_are_regular_and_bounded | pending committed source |
| S25 | Offline eval JSON/hash/aggregation reproducible and labeled heuristic | NOT_RUN | cargo test --locked --offline; eval::tests::offline_contract_and_aggregation_are_deterministic; python3 scripts/runtime_smoke.py | pending committed source |
| S26 | Judge consent/config gate and synthetic adapter work; unknown accounting halts | NOT_RUN | cargo test --locked --offline; eval::tests::llm_judge_adapter_uses_only_synthetic_local_mock and judge_stops_after_unknown_accounting_without_fabricating_scores; python3 scripts/runtime_smoke.py | pending committed source |
| S27 | Current full-history and working-tree secret scans without suppression | NOT_RUN | gitleaks git . --log-opts='--all --full-history' --redact; gitleaks dir . --redact | pending committed source |
| S28 | Both retained lockfiles pass vulnerability/unsoundness gates | NOT_RUN | cargo-audit audit --deny unsound --file Cargo.lock; cargo-audit audit --deny unsound --file rag-harness/Cargo.lock | pending committed source |
| S29 | Declared dependency-license inventory matches both locks | NOT_RUN | python3 scripts/license_inventory.py | pending committed source |
| S30 | Project copyright/inherited-source rights permit MIT grant | BLOCKED | Owner review of NOTICE and LICENSE.proposed required; no grant activated | d73849824cdf4eddaeb7898d375818f259450dec plus proposed changes |
| S31 | Pinned Rust format, strict Clippy, locked offline tests/build | NOT_RUN | python3 scripts/verify.py --output /tmp/ragops-spec001-final-evidence | pending committed source |
| S32 | Source archive matches every Git blob/mode, excludes operator state, reproduces bytewise | NOT_RUN | python3 scripts/source_gate.py --revision HEAD --output NEW_DIRECTORY (called by verify.py) | pending committed source |
| S33 | Clean build from reviewed source archive with empty target | NOT_RUN | cargo build --locked --offline --target-dir NEW_DIRECTORY (called by verify.py) | pending committed source |
| S34 | GitHub verified-source checks exact Draft PR HEAD | NOT_RUN | Await actual Source CI jobs/run/artifact identifiers | pending committed source |
| S35 | Live provider/judge compatibility and external target security assessment | NOT_RUN | Prohibited by SPEC-001 hard limits; no execution command run | no live source validation |
| S36 | Immutable provenance, SHA/digests, accurate docs and unapplied ruleset proposals | NOT_RUN | verify.py evidence.json; exact GitHub run/artifact records; source review | pending committed source |

S19/S20/S31 cover the explicitly invoked fixture setup test separately from the
normal suite's one ignored setup helper. Archive reproducibility is bytewise
source tar reproducibility, not a bit-identical compiled-binary claim. S23 proves
one narrowed local request and redirect refusal; remote authorization remains an
operator assertion and no external target is scanned. Source CI cannot establish
ownership or independently certify safety.
