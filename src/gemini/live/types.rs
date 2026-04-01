use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub use crate::gemini::count_tokens::types::{
    GeminiBlob, GeminiContent, GeminiFunctionCall, GeminiFunctionResponse, GeminiGenerationConfig,
    GeminiModalityTokenCount, GeminiTool, HttpMethod,
};
pub use crate::gemini::generate_content::types::{
    GeminiGroundingMetadata, GeminiUrlContextMetadata,
};
pub use crate::gemini::types::{
    GeminiApiError, GeminiApiErrorResponse, GeminiResponseHeaders, JsonObject,
};

/// Union envelope for client frames in `BidiGenerateContent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiBidiGenerateContentClientMessage {
    /// Exactly one field should be set.
    #[serde(flatten)]
    pub message_type: GeminiBidiGenerateContentClientMessageType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GeminiBidiGenerateContentClientMessageType {
    Setup {
        setup: GeminiBidiGenerateContentSetup,
    },
    ClientContent {
        #[serde(rename = "clientContent")]
        client_content: GeminiBidiGenerateContentClientContent,
    },
    RealtimeInput {
        #[serde(rename = "realtimeInput")]
        realtime_input: GeminiBidiGenerateContentRealtimeInput,
    },
    ToolResponse {
        #[serde(rename = "toolResponse")]
        tool_response: GeminiBidiGenerateContentToolResponse,
    },
}

/// Message to configure one Live WebSocket session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentSetup {
    /// Required model resource id in format `models/{model}`.
    pub model: String,
    #[serde(
        rename = "generationConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generation_config: Option<GeminiGenerationConfig>,
    #[serde(
        rename = "systemInstruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_instruction: Option<GeminiContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
    #[serde(
        rename = "realtimeInputConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub realtime_input_config: Option<GeminiRealtimeInputConfig>,
    #[serde(
        rename = "sessionResumption",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub session_resumption: Option<GeminiSessionResumptionConfig>,
    #[serde(
        rename = "contextWindowCompression",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub context_window_compression: Option<GeminiContextWindowCompressionConfig>,
    #[serde(
        rename = "inputAudioTranscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_audio_transcription: Option<GeminiAudioTranscriptionConfig>,
    #[serde(
        rename = "outputAudioTranscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub output_audio_transcription: Option<GeminiAudioTranscriptionConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proactivity: Option<GeminiProactivityConfig>,
    #[serde(
        rename = "prefixTurns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_turns: Option<Vec<GeminiContent>>,
    #[serde(
        rename = "historyConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub history_config: Option<GeminiHistoryConfig>,
}

/// Incremental conversation content from client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentClientContent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turns: Option<Vec<GeminiContent>>,
    #[serde(
        rename = "turnComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub turn_complete: Option<bool>,
}

/// Realtime (audio/video/text) user input stream chunk.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentRealtimeInput {
    /// Deprecated in upstream docs; prefer `audio`/`video`/`text`.
    #[serde(
        rename = "mediaChunks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_chunks: Option<Vec<GeminiBlob>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<GeminiBlob>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<GeminiBlob>,
    #[serde(
        rename = "activityStart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_start: Option<GeminiActivityStart>,
    #[serde(
        rename = "activityEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_end: Option<GeminiActivityEnd>,
    #[serde(
        rename = "audioStreamEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_stream_end: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiActivityStart {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiActivityEnd {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentToolResponse {
    #[serde(
        rename = "functionResponses",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_responses: Option<Vec<GeminiFunctionResponse>>,
}

/// Union envelope for server frames in `BidiGenerateContent`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiBidiGenerateContentServerMessage {
    #[serde(
        rename = "usageMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub usage_metadata: Option<GeminiLiveUsageMetadata>,
    /// Exactly one field should be set.
    #[serde(flatten)]
    pub message_type: GeminiBidiGenerateContentServerMessageType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum GeminiBidiGenerateContentServerMessageType {
    SetupComplete {
        #[serde(rename = "setupComplete")]
        setup_complete: GeminiBidiGenerateContentSetupComplete,
    },
    ServerContent {
        #[serde(rename = "serverContent")]
        server_content: GeminiBidiGenerateContentServerContent,
    },
    ToolCall {
        #[serde(rename = "toolCall")]
        tool_call: GeminiBidiGenerateContentToolCall,
    },
    ToolCallCancellation {
        #[serde(rename = "toolCallCancellation")]
        tool_call_cancellation: GeminiBidiGenerateContentToolCallCancellation,
    },
    GoAway {
        #[serde(rename = "goAway")]
        go_away: GeminiGoAway,
    },
    SessionResumptionUpdate {
        #[serde(rename = "sessionResumptionUpdate")]
        session_resumption_update: GeminiSessionResumptionUpdate,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentSetupComplete {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentServerContent {
    #[serde(
        rename = "generationComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generation_complete: Option<bool>,
    #[serde(
        rename = "turnComplete",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub turn_complete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interrupted: Option<bool>,
    #[serde(
        rename = "groundingMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_metadata: Option<GeminiGroundingMetadata>,
    #[serde(
        rename = "inputTranscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub input_transcription: Option<GeminiBidiGenerateContentTranscription>,
    #[serde(
        rename = "outputTranscription",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub output_transcription: Option<GeminiBidiGenerateContentTranscription>,
    #[serde(
        rename = "urlContextMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_context_metadata: Option<GeminiUrlContextMetadata>,
    #[serde(rename = "modelTurn", default, skip_serializing_if = "Option::is_none")]
    pub model_turn: Option<GeminiContent>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiBidiGenerateContentTranscription {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentToolCall {
    #[serde(
        rename = "functionCalls",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_calls: Option<Vec<GeminiFunctionCall>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiBidiGenerateContentToolCallCancellation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiGoAway {
    /// Protobuf `Duration` JSON string (example: `"15s"`).
    #[serde(rename = "timeLeft")]
    pub time_left: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiLiveUsageMetadata {
    #[serde(
        rename = "promptTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prompt_token_count: Option<u64>,
    #[serde(
        rename = "cachedContentTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content_token_count: Option<u64>,
    #[serde(
        rename = "responseTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_token_count: Option<u64>,
    #[serde(
        rename = "toolUsePromptTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_use_prompt_token_count: Option<u64>,
    #[serde(
        rename = "thoughtsTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thoughts_token_count: Option<u64>,
    #[serde(
        rename = "totalTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_token_count: Option<u64>,
    #[serde(
        rename = "promptTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prompt_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
    #[serde(
        rename = "cacheTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
    #[serde(
        rename = "responseTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
    #[serde(
        rename = "toolUsePromptTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_use_prompt_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiHistoryConfig {
    #[serde(
        rename = "initialHistoryInClientContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub initial_history_in_client_content: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiAudioTranscriptionConfig {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiRealtimeInputConfig {
    #[serde(
        rename = "automaticActivityDetection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub automatic_activity_detection: Option<GeminiAutomaticActivityDetection>,
    #[serde(
        rename = "activityHandling",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_handling: Option<GeminiActivityHandling>,
    #[serde(
        rename = "turnCoverage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub turn_coverage: Option<GeminiTurnCoverage>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiAutomaticActivityDetection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(
        rename = "startOfSpeechSensitivity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_of_speech_sensitivity: Option<GeminiStartSensitivity>,
    #[serde(
        rename = "prefixPaddingMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prefix_padding_ms: Option<i32>,
    #[serde(
        rename = "endOfSpeechSensitivity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_of_speech_sensitivity: Option<GeminiEndSensitivity>,
    #[serde(
        rename = "silenceDurationMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub silence_duration_ms: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiActivityHandling {
    #[serde(rename = "ACTIVITY_HANDLING_UNSPECIFIED")]
    ActivityHandlingUnspecified,
    #[serde(rename = "START_OF_ACTIVITY_INTERRUPTS")]
    StartOfActivityInterrupts,
    #[serde(rename = "NO_INTERRUPTION")]
    NoInterruption,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiStartSensitivity {
    #[serde(rename = "START_SENSITIVITY_UNSPECIFIED")]
    StartSensitivityUnspecified,
    #[serde(rename = "START_SENSITIVITY_HIGH")]
    StartSensitivityHigh,
    #[serde(rename = "START_SENSITIVITY_LOW")]
    StartSensitivityLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiEndSensitivity {
    #[serde(rename = "END_SENSITIVITY_UNSPECIFIED")]
    EndSensitivityUnspecified,
    #[serde(rename = "END_SENSITIVITY_HIGH")]
    EndSensitivityHigh,
    #[serde(rename = "END_SENSITIVITY_LOW")]
    EndSensitivityLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiTurnCoverage {
    #[serde(rename = "TURN_COVERAGE_UNSPECIFIED")]
    TurnCoverageUnspecified,
    #[serde(rename = "TURN_INCLUDES_ONLY_ACTIVITY")]
    TurnIncludesOnlyActivity,
    #[serde(rename = "TURN_INCLUDES_ALL_INPUT")]
    TurnIncludesAllInput,
    #[serde(rename = "TURN_INCLUDES_AUDIO_ACTIVITY_AND_ALL_VIDEO")]
    TurnIncludesAudioActivityAndAllVideo,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiSessionResumptionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiSessionResumptionUpdate {
    #[serde(rename = "newHandle", default, skip_serializing_if = "Option::is_none")]
    pub new_handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resumable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiContextWindowCompressionConfig {
    #[serde(
        rename = "slidingWindow",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sliding_window: Option<GeminiSlidingWindow>,
    #[serde(
        rename = "triggerTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_tokens: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiSlidingWindow {
    #[serde(
        rename = "targetTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub target_tokens: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiProactivityConfig {
    #[serde(
        rename = "proactiveAudio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub proactive_audio: Option<bool>,
}

/// Request for `AuthTokenService.CreateToken`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiCreateAuthTokenRequest {
    #[serde(rename = "authToken")]
    pub auth_token: GeminiAuthToken,
}

/// Ephemeral authentication token configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiAuthToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "expireTime",
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub expire_time: Option<OffsetDateTime>,
    #[serde(
        rename = "newSessionExpireTime",
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub new_session_expire_time: Option<OffsetDateTime>,
    /// Protobuf `FieldMask` JSON string.
    #[serde(rename = "fieldMask", default, skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
    #[serde(
        rename = "bidiGenerateContentSetup",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub bidi_generate_content_setup: Option<GeminiBidiGenerateContentSetup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uses: Option<i32>,
}
