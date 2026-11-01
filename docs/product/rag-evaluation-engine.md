# RAG evaluation contract — LLMOps Toolkit

Input remains a JSON array of question/context/answer string records. Strict
validation rejects unknown fields, duplicate struct fields, malformed JSON,
empty arrays/fields, NUL content, more than 100 rows, fields above 16384 bytes and
files above 1 MiB. Input must be a regular file, not a symlink/device/FIFO.
Local filesystem races are outside the trust boundary.

Default `eval --dataset PATH` uses lexical_subset_heuristic_v1: normalized
alphanumeric answer words must be a subset of context words to score 1, otherwise
0. No lexical words causes abstention. It does not understand entailment,
negation or paraphrase and must never be called ground-truth faithfulness.
Question is required and validated but is not part of context/answer scoring.

Output JSON includes schema/method/limitations, original dataset SHA-256, ordered
row indices and nullable scores, evaluated/abstention counts and mean over scored
rows (null if none). No dataset content/path is printed. Integer score sums and
input ordering make offline results reproducible. No provider is initialized
for the offline path. Hashes of low-entropy data can still reveal content.

Live binary LLM-as-judge is separate: requires --judge, --consent-send-dataset,
--reviewed-config, --provider openai, exact --model, reviewed versioned --pricing,
explicit --max-live-requests (1..100), --max-judge-tokens (1..256), positive bounded
--max-estimated-usd, independent LLMOPS_PROXY_TOKEN and OPENAI_API_KEY. The entire
dataset is validated against request count before any egress. Unknown-model cost
policy is not allowed. Context/answer is sent as untrusted JSON under a system
instruction; binary replies other than exactly 0/1 are abstentions/errors. This
does not eliminate prompt injection or nondeterministic judgments.

Live calls reuse the bounded no-redirect proxy transport and conservative
reservation policy; no automatic retry. Unknown/mismatched model or unavailable
usage stops subsequent judging; remaining rows are explicitly not attempted. Receipts live only in a temporary
in-memory database for judging. Output labels method/model, price provenance,
nullable estimated cost and failed/abstained rows. Privacy consent acknowledges
provider processing of context/answer; do not use private data without review.
Actual live inference remains NOT_RUN by SPEC-001 hard limits; local synthetic
judge-adapter proof is separate. There is no claim that offline heuristic scores
are a substitute for live or human faithfulness validation.

Current LLMOPS_* environment names retain deprecated RAGOPS_* aliases: use the
current value when present, legacy fallback when absent, accept equal pairs and
reject conflicting pairs without logging values. See the [README environment table](../../README.md).
The default ragops_harness.sqlite3 filename remains unchanged intentionally; no
branding-driven database migration, move or rename occurs.
