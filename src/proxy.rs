use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::body::to_bytes;
use axum::extract::{Request, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tokio::sync::{Mutex, Semaphore};
use uuid::Uuid;

use crate::config::{
    self, ServeOptions, CONCURRENCY, DEADLINE_SECS, MAX_MESSAGES, REQUEST_BYTES, RESPONSE_BYTES,
};
use crate::db::{Database, Receipt};
use crate::pricing::{Catalog, Usage};

const PROVIDER_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Clone)]
pub struct AppState(Arc<Inner>);
struct Inner {
    client: reqwest::Client,
    db: Database,
    options: ServeOptions,
    catalog: Catalog,
    owner_token: Option<String>,
    provider_key: Option<String>,
    endpoint: String,
    concurrent: Semaphore,
    ingress: Semaphore,
    deadline: Duration,
    budget: Mutex<Budget>,
}
#[derive(Default)]
struct Budget {
    attempts: u32,
    reserved_usd: f64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub max_completion_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl ChatRequest {
    pub fn validate(&self, options: &ServeOptions) -> Result<u32, &'static str> {
        if !options.models.contains(&self.model) || !config::valid_model(&self.model) {
            return Err("model_not_allowed");
        }
        if self.stream {
            return Err("streaming_not_supported");
        }
        if self.messages.is_empty()
            || self.messages.len() > MAX_MESSAGES
            || self.messages.iter().any(|m| {
                !["system", "developer", "user", "assistant"].contains(&m.role.as_str())
                    || m.content.trim().is_empty()
                    || m.content.len() > REQUEST_BYTES
            })
        {
            return Err("invalid_messages");
        }
        if self
            .temperature
            .is_some_and(|n| !n.is_finite() || !(0.0..=2.0).contains(&n))
        {
            return Err("invalid_temperature");
        }
        if self.max_tokens.is_some() && self.max_completion_tokens.is_some() {
            return Err("ambiguous_output_budget");
        }
        let output = self
            .max_tokens
            .or(self.max_completion_tokens)
            .unwrap_or(options.max_output_tokens);
        if output == 0 || output > options.max_output_tokens {
            return Err("output_budget_exceeded");
        }
        Ok(output)
    }
    fn payload(&self, output: u32) -> Value {
        // Only supported typed fields leave the process; caller headers never do.
        let mut payload = json!({"model":self.model,"messages":self.messages,"stream":false});
        let field = if self.max_completion_tokens.is_some() {
            "max_completion_tokens"
        } else {
            "max_tokens"
        };
        payload[field] = json!(output);
        if let Some(t) = self.temperature {
            payload["temperature"] = json!(t);
        }
        payload
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: &'static str,
    request_id: String,
}
impl ApiError {
    fn new(status: StatusCode, code: &'static str, request_id: &str) -> Self {
        Self {
            status,
            code,
            request_id: request_id.to_owned(),
        }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(json!({"error":{"code":self.code,"request_id":self.request_id,
            "message": "Request failed. Consult metadata receipts; do not blindly retry potentially billable work."}}))).into_response()
    }
}

impl AppState {
    pub fn new(
        options: ServeOptions,
        db: Database,
        catalog: Catalog,
        owner_token: Option<String>,
        provider_key: Option<String>,
    ) -> Result<Self, &'static str> {
        options.validate()?;
        if owner_token
            .as_deref()
            .is_some_and(|s| !config::valid_secret(s))
            || provider_key
                .as_deref()
                .is_some_and(|s| !config::valid_secret(s))
        {
            return Err("credentials must contain 32..512 non-space ASCII characters");
        }
        if owner_token.is_some() && owner_token == provider_key {
            return Err("caller and provider credentials must be distinct");
        }
        if options.enable_provider
            && (owner_token.is_none() || provider_key.is_none() || options.models.is_empty())
        {
            return Err("provider enablement requires independent owner/provider credentials and exact models");
        }
        if options.enable_provider
            && !options.allow_unknown_price
            && options
                .models
                .iter()
                .any(|m| !catalog.models.contains_key(m))
        {
            return Err(
                "every enabled model requires reviewed pricing or explicit --allow-unknown-price",
            );
        }
        let client = crate::http_client::client().map_err(|_| "cannot initialize HTTP client")?;
        Ok(Self(Arc::new(Inner {
            client,
            db,
            options,
            catalog,
            owner_token,
            provider_key,
            endpoint: PROVIDER_URL.into(),
            concurrent: Semaphore::new(CONCURRENCY),
            ingress: Semaphore::new(CONCURRENCY * 4),
            deadline: Duration::from_secs(DEADLINE_SECS),
            budget: Mutex::new(Budget::default()),
        })))
    }

    fn authorized(&self, headers: &HeaderMap) -> bool {
        let Some(expected) = &self.0.owner_token else {
            return false;
        };
        if headers.get_all(header::AUTHORIZATION).iter().count() != 1 {
            return false;
        }
        let Some(value) = headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
        else {
            return false;
        };
        let Some(token) = value.strip_prefix("Bearer ") else {
            return false;
        };
        Sha256::digest(token.as_bytes())
            .ct_eq(&Sha256::digest(expected.as_bytes()))
            .into()
    }

    pub async fn forward(&self, request: ChatRequest) -> Result<Value, ApiError> {
        let id = Uuid::new_v4().to_string();
        let fail = |status, code| ApiError::new(status, code, &id);
        if !self.0.options.enable_provider {
            return Err(fail(StatusCode::SERVICE_UNAVAILABLE, "provider_disabled"));
        }
        let output = request
            .validate(&self.0.options)
            .map_err(|code| fail(StatusCode::BAD_REQUEST, code))?;
        let payload = request.payload(output);
        let input_bytes = serde_json::to_vec(&payload)
            .map_err(|_| fail(StatusCode::BAD_REQUEST, "invalid_request"))?;
        if input_bytes.len() > REQUEST_BYTES {
            return Err(fail(StatusCode::PAYLOAD_TOO_LARGE, "request_size_limit"));
        }
        let _permit = self
            .0
            .concurrent
            .try_acquire()
            .map_err(|_| fail(StatusCode::TOO_MANY_REQUESTS, "concurrency_limit"))?;
        let reserve = self
            .0
            .catalog
            .reserve(&request.model, input_bytes.len(), output);
        if reserve.is_none() && !self.0.options.allow_unknown_price {
            return Err(fail(
                StatusCode::BAD_REQUEST,
                "unknown_price_not_authorized",
            ));
        }
        {
            let mut budget = self.0.budget.lock().await;
            if budget.attempts >= self.0.options.max_requests {
                return Err(fail(
                    StatusCode::TOO_MANY_REQUESTS,
                    "request_budget_exhausted",
                ));
            }
            if budget.reserved_usd + reserve.unwrap_or(0.0) > self.0.options.max_estimated_usd {
                return Err(fail(
                    StatusCode::TOO_MANY_REQUESTS,
                    "estimated_cost_budget_exhausted",
                ));
            }
            // Never refund an indeterminate reservation or retry automatically.
            budget.attempts += 1;
            budget.reserved_usd += reserve.unwrap_or(0.0);
        }
        self.0
            .db
            .begin(id.clone(), request.model.clone())
            .await
            .map_err(|_| {
                fail(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "telemetry_unavailable_no_forward",
                )
            })?;
        let started = Instant::now();
        let mut receipt = Receipt {
            request_id: id.clone(),
            response_model: None,
            outcome: "indeterminate",
            upstream_status: None,
            usage: None,
            estimated_usd: None,
            price_provenance: None,
            latency_ms: 0,
            error_class: None,
        };
        let operation = async {
            let response = self
                .0
                .client
                .post(&self.0.endpoint)
                .bearer_auth(
                    self.0
                        .provider_key
                        .as_deref()
                        .ok_or("provider_not_configured")?,
                )
                .json(&payload)
                .send()
                .await
                .map_err(|_| "upstream_transport_failed")?;
            receipt.upstream_status = Some(response.status().as_u16());
            if !response.status().is_success() {
                receipt.outcome = "upstream_error";
                // Do not even read provider error text: it may contain secrets.
                return Err("upstream_error_redacted");
            }
            let bytes = crate::http_client::bounded_body(response, RESPONSE_BYTES).await?;
            let mut value: Value =
                serde_json::from_slice(&bytes).map_err(|_| "invalid_upstream_json")?;
            if !value
                .get("choices")
                .and_then(Value::as_array)
                .is_some_and(|choices| {
                    choices.len() == 1 && choices[0].get("message").is_some_and(Value::is_object)
                })
                || value.get("error").is_some()
            {
                return Err("invalid_upstream_shape");
            }
            receipt.response_model = value
                .get("model")
                .and_then(Value::as_str)
                .filter(|s| config::valid_model(s))
                .map(str::to_owned);
            receipt.usage = Usage::parse(&value);
            // Missing response model cannot be priced by silently substituting the requested model.
            if let (Some(model), Some(usage)) = (&receipt.response_model, &receipt.usage) {
                receipt.estimated_usd = self.0.catalog.estimate(model, usage);
                receipt.price_provenance = self.0.catalog.provenance(model);
            }
            receipt.outcome = "success";
            if receipt.usage.is_none() {
                receipt.error_class = Some("usage_unavailable");
            }
            scrub_value(
                &mut value,
                self.0.provider_key.as_deref(),
                self.0.owner_token.as_deref(),
            );
            Ok(value)
        };
        let result = match tokio::time::timeout(self.0.deadline, operation).await {
            Ok(result) => result,
            Err(_) => Err("upstream_deadline_exceeded"),
        };
        receipt.latency_ms = started.elapsed().as_millis().min(u64::MAX as u128) as u64;
        if let Err(code) = &result {
            receipt.error_class = Some(code);
        }
        if self.0.db.finish(receipt).await.is_err() {
            tracing::error!(request_id=%id,"potentially billable work: final telemetry write failed; pending receipt may remain");
            return Err(fail(
                StatusCode::BAD_GATEWAY,
                "potentially_billable_telemetry_incomplete",
            ));
        }
        result.map_err(|code| fail(StatusCode::BAD_GATEWAY, code))
    }
}

fn scrub_value(value: &mut Value, provider: Option<&str>, owner: Option<&str>) {
    match value {
        Value::String(s) => {
            for secret in [provider, owner].into_iter().flatten() {
                *s = s.replace(secret, "[REDACTED]");
            }
        }
        Value::Array(a) => {
            for v in a {
                scrub_value(v, provider, owner);
            }
        }
        Value::Object(o) => {
            // Keys can contain sensitive strings too. Only remove such entries.
            o.retain(|k, _| {
                ![provider, owner]
                    .into_iter()
                    .flatten()
                    .any(|s| k.contains(s))
            });
            for v in o.values_mut() {
                scrub_value(v, provider, owner);
            }
        }
        _ => {}
    }
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route(
            "/health",
            get(|| async { Json(json!({"status":"ok","service":"ragops-harness"})) }),
        )
        .route("/v1/chat/completions", post(chat_completions))
        .with_state(state)
}

async fn chat_completions(State(state): State<AppState>, request: Request) -> Response {
    let id = Uuid::new_v4().to_string();
    if !state.authorized(request.headers()) {
        return ApiError::new(StatusCode::UNAUTHORIZED, "unauthorized", &id).into_response();
    }
    let _ingress = match state.0.ingress.try_acquire() {
        Ok(permit) => permit,
        Err(_) => {
            return ApiError::new(StatusCode::TOO_MANY_REQUESTS, "ingress_limit", &id)
                .into_response()
        }
    };
    let content_type = request
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if content_type.split(';').next().unwrap_or("").trim() != "application/json" {
        return ApiError::new(StatusCode::UNSUPPORTED_MEDIA_TYPE, "json_required", &id)
            .into_response();
    }
    let body = match tokio::time::timeout(
        Duration::from_secs(5),
        to_bytes(request.into_body(), REQUEST_BYTES),
    )
    .await
    {
        Ok(Ok(body)) => body,
        Ok(Err(_)) => {
            return ApiError::new(StatusCode::PAYLOAD_TOO_LARGE, "request_size_limit", &id)
                .into_response()
        }
        Err(_) => {
            return ApiError::new(StatusCode::REQUEST_TIMEOUT, "body_deadline_exceeded", &id)
                .into_response()
        }
    };
    let parsed: ChatRequest = match serde_json::from_slice(&body) {
        Ok(parsed) => parsed,
        Err(_) => {
            return ApiError::new(StatusCode::BAD_REQUEST, "invalid_typed_request", &id)
                .into_response()
        }
    };
    match state.forward(parsed).await {
        Ok(value) => Json(value).into_response(),
        Err(error) => error.into_response(),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use clap::Parser;
    use std::sync::atomic::{AtomicUsize, Ordering};
    const OWNER: &str = "synthetic-owner-token-for-local-tests-only";
    const PROVIDER: &str = "synthetic-provider-key-for-local-tests-only";
    #[derive(Parser)]
    struct TestCli {
        #[command(flatten)]
        options: ServeOptions,
    }
    fn options() -> ServeOptions {
        TestCli::parse_from(["test", "--enable-provider", "--model", "fixture-model"]).options
    }
    pub fn request() -> ChatRequest {
        serde_json::from_value(
            json!({"model":"fixture-model","messages":[{"role":"user","content":"synthetic"}]}),
        )
        .unwrap()
    }
    pub async fn mock(
        body: Value,
        status: StatusCode,
    ) -> (AppState, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
        let count = Arc::new(AtomicUsize::new(0));
        let c = count.clone();
        let app = Router::new().route(
            "/",
            post(move |headers: HeaderMap, Json(v): Json<Value>| {
                let c = c.clone();
                let b = body.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    assert_eq!(headers[header::AUTHORIZATION], format!("Bearer {PROVIDER}"));
                    assert!(v.get("max_tokens").is_some());
                    (status, Json(b))
                }
            }),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap());
        let task = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        let mut state = AppState::new(
            options(),
            Database::open(":memory:").unwrap(),
            crate::pricing::tests::catalog(),
            Some(OWNER.into()),
            Some(PROVIDER.into()),
        )
        .unwrap();
        Arc::get_mut(&mut state.0).unwrap().endpoint = url;
        (state, count, task)
    }
    fn success() -> Value {
        json!({"model":"fixture-model","choices":[{"message":{"content":"synthetic reply"}}],"usage":{"prompt_tokens":10,"completion_tokens":2,"total_tokens":12}})
    }
    #[test]
    fn typed_request_rejects_unsupported_or_unbounded_fields() {
        let mut r = request();
        let o = options();
        assert_eq!(r.validate(&o).unwrap(), 256);
        r.stream = true;
        assert!(r.validate(&o).is_err());
        r.stream = false;
        r.max_tokens = Some(257);
        assert!(r.validate(&o).is_err());
        r.max_tokens = None;
        r.model = "fixture-model-alias".into();
        assert!(r.validate(&o).is_err());
        assert!(serde_json::from_value::<ChatRequest>(
            json!({"model":"fixture-model","messages":[],"n":100})
        )
        .is_err());
    }
    #[tokio::test]
    async fn auth_health_validation_and_credential_isolation() {
        let (state, count, mock) = mock(success(), StatusCode::OK).await;
        let l = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", l.local_addr().unwrap());
        let app = router(state.clone());
        let server = tokio::spawn(async move {
            axum::serve(l, app).await.unwrap();
        });
        let client = crate::http_client::client().unwrap();
        assert!(client
            .get(format!("{url}/health"))
            .send()
            .await
            .unwrap()
            .status()
            .is_success());
        assert_eq!(count.load(Ordering::SeqCst), 0);
        for token in [None, Some("wrong"), Some(PROVIDER)] {
            let mut r = client
                .post(format!("{url}/v1/chat/completions"))
                .json(&request());
            if let Some(t) = token {
                r = r.bearer_auth(t);
            }
            assert_eq!(r.send().await.unwrap().status(), StatusCode::UNAUTHORIZED);
        }
        let endpoint = format!("{url}/v1/chat/completions");
        assert_eq!(
            client
                .post(&endpoint)
                .bearer_auth(OWNER)
                .body("{}")
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
        assert_eq!(
            client
                .post(&endpoint)
                .bearer_auth(OWNER)
                .header("content-type", "application/json")
                .body("x".repeat(REQUEST_BYTES + 1))
                .send()
                .await
                .unwrap()
                .status(),
            StatusCode::PAYLOAD_TOO_LARGE
        );
        assert_eq!(count.load(Ordering::SeqCst), 0);
        assert!(client
            .post(&endpoint)
            .bearer_auth(OWNER)
            .json(&request())
            .send()
            .await
            .unwrap()
            .status()
            .is_success());
        assert_eq!(count.load(Ordering::SeqCst), 1);
        let s = state.0.db.summary().await.unwrap();
        assert_eq!(s.attempts, 1);
        assert_eq!(s.unknown_cost_count, 0);
        server.abort();
        mock.abort();
    }
    #[tokio::test]
    async fn failures_and_missing_usage_are_truthful_and_redacted() {
        let (s, _, t) = mock(
            json!({"error":{"message":PROVIDER}}),
            StatusCode::UNAUTHORIZED,
        )
        .await;
        let e = s.forward(request()).await.unwrap_err();
        assert_eq!(e.code, "upstream_error_redacted");
        assert!(!format!("{e:?}").contains(PROVIDER));
        let summary = s.0.db.summary().await.unwrap();
        assert_eq!(summary.failed_or_pending_count, 1);
        assert_eq!(summary.unknown_cost_count, 1);
        t.abort();
        let (s, _, t) = mock(
            json!({"model":"unknown-model","choices":[{"message":{"content":"synthetic"}}]}),
            StatusCode::OK,
        )
        .await;
        assert!(s.forward(request()).await.is_ok());
        let sum = s.0.db.summary().await.unwrap();
        assert_eq!(sum.unknown_cost_count, 1);
        assert!(sum.latest[0]["estimated_usd"].is_null());
        assert!(sum.latest[0]["tokens"].is_null());
        t.abort();
    }
    #[tokio::test]
    async fn request_concurrency_cost_and_persistence_limits_prevent_egress() {
        let (mut s, count, t) = mock(success(), StatusCode::OK).await;
        Arc::get_mut(&mut s.0).unwrap().options.max_requests = 1;
        s.forward(request()).await.unwrap();
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "request_budget_exhausted"
        );
        assert_eq!(count.load(Ordering::SeqCst), 1);
        t.abort();
        let (mut s, count, t) = mock(success(), StatusCode::OK).await;
        Arc::get_mut(&mut s.0).unwrap().options.max_estimated_usd = 0.00000001;
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "estimated_cost_budget_exhausted"
        );
        assert_eq!(count.load(Ordering::SeqCst), 0);
        t.abort();
        let (s, count, t) = mock(success(), StatusCode::OK).await;
        let permits =
            s.0.concurrent
                .acquire_many(CONCURRENCY as u32)
                .await
                .unwrap();
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "concurrency_limit"
        );
        drop(permits);
        s.0.db
            .with_connection(|c| {
                c.execute_batch("DROP TABLE request_receipts")?;
                Ok(())
            })
            .await
            .unwrap();
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "telemetry_unavailable_no_forward"
        );
        assert_eq!(count.load(Ordering::SeqCst), 0);
        t.abort();
    }
    #[test]
    fn credential_echo_is_redacted_recursively() {
        let mut v = json!({"choices":[{"content":PROVIDER}],OWNER:"secret key name"});
        scrub_value(&mut v, Some(PROVIDER), Some(OWNER));
        let s = v.to_string();
        assert!(!s.contains(PROVIDER));
        assert!(!s.contains(OWNER));
    }
    #[tokio::test]
    async fn unknown_response_price_with_observed_usage_stays_null() {
        let mut response = success();
        response["model"] = json!("unknown-alias");
        let (s, _, t) = mock(response, StatusCode::OK).await;
        s.forward(request()).await.unwrap();
        let summary = s.0.db.summary().await.unwrap();
        assert_eq!(summary.unknown_cost_count, 1);
        assert_eq!(summary.latest[0]["tokens"], 12);
        assert!(summary.latest[0]["estimated_usd"].is_null());
        t.abort();
    }
    #[tokio::test]
    async fn final_log_failure_preserves_pending_and_warns_against_retries() {
        let (s, count, t) = mock(success(), StatusCode::OK).await;
        s.0.db.with_connection(|c| {c.execute_batch("CREATE TRIGGER synthetic_failure BEFORE UPDATE ON request_receipts BEGIN SELECT RAISE(FAIL,'synthetic log failure'); END;")?;Ok(())}).await.unwrap();
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "potentially_billable_telemetry_incomplete"
        );
        assert_eq!(count.load(Ordering::SeqCst), 1);
        let summary = s.0.db.summary().await.unwrap();
        assert_eq!(summary.latest[0]["outcome"], "pending");
        assert_eq!(summary.unknown_cost_count, 1);
        t.abort();
    }
    #[tokio::test]
    async fn oversized_and_malformed_success_responses_are_indeterminate() {
        let mut response = success();
        response["choices"][0]["message"]["content"] = json!("x".repeat(RESPONSE_BYTES + 1));
        let (s, _, t) = mock(response, StatusCode::OK).await;
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "response_size_limit"
        );
        assert_eq!(s.0.db.summary().await.unwrap().failed_or_pending_count, 1);
        t.abort();
        let (s, _, t) = mock(json!({"unexpected":"synthetic"}), StatusCode::OK).await;
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "invalid_upstream_shape"
        );
        assert!(s.0.db.summary().await.unwrap().latest[0]["tokens"].is_null());
        t.abort();
    }
    #[tokio::test]
    async fn upstream_deadline_and_transport_failures_record_uncertain_work() {
        let (mut s, _, old) = mock(success(), StatusCode::OK).await;
        old.abort();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let inner = Arc::get_mut(&mut s.0).unwrap();
        inner.endpoint = format!("http://{}/", listener.local_addr().unwrap());
        inner.deadline = Duration::from_millis(30);
        let app = Router::new().route(
            "/",
            post(|| async {
                tokio::time::sleep(Duration::from_secs(2)).await;
                Json(success())
            }),
        );
        let t = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        assert_eq!(
            s.forward(request()).await.unwrap_err().code,
            "upstream_deadline_exceeded"
        );
        let summary = s.0.db.summary().await.unwrap();
        assert_eq!(summary.latest[0]["outcome"], "indeterminate");
        assert!(summary.latest[0]["estimated_usd"].is_null());
        t.abort();
    }
    #[test]
    fn enablement_requires_separate_credentials_and_reviewed_models() {
        let o = options();
        assert!(AppState::new(
            o.clone(),
            Database::open(":memory:").unwrap(),
            Catalog::default(),
            None,
            None
        )
        .is_err());
        assert!(AppState::new(
            o.clone(),
            Database::open(":memory:").unwrap(),
            crate::pricing::tests::catalog(),
            Some(OWNER.into()),
            Some(OWNER.into())
        )
        .is_err());
        assert!(AppState::new(
            o,
            Database::open(":memory:").unwrap(),
            Catalog::default(),
            Some(OWNER.into()),
            Some(PROVIDER.into())
        )
        .is_err());
    }
    #[tokio::test]
    #[ignore = "Run by scripts/runtime_smoke.py with an isolated synthetic database"]
    async fn prepare_full_flow_receipt() {
        let path = std::env::var("RAGOPS_TEST_DB").expect("isolated synthetic path required");
        assert!(
            !std::path::Path::new(&path).exists(),
            "fixture must never overwrite a database"
        );
        let (mut state, count, mock) = mock(success(), StatusCode::OK).await;
        Arc::get_mut(&mut state.0).unwrap().db = Database::open(&path).unwrap();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!(
            "http://{}/v1/chat/completions",
            listener.local_addr().unwrap()
        );
        let server = tokio::spawn(async move {
            axum::serve(listener, router(state)).await.unwrap();
        });
        let response = crate::http_client::client()
            .unwrap()
            .post(endpoint)
            .bearer_auth(OWNER)
            .json(&request())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        server.abort();
        mock.abort();
    }
}
