use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;

pub use crate::gemini::types::{
    GeminiApiError, GeminiApiErrorResponse, GeminiResponseHeaders, HttpMethod, JsonObject,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiModalityTokenCount {
    pub modality: GeminiModality,
    #[serde(rename = "tokenCount")]
    pub token_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiModality {
    #[serde(rename = "MODALITY_UNSPECIFIED")]
    ModalityUnspecified,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "AUDIO")]
    Audio,
    #[serde(rename = "DOCUMENT")]
    Document,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiContent {
    /// Ordered parts that constitute one message.
    pub parts: Vec<GeminiPart>,
    /// Producer role (`user` or `model`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<GeminiContentRole>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiContentRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "model")]
    Model,
}

/// A single part in a content message.
///
/// `Part.data` is a union in upstream schema; only one data field should be set.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiPart {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thought: Option<bool>,
    #[serde(
        rename = "thoughtSignature",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thought_signature: Option<String>,
    #[serde(
        rename = "partMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub part_metadata: Option<JsonObject>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(
        rename = "inlineData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_data: Option<GeminiBlob>,
    #[serde(
        rename = "functionCall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_call: Option<GeminiFunctionCall>,
    #[serde(
        rename = "functionResponse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_response: Option<GeminiFunctionResponse>,
    #[serde(rename = "fileData", default, skip_serializing_if = "Option::is_none")]
    pub file_data: Option<GeminiFileData>,
    #[serde(
        rename = "executableCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub executable_code: Option<GeminiExecutableCode>,
    #[serde(
        rename = "codeExecutionResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_execution_result: Option<GeminiCodeExecutionResult>,

    #[serde(
        rename = "videoMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub video_metadata: Option<GeminiVideoMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiBlob {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiFunctionCall {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiFunctionResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
    pub response: JsonObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<GeminiFunctionResponsePart>>,
    #[serde(
        rename = "willContinue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub will_continue: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduling: Option<GeminiScheduling>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiFunctionResponsePart {
    #[serde(
        rename = "inlineData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inline_data: Option<GeminiFunctionResponseBlob>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiFunctionResponseBlob {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiScheduling {
    #[serde(rename = "SCHEDULING_UNSPECIFIED")]
    SchedulingUnspecified,
    #[serde(rename = "SILENT")]
    Silent,
    #[serde(rename = "WHEN_IDLE")]
    WhenIdle,
    #[serde(rename = "INTERRUPT")]
    Interrupt,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiFileData {
    #[serde(rename = "mimeType", default, skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(rename = "fileUri")]
    pub file_uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiExecutableCode {
    pub language: GeminiLanguage,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiLanguage {
    #[serde(rename = "LANGUAGE_UNSPECIFIED")]
    LanguageUnspecified,
    #[serde(rename = "PYTHON")]
    Python,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiCodeExecutionResult {
    pub outcome: GeminiOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiOutcome {
    #[serde(rename = "OUTCOME_UNSPECIFIED")]
    OutcomeUnspecified,
    #[serde(rename = "OUTCOME_OK")]
    OutcomeOk,
    #[serde(rename = "OUTCOME_FAILED")]
    OutcomeFailed,
    #[serde(rename = "OUTCOME_DEADLINE_EXCEEDED")]
    OutcomeDeadlineExceeded,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVideoMetadata {
    #[serde(
        rename = "startOffset",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_offset: Option<String>,
    #[serde(rename = "endOffset", default, skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
}

/// Full GenerateContentRequest object accepted by `countTokens.generateContentRequest`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerateContentRequest {
    pub model: String,
    pub contents: Vec<GeminiContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<GeminiTool>>,
    #[serde(
        rename = "toolConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_config: Option<GeminiToolConfig>,
    #[serde(
        rename = "safetySettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub safety_settings: Option<Vec<GeminiSafetySetting>>,
    #[serde(
        rename = "systemInstruction",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub system_instruction: Option<GeminiContent>,
    #[serde(
        rename = "generationConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub generation_config: Option<GeminiGenerationConfig>,
    #[serde(
        rename = "cachedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiTool {
    #[serde(
        rename = "functionDeclarations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_declarations: Option<Vec<GeminiFunctionDeclaration>>,
    #[serde(
        rename = "googleSearchRetrieval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub google_search_retrieval: Option<GeminiGoogleSearchRetrieval>,
    #[serde(
        rename = "codeExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub code_execution: Option<GeminiCodeExecution>,
    #[serde(
        rename = "googleSearch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub google_search: Option<GeminiGoogleSearch>,
    #[serde(
        rename = "computerUse",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub computer_use: Option<GeminiComputerUse>,
    #[serde(
        rename = "urlContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_context: Option<GeminiUrlContext>,
    #[serde(
        rename = "fileSearch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_search: Option<GeminiFileSearch>,
    #[serde(
        rename = "googleMaps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub google_maps: Option<GeminiGoogleMaps>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiFunctionDeclaration {
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub behavior: Option<GeminiFunctionBehavior>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<GeminiSchema>,
    #[serde(
        rename = "parametersJsonSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parameters_json_schema: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<GeminiSchema>,
    #[serde(
        rename = "responseJsonSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_json_schema: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiFunctionBehavior {
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "BLOCKING")]
    Blocking,
    #[serde(rename = "NON_BLOCKING")]
    NonBlocking,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiSchema {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<GeminiSchemaType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<String>>,
    #[serde(rename = "maxItems", default, skip_serializing_if = "Option::is_none")]
    pub max_items: Option<String>,
    #[serde(rename = "minItems", default, skip_serializing_if = "Option::is_none")]
    pub min_items: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<BTreeMap<String, GeminiSchema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(
        rename = "minProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_properties: Option<String>,
    #[serde(
        rename = "maxProperties",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_properties: Option<String>,
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<String>,
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<Value>,
    #[serde(rename = "anyOf", default, skip_serializing_if = "Option::is_none")]
    pub any_of: Option<Vec<GeminiSchema>>,
    #[serde(
        rename = "propertyOrdering",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub property_ordering: Option<Vec<String>>,
    #[serde(rename = "default", default, skip_serializing_if = "Option::is_none")]
    pub default_value: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<GeminiSchema>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiSchemaType {
    #[serde(rename = "TYPE_UNSPECIFIED")]
    TypeUnspecified,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "NUMBER")]
    Number,
    #[serde(rename = "INTEGER")]
    Integer,
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "ARRAY")]
    Array,
    #[serde(rename = "OBJECT")]
    Object,
    #[serde(rename = "NULL")]
    Null,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGoogleSearchRetrieval {
    #[serde(
        rename = "dynamicRetrievalConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_retrieval_config: Option<GeminiDynamicRetrievalConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiDynamicRetrievalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GeminiDynamicRetrievalMode>,
    #[serde(
        rename = "dynamicThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dynamic_threshold: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiDynamicRetrievalMode {
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,
    #[serde(rename = "MODE_DYNAMIC")]
    ModeDynamic,
}

/// Code-execution tool configuration (empty object).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiCodeExecution {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGoogleSearch {
    #[serde(
        rename = "timeRangeFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub time_range_filter: Option<GeminiInterval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiInterval {
    #[serde(
        rename = "startTime",
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub start_time: Option<OffsetDateTime>,
    #[serde(
        rename = "endTime",
        default,
        with = "time::serde::rfc3339::option",
        skip_serializing_if = "Option::is_none"
    )]
    pub end_time: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiComputerUse {
    pub environment: GeminiEnvironment,
    #[serde(
        rename = "excludedPredefinedFunctions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub excluded_predefined_functions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiEnvironment {
    #[serde(rename = "ENVIRONMENT_UNSPECIFIED")]
    EnvironmentUnspecified,
    #[serde(rename = "ENVIRONMENT_BROWSER")]
    EnvironmentBrowser,
}

/// URL-context tool configuration (empty object).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiUrlContext {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiFileSearch {
    #[serde(
        rename = "fileSearchStoreNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_search_store_names: Vec<String>,
    #[serde(
        rename = "metadataFilter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_filter: Option<String>,
    #[serde(rename = "topK", default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiGoogleMaps {
    #[serde(
        rename = "enableWidget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_widget: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiToolConfig {
    #[serde(
        rename = "functionCallingConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_calling_config: Option<GeminiFunctionCallingConfig>,
    #[serde(
        rename = "retrievalConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retrieval_config: Option<GeminiRetrievalConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiFunctionCallingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<GeminiFunctionCallingMode>,
    #[serde(
        rename = "allowedFunctionNames",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_function_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiFunctionCallingMode {
    #[serde(rename = "MODE_UNSPECIFIED")]
    ModeUnspecified,
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "ANY")]
    Any,
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "VALIDATED")]
    Validated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiRetrievalConfig {
    #[serde(rename = "latLng", default, skip_serializing_if = "Option::is_none")]
    pub lat_lng: Option<GeminiLatLng>,
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiLatLng {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiSafetySetting {
    pub category: GeminiHarmCategory,
    pub threshold: GeminiHarmBlockThreshold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiHarmCategory {
    #[serde(rename = "HARM_CATEGORY_UNSPECIFIED")]
    HarmCategoryUnspecified,
    #[serde(rename = "HARM_CATEGORY_DEROGATORY")]
    HarmCategoryDerogatory,
    #[serde(rename = "HARM_CATEGORY_TOXICITY")]
    HarmCategoryToxicity,
    #[serde(rename = "HARM_CATEGORY_VIOLENCE")]
    HarmCategoryViolence,
    #[serde(rename = "HARM_CATEGORY_SEXUAL")]
    HarmCategorySexual,
    #[serde(rename = "HARM_CATEGORY_MEDICAL")]
    HarmCategoryMedical,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS")]
    HarmCategoryDangerous,
    #[serde(rename = "HARM_CATEGORY_HARASSMENT")]
    HarmCategoryHarassment,
    #[serde(rename = "HARM_CATEGORY_HATE_SPEECH")]
    HarmCategoryHateSpeech,
    #[serde(rename = "HARM_CATEGORY_SEXUALLY_EXPLICIT")]
    HarmCategorySexuallyExplicit,
    #[serde(rename = "HARM_CATEGORY_DANGEROUS_CONTENT")]
    HarmCategoryDangerousContent,
    #[serde(rename = "HARM_CATEGORY_CIVIC_INTEGRITY")]
    HarmCategoryCivicIntegrity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiHarmBlockThreshold {
    #[serde(rename = "HARM_BLOCK_THRESHOLD_UNSPECIFIED")]
    HarmBlockThresholdUnspecified,
    #[serde(rename = "BLOCK_LOW_AND_ABOVE")]
    BlockLowAndAbove,
    #[serde(rename = "BLOCK_MEDIUM_AND_ABOVE")]
    BlockMediumAndAbove,
    #[serde(rename = "BLOCK_ONLY_HIGH")]
    BlockOnlyHigh,
    #[serde(rename = "BLOCK_NONE")]
    BlockNone,
    #[serde(rename = "OFF")]
    Off,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGenerationConfig {
    #[serde(
        rename = "stopSequences",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub stop_sequences: Option<Vec<String>>,
    #[serde(
        rename = "responseMimeType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_mime_type: Option<String>,
    #[serde(
        rename = "responseSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_schema: Option<GeminiSchema>,
    #[serde(
        rename = "_responseJsonSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_json_schema_legacy: Option<Value>,
    #[serde(
        rename = "responseJsonSchema",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_json_schema: Option<Value>,
    #[serde(
        rename = "responseModalities",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_modalities: Option<Vec<GeminiModality>>,
    #[serde(
        rename = "candidateCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub candidate_count: Option<u32>,
    #[serde(
        rename = "maxOutputTokens",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_output_tokens: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(rename = "topP", default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(rename = "topK", default, skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<u32>,
    #[serde(
        rename = "presencePenalty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub presence_penalty: Option<f64>,
    #[serde(
        rename = "frequencyPenalty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub frequency_penalty: Option<f64>,
    #[serde(
        rename = "responseLogprobs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub response_logprobs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(
        rename = "enableEnhancedCivicAnswers",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_enhanced_civic_answers: Option<bool>,
    #[serde(
        rename = "speechConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub speech_config: Option<GeminiSpeechConfig>,
    #[serde(
        rename = "thinkingConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_config: Option<GeminiThinkingConfig>,
    #[serde(
        rename = "imageConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_config: Option<GeminiImageConfig>,
    #[serde(
        rename = "mediaResolution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub media_resolution: Option<GeminiMediaResolution>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiSpeechConfig {
    #[serde(
        rename = "voiceConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_config: Option<GeminiVoiceConfig>,
    #[serde(
        rename = "multiSpeakerVoiceConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub multi_speaker_voice_config: Option<GeminiMultiSpeakerVoiceConfig>,
    #[serde(
        rename = "languageCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub language_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiVoiceConfig {
    #[serde(
        rename = "prebuiltVoiceConfig",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub prebuilt_voice_config: Option<GeminiPrebuiltVoiceConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiPrebuiltVoiceConfig {
    #[serde(rename = "voiceName")]
    pub voice_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiMultiSpeakerVoiceConfig {
    #[serde(rename = "speakerVoiceConfigs")]
    pub speaker_voice_configs: Vec<GeminiSpeakerVoiceConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeminiSpeakerVoiceConfig {
    pub speaker: String,
    #[serde(rename = "voiceConfig")]
    pub voice_config: GeminiVoiceConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiThinkingConfig {
    #[serde(
        rename = "includeThoughts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_thoughts: Option<bool>,
    #[serde(
        rename = "thinkingBudget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_budget: Option<i64>,
    #[serde(
        rename = "thinkingLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_level: Option<GeminiThinkingLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiThinkingLevel {
    #[serde(rename = "THINKING_LEVEL_UNSPECIFIED")]
    ThinkingLevelUnspecified,
    #[serde(rename = "MINIMAL")]
    Minimal,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiImageConfig {
    #[serde(
        rename = "aspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aspect_ratio: Option<String>,
    #[serde(rename = "imageSize", default, skip_serializing_if = "Option::is_none")]
    pub image_size: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiMediaResolution {
    #[serde(rename = "MEDIA_RESOLUTION_UNSPECIFIED")]
    MediaResolutionUnspecified,
    #[serde(rename = "MEDIA_RESOLUTION_LOW")]
    MediaResolutionLow,
    #[serde(rename = "MEDIA_RESOLUTION_MEDIUM")]
    MediaResolutionMedium,
    #[serde(rename = "MEDIA_RESOLUTION_HIGH")]
    MediaResolutionHigh,
}
