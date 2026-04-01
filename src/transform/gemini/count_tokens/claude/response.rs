use crate::claude::count_tokens::response::ClaudeCountTokensResponse;
use crate::gemini::count_tokens::response::{GeminiCountTokensResponse, ResponseBody};
use crate::gemini::count_tokens::types::{GeminiModality, GeminiModalityTokenCount};
use crate::gemini::types::GeminiResponseHeaders;
use crate::transform::gemini::count_tokens::claude::utils::gemini_error_response_from_claude;
use crate::transform::utils::TransformError;

impl TryFrom<ClaudeCountTokensResponse> for GeminiCountTokensResponse {
    type Error = TransformError;

    fn try_from(value: ClaudeCountTokensResponse) -> Result<Self, TransformError> {
        Ok(match value {
            ClaudeCountTokensResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let total_tokens = body.input_tokens;

                GeminiCountTokensResponse::Success {
                    stats_code,
                    headers: GeminiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ResponseBody {
                        total_tokens,
                        cached_content_token_count: None,
                        prompt_tokens_details: Some(vec![GeminiModalityTokenCount {
                            modality: GeminiModality::Text,
                            token_count: total_tokens,
                        }]),
                        cache_tokens_details: None,
                    },
                }
            }
            ClaudeCountTokensResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiCountTokensResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: gemini_error_response_from_claude(stats_code, body),
            },
        })
    }
}
