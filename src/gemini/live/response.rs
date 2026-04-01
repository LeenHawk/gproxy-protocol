use serde::{Deserialize, Serialize};

use crate::gemini::live::types::{
    GeminiApiErrorResponse, GeminiAuthToken, GeminiBidiGenerateContentServerMessage,
};

/// Parsed Live WebSocket frame from Gemini.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GeminiLiveMessageResponse {
    /// Regular server message frame.
    Message(GeminiBidiGenerateContentServerMessage),
    /// Google API style error envelope.
    Error(GeminiApiErrorResponse),
}

/// Successful body for `AuthTokenService.CreateToken`.
pub type GeminiCreateAuthTokenResponse = GeminiAuthToken;
