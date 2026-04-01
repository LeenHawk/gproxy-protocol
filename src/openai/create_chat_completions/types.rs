use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub use crate::openai::types::{
    HttpMethod, OpenAiApiError, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// JSON object type used for fields documented as `map[unknown]`.
pub type JsonObject = BTreeMap<String, Value>;

/// Metadata map (string key-value pairs).
pub type Metadata = BTreeMap<String, String>;

/// JSON Schema-like function parameters.
pub type FunctionParameters = JsonObject;

/// Token-id to bias score mapping (`map[number]`).
pub type LogitBias = BTreeMap<String, f64>;

/// Chat model identifier.
pub type Model = String;

/// Input message union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionMessageParam {
    Developer(ChatCompletionDeveloperMessageParam),
    System(ChatCompletionSystemMessageParam),
    User(ChatCompletionUserMessageParam),
    Assistant(ChatCompletionAssistantMessageParam),
    Tool(ChatCompletionToolMessageParam),
    Function(ChatCompletionFunctionMessageParam),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionDeveloperMessageParam {
    pub content: ChatCompletionTextContent,
    pub role: ChatCompletionDeveloperRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionDeveloperRole {
    #[serde(rename = "developer")]
    Developer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionSystemMessageParam {
    pub content: ChatCompletionTextContent,
    pub role: ChatCompletionSystemRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionSystemRole {
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionUserMessageParam {
    pub content: ChatCompletionUserContent,
    pub role: ChatCompletionUserRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionUserRole {
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAssistantMessageParam {
    pub role: ChatCompletionAssistantRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<ChatCompletionAssistantAudioRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ChatCompletionAssistantContent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAssistantAudioRef {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionToolMessageParam {
    pub content: ChatCompletionTextContent,
    pub role: ChatCompletionToolRole,
    pub tool_call_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionToolRole {
    #[serde(rename = "tool")]
    Tool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionMessageParam {
    pub content: String,
    pub name: String,
    pub role: ChatCompletionFunctionRole,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionFunctionRole {
    #[serde(rename = "function")]
    Function,
}

/// `string` or `array<ChatCompletionContentPartText>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionTextContent {
    Text(String),
    Parts(Vec<ChatCompletionContentPartText>),
}

/// `string` or `array<ChatCompletionContentPart>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionUserContent {
    Text(String),
    Parts(Vec<ChatCompletionContentPart>),
}

/// `string` or `array<text/refusal parts>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionAssistantContent {
    Text(String),
    Parts(Vec<ChatCompletionAssistantContentPart>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionAssistantContentPart {
    Text(ChatCompletionContentPartText),
    Refusal(ChatCompletionContentPartRefusal),
}

/// User content part union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionContentPart {
    Text(ChatCompletionContentPartText),
    Image(ChatCompletionContentPartImage),
    InputAudio(ChatCompletionContentPartInputAudio),
    File(ChatCompletionContentPartFile),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionContentPartText {
    pub text: String,
    #[serde(rename = "type")]
    pub type_: ChatCompletionContentPartTextType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionContentPartTextType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionContentPartRefusal {
    pub refusal: String,
    #[serde(rename = "type")]
    pub type_: ChatCompletionContentPartRefusalType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionContentPartRefusalType {
    #[serde(rename = "refusal")]
    Refusal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionContentPartImage {
    pub image_url: ChatCompletionImageUrl,
    #[serde(rename = "type")]
    pub type_: ChatCompletionContentPartImageType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionImageUrl {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ChatCompletionImageDetail>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionImageDetail {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "original")]
    Original,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionContentPartImageType {
    #[serde(rename = "image_url")]
    ImageUrl,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionContentPartInputAudio {
    pub input_audio: ChatCompletionInputAudio,
    #[serde(rename = "type")]
    pub type_: ChatCompletionContentPartInputAudioType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionInputAudio {
    pub data: String,
    pub format: ChatCompletionInputAudioFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionInputAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionContentPartInputAudioType {
    #[serde(rename = "input_audio")]
    InputAudio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionContentPartFile {
    pub file: ChatCompletionFileInput,
    #[serde(rename = "type")]
    pub type_: ChatCompletionContentPartFileType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionFileInput {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionContentPartFileType {
    #[serde(rename = "file")]
    File,
}

/// Parameters for audio output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAudioParam {
    pub format: ChatCompletionAudioFormat,
    pub voice: ChatCompletionVoice,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "pcm16")]
    Pcm16,
}

/// Voice selector: known id, custom string, or `{ "id": ... }` object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionVoice {
    Known(ChatCompletionVoiceKnown),
    Custom(String),
    Id(ChatCompletionVoiceId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionVoiceKnown {
    #[serde(rename = "alloy")]
    Alloy,
    #[serde(rename = "ash")]
    Ash,
    #[serde(rename = "ballad")]
    Ballad,
    #[serde(rename = "coral")]
    Coral,
    #[serde(rename = "echo")]
    Echo,
    #[serde(rename = "fable")]
    Fable,
    #[serde(rename = "nova")]
    Nova,
    #[serde(rename = "onyx")]
    Onyx,
    #[serde(rename = "sage")]
    Sage,
    #[serde(rename = "shimmer")]
    Shimmer,
    #[serde(rename = "verse")]
    Verse,
    #[serde(rename = "marin")]
    Marin,
    #[serde(rename = "cedar")]
    Cedar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionVoiceId {
    pub id: String,
}

/// Deprecated `function_call` request parameter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionFunctionCallOptionParam {
    Mode(ChatCompletionFunctionCallMode),
    Named(ChatCompletionFunctionCallOption),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionFunctionCallMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionCallOption {
    pub name: String,
}

/// Deprecated `functions` item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionLegacyFunction {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FunctionParameters>,
}

/// Predicted output content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionPredictionContent {
    pub content: ChatCompletionPredictionContentValue,
    #[serde(rename = "type")]
    pub type_: ChatCompletionPredictionContentType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionPredictionContentValue {
    Text(String),
    Parts(Vec<ChatCompletionContentPartText>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionPredictionContentType {
    #[serde(rename = "content")]
    Content,
}

/// Prompt cache retention policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionPromptCacheRetention {
    #[serde(rename = "in-memory")]
    InMemory,
    #[serde(rename = "24h")]
    H24,
}

/// Reasoning effort levels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionReasoningEffort {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "xhigh")]
    XHigh,
}

/// Output response format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionResponseFormat {
    Text(ChatCompletionResponseFormatText),
    JsonSchema(ChatCompletionResponseFormatJsonSchema),
    JsonObject(ChatCompletionResponseFormatJsonObject),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionResponseFormatText {
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseFormatTextType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionResponseFormatTextType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseFormatJsonSchema {
    pub json_schema: ChatCompletionResponseFormatJsonSchemaConfig,
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseFormatJsonSchemaType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionResponseFormatJsonSchemaConfig {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<JsonObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionResponseFormatJsonSchemaType {
    #[serde(rename = "json_schema")]
    JsonSchema,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionResponseFormatJsonObject {
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseFormatJsonObjectType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionResponseFormatJsonObjectType {
    #[serde(rename = "json_object")]
    JsonObject,
}

/// Request/response service tier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionServiceTier {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default", alias = "on_demand")]
    Default,
    #[serde(rename = "flex")]
    Flex,
    #[serde(rename = "scale")]
    Scale,
    #[serde(rename = "priority")]
    Priority,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionStop {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ChatCompletionStreamOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Tool choice policy.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionToolChoiceOption {
    Mode(ChatCompletionToolChoiceMode),
    Allowed(ChatCompletionAllowedToolChoice),
    NamedFunction(ChatCompletionNamedToolChoice),
    NamedCustom(ChatCompletionNamedToolChoiceCustom),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionToolChoiceMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAllowedToolChoice {
    pub allowed_tools: ChatCompletionAllowedTools,
    #[serde(rename = "type")]
    pub type_: ChatCompletionAllowedToolChoiceType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAllowedTools {
    pub mode: ChatCompletionAllowedToolsMode,
    /// Tool definitions typed as `map[unknown]` by upstream spec.
    pub tools: Vec<JsonObject>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionAllowedToolsMode {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionAllowedToolChoiceType {
    #[serde(rename = "allowed_tools")]
    AllowedTools,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoice {
    pub function: ChatCompletionNamedFunction,
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionNamedFunction {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionNamedToolChoiceType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionNamedToolChoiceCustom {
    pub custom: ChatCompletionNamedCustomTool,
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceCustomType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionNamedCustomTool {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionNamedToolChoiceCustomType {
    #[serde(rename = "custom")]
    Custom,
}

/// Tool definition union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionTool {
    Function(ChatCompletionFunctionTool),
    Custom(ChatCompletionCustomTool),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionTool {
    pub function: ChatCompletionFunctionDefinition,
    #[serde(rename = "type")]
    pub type_: ChatCompletionFunctionToolType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionFunctionToolType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionDefinition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<FunctionParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionCustomTool {
    pub custom: ChatCompletionCustomToolSpec,
    #[serde(rename = "type")]
    pub type_: ChatCompletionCustomToolType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionCustomToolType {
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionCustomToolSpec {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ChatCompletionCustomToolFormat>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionCustomToolFormat {
    Text(ChatCompletionCustomToolTextFormat),
    Grammar(ChatCompletionCustomToolGrammarFormat),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionCustomToolTextFormat {
    #[serde(rename = "type")]
    pub type_: ChatCompletionCustomToolTextFormatType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionCustomToolTextFormatType {
    #[serde(rename = "text")]
    Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionCustomToolGrammarFormat {
    pub grammar: ChatCompletionCustomToolGrammar,
    #[serde(rename = "type")]
    pub type_: ChatCompletionCustomToolGrammarFormatType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionCustomToolGrammar {
    pub definition: String,
    pub syntax: ChatCompletionCustomToolGrammarSyntax,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionCustomToolGrammarSyntax {
    #[serde(rename = "lark")]
    Lark,
    #[serde(rename = "regex")]
    Regex,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionCustomToolGrammarFormatType {
    #[serde(rename = "grammar")]
    Grammar,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionVerbosity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionWebSearchOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_context_size: Option<ChatCompletionWebSearchContextSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_location: Option<ChatCompletionWebSearchUserLocation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionWebSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionWebSearchUserLocation {
    pub approximate: ChatCompletionWebSearchLocationApproximate,
    #[serde(rename = "type")]
    pub type_: ChatCompletionWebSearchUserLocationType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionWebSearchUserLocationType {
    #[serde(rename = "approximate")]
    Approximate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionWebSearchLocationApproximate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionClaudeThinkingConfig {
    Enabled(ChatCompletionClaudeThinkingEnabled),
    Disabled(ChatCompletionClaudeThinkingDisabled),
    Adaptive(ChatCompletionClaudeThinkingAdaptive),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionClaudeThinkingEnabled {
    pub budget_tokens: u64,
    #[serde(rename = "type")]
    pub type_: ChatCompletionClaudeThinkingEnabledType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionClaudeThinkingEnabledType {
    #[serde(rename = "enabled")]
    Enabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionClaudeThinkingDisabled {
    #[serde(rename = "type")]
    pub type_: ChatCompletionClaudeThinkingDisabledType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionClaudeThinkingDisabledType {
    #[serde(rename = "disabled")]
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionClaudeThinkingAdaptive {
    #[serde(rename = "type")]
    pub type_: ChatCompletionClaudeThinkingAdaptiveType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionClaudeThinkingAdaptiveType {
    #[serde(rename = "adaptive")]
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionGeminiExtraThinkingConfig {
    #[serde(
        rename = "include_thoughts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_thoughts: Option<bool>,
    #[serde(
        rename = "thinking_budget",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_budget: Option<i64>,
    #[serde(
        rename = "thinking_level",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_level: Option<ChatCompletionGeminiExtraThinkingLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionGeminiExtraThinkingLevel {
    #[serde(rename = "minimal")]
    Minimal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

/// Standard chat completion response body.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub choices: Vec<ChatCompletionChoice>,
    pub created: u64,
    pub model: String,
    pub object: ChatCompletionObject,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ChatCompletionServiceTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionObject {
    #[serde(rename = "chat.completion")]
    ChatCompletion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionChoice {
    pub finish_reason: ChatCompletionFinishReason,
    pub index: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<ChatCompletionLogprobs>,
    pub message: ChatCompletionMessage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ChatCompletionLogprobs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionTokenLogprob {
    pub token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    pub logprob: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub top_logprobs: Vec<ChatCompletionTopLogprob>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionTopLogprob {
    pub token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<Vec<u8>>,
    pub logprob: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_details: Option<Vec<ChatCompletionReasoningDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    pub role: ChatCompletionAssistantRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<ChatCompletionAnnotation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<ChatCompletionAudio>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCall>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionReasoningDetail {
    #[serde(rename = "type")]
    pub type_: ChatCompletionReasoningDetailType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionReasoningDetailType {
    #[serde(rename = "reasoning.encrypted")]
    ReasoningEncrypted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionAssistantRole {
    #[serde(rename = "assistant")]
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAnnotation {
    #[serde(rename = "type")]
    pub type_: ChatCompletionAnnotationType,
    pub url_citation: ChatCompletionUrlCitation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionAnnotationType {
    #[serde(rename = "url_citation")]
    UrlCitation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionUrlCitation {
    pub end_index: u64,
    pub start_index: u64,
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChatCompletionAudio {
    pub id: String,
    pub data: String,
    pub expires_at: u64,
    pub transcript: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionFunctionCall {
    pub arguments: String,
    pub name: String,
}

/// Tool-call union emitted in assistant message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatCompletionMessageToolCall {
    Function(ChatCompletionMessageFunctionToolCall),
    Custom(ChatCompletionMessageCustomToolCall),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionMessageFunctionToolCall {
    pub id: String,
    pub function: ChatCompletionFunctionCall,
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageFunctionToolCallType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionMessageFunctionToolCallType {
    #[serde(rename = "function")]
    Function,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionMessageCustomToolCall {
    pub id: String,
    pub custom: ChatCompletionMessageCustomToolCallPayload,
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageCustomToolCallType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatCompletionMessageCustomToolCallPayload {
    pub input: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionMessageCustomToolCallType {
    #[serde(rename = "custom")]
    Custom,
}

/// Usage statistics for a completion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletionUsage {
    pub completion_tokens: u64,
    pub prompt_tokens: u64,
    pub total_tokens: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<CompletionTokensDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CompletionTokensDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_prediction_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_prediction_tokens: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PromptTokensDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assistant_request_reasoning_content_roundtrip() {
        let assistant = ChatCompletionAssistantMessageParam {
            role: ChatCompletionAssistantRole::Assistant,
            audio: None,
            content: None,
            reasoning_content: Some("reasoning text".to_string()),
            function_call: None,
            name: None,
            refusal: None,
            tool_calls: None,
        };

        let value = serde_json::to_value(&assistant).unwrap();
        assert_eq!(value["reasoning_content"], "reasoning text");

        let decoded: ChatCompletionAssistantMessageParam = serde_json::from_value(value).unwrap();
        assert_eq!(decoded.reasoning_content.as_deref(), Some("reasoning text"));
    }

    #[test]
    fn assistant_response_reasoning_content_roundtrip() {
        let message = ChatCompletionMessage {
            content: None,
            reasoning_content: Some("reasoning text".to_string()),
            reasoning_details: None,
            refusal: None,
            role: ChatCompletionAssistantRole::Assistant,
            annotations: None,
            audio: None,
            function_call: None,
            tool_calls: None,
        };

        let value = serde_json::to_value(&message).unwrap();
        assert_eq!(value["reasoning_content"], "reasoning text");

        let decoded: ChatCompletionMessage = serde_json::from_value(value).unwrap();
        assert_eq!(decoded.reasoning_content.as_deref(), Some("reasoning text"));
    }

    #[test]
    fn assistant_response_reasoning_details_roundtrip() {
        let message = ChatCompletionMessage {
            content: None,
            reasoning_content: None,
            reasoning_details: Some(vec![ChatCompletionReasoningDetail {
                type_: ChatCompletionReasoningDetailType::ReasoningEncrypted,
                id: Some("reasoning_0".to_string()),
                data: Some("sig".to_string()),
            }]),
            refusal: None,
            role: ChatCompletionAssistantRole::Assistant,
            annotations: None,
            audio: None,
            function_call: None,
            tool_calls: None,
        };

        let value = serde_json::to_value(&message).unwrap();
        assert_eq!(value["reasoning_details"][0]["type"], "reasoning.encrypted");
        assert_eq!(value["reasoning_details"][0]["id"], "reasoning_0");
        assert_eq!(value["reasoning_details"][0]["data"], "sig");

        let decoded: ChatCompletionMessage = serde_json::from_value(value).unwrap();
        assert_eq!(
            decoded
                .reasoning_details
                .as_ref()
                .and_then(|details| details.first())
                .and_then(|detail| detail.id.as_deref()),
            Some("reasoning_0")
        );
    }

    #[test]
    fn image_detail_original_roundtrip() {
        let image = ChatCompletionContentPartImage {
            image_url: ChatCompletionImageUrl {
                url: "https://example.com/screenshot.png".to_string(),
                detail: Some(ChatCompletionImageDetail::Original),
            },
            type_: ChatCompletionContentPartImageType::ImageUrl,
        };

        let value = serde_json::to_value(&image).unwrap();
        assert_eq!(value["image_url"]["detail"], "original");

        let decoded: ChatCompletionContentPartImage = serde_json::from_value(value).unwrap();
        assert!(matches!(
            decoded.image_url.detail,
            Some(ChatCompletionImageDetail::Original)
        ));
    }
}

/// Generic message role.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "function")]
    Function,
}

/// Delta role union used in streamed chunks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChatCompletionDeltaRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}
