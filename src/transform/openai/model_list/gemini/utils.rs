use http::StatusCode;

use crate::gemini::types::{GeminiApiErrorResponse, GeminiModelInfo};
use crate::openai::types::{
    OpenAiApiError, OpenAiApiErrorResponse, OpenAiModel, OpenAiModelObject,
};

pub fn strip_models_prefix(value: &str) -> String {
    value.strip_prefix("models/").unwrap_or(value).to_string()
}

fn openai_error_type_from_google_status(status: &str) -> Option<&'static str> {
    match status {
        "INVALID_ARGUMENT" => Some("invalid_request_error"),
        "UNAUTHENTICATED" => Some("authentication_error"),
        "PERMISSION_DENIED" => Some("permission_error"),
        "NOT_FOUND" => Some("not_found_error"),
        "RESOURCE_EXHAUSTED" => Some("rate_limit_error"),
        "DEADLINE_EXCEEDED" => Some("timeout_error"),
        "FAILED_PRECONDITION" => Some("invalid_request_error"),
        "ABORTED" | "INTERNAL" | "UNAVAILABLE" => Some("api_error"),
        _ => None,
    }
}

fn openai_error_type_from_http_status(status_code: StatusCode) -> &'static str {
    match status_code.as_u16() {
        400 | 413 => "invalid_request_error",
        401 => "authentication_error",
        403 => "permission_error",
        404 => "not_found_error",
        429 => "rate_limit_error",
        504 => "timeout_error",
        _ => "api_error",
    }
}

pub fn openai_model_from_gemini_model(model: GeminiModelInfo) -> OpenAiModel {
    let id = model
        .base_model_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| strip_models_prefix(&model.name));

    OpenAiModel {
        id,
        created: 0,
        object: OpenAiModelObject::Model,
        owned_by: "google".to_string(),
    }
}

pub fn openai_error_response_from_gemini(
    status_code: StatusCode,
    body: GeminiApiErrorResponse,
) -> OpenAiApiErrorResponse {
    let error = body.error;
    let type_ = error
        .status
        .as_deref()
        .and_then(openai_error_type_from_google_status)
        .unwrap_or_else(|| openai_error_type_from_http_status(status_code));

    OpenAiApiErrorResponse {
        error: OpenAiApiError {
            message: error.message,
            type_: type_.to_string(),
            param: None,
            code: error.status.or(Some(error.code.to_string())),
        },
    }
}
