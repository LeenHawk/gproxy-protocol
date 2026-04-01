use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::gemini::stream_generate_content::request::GeminiStreamGenerateContentRequest;
use crate::gemini::stream_generate_content::response::GeminiStreamGenerateContentResponse;
use crate::transform::utils::TransformError;

impl TryFrom<&GeminiGenerateContentRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiGenerateContentRequest) -> Result<Self, TransformError> {
        Ok(GeminiStreamGenerateContentRequest {
            method: value.method,
            path: crate::gemini::stream_generate_content::request::PathParameters {
                model: value.path.model.clone(),
            },
            query: crate::gemini::stream_generate_content::request::QueryParameters::default(),
            headers: crate::gemini::stream_generate_content::request::RequestHeaders::default(),
            body: value.body.clone(),
        })
    }
}

impl TryFrom<GeminiGenerateContentRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentRequest) -> Result<Self, TransformError> {
        GeminiStreamGenerateContentRequest::try_from(&value)
    }
}

impl TryFrom<GeminiGenerateContentResponse> for GeminiStreamGenerateContentResponse {
    type Error = TransformError;

    fn try_from(value: GeminiGenerateContentResponse) -> Result<Self, TransformError> {
        Ok(match value {
            GeminiGenerateContentResponse::Success {
                stats_code,
                headers,
                ..
            } => GeminiStreamGenerateContentResponse::Success {
                stats_code,
                headers,
            },
            GeminiGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            } => GeminiStreamGenerateContentResponse::Error {
                stats_code,
                headers,
                body,
            },
        })
    }
}
