CREATE TABLE IF NOT EXISTS llm_request_logs (
                request_id TEXT PRIMARY KEY,
                created_at TEXT NOT NULL,
                provider TEXT NOT NULL,
                endpoint TEXT NOT NULL,
                model TEXT NOT NULL,
                upstream_id TEXT,
                prompt_tokens INTEGER NOT NULL CHECK (prompt_tokens >= 0),
                completion_tokens INTEGER NOT NULL CHECK (completion_tokens >= 0),
                total_tokens INTEGER NOT NULL CHECK (total_tokens >= 0),
                cached_prompt_tokens INTEGER NOT NULL DEFAULT 0 CHECK (cached_prompt_tokens >= 0),
                cost_usd REAL NOT NULL CHECK (cost_usd >= 0),
                latency_ms INTEGER NOT NULL CHECK (latency_ms >= 0),
                status_code INTEGER NOT NULL CHECK (status_code BETWEEN 100 AND 599),
                pricing_known INTEGER NOT NULL DEFAULT 0 CHECK (pricing_known IN (0, 1))
            );

            CREATE INDEX IF NOT EXISTS idx_llm_request_logs_created_at
                ON llm_request_logs (created_at);

            CREATE INDEX IF NOT EXISTS idx_llm_request_logs_model_created_at
                ON llm_request_logs (model, created_at);
