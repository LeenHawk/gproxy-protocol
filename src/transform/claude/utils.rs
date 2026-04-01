use http::StatusCode;

use crate::claude::count_tokens::types::{
    BetaContentBlockParam, BetaMessageContent, BetaSystemPrompt, Model, ModelKnown,
};
use crate::claude::types::{
    BetaApiError, BetaApiErrorType, BetaAuthenticationError, BetaAuthenticationErrorType,
    BetaBillingError, BetaBillingErrorType, BetaError, BetaErrorResponse, BetaErrorResponseType,
    BetaGatewayTimeoutError, BetaGatewayTimeoutErrorType, BetaInvalidRequestError,
    BetaInvalidRequestErrorType, BetaNotFoundError, BetaNotFoundErrorType, BetaOverloadedError,
    BetaOverloadedErrorType, BetaPermissionError, BetaPermissionErrorType, BetaRateLimitError,
    BetaRateLimitErrorType,
};

pub fn beta_error_response_from_status_message(
    status_code: StatusCode,
    message: String,
) -> BetaErrorResponse {
    let error = match status_code.as_u16() {
        400 | 413 => BetaError::InvalidRequest(BetaInvalidRequestError {
            message,
            type_: BetaInvalidRequestErrorType::InvalidRequestError,
        }),
        401 => BetaError::Authentication(BetaAuthenticationError {
            message,
            type_: BetaAuthenticationErrorType::AuthenticationError,
        }),
        402 => BetaError::Billing(BetaBillingError {
            message,
            type_: BetaBillingErrorType::BillingError,
        }),
        403 => BetaError::Permission(BetaPermissionError {
            message,
            type_: BetaPermissionErrorType::PermissionError,
        }),
        404 => BetaError::NotFound(BetaNotFoundError {
            message,
            type_: BetaNotFoundErrorType::NotFoundError,
        }),
        429 => BetaError::RateLimit(BetaRateLimitError {
            message,
            type_: BetaRateLimitErrorType::RateLimitError,
        }),
        504 => BetaError::GatewayTimeout(BetaGatewayTimeoutError {
            message,
            type_: BetaGatewayTimeoutErrorType::TimeoutError,
        }),
        529 => BetaError::Overloaded(BetaOverloadedError {
            message,
            type_: BetaOverloadedErrorType::OverloadedError,
        }),
        _ => BetaError::Api(BetaApiError {
            message,
            type_: BetaApiErrorType::ApiError,
        }),
    };

    BetaErrorResponse {
        error,
        request_id: String::new(),
        type_: BetaErrorResponseType::Error,
    }
}

pub fn claude_model_to_string(model: &Model) -> String {
    match model {
        Model::Custom(model) => model.clone(),
        Model::Known(model) => match model {
            ModelKnown::ClaudeOpus46 => "claude-opus-4-6",
            ModelKnown::ClaudeOpus4520251101 => "claude-opus-4-5-20251101",
            ModelKnown::ClaudeOpus45 => "claude-opus-4-5",
            ModelKnown::Claude37SonnetLatest => "claude-3-7-sonnet-latest",
            ModelKnown::Claude37Sonnet20250219 => "claude-3-7-sonnet-20250219",
            ModelKnown::Claude35HaikuLatest => "claude-3-5-haiku-latest",
            ModelKnown::Claude35Haiku20241022 => "claude-3-5-haiku-20241022",
            ModelKnown::ClaudeHaiku45 => "claude-haiku-4-5",
            ModelKnown::ClaudeHaiku4520251001 => "claude-haiku-4-5-20251001",
            ModelKnown::ClaudeSonnet420250514 => "claude-sonnet-4-20250514",
            ModelKnown::ClaudeSonnet40 => "claude-sonnet-4-0",
            ModelKnown::Claude4Sonnet20250514 => "claude-4-sonnet-20250514",
            ModelKnown::ClaudeSonnet45 => "claude-sonnet-4-5",
            ModelKnown::ClaudeSonnet4520250929 => "claude-sonnet-4-5-20250929",
            ModelKnown::ClaudeSonnet46 => "claude-sonnet-4-6",
            ModelKnown::ClaudeOpus40 => "claude-opus-4-0",
            ModelKnown::ClaudeOpus420250514 => "claude-opus-4-20250514",
            ModelKnown::Claude4Opus20250514 => "claude-4-opus-20250514",
            ModelKnown::ClaudeOpus4120250805 => "claude-opus-4-1-20250805",
            ModelKnown::Claude3OpusLatest => "claude-3-opus-latest",
            ModelKnown::Claude3Opus20240229 => "claude-3-opus-20240229",
            ModelKnown::Claude3Haiku20240307 => "claude-3-haiku-20240307",
        }
        .to_string(),
    }
}

pub fn beta_message_content_to_text(content: &BetaMessageContent) -> String {
    match content {
        BetaMessageContent::Text(text) => text.clone(),
        BetaMessageContent::Blocks(blocks) => blocks
            .iter()
            .map(beta_content_block_to_text)
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn beta_system_prompt_to_text(system: Option<BetaSystemPrompt>) -> Option<String> {
    let text = match system {
        Some(BetaSystemPrompt::Text(text)) => text,
        Some(BetaSystemPrompt::Blocks(blocks)) => blocks
            .into_iter()
            .map(|block| block.text)
            .collect::<Vec<_>>()
            .join("\n"),
        None => String::new(),
    };

    if text.is_empty() { None } else { Some(text) }
}

fn beta_content_block_to_text(block: &BetaContentBlockParam) -> String {
    match block {
        BetaContentBlockParam::Text(block) => block.text.clone(),
        _ => "[unsupported_content_block]".to_string(),
    }
}
