use std::collections::BTreeMap;

use http::StatusCode;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

/// API version for the `anthropic-version` request header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum AnthropicVersion {
    /// Latest stable API version.
    #[default]
    #[serde(rename = "2023-06-01")]
    V20230601,
    /// Initial API release version.
    #[serde(rename = "2023-01-01")]
    V20230101,
}

/// HTTP method used by generated request descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

/// Common envelope for HTTP responses from Claude endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClaudeApiResponse<T> {
    /// HTTP status code returned by server.
    #[serde(with = "crate::claude::types::status_code_serde")]
    pub stats_code: StatusCode,
    /// Response headers.
    pub headers: ClaudeResponseHeaders,
    /// Response body.
    pub body: T,
}

/// Common response headers returned by Claude endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaudeResponseHeaders {
    /// Additional response headers.
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Serde helpers for `http::StatusCode` as numeric code (e.g. 200, 404, 529).
pub mod status_code_serde {
    use http::StatusCode;
    use serde::de::Error as _;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(value.as_u16())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;
        StatusCode::from_u16(code).map_err(D::Error::custom)
    }
}

/// Anthropic beta header value.
///
/// The API accepts both known beta tags and arbitrary strings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnthropicBeta {
    Known(AnthropicBetaKnown),
    Custom(String),
}

/// Known Anthropic beta tags documented by upstream specs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnthropicBetaKnown {
    #[serde(rename = "message-batches-2024-09-24")]
    MessageBatches20240924,
    #[serde(rename = "prompt-caching-2024-07-31")]
    PromptCaching20240731,
    #[serde(rename = "computer-use-2024-10-22")]
    ComputerUse20241022,
    #[serde(rename = "computer-use-2025-01-24")]
    ComputerUse20250124,
    #[serde(rename = "pdfs-2024-09-25")]
    Pdfs20240925,
    #[serde(rename = "token-counting-2024-11-01")]
    TokenCounting20241101,
    #[serde(rename = "token-efficient-tools-2025-02-19")]
    TokenEfficientTools20250219,
    #[serde(rename = "output-128k-2025-02-19")]
    Output128k20250219,
    #[serde(rename = "files-api-2025-04-14")]
    FilesApi20250414,
    #[serde(rename = "mcp-client-2025-04-04")]
    McpClient20250404,
    #[serde(rename = "mcp-client-2025-11-20")]
    McpClient20251120,
    #[serde(rename = "dev-full-thinking-2025-05-14")]
    DevFullThinking20250514,
    #[serde(rename = "interleaved-thinking-2025-05-14")]
    InterleavedThinking20250514,
    #[serde(rename = "code-execution-2025-05-22")]
    CodeExecution20250522,
    #[serde(rename = "extended-cache-ttl-2025-04-11")]
    ExtendedCacheTtl20250411,
    #[serde(rename = "context-1m-2025-08-07")]
    Context1m20250807,
    #[serde(rename = "context-management-2025-06-27")]
    ContextManagement20250627,
    #[serde(rename = "model-context-window-exceeded-2025-08-26")]
    ModelContextWindowExceeded20250826,
    #[serde(rename = "skills-2025-10-02")]
    Skills20251002,
    #[serde(rename = "fast-mode-2026-02-01")]
    FastMode20260201,
    #[serde(rename = "compact-2026-01-12")]
    Compact20260112,
}

/// Claude model metadata returned by list/get model endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaModelInfo {
    /// Unique model identifier.
    pub id: String,
    /// RFC 3339 datetime representing model release timestamp.
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    /// Human-readable model name.
    pub display_name: String,
    /// Object type, always "model".
    #[serde(rename = "type")]
    pub type_: BetaModelType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaModelType {
    #[serde(rename = "model")]
    Model,
}

/// Typed beta error codes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaErrorType {
    /// HTTP 400 (and may be used for other unlisted 4XX errors).
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
    /// HTTP 401.
    #[serde(rename = "authentication_error")]
    AuthenticationError,
    /// Billing-related error type; HTTP status is not explicitly defined in `Errors.md`.
    #[serde(rename = "billing_error")]
    BillingError,
    /// HTTP 403.
    #[serde(rename = "permission_error")]
    PermissionError,
    /// HTTP 413.
    #[serde(rename = "request_too_large")]
    RequestTooLarge,
    /// HTTP 404.
    #[serde(rename = "not_found_error")]
    NotFoundError,
    /// HTTP 429.
    #[serde(rename = "rate_limit_error")]
    RateLimitError,
    /// Timeout-related error type; HTTP status is not explicitly defined in `Errors.md`.
    #[serde(rename = "timeout_error")]
    TimeoutError,
    /// HTTP 500.
    #[serde(rename = "api_error")]
    ApiError,
    /// HTTP 529.
    #[serde(rename = "overloaded_error")]
    OverloadedError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaApiError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaApiErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaApiErrorType {
    #[serde(rename = "api_error")]
    ApiError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaAuthenticationError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaAuthenticationErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaAuthenticationErrorType {
    #[serde(rename = "authentication_error")]
    AuthenticationError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaBillingError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaBillingErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaBillingErrorType {
    #[serde(rename = "billing_error")]
    BillingError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaGatewayTimeoutError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaGatewayTimeoutErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaGatewayTimeoutErrorType {
    #[serde(rename = "timeout_error")]
    TimeoutError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaInvalidRequestError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaInvalidRequestErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaInvalidRequestErrorType {
    #[serde(rename = "invalid_request_error")]
    InvalidRequestError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaNotFoundError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaNotFoundErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaNotFoundErrorType {
    #[serde(rename = "not_found_error")]
    NotFoundError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaOverloadedError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaOverloadedErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaOverloadedErrorType {
    #[serde(rename = "overloaded_error")]
    OverloadedError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaPermissionError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaPermissionErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaPermissionErrorType {
    #[serde(rename = "permission_error")]
    PermissionError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaRateLimitError {
    pub message: String,
    #[serde(rename = "type")]
    pub type_: BetaRateLimitErrorType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaRateLimitErrorType {
    #[serde(rename = "rate_limit_error")]
    RateLimitError,
}

/// Error union returned by beta endpoints.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BetaError {
    InvalidRequest(BetaInvalidRequestError),
    Authentication(BetaAuthenticationError),
    Billing(BetaBillingError),
    Permission(BetaPermissionError),
    NotFound(BetaNotFoundError),
    RateLimit(BetaRateLimitError),
    GatewayTimeout(BetaGatewayTimeoutError),
    Api(BetaApiError),
    Overloaded(BetaOverloadedError),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BetaErrorResponseType {
    #[serde(rename = "error")]
    Error,
}

/// Top-level beta error response wrapper.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BetaErrorResponse {
    pub error: BetaError,
    pub request_id: String,
    #[serde(rename = "type")]
    pub type_: BetaErrorResponseType,
}
