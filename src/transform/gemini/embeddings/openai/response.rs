use crate::gemini::embeddings::response::{GeminiEmbedContentResponse, ResponseBody};
use crate::gemini::embeddings::types::GeminiContentEmbedding;
use crate::gemini::types::{GeminiApiError, GeminiApiErrorResponse, GeminiResponseHeaders};
use crate::openai::embeddings::response::OpenAiEmbeddingsResponse;
use crate::openai::embeddings::types::OpenAiEmbeddingVector;
use crate::transform::utils::TransformError;

fn google_status_from_http(status_code: http::StatusCode) -> Option<String> {
    let status = match status_code.as_u16() {
        400 => "INVALID_ARGUMENT",
        401 => "UNAUTHENTICATED",
        403 => "PERMISSION_DENIED",
        404 => "NOT_FOUND",
        409 => "ABORTED",
        429 => "RESOURCE_EXHAUSTED",
        500 => "INTERNAL",
        503 | 529 => "UNAVAILABLE",
        504 => "DEADLINE_EXCEEDED",
        _ => return None,
    };
    Some(status.to_string())
}

impl TryFrom<OpenAiEmbeddingsResponse> for GeminiEmbedContentResponse {
    type Error = TransformError;

    fn try_from(value: OpenAiEmbeddingsResponse) -> Result<Self, TransformError> {
        Ok(match value {
            OpenAiEmbeddingsResponse::Success {
                stats_code,
                headers,
                body,
            } => {
                let values = body
                    .data
                    .into_iter()
                    .next()
                    .map(|item| match item.embedding {
                        OpenAiEmbeddingVector::FloatArray(values) => values,
                        OpenAiEmbeddingVector::Base64(_) => Vec::new(),
                    })
                    .unwrap_or_default();

                GeminiEmbedContentResponse::Success {
                    stats_code,
                    headers: GeminiResponseHeaders {
                        extra: headers.extra,
                    },
                    body: ResponseBody {
                        embedding: GeminiContentEmbedding {
                            values,
                            shape: None,
                        },
                    },
                }
            }
            OpenAiEmbeddingsResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiEmbedContentResponse::Error {
                stats_code,
                headers: GeminiResponseHeaders {
                    extra: headers.extra,
                },
                body: GeminiApiErrorResponse {
                    error: GeminiApiError {
                        code: i32::from(stats_code.as_u16()),
                        message: body.error.message,
                        status: google_status_from_http(stats_code),
                        details: None,
                    },
                },
            },
        })
    }
}
