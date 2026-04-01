use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_video::types::{
    HttpMethod, OpenAiVideoImageReference, OpenAiVideoModel, OpenAiVideoSeconds, OpenAiVideoSize,
};

/// Request descriptor for OpenAI `videos.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateVideoRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for OpenAiCreateVideoRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

/// OpenAI `/videos` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// OpenAI `/videos` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Request body for OpenAI `/videos`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Text prompt that describes the video to generate.
    pub prompt: String,
    /// Optional JSON-safe image reference that guides generation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<OpenAiVideoImageReference>,
    /// Optional multipart reference asset that guides generation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_reference: Option<String>,
    /// Video generation model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<OpenAiVideoModel>,
    /// Clip duration in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<OpenAiVideoSeconds>,
    /// Output resolution.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<OpenAiVideoSize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_body_supports_documented_video_fields() {
        let payload = serde_json::json!({
            "prompt": "A tiny robot walking in the snow",
            "image_reference": {
                "image_url": "data:image/png;base64,abc"
            },
            "input_reference": "file_ref_123",
            "model": "sora-2-pro",
            "seconds": "8",
            "size": "1280x720"
        });

        let decoded: RequestBody = serde_json::from_value(payload).unwrap();
        assert_eq!(decoded.prompt, "A tiny robot walking in the snow");
        assert_eq!(
            decoded.image_reference.unwrap().image_url.as_deref(),
            Some("data:image/png;base64,abc")
        );
        assert_eq!(decoded.input_reference.as_deref(), Some("file_ref_123"));
        assert_eq!(decoded.seconds, Some(OpenAiVideoSeconds::S8));
        assert_eq!(decoded.size, Some(OpenAiVideoSize::S1280x720));
    }
}
