use http::StatusCode;

use crate::gemini::types::{GeminiApiError, GeminiApiErrorResponse, GeminiModelInfo};
use crate::openai::types::{OpenAiApiErrorResponse, OpenAiModel};

fn ensure_models_prefix(value: &str) -> String {
    if value.starts_with("models/") {
        value.to_string()
    } else {
        format!("models/{value}")
    }
}

fn google_status_from_http(status_code: StatusCode) -> Option<String> {
    let status = match status_code.as_u16() {
        400 => "INVALID_ARGUMENT",
        401 => "UNAUTHENTICATED",
        403 => "PERMISSION_DENIED",
        404 => "NOT_FOUND",
        409 => "ABORTED",
        429 => "RESOURCE_EXHAUSTED",
        500 => "INTERNAL",
        503 | 529 => "UNAVAILABLE",
        504 => "DEADLINE_EXCEEDED",
        _ => return None,
    };
    Some(status.to_string())
}

pub fn gemini_model_info_from_openai_model(model: OpenAiModel) -> GeminiModelInfo {
    GeminiModelInfo {
        name: ensure_models_prefix(&model.id),
        base_model_id: Some(model.id.clone()),
        version: None,
        display_name: Some(model.id),
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

pub fn gemini_error_response_from_openai(
    status_code: StatusCode,
    body: OpenAiApiErrorResponse,
) -> GeminiApiErrorResponse {
    GeminiApiErrorResponse {
        error: GeminiApiError {
            code: i32::from(status_code.as_u16()),
            message: body.error.message,
            status: google_status_from_http(status_code),
            details: None,
        },
    }
}
