use serde::{Deserialize, Serialize};

pub use crate::openai::types::{
    HttpMethod, OpenAiApiError, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Supported image generation model identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiImageModel {
    Known(OpenAiImageModelKnown),
    Custom(String),
}

/// Known model constants documented for OpenAI image generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageModelKnown {
    #[serde(rename = "gpt-image-1.5")]
    GptImage15,
    #[serde(rename = "dall-e-2")]
    DallE2,
    #[serde(rename = "dall-e-3")]
    DallE3,
    #[serde(rename = "gpt-image-1")]
    GptImage1,
    #[serde(rename = "gpt-image-1-mini")]
    GptImage1Mini,
}

/// Background configuration for image generation requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "auto")]
    Auto,
}

/// Moderation level for GPT image models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageModeration {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "auto")]
    Auto,
}

/// Output image format for GPT image models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageOutputFormat {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
}

/// Requested image quality.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageQuality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hd")]
    Hd,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
}

/// Legacy response format for DALL·E models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageResponseFormat {
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}

/// Requested output image size.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageSize {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1536x1024")]
    S1536x1024,
    #[serde(rename = "1024x1536")]
    S1024x1536,
    #[serde(rename = "256x256")]
    S256x256,
    #[serde(rename = "512x512")]
    S512x512,
    #[serde(rename = "1792x1024")]
    S1792x1024,
    #[serde(rename = "1024x1792")]
    S1024x1792,
}

/// Style control for DALL·E 3.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiImageStyle {
    #[serde(rename = "vivid")]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}

/// Background value echoed in image generation responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiGeneratedImageBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
}

/// Output quality echoed in successful image generation responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiGeneratedImageQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

/// Output size echoed in successful image generation responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiGeneratedImageSize {
    #[serde(rename = "1024x1024")]
    S1024x1024,
    #[serde(rename = "1024x1536")]
    S1024x1536,
    #[serde(rename = "1536x1024")]
    S1536x1024,
}

/// Token counts by media type for image generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiImageTokenDetails {
    pub image_tokens: u64,
    pub text_tokens: u64,
}

/// Usage metrics returned by GPT image models.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiImageUsage {
    pub input_tokens: u64,
    pub input_tokens_details: OpenAiImageTokenDetails,
    pub output_tokens: u64,
    pub total_tokens: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<OpenAiImageTokenDetails>,
}

/// A single generated image payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiGeneratedImage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub b64_json: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revised_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Successful response payload for OpenAI `/images/generations`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateImageResponseBody {
    pub created: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<OpenAiGeneratedImageBackground>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<OpenAiGeneratedImage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OpenAiImageOutputFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<OpenAiGeneratedImageQuality>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<OpenAiGeneratedImageSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<OpenAiImageUsage>,
}
