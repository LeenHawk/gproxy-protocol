use serde::{Deserialize, Serialize};

pub use crate::openai::types::{HttpMethod, OpenAiApiErrorResponse, OpenAiResponseHeaders};

/// JSON-safe image reference for video generation guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OpenAiVideoImageReference {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Supported video generation model identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiVideoModel {
    Known(OpenAiVideoModelKnown),
    Custom(String),
}

/// Known model constants documented for OpenAI video generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoModelKnown {
    #[serde(rename = "sora-2")]
    Sora2,
    #[serde(rename = "sora-2-pro")]
    Sora2Pro,
    #[serde(rename = "sora-2-2025-10-06")]
    Sora220251006,
    #[serde(rename = "sora-2-pro-2025-10-06")]
    Sora2Pro20251006,
    #[serde(rename = "sora-2-2025-12-08")]
    Sora220251208,
}

/// Allowed request clip durations for video generation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoSeconds {
    #[serde(rename = "4")]
    S4,
    #[serde(rename = "8")]
    S8,
    #[serde(rename = "12")]
    S12,
}

/// Supported video generation output sizes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoSize {
    #[serde(rename = "720x1280")]
    S720x1280,
    #[serde(rename = "1280x720")]
    S1280x720,
    #[serde(rename = "1024x1792")]
    S1024x1792,
    #[serde(rename = "1792x1024")]
    S1792x1024,
}

/// Error details returned for failed video generation jobs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenAiVideoCreateError {
    pub code: String,
    pub message: String,
}

/// Object discriminator for OpenAI video resources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoObject {
    #[serde(rename = "video")]
    Video,
}

/// Lifecycle states for a generated video job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpenAiVideoStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}

/// Structured information describing a generated video job.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiVideo {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u64>,
    pub created_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OpenAiVideoCreateError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    pub model: OpenAiVideoModel,
    pub object: OpenAiVideoObject,
    pub progress: f64,
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remixed_from_video_id: Option<String>,
    pub seconds: String,
    pub size: OpenAiVideoSize,
    pub status: OpenAiVideoStatus,
}
