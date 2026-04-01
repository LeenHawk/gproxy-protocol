use crate::claude::count_tokens::response::ClaudeCountTokensResponse;
use crate::openai::count_tokens::response::{
    OpenAiCountTokensObject, OpenAiCountTokensResponse, ResponseBody,
};
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_list::claude::utils::openai_error_response_from_claude;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeCountTokensResponse> for OpenAiCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let input_tokens = body.input_tokens;
                OpenAiCountTokensResponse::Success {
                    stats_code,
                    headers: OpenAiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ResponseBody {
                        input_tokens,
                        object: OpenAiCountTokensObject::ResponseInputTokens,
                    },
                }
            }
            ClaudeCountTokensResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCountTokensResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_claude(stats_code, body),
            },
        })
    }
}
