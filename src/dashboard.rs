use crate::db::Database;
use comfy_table::{presets::ASCII_FULL, Table};

type DashboardResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn run_dashboard(db: &Database, json: bool) -> DashboardResult<()> {
    let report = db
        .summary()
        .await
        .map_err(|_| "cannot read telemetry summary")?;
    if json {
        println!("{}", serde_json::to_string(&report)?);
        return Ok(());
    }
    println!("LLMOps Toolkit local FinOps monitor");
    println!("Attempts: {} | Estimated subtotal: ${:.6} | Unknown cost: {} | Failed/pending: {} | Legacy: {}",
        report.attempts,report.estimated_subtotal_usd,report.unknown_cost_count,report.failed_or_pending_count,report.legacy_count);
    println!(
        "Average observed latency: {}",
        report
            .avg_latency_ms
            .map(|n| format!("{n:.2} ms"))
            .unwrap_or_else(|| "UNKNOWN".into())
    );
    println!("Subtotal is an estimate, excludes unknown and unverified legacy costs; pending work may be billable.");
    let mut table = Table::new();
    table.load_preset(ASCII_FULL);
    table.set_header([
        "Request ID",
        "Timestamp",
        "Model",
        "Outcome",
        "Tokens",
        "Latency ms",
        "Estimated USD",
        "HTTP",
    ]);
    for row in report.latest {
        let text = |key: &str| -> String {
            let value = &row[key];
            if value.is_null() {
                return "UNKNOWN".into();
            }
            let s = value
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| value.to_string());
            s.chars().filter(|c| !c.is_control()).take(128).collect()
        };
        table.add_row([
            text("request_id"),
            text("created_at"),
            text("model"),
            text("outcome"),
            text("tokens"),
            text("latency_ms"),
            text("estimated_usd"),
            text("upstream_status"),
        ]);
    }
    println!("{table}");
    Ok(())
}
