use http::StatusCode;

use crate::claude::types::{BetaError, BetaErrorResponse, BetaModelInfo};
use crate::openai::types::{
    OpenAiApiError, OpenAiApiErrorResponse, OpenAiModel, OpenAiModelObject,
};

fn claude_error_type(error: &BetaError) -> &'static str {
    match error {
        BetaError::InvalidRequest(_) => "invalid_request_error",
        BetaError::Authentication(_) => "authentication_error",
        BetaError::Billing(_) => "billing_error",
        BetaError::Permission(_) => "permission_error",
        BetaError::NotFound(_) => "not_found_error",
        BetaError::RateLimit(_) => "rate_limit_error",
        BetaError::GatewayTimeout(_) => "timeout_error",
        BetaError::Api(_) => "api_error",
        BetaError::Overloaded(_) => "overloaded_error",
    }
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

pub fn openai_model_from_claude_model(model: BetaModelInfo) -> OpenAiModel {
    OpenAiModel {
        id: model.id,
        created: u64::try_from(model.created_at.unix_timestamp()).unwrap_or_default(),
        object: OpenAiModelObject::Model,
        owned_by: "anthropic".to_string(),
    }
}

pub fn openai_error_response_from_claude(
    _status_code: StatusCode,
    body: BetaErrorResponse,
) -> OpenAiApiErrorResponse {
    OpenAiApiErrorResponse {
        error: OpenAiApiError {
            message: claude_error_message(&body.error),
            type_: claude_error_type(&body.error).to_string(),
            param: None,
            code: None,
        },
    }
}
