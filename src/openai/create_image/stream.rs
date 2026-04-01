use serde::{Deserialize, Serialize};

use crate::openai::create_image::types::{
    OpenAiApiError, OpenAiImageBackground, OpenAiImageOutputFormat, OpenAiImageQuality,
    OpenAiImageSize, OpenAiImageUsage,
};

/// Parsed SSE stream body for `POST /images/generations` with `stream=true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateImageSseStreamBody {
    /// SSE events in receive order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<OpenAiCreateImageSseEvent>,
}

/// A single SSE event frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateImageSseEvent {
    /// Optional SSE `event` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// SSE `data` field payload.
    pub data: OpenAiCreateImageSseData,
}

/// SSE `data` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenAiCreateImageSseData {
    /// A regular stream event object.
    Event(ImageGenerationStreamEvent),
    /// Stream end marker (`[DONE]`).
    Done(String),
}

impl OpenAiCreateImageSseData {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done(marker) if marker == "[DONE]")
    }
}

/// Stream event union documented by OpenAI image generation streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ImageGenerationStreamEvent {
    #[serde(rename = "image_generation.partial_image")]
    PartialImage {
        b64_json: String,
        background: OpenAiImageBackground,
        created_at: u64,
        output_format: OpenAiImageOutputFormat,
        partial_image_index: u32,
        quality: OpenAiImageQuality,
        size: OpenAiImageSize,
    },
    #[serde(rename = "image_generation.completed")]
    Completed {
        b64_json: String,
        background: OpenAiImageBackground,
        created_at: u64,
        output_format: OpenAiImageOutputFormat,
        quality: OpenAiImageQuality,
        size: OpenAiImageSize,
        usage: OpenAiImageUsage,
    },
    #[serde(rename = "error")]
    Error { error: OpenAiApiError },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_image_event_roundtrip() {
        let payload = serde_json::json!({
            "type": "image_generation.partial_image",
            "b64_json": "abc",
            "background": "transparent",
            "created_at": 1741383474,
            "output_format": "png",
            "partial_image_index": 0,
            "quality": "high",
            "size": "1024x1024"
        });

        let event: ImageGenerationStreamEvent = serde_json::from_value(payload.clone()).unwrap();
        let encoded = serde_json::to_value(event).unwrap();
        assert_eq!(encoded, payload);
    }

    #[test]
    fn completed_image_event_roundtrip() {
        let payload = serde_json::json!({
            "type": "image_generation.completed",
            "b64_json": "abc",
            "background": "opaque",
            "created_at": 1741383474,
            "output_format": "png",
            "quality": "high",
            "size": "1024x1024",
            "usage": {
                "total_tokens": 314,
                "input_tokens": 271,
                "output_tokens": 43,
                "input_tokens_details": {
                    "text_tokens": 34,
                    "image_tokens": 237
                }
            }
        });

        let event: ImageGenerationStreamEvent = serde_json::from_value(payload.clone()).unwrap();
        let encoded = serde_json::to_value(event).unwrap();
        assert_eq!(encoded, payload);
    }

    #[test]
    fn sse_done_marker_is_detected() {
        assert!(OpenAiCreateImageSseData::Done("[DONE]".to_string()).is_done());
        assert!(!OpenAiCreateImageSseData::Done("done".to_string()).is_done());
    }
}
