use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_image_edit::types::{
    HttpMethod, OpenAiImageBackground, OpenAiImageEditInputFidelity, OpenAiImageEditInputImage,
    OpenAiImageEditModel, OpenAiImageEditQuality, OpenAiImageEditSize, OpenAiImageModeration,
    OpenAiImageOutputFormat,
};

/// Request descriptor for OpenAI `images.edit` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateImageEditRequest {
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

impl Default for OpenAiCreateImageEditRequest {
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

/// OpenAI `/images/edits` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// OpenAI `/images/edits` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Request body for OpenAI `/images/edits`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// Input images to edit or extend.
    pub images: Vec<OpenAiImageEditInputImage>,
    /// Text description of the desired edit.
    pub prompt: String,
    /// Background behavior for generated image output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<OpenAiImageBackground>,
    /// Fidelity to the original input images.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_fidelity: Option<OpenAiImageEditInputFidelity>,
    /// Optional mask reference. Upstream requires exactly one of `file_id` or `image_url`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mask: Option<OpenAiImageEditInputImage>,
    /// Model identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<OpenAiImageEditModel>,
    /// Moderation level for GPT image models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation: Option<OpenAiImageModeration>,
    /// Number of edited images to generate.
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
    /// Requested output quality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quality: Option<OpenAiImageEditQuality>,
    /// Requested output dimensions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<OpenAiImageEditSize>,
    /// Whether to stream partial image events.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Stable end-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
