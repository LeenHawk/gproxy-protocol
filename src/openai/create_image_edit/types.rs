use serde::{Deserialize, Serialize};

pub use crate::openai::create_image::types::{
    OpenAiGeneratedImage, OpenAiGeneratedImageBackground, OpenAiGeneratedImageQuality,
    OpenAiGeneratedImageSize, OpenAiImageBackground, OpenAiImageModeration,
    OpenAiImageOutputFormat, OpenAiImageTokenDetails, OpenAiImageUsage,
};
pub use crate::openai::types::{
    HttpMethod, OpenAiApiError, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Successful response payload for OpenAI `/images/edits`.
pub type OpenAiCreateImageEditResponseBody =
    crate::openai::create_image::types::OpenAiCreateImageResponseBody;

/// Reference to an input image by uploaded file id or image URL.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiImageEditInputImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Input fidelity for image edit requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageEditInputFidelity {
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
}

/// Supported image editing model identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiImageEditModel {
    Known(OpenAiImageEditModelKnown),
    Custom(String),
}

/// Known model constants documented for OpenAI image editing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageEditModelKnown {
    #[serde(rename = "gpt-image-1.5")]
    GptImage15,
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
    #[serde(rename = "chatgpt-image-latest")]
    ChatgptImageLatest,
}

/// Requested quality for image edit output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageEditQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
}

/// Requested output image size for image edits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageEditSize {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1536x1024")]
    S1536x1024,
    #[serde(rename = "1024x1536")]
    S1024x1536,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_edit_model_supports_known_and_custom_values() {
        let known: OpenAiImageEditModel = serde_json::from_str("\"chatgpt-image-latest\"").unwrap();
        assert_eq!(
            known,
            OpenAiImageEditModel::Known(OpenAiImageEditModelKnown::ChatgptImageLatest)
        );

        let custom: OpenAiImageEditModel = serde_json::from_str("\"dall-e-2\"").unwrap();
        assert_eq!(custom, OpenAiImageEditModel::Custom("dall-e-2".to_string()));
    }

    #[test]
    fn image_edit_input_image_roundtrip() {
        let payload = serde_json::json!({
            "file_id": "file_123",
            "image_url": "data:image/png;base64,abc"
        });

        let decoded: OpenAiImageEditInputImage = serde_json::from_value(payload.clone()).unwrap();
        assert_eq!(decoded.file_id.as_deref(), Some("file_123"));
        assert_eq!(
            decoded.image_url.as_deref(),
            Some("data:image/png;base64,abc")
        );

        let encoded = serde_json::to_value(decoded).unwrap();
        assert_eq!(encoded, payload);
    }
}
