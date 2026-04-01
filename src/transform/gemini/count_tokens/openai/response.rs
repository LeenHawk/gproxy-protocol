use crate::gemini::count_tokens::response::{GeminiCountTokensResponse, ResponseBody};
use crate::gemini::count_tokens::types::{GeminiModality, GeminiModalityTokenCount};
use crate::gemini::types::GeminiResponseHeaders;
use crate::openai::count_tokens::response::OpenAiCountTokensResponse;
use crate::transform::gemini::count_tokens::openai::utils::gemini_error_response_from_openai;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCountTokensResponse> for GeminiCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => GeminiCountTokensResponse::Success {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: ResponseBody {
                    total_tokens: body.input_tokens,
                    cached_content_token_count: None,
                    prompt_tokens_details: Some(vec![GeminiModalityTokenCount {
                        modality: GeminiModality::Text,
                        token_count: body.input_tokens,
                    }]),
                    cache_tokens_details: None,
                },
            },
            OpenAiCountTokensResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiCountTokensResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_openai(stats_code, body),
            },
        })
    }
}
