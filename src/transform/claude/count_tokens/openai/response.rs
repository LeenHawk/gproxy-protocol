use crate::claude::count_tokens::response::ClaudeCountTokensResponse;
use crate::claude::count_tokens::types::BetaMessageTokensCount;
use crate::claude::types::ClaudeResponseHeaders;
use crate::openai::count_tokens::response::OpenAiCountTokensResponse;
use crate::transform::claude::utils::beta_error_response_from_status_message;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCountTokensResponse> for ClaudeCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let input_tokens = body.input_tokens;
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
            OpenAiCountTokensResponse::Error {
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
