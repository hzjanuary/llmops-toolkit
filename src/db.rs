use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use rusqlite::{params, Connection, OpenFlags};
use serde::Serialize;

use crate::pricing::Usage;

type DbResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

#[derive(Clone, Debug)]
pub struct Receipt {
    pub request_id: String,
    pub response_model: Option<String>,
    pub outcome: &'static str,
    pub upstream_status: Option<u16>,
    pub usage: Option<Usage>,
    pub estimated_usd: Option<f64>,
    pub price_provenance: Option<String>,
    pub latency_ms: u64,
    pub error_class: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub attempts: u64,
    pub estimated_subtotal_usd: f64,
    pub unknown_cost_count: u64,
    pub failed_or_pending_count: u64,
    pub legacy_count: u64,
    pub latest: Vec<serde_json::Value>,
}

impl Database {
    pub fn open(path: impl AsRef<Path>) -> DbResult<Self> {
        let mut c = Connection::open(path)?;
        c.busy_timeout(Duration::from_secs(2))?;
        // Acquire the migration lock before inspecting the version (concurrent starts).
        let tx = c.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let version: u32 = tx.pragma_query_value(None, "user_version", |r| r.get(0))?;
        if version > 1 {
            return Err("unsupported future database version; no changes made".into());
        }
        if version == 0 {
            tx.execute_batch(include_str!("legacy_schema.sql"))?;
            tx.execute_batch("CREATE TABLE request_receipts (
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
            PRAGMA user_version=1;")?;
        }
        tx.commit()?;
        c.pragma_update(None, "journal_mode", "WAL")?;
        c.pragma_update(None, "synchronous", "FULL")?;
        Ok(Self {
            connection: Arc::new(Mutex::new(c)),
        })
    }

    pub fn read_only(path: impl AsRef<Path>) -> DbResult<Self> {
        let c = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        c.busy_timeout(Duration::from_secs(2))?;
        Ok(Self {
            connection: Arc::new(Mutex::new(c)),
        })
    }

    pub async fn with_connection<T, F>(&self, action: F) -> DbResult<T>
    where
        T: Send + 'static,
        F: FnOnce(&Connection) -> DbResult<T> + Send + 'static,
    {
        let shared = self.connection.clone();
        tokio::task::spawn_blocking(move || {
            let guard = shared.lock().map_err(|_| "database lock poisoned")?;
            action(&guard)
        })
        .await
        .map_err(|_| "database task failed")?
    }

    // A durable pending receipt MUST precede provider egress. A crash leaves
    // explicitly indeterminate work, never an invented successful/zero-cost row.
    pub async fn begin(&self, id: String, model: String) -> DbResult<()> {
        self.with_connection(move |c| {
            c.execute(
                "INSERT INTO request_receipts(request_id,created_at,requested_model,outcome)
                VALUES(?1,?2,?3,'pending')",
                params![id, chrono::Utc::now().to_rfc3339(), model],
            )?;
            Ok(())
        })
        .await
    }

    pub async fn finish(&self, r: Receipt) -> DbResult<()> {
        self.with_connection(move |c| {
            let u = r.usage.as_ref();
            let changed = c.execute("UPDATE request_receipts SET response_model=?2,outcome=?3,upstream_status=?4,
                prompt_tokens=?5,completion_tokens=?6,total_tokens=?7,cached_tokens=?8,estimated_usd=?9,
                price_provenance=?10,latency_ms=?11,error_class=?12 WHERE request_id=?1 AND outcome='pending'",
                params![r.request_id,r.response_model,r.outcome,r.upstream_status,
                    u.map(|u| u.prompt_tokens as i64),u.map(|u| u.completion_tokens as i64),
                    u.map(|u| u.total_tokens as i64),u.map(|u| u.prompt_tokens_details.cached_tokens as i64),
                    r.estimated_usd,r.price_provenance,r.latency_ms as i64,r.error_class])?;
            if changed != 1 { return Err("receipt not pending".into()); }
            Ok(())
        }).await
    }

    pub async fn summary(&self) -> DbResult<Summary> {
        self.with_connection(|c| {
            let has_new: bool = c.query_row("SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE name='request_receipts' AND type='table')",[],|r|r.get(0))?;
            let legacy_count: u64 = c.query_row("SELECT COUNT(*) FROM llm_request_logs",[],|r|r.get(0))?;
            let mut s = Summary{attempts:legacy_count,estimated_subtotal_usd:0.0,unknown_cost_count:legacy_count,
                failed_or_pending_count:0,legacy_count,latest:Vec::new()};
            // Legacy prices have no reviewed provenance. Keep rows intact and
            // exclude them from the new priced subtotal; report their count.
            if has_new {
                let (n,cost,unknown,failed):(u64,f64,u64,u64)=c.query_row(
                    "SELECT COUNT(*),COALESCE(SUM(estimated_usd),0.0),
                    COALESCE(SUM(estimated_usd IS NULL),0),COALESCE(SUM(outcome != 'success'),0) FROM request_receipts",[],
                    |r|Ok((r.get(0)?,r.get(1)?,r.get(2)?,r.get(3)?)))?;
                s.attempts+=n;s.estimated_subtotal_usd=cost;s.unknown_cost_count+=unknown;s.failed_or_pending_count=failed;
                let mut q=c.prepare("SELECT request_id,requested_model,outcome,total_tokens,estimated_usd,upstream_status FROM request_receipts ORDER BY created_at DESC,request_id DESC LIMIT 5")?;
                s.latest=q.query_map([],|r|Ok(serde_json::json!({
                    "request_id":r.get::<_,String>(0)?,"model":r.get::<_,String>(1)?,"outcome":r.get::<_,String>(2)?,
                    "tokens":r.get::<_,Option<u64>>(3)?,"estimated_usd":r.get::<_,Option<f64>>(4)?,"upstream_status":r.get::<_,Option<u16>>(5)?
                })))?.collect::<Result<_,_>>()?;
            }
            Ok(s)
        }).await
    }

    pub async fn backup(&self, destination: &Path) -> DbResult<()> {
        // Atomically reserve a new destination. Never replace an operator file.
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(destination)?;
        drop(file);
        let destination = destination.to_owned();
        self.with_connection(move |c| {
            c.backup(rusqlite::DatabaseName::Main, &destination, None)?;
            Ok(())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn migration_preserves_legacy_restart_and_backup() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("old.sqlite3");
        {
            let c = Connection::open(&path).unwrap();
            c.execute_batch(include_str!("legacy_schema.sql")).unwrap();
            c.execute("INSERT INTO llm_request_logs VALUES('old','2020-01-01','openai','/v1/chat/completions','old-model',NULL,1,1,2,0,0,1,200,0)",[]).unwrap();
        }
        let db = Database::open(&path).unwrap();
        db.begin("new".into(), "fixture-model".into())
            .await
            .unwrap();
        assert_eq!(db.summary().await.unwrap().unknown_cost_count, 2);
        db.with_connection(|c| {
            assert_eq!(
                c.query_row(
                    "SELECT cost_usd FROM llm_request_logs WHERE request_id='old'",
                    [],
                    |r| r.get::<_, f64>(0)
                )?,
                0.0
            );
            Ok(())
        })
        .await
        .unwrap();
        let backup = temp.path().join("backup.sqlite3");
        db.backup(&backup).await.unwrap();
        assert!(db.backup(&backup).await.is_err());
        drop(db);
        assert_eq!(
            Database::open(&path)
                .unwrap()
                .summary()
                .await
                .unwrap()
                .attempts,
            2
        );
        assert_eq!(
            Database::open(&backup)
                .unwrap()
                .summary()
                .await
                .unwrap()
                .attempts,
            2
        );
        assert_eq!(
            Database::read_only(&path)
                .unwrap()
                .summary()
                .await
                .unwrap()
                .legacy_count,
            1
        );
    }
    #[tokio::test]
    async fn concurrent_connections_and_readers_preserve_receipts() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("test.sqlite3");
        let a = Database::open(&path).unwrap();
        let b = Database::open(&path).unwrap();
        let mut tasks = tokio::task::JoinSet::new();
        for i in 0..32 {
            let db = if i % 2 == 0 { a.clone() } else { b.clone() };
            tasks.spawn(async move {
                db.begin(format!("id-{i}"), "fixture-model".into())
                    .await
                    .unwrap();
                db.summary().await.unwrap();
            });
        }
        while let Some(t) = tasks.join_next().await {
            t.unwrap();
        }
        assert_eq!(a.summary().await.unwrap().attempts, 32);
    }
    #[test]
    fn future_version_is_rejected_without_changing_data() {
        let t = tempfile::tempdir().unwrap();
        let p = t.path().join("future.sqlite3");
        let c = Connection::open(&p).unwrap();
        c.pragma_update(None, "user_version", 99).unwrap();
        assert!(Database::open(&p).is_err());
        assert_eq!(
            c.pragma_query_value(None, "user_version", |r| r.get::<_, u32>(0))
                .unwrap(),
            99
        );
    }
}
