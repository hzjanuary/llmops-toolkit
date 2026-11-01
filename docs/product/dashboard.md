# Terminal monitoring contract — LLMOps Toolkit

`serve` renders on a real TTY every two seconds using alternate screen and hidden
cursor. A guard restores both on normal return, recoverable error and Rust
unwinding. SIGINT/SIGTERM supervise and drain the Axum task before return.
Non-TTY and --no-tui modes emit no terminal controls. SIGKILL/abort/power loss and
failed device writes are outside restoration guarantees.

`dashboard` opens an existing SQLite DB read-only without migration, prints one
comfy-table report and exits. `dashboard --json` emits deterministic structured
metadata for the selected snapshot. It never creates a DB or calls a provider.

Reports show attempt count, estimated subtotal, unknown-cost count, failed/pending
count, legacy count, average observed latency (excluding pending work), and five latest new receipts (timestamp then request ID order).
Each row retains timestamp and latency; JSON also exposes bounded upstream
correlation IDs. Unknown numeric fields display UNKNOWN or JSON null. Legacy values are preserved
but excluded from the new subtotal and latest table because provenance is
unverified. No 'vulnerability' flags are inferred from endpoint names or statuses.
Control characters are removed from rendered cell text. No web UI/API is present.

Current heading: `LLMOps Toolkit local FinOps monitor`.
