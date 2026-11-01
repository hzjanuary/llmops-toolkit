use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub version: u32,
    pub source: String,
    pub as_of: String,
    pub models: BTreeMap<String, Rate>,
    #[serde(skip)]
    pub sha256: String,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rate {
    pub input_per_million: f64,
    pub cached_input_per_million: f64,
    pub output_per_million: f64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    #[serde(default)]
    pub prompt_tokens_details: Cached,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Cached {
    #[serde(default)]
    pub cached_tokens: u64,
}

impl Usage {
    pub fn parse(value: &serde_json::Value) -> Option<Self> {
        let u: Self = serde_json::from_value(value.get("usage")?.clone()).ok()?;
        (u.prompt_tokens <= 10_000_000
            && u.completion_tokens <= 10_000_000
            && u.prompt_tokens.checked_add(u.completion_tokens) == Some(u.total_tokens)
            && u.prompt_tokens_details.cached_tokens <= u.prompt_tokens)
            .then_some(u)
    }
}

impl Catalog {
    pub fn load(path: &Path) -> Result<Self, &'static str> {
        let bytes = crate::config::read_bounded(path, 65_536)?;
        Self::parse(&bytes)
    }
    pub fn parse(bytes: &[u8]) -> Result<Self, &'static str> {
        let mut c: Self = serde_json::from_slice(bytes).map_err(|_| "invalid pricing catalog")?;
        if c.version != 1
            || c.source.trim().is_empty()
            || c.source.len() > 256
            || c.source.chars().any(char::is_control)
            || chrono::NaiveDate::parse_from_str(&c.as_of, "%Y-%m-%d").is_err()
            || c.models.is_empty()
            || c.models.len() > 32
        {
            return Err("invalid pricing version, provenance or model count");
        }
        for (model, r) in &c.models {
            if !crate::config::valid_model(model)
                || [
                    r.input_per_million,
                    r.cached_input_per_million,
                    r.output_per_million,
                ]
                .iter()
                .any(|n| !n.is_finite() || *n < 0.0 || *n > 1_000_000.0)
            {
                return Err("invalid exact model rate");
            }
        }
        c.sha256 = format!("{:x}", Sha256::digest(bytes));
        Ok(c)
    }
    pub fn estimate(&self, model: &str, usage: &Usage) -> Option<f64> {
        let r = self.models.get(model)?;
        let cached = usage.prompt_tokens_details.cached_tokens;
        Some(
            ((usage.prompt_tokens - cached) as f64 * r.input_per_million
                + cached as f64 * r.cached_input_per_million
                + usage.completion_tokens as f64 * r.output_per_million)
                / 1_000_000.0,
        )
    }
    // Every UTF-8 input byte plus framing allowance reserved as an input token.
    // This is a conservative text-only estimate, not a provider billing guarantee.
    pub fn reserve(&self, model: &str, input_bytes: usize, output: u32) -> Option<f64> {
        let r = self.models.get(model)?;
        Some(
            ((input_bytes + 4096) as f64 * r.input_per_million.max(r.cached_input_per_million)
                + output as f64 * r.output_per_million)
                / 1_000_000.0,
        )
    }
    pub fn provenance(&self, model: &str) -> Option<String> {
        self.models.contains_key(model).then(|| {
            format!(
                "v{}:{}:{}:{}",
                self.version, self.as_of, self.source, self.sha256
            )
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    pub fn catalog() -> Catalog {
        Catalog::parse(br#"{"version":1,"source":"synthetic unit fixture, not provider prices","as_of":"2026-01-01","models":{"fixture-model":{"input_per_million":2.0,"cached_input_per_million":1.0,"output_per_million":4.0}}}"#).unwrap()
    }
    #[test]
    fn unknown_price_is_none_and_matching_is_exact() {
        let c = catalog();
        let u = Usage {
            prompt_tokens: 100,
            completion_tokens: 25,
            total_tokens: 125,
            prompt_tokens_details: Cached { cached_tokens: 10 },
        };
        assert_eq!(c.estimate("unknown", &u), None);
        assert_eq!(c.estimate("fixture-model-alias", &u), None);
        assert!((c.estimate("fixture-model", &u).unwrap() - 0.00029).abs() < 1e-12);
        assert!(c.provenance("fixture-model").unwrap().contains(&c.sha256));
    }
    #[test]
    fn invalid_catalog_and_usage_are_rejected() {
        assert!(Catalog::parse(br#"{"version":2}"#).is_err());
        assert!(Usage::parse(
            &serde_json::json!({"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":0}})
        )
        .is_none());
        assert!(Usage::parse(&serde_json::json!({"usage":{"prompt_tokens":1,"completion_tokens":2,"total_tokens":3,"prompt_tokens_details":{"cached_tokens":9}}})).is_none());
    }
}
