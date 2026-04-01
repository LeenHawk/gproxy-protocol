use http::StatusCode;

use crate::claude::types::{BetaError, BetaErrorResponse, BetaModelInfo};
use crate::gemini::types::{GeminiApiError, GeminiApiErrorResponse, GeminiModelInfo};

fn ensure_models_prefix(value: &str) -> String {
    if value.starts_with("models/") {
        value.to_string()
    } else {
        format!("models/{value}")
    }
}

fn gemini_status_from_claude_error(error: &BetaError) -> Option<String> {
    let status = match error {
        BetaError::InvalidRequest(_) => "INVALID_ARGUMENT",
        BetaError::Authentication(_) => "UNAUTHENTICATED",
        BetaError::Billing(_) => "FAILED_PRECONDITION",
        BetaError::Permission(_) => "PERMISSION_DENIED",
        BetaError::NotFound(_) => "NOT_FOUND",
        BetaError::RateLimit(_) => "RESOURCE_EXHAUSTED",
        BetaError::GatewayTimeout(_) => "DEADLINE_EXCEEDED",
        BetaError::Api(_) => "INTERNAL",
        BetaError::Overloaded(_) => "UNAVAILABLE",
    };
    Some(status.to_string())
}

fn claude_error_message(error: &BetaError) -> String {
    match error {
        BetaError::InvalidRequest(err) => err.message.clone(),
        BetaError::Authentication(err) => err.message.clone(),
        BetaError::Billing(err) => err.message.clone(),
        BetaError::Permission(err) => err.message.clone(),
        BetaError::NotFound(err) => err.message.clone(),
        BetaError::RateLimit(err) => err.message.clone(),
        BetaError::GatewayTimeout(err) => err.message.clone(),
        BetaError::Api(err) => err.message.clone(),
        BetaError::Overloaded(err) => err.message.clone(),
    }
}

pub fn gemini_model_info_from_claude_model(model: BetaModelInfo) -> GeminiModelInfo {
    GeminiModelInfo {
        name: ensure_models_prefix(&model.id),
        base_model_id: Some(model.id),
        version: None,
        display_name: if model.display_name.is_empty() {
            None
        } else {
            Some(model.display_name)
        },
        description: None,
        input_token_limit: None,
        output_token_limit: None,
        supported_generation_methods: None,
        thinking: None,
        temperature: None,
        max_temperature: None,
        top_p: None,
        top_k: None,
    }
}

pub fn gemini_error_response_from_claude(
    status_code: StatusCode,
    body: BetaErrorResponse,
) -> GeminiApiErrorResponse {
    GeminiApiErrorResponse {
        error: GeminiApiError {
            code: i32::from(status_code.as_u16()),
            message: claude_error_message(&body.error),
            status: gemini_status_from_claude_error(&body.error),
            details: None,
        },
    }
}
