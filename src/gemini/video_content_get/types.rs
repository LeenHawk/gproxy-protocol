use serde::{Deserialize, Serialize};

pub use crate::gemini::types::{GeminiApiErrorResponse, GeminiResponseHeaders, HttpMethod};

/// Proxy-side helper request for downloading Veo generated video bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiVideoContentVariant {
    #[serde(rename = "video")]
    Video,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiVideoContentBody {
    pub bytes: Vec<u8>,
}
