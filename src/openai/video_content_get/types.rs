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
