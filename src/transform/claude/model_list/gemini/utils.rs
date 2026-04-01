use http::StatusCode;
use time::OffsetDateTime;

use crate::claude::types::{BetaErrorResponse, BetaModelInfo, BetaModelType};
use crate::gemini::types::{GeminiApiErrorResponse, GeminiModelInfo};
use crate::transform::claude::utils::beta_error_response_from_status_message;

pub fn strip_models_prefix(value: &str) -> String {
    value.strip_prefix("models/").unwrap_or(value).to_string()
}

pub fn ensure_models_prefix(value: &str) -> String {
    if value.starts_with("models/") {
        value.to_string()
    } else {
        format!("models/{value}")
    }
}

pub fn beta_model_info_from_gemini_model(model: GeminiModelInfo) -> BetaModelInfo {
    let id = model
        .base_model_id
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| strip_models_prefix(&model.name));
    BetaModelInfo {
        id: id.clone(),
        created_at: OffsetDateTime::UNIX_EPOCH,
        display_name: model.display_name.unwrap_or_else(|| id.clone()),
        max_input_tokens: model.input_token_limit,
        max_tokens: model.output_token_limit,
        capabilities: None,
        type_: BetaModelType::Model,
    }
}

pub fn beta_error_response_from_gemini(
    status_code: StatusCode,
    body: GeminiApiErrorResponse,
) -> BetaErrorResponse {
    beta_error_response_from_status_message(status_code, body.error.message)
}
