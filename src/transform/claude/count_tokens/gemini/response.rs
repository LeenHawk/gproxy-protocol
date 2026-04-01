use crate::claude::count_tokens::response::ClaudeCountTokensResponse;
use crate::claude::count_tokens::types::BetaMessageTokensCount;
use crate::claude::types::ClaudeResponseHeaders;
use crate::gemini::count_tokens::response::GeminiCountTokensResponse;
use crate::transform::claude::utils::beta_error_response_from_status_message;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiCountTokensResponse> for ClaudeCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: GeminiCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let input_tokens = body.total_tokens;

                ClaudeCountTokensResponse::Success {
                    stats_code,
                    headers: ClaudeResponseHeaders {
                        extra: headers.extra,
                    },
                    body: BetaMessageTokensCount {
                        context_management: None,
                        input_tokens,
                    },
                }
            }
            GeminiCountTokensResponse::Error {
                stats_code,
                headers,
                body,
            } => ClaudeCountTokensResponse::Error {
                stats_code,
                headers: ClaudeResponseHeaders {
                    extra: headers.extra,
                },
                body: beta_error_response_from_status_message(stats_code, body.error.message),
            },
        })
    }
}
