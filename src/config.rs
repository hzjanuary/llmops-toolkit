use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Args;

pub const REQUEST_BYTES: usize = 65_536;
pub const RESPONSE_BYTES: usize = 2_097_152;
pub const CONCURRENCY: usize = 4;
pub const DEADLINE_SECS: u64 = 30;
pub const MAX_MESSAGES: usize = 64;
pub const DATASET_BYTES: usize = 1_048_576;
pub const DATASET_ROWS: usize = 100;
pub const FIELD_BYTES: usize = 16_384;

#[derive(Clone, Args)]
pub struct ServeOptions {
    #[arg(long, default_value = "127.0.0.1:8000")]
    pub listen: SocketAddr,
    /// Explicitly permit a non-loopback bind. Use a trusted TLS terminator.
    #[arg(long)]
    pub allow_non_loopback: bool,
    /// Permit provider egress; startup itself never calls the provider.
    #[arg(long)]
    pub enable_provider: bool,
    /// Exact allowed model IDs (repeatable). No wildcard matching.
    #[arg(long = "model")]
    pub models: Vec<String>,
    #[arg(long, default_value_t = 256, value_parser = clap::value_parser!(u32).range(1..=4096))]
    pub max_output_tokens: u32,
    /// Per-process request-attempt cap, including indeterminate attempts.
    #[arg(long, default_value_t = 100, value_parser = clap::value_parser!(u32).range(1..=1000))]
    pub max_requests: u32,
    #[arg(long)]
    pub pricing: Option<PathBuf>,
    /// Permit unknown cost for proxy traffic only. Never allowed for live judging.
    #[arg(long)]
    pub allow_unknown_price: bool,
    /// Conservative estimated USD reservation budget for this process.
    #[arg(long, default_value_t = 1.0)]
    pub max_estimated_usd: f64,
    #[arg(long)]
    pub no_tui: bool,
}

impl ServeOptions {
    pub fn validate(&self) -> Result<(), &'static str> {
        if !self.listen.ip().is_loopback() && !self.allow_non_loopback {
            return Err("non-loopback binding requires --allow-non-loopback");
        }
        if self.models.len() > 32 || self.models.iter().any(|m| !valid_model(m)) {
            return Err("configure at most 32 exact ASCII model IDs");
        }
        if !(1..=4096).contains(&self.max_output_tokens)
            || !(1..=1000).contains(&self.max_requests)
            || !self.max_estimated_usd.is_finite()
            || self.max_estimated_usd <= 0.0
            || self.max_estimated_usd > 1000.0
        {
            return Err("invalid request, token or estimated cost budget");
        }
        Ok(())
    }
}

pub fn valid_model(model: &str) -> bool {
    !model.is_empty()
        && model.len() <= 128
        && model
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || b"-_.:".contains(&c))
}

pub fn valid_secret(secret: &str) -> bool {
    (32..=512).contains(&secret.len()) && secret.bytes().all(|c| c.is_ascii_graphic())
}

use std::io::Read;
pub fn read_bounded(path: &std::path::Path, limit: usize) -> Result<Vec<u8>, &'static str> {
    // Reject devices, directories, FIFOs and symlinks before opening. The local
    // filesystem owner is trusted; this is not a sandbox against concurrent swaps.
    let meta = std::fs::symlink_metadata(path).map_err(|_| "cannot inspect input file")?;
    if !meta.file_type().is_file() || meta.len() > limit as u64 {
        return Err("input must be a bounded regular file (no symlinks)");
    }
    let file = std::fs::File::open(path).map_err(|_| "cannot open input file")?;
    if !file
        .metadata()
        .map_err(|_| "cannot inspect open input")?
        .is_file()
    {
        return Err("input must be regular");
    }
    let mut bytes = Vec::new();
    file.take(limit as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| "cannot read input")?;
    if bytes.len() > limit {
        return Err("input size limit exceeded");
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        options: ServeOptions,
    }
    #[test]
    fn default_loopback_and_egress_disabled() {
        let c = TestCli::parse_from(["test"]).options;
        assert!(c.listen.ip().is_loopback());
        assert!(!c.enable_provider);
        assert!(c.models.is_empty());
        assert!(c.validate().is_ok());
    }
    #[test]
    fn public_binding_requires_explicit_opt_in() {
        let mut c = TestCli::parse_from(["test", "--listen", "0.0.0.0:8000"]).options;
        assert!(c.validate().is_err());
        c.allow_non_loopback = true;
        assert!(c.validate().is_ok());
    }
    #[test]
    fn rejects_invalid_budgets_models_and_secrets() {
        let mut c = TestCli::parse_from(["test"]).options;
        c.max_estimated_usd = f64::NAN;
        assert!(c.validate().is_err());
        assert!(!valid_model("x\x1b[2J"));
        assert!(!valid_secret("short"));
        assert!(!valid_secret(&" ".repeat(32)));
    }
}
