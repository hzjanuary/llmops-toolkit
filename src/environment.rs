//! Resolve current/deprecated environment names once, before any operator I/O.
//! Never include supplied values in errors (including non-Unicode credentials).
use std::ffi::OsString;
use std::path::PathBuf;

pub const DEFAULT_DATABASE: &str = "ragops_harness.sqlite3";

pub struct Environment {
    pub database: Option<PathBuf>,
    pub proxy_token: Option<String>,
    pub scan_token: Option<String>,
}

fn alias(
    current: Option<OsString>,
    legacy: Option<OsString>,
) -> Result<Option<OsString>, &'static str> {
    match (current, legacy) {
        (Some(current), Some(legacy)) if current != legacy => {
            Err("conflicting LLMOPS/RAGOPS environment aliases; unset the deprecated alias")
        }
        (Some(current), _) => Ok(Some(current)),
        (None, legacy) => Ok(legacy),
    }
}

fn token(value: Option<OsString>) -> Result<Option<String>, &'static str> {
    value
        .map(|value| {
            value
                .into_string()
                .map_err(|_| "credential environment variable must contain valid Unicode")
        })
        .transpose()
}

impl Environment {
    pub fn read() -> Result<Self, &'static str> {
        Self::from_lookup(|name| std::env::var_os(name))
    }

    fn from_lookup(mut get: impl FnMut(&str) -> Option<OsString>) -> Result<Self, &'static str> {
        Ok(Self {
            database: alias(get("LLMOPS_DB_PATH"), get("RAGOPS_DB_PATH"))?.map(PathBuf::from),
            proxy_token: token(alias(get("LLMOPS_PROXY_TOKEN"), get("RAGOPS_PROXY_TOKEN"))?)?,
            scan_token: token(alias(get("LLMOPS_SCAN_TOKEN"), get("RAGOPS_SCAN_TOKEN"))?)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn resolve(values: &[(&str, &str)]) -> Result<Environment, &'static str> {
        let values: BTreeMap<_, _> = values.iter().copied().collect();
        Environment::from_lookup(|name| values.get(name).map(OsString::from))
    }

    #[test]
    fn current_names_and_deprecated_fallback_resolve_identically() {
        let current = resolve(&[
            ("LLMOPS_DB_PATH", "synthetic.sqlite3"),
            ("LLMOPS_PROXY_TOKEN", "synthetic-owner-value"),
            ("LLMOPS_SCAN_TOKEN", "synthetic-target-value"),
        ])
        .unwrap();
        let legacy = resolve(&[
            ("RAGOPS_DB_PATH", "synthetic.sqlite3"),
            ("RAGOPS_PROXY_TOKEN", "synthetic-owner-value"),
            ("RAGOPS_SCAN_TOKEN", "synthetic-target-value"),
        ])
        .unwrap();
        assert_eq!(current.database, legacy.database);
        assert_eq!(current.proxy_token, legacy.proxy_token);
        assert_eq!(current.scan_token, legacy.scan_token);
        assert_eq!(current.database, Some(PathBuf::from("synthetic.sqlite3")));
        assert_eq!(
            current.proxy_token.as_deref(),
            Some("synthetic-owner-value")
        );
        assert_eq!(
            current.scan_token.as_deref(),
            Some("synthetic-target-value")
        );
    }

    #[test]
    fn equal_aliases_are_allowed_but_every_conflicting_pair_fails_closed() {
        for suffix in ["DB_PATH", "PROXY_TOKEN", "SCAN_TOKEN"] {
            let current = format!("LLMOPS_{suffix}");
            let legacy = format!("RAGOPS_{suffix}");
            assert!(resolve(&[(&current, "synthetic-a"), (&legacy, "synthetic-a")]).is_ok());
            let error = resolve(&[(&current, "synthetic-a"), (&legacy, "synthetic-b")])
                .err()
                .unwrap();
            assert!(error.contains("conflicting"));
            assert!(!error.contains("synthetic-a") && !error.contains("synthetic-b"));
            assert!(resolve(&[(&current, ""), (&legacy, "synthetic-b")]).is_err());
        }
    }

    #[test]
    fn missing_values_do_not_invent_credentials_or_change_the_default_database() {
        let empty = resolve(&[]).unwrap();
        assert!(empty.database.is_none());
        assert!(empty.proxy_token.is_none() && empty.scan_token.is_none());
        assert_eq!(DEFAULT_DATABASE, "ragops_harness.sqlite3");
        assert_eq!(
            resolve(&[("LLMOPS_PROXY_TOKEN", "")]).unwrap().proxy_token,
            Some(String::new())
        );
    }

    #[test]
    #[cfg(unix)]
    fn non_unicode_credentials_fail_without_disclosing_bytes() {
        use std::os::unix::ffi::OsStringExt;
        let result = Environment::from_lookup(|name| {
            (name == "LLMOPS_PROXY_TOKEN").then(|| OsString::from_vec(vec![0xff, 0xfe]))
        });
        assert_eq!(
            result.err(),
            Some("credential environment variable must contain valid Unicode")
        );
    }
}
