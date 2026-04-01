use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_image::types::{
    HttpMethod, OpenAiImageBackground, OpenAiImageModel, OpenAiImageModeration,
    OpenAiImageOutputFormat, OpenAiImageQuality, OpenAiImageResponseFormat, OpenAiImageSize,
    OpenAiImageStyle,
};

/// Request descriptor for OpenAI `images.generate` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateImageRequest {
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

impl Default for OpenAiCreateImageRequest {
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

/// OpenAI `/images/generations` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// OpenAI `/images/generations` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Request body for OpenAI `/images/generations`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Text description of the desired image output.
    pub prompt: String,
    /// Background behavior for supported GPT image models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<OpenAiImageBackground>,
    /// Model identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<OpenAiImageModel>,
    /// Moderation level for GPT image models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation: Option<OpenAiImageModeration>,
    /// Number of images to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Compression percentage for `jpeg` or `webp` output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_compression: Option<u8>,
    /// Output encoding format for GPT image models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_format: Option<OpenAiImageOutputFormat>,
    /// Number of partial images requested in streaming mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_images: Option<u32>,
    /// Quality selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<OpenAiImageQuality>,
    /// Legacy DALL·E image return format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<OpenAiImageResponseFormat>,
    /// Requested output dimensions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<OpenAiImageSize>,
    /// Whether to stream partial image events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// DALL·E 3 style control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<OpenAiImageStyle>,
    /// Stable end-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}