mod config;
mod dashboard;
mod db;
mod eval;
mod http_client;
mod pricing;
mod proxy;
mod red_team;

use std::error::Error;
use std::io::{stdout, IsTerminal, Write};
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Parser)]
#[command(
    name = "ragops-harness",
    version,
    about = "Local-first proxy telemetry, terminal monitoring and bounded heuristic evaluation"
)]
struct Cli {
    #[arg(
        long,
        global = true,
        env = "RAGOPS_DB_PATH",
        default_value = "ragops_harness.sqlite3"
    )]
    db: PathBuf,
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// Start a loopback proxy, locked against forwarding unless explicitly enabled.
    Serve(config::ServeOptions),
    /// Read an existing database without migrating or writing it.
    Dashboard {
        #[arg(long)]
        json: bool,
    },
    /// Offline fixtures by default; live targets require explicit scoped authorization.
    Scan(red_team::ScanOptions),
    /// Offline lexical heuristic by default; live judging is separately opt-in.
    Eval(eval::EvalOptions),
    /// SQLite-consistent backup to a NEW destination, never overwrite a file.
    Backup {
        #[arg(long)]
        destination: PathBuf,
    },
}

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();
    let cli = Cli::parse();
    // Deliberately do not auto-load .env or discover credentials in parent folders.
    match cli.command {
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            println!();
        }
        Some(Commands::Serve(options)) => run_proxy(&cli.db, options).await?,
        Some(Commands::Dashboard { json }) => {
            let db = db::Database::read_only(&cli.db)
                .map_err(|_| "cannot open existing telemetry database")?;
            dashboard::run_dashboard(&db, json).await?;
        }
        Some(Commands::Scan(options)) => red_team::run_scan(options).await?,
        Some(Commands::Eval(options)) => eval::run_eval(options).await?,
        Some(Commands::Backup { destination }) => {
            db::Database::read_only(&cli.db)
                .map_err(|_| "cannot open backup source")?
                .backup(&destination)
                .await
                .map_err(|_| {
                    "backup failed; destination must be a new file (partial files are preserved)"
                })?;
            println!("Backup completed");
        }
    }
    Ok(())
}

async fn run_proxy(path: &std::path::Path, options: config::ServeOptions) -> AppResult<()> {
    options.validate()?;
    let catalog = options
        .pricing
        .as_deref()
        .map(pricing::Catalog::load)
        .transpose()?
        .unwrap_or_default();
    let tui = !options.no_tui && stdout().is_terminal();
    // Validate configuration before creating/opening the operator's database.
    let owner = std::env::var("RAGOPS_PROXY_TOKEN").ok();
    let provider = std::env::var("OPENAI_API_KEY").ok();
    let validated = proxy::AppState::new(
        options.clone(),
        db::Database::open(":memory:")?,
        catalog.clone(),
        owner.clone(),
        provider.clone(),
    )?;
    drop(validated);
    let db = db::Database::open(path)
        .map_err(|_| "cannot initialize telemetry database; back up and inspect schema")?;
    let state = proxy::AppState::new(options.clone(), db.clone(), catalog, owner, provider)?;
    let listener = tokio::net::TcpListener::bind(options.listen)
        .await
        .map_err(|_| "cannot bind configured listener")?;
    tracing::info!(address=%listener.local_addr()?,provider_enabled=options.enable_provider,"proxy listening");
    let (shutdown, mut receiver) = tokio::sync::watch::channel(false);
    let _terminal = TerminalGuard::enter(tui)?;
    let mut server = tokio::spawn(async move {
        axum::serve(listener, proxy::router(state))
            .with_graceful_shutdown(async move {
                let _ = receiver.changed().await;
            })
            .await
    });
    let signal = shutdown_signal();
    tokio::pin!(signal);
    let mut ticks = tokio::time::interval(Duration::from_secs(2));
    let result: AppResult<()> = loop {
        tokio::select! {
            result=&mut server => {
                break match result {Ok(Ok(()))=>Ok(()),_=>Err("proxy server stopped unexpectedly".into())};
            }
            _=&mut signal => {break Ok(());}
            _=ticks.tick(), if tui => {
                let draw=async {
                    execute!(stdout(),MoveTo(0,0),Clear(ClearType::All))?;
                    dashboard::run_dashboard(&db,false).await?;
                    stdout().flush()?;
                    Ok::<(),Box<dyn Error+Send+Sync>>(())
                }.await;
                if let Err(error)=draw {break Err(error);}
            }
        }
    };
    let _ = shutdown.send(true);
    if !server.is_finished() {
        match tokio::time::timeout(Duration::from_secs(config::DEADLINE_SECS + 5), &mut server)
            .await
        {
            Ok(Ok(Ok(()))) => {}
            Ok(_) => return Err("proxy shutdown failed".into()),
            Err(_) => {
                server.abort();
                let _ = server.await;
                return Err("proxy shutdown deadline exceeded; pending receipts may remain".into());
            }
        }
    }
    result
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut terminate) => {
                tokio::select! {_=tokio::signal::ctrl_c()=>{},_=terminate.recv()=>{}}
            }
            Err(_) => {
                let _ = tokio::signal::ctrl_c().await;
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

struct TerminalGuard {
    entered: bool,
}
impl TerminalGuard {
    fn enter(enabled: bool) -> std::io::Result<Self> {
        let mut guard = Self { entered: false };
        if enabled {
            execute!(stdout(), EnterAlternateScreen)?;
            guard.entered = true;
            execute!(stdout(), Hide)?;
        }
        Ok(guard)
    }
}
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if self.entered {
            let _ = execute!(stdout(), Show, LeaveAlternateScreen);
            let _ = stdout().flush();
        }
    }
}
