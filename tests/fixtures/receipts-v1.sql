-- Historical schema from 7b2349a7168a254b46b65a021863b57970380bee, not regenerated at runtime.
CREATE TABLE request_receipts (
                request_id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                requested_model TEXT NOT NULL,
                response_model TEXT,
                outcome TEXT NOT NULL CHECK(outcome IN ('pending','success','upstream_error','indeterminate')),
                upstream_status INTEGER,
                prompt_tokens INTEGER, completion_tokens INTEGER, total_tokens INTEGER, cached_tokens INTEGER,
                estimated_usd REAL CHECK(estimated_usd IS NULL OR estimated_usd >= 0),
                price_provenance TEXT,
                latency_ms INTEGER NOT NULL DEFAULT 0,
                error_class TEXT,
                potentially_billable INTEGER NOT NULL DEFAULT 1 CHECK(potentially_billable IN (0,1))
            ); CREATE INDEX receipts_created ON request_receipts(created_at);
            PRAGMA user_version=1;
