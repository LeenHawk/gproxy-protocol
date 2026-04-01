use serde::{Deserialize, Serialize};

pub use crate::gemini::count_tokens::types::{
    GeminiApiError, GeminiApiErrorResponse, GeminiBlob, GeminiFileData, GeminiResponseHeaders,
    HttpMethod,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVideoGenerationAsset {
    #[serde(
        rename = "inlineData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_data: Option<GeminiBlob>,
    #[serde(rename = "fileData", default, skip_serializing_if = "Option::is_none")]
    pub file_data: Option<GeminiFileData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVideoReferenceImage {
    pub image: GeminiVideoGenerationAsset,
    #[serde(
        rename = "referenceType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerateVideosInstance {
    pub prompt: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<GeminiVideoGenerationAsset>,
    #[serde(rename = "lastFrame", default, skip_serializing_if = "Option::is_none")]
    pub last_frame: Option<GeminiVideoGenerationAsset>,
    #[serde(
        rename = "referenceImages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reference_images: Option<Vec<GeminiVideoReferenceImage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<GeminiVideoGenerationAsset>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiVideoPersonGeneration {
    #[serde(rename = "allow_all")]
    AllowAll,
    #[serde(rename = "allow_adult")]
    AllowAdult,
    #[serde(rename = "dont_allow")]
    DontAllow,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerateVideosParameters {
    #[serde(
        rename = "aspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[serde(
        rename = "durationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_seconds: Option<String>,
    #[serde(
        rename = "personGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub person_generation: Option<GeminiVideoPersonGeneration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(
        rename = "numberOfVideos",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_videos: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVideoFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGeneratedVideoSample {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<GeminiVideoFile>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerateVideoResponse {
    #[serde(
        rename = "generatedSamples",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generated_samples: Option<Vec<GeminiGeneratedVideoSample>>,
    #[serde(
        rename = "generatedVideos",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generated_videos: Option<Vec<GeminiGeneratedVideoSample>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerateVideosOperationResult {
    #[serde(
        rename = "generateVideoResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generate_video_response: Option<GeminiGenerateVideoResponse>,
    #[serde(
        rename = "generatedVideos",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generated_videos: Option<Vec<GeminiGeneratedVideoSample>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVideoOperationMetadata {
    #[serde(
        rename = "createTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub create_time: Option<String>,
    #[serde(
        rename = "updateTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub update_time: Option<String>,
    #[serde(rename = "endTime", default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(
        rename = "aspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[serde(
        rename = "durationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_seconds: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    #[serde(
        rename = "progressPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub progress_percent: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}
