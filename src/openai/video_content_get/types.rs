use serde::{Deserialize, Serialize};

pub use crate::openai::types::{HttpMethod, OpenAiApiErrorResponse, OpenAiResponseHeaders};

/// Downloadable video asset variants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoContentVariant {
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "thumbnail")]
    Thumbnail,
    #[serde(rename = "spritesheet")]
    Spritesheet,
}

/// Successful binary body for OpenAI `videos.content` endpoint.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiVideoContentBody {
    pub bytes: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn video_content_variant_roundtrip() {
        let payload = serde_json::json!("thumbnail");
        let decoded: OpenAiVideoContentVariant = serde_json::from_value(payload.clone()).unwrap();
        assert_eq!(decoded, OpenAiVideoContentVariant::Thumbnail);
        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded, payload);
    }

    #[test]
    fn video_content_body_roundtrip() {
        let payload = serde_json::json!({
            "bytes": [0, 1, 2, 255]
        });

        let decoded: OpenAiVideoContentBody = serde_json::from_value(payload.clone()).unwrap();
        assert_eq!(decoded.bytes, vec![0, 1, 2, 255]);
        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded, payload);
    }
}
