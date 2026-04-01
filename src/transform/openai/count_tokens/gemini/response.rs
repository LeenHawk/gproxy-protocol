use crate::gemini::count_tokens::response::GeminiCountTokensResponse;
use crate::openai::count_tokens::response::{
    OpenAiCountTokensObject, OpenAiCountTokensResponse, ResponseBody,
};
use crate::openai::types::OpenAiResponseHeaders;
use crate::transform::openai::model_list::gemini::utils::openai_error_response_from_gemini;
use crate::transform::utils::TransformError;

impl TryFrom<GeminiCountTokensResponse> for OpenAiCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: GeminiCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => OpenAiCountTokensResponse::Success {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: ResponseBody {
                    input_tokens: body.total_tokens,
                    object: OpenAiCountTokensObject::ResponseInputTokens,
                },
            },
            GeminiCountTokensResponse::Error {
                stats_code,
                headers,
                body,
            } => OpenAiCountTokensResponse::Error {
                stats_code,
                headers: OpenAiResponseHeaders {
                    extra: headers.extra,
                },
                body: openai_error_response_from_gemini(stats_code, body),
            },
        })
    }
}
