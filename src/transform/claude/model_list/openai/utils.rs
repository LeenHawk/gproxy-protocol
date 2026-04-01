use http::StatusCode;
use time::OffsetDateTime;

use crate::claude::types::{BetaErrorResponse, BetaModelInfo, BetaModelType};
use crate::openai::types::{OpenAiApiErrorResponse, OpenAiModel};
use crate::transform::claude::utils::beta_error_response_from_status_message;

pub fn beta_model_info_from_openai_model(model: OpenAiModel) -> BetaModelInfo {
    let id = model.id;
    BetaModelInfo {
        id: id.clone(),
        created_at: OffsetDateTime::from_unix_timestamp(model.created as i64)
            .unwrap_or(OffsetDateTime::UNIX_EPOCH),
        display_name: id,
        max_input_tokens: None,
        max_tokens: None,
        capabilities: None,
        type_: BetaModelType::Model,
    }
}

pub fn beta_error_response_from_openai(
    status_code: StatusCode,
    body: OpenAiApiErrorResponse,
) -> BetaErrorResponse {
    beta_error_response_from_status_message(status_code, body.error.message)
}
