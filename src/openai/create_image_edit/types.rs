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
