use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub use crate::gemini::count_tokens::types::{
    GeminiCodeExecution, GeminiComputerUse, GeminiContent, GeminiFunctionBehavior,
    GeminiFunctionCallingConfig, GeminiFunctionCallingMode, GeminiFunctionDeclaration,
    GeminiGenerationConfig, GeminiGoogleMaps, GeminiGoogleSearch, GeminiGoogleSearchRetrieval,
    GeminiHarmBlockThreshold, GeminiHarmCategory, GeminiImageConfig, GeminiLatLng,
    GeminiMediaResolution, GeminiModality, GeminiModalityTokenCount, GeminiRetrievalConfig,
    GeminiSafetySetting, GeminiSchema, GeminiSchemaType, GeminiSpeechConfig, GeminiThinkingConfig,
    GeminiThinkingLevel, GeminiTool, GeminiToolConfig, GeminiUrlContext, GeminiVideoMetadata,
    HttpMethod,
};
pub use crate::gemini::types::{
    GeminiApiError, GeminiApiErrorResponse, GeminiResponseHeaders, JsonObject,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiPromptFeedback {
    #[serde(
        rename = "blockReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_reason: Option<GeminiBlockReason>,
    #[serde(
        rename = "safetyRatings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub safety_ratings: Option<Vec<GeminiSafetyRating>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiBlockReason {
    #[serde(rename = "BLOCK_REASON_UNSPECIFIED")]
    BlockReasonUnspecified,
    #[serde(rename = "SAFETY")]
    Safety,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,
    #[serde(rename = "IMAGE_SAFETY")]
    ImageSafety,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiUsageMetadata {
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
        rename = "candidatesTokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub candidates_token_count: Option<u64>,
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
        rename = "candidatesTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub candidates_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
    #[serde(
        rename = "toolUsePromptTokensDetails",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tool_use_prompt_tokens_details: Option<Vec<GeminiModalityTokenCount>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiModelStatus {
    #[serde(
        rename = "modelStage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub model_stage: Option<GeminiModelStage>,
    #[serde(
        rename = "retirementTime",
        default,
        skip_serializing_if = "Option::is_none",
        with = "time::serde::rfc3339::option"
    )]
    pub retirement_time: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiModelStage {
    #[serde(rename = "MODEL_STAGE_UNSPECIFIED")]
    ModelStageUnspecified,
    #[serde(rename = "UNSTABLE_EXPERIMENTAL")]
    UnstableExperimental,
    #[serde(rename = "EXPERIMENTAL")]
    Experimental,
    #[serde(rename = "PREVIEW")]
    Preview,
    #[serde(rename = "STABLE")]
    Stable,
    #[serde(rename = "LEGACY")]
    Legacy,
    #[serde(rename = "DEPRECATED")]
    Deprecated,
    #[serde(rename = "RETIRED")]
    Retired,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiCandidate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<GeminiContent>,
    #[serde(
        rename = "finishReason",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub finish_reason: Option<GeminiFinishReason>,
    #[serde(
        rename = "safetyRatings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub safety_ratings: Option<Vec<GeminiSafetyRating>>,
    #[serde(
        rename = "citationMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citation_metadata: Option<GeminiCitationMetadata>,
    #[serde(
        rename = "tokenCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub token_count: Option<u64>,
    #[serde(
        rename = "groundingAttributions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_attributions: Option<Vec<GeminiGroundingAttribution>>,
    #[serde(
        rename = "groundingMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_metadata: Option<GeminiGroundingMetadata>,
    #[serde(
        rename = "avgLogprobs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub avg_logprobs: Option<f64>,
    #[serde(
        rename = "logprobsResult",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub logprobs_result: Option<GeminiLogprobsResult>,
    #[serde(
        rename = "urlContextMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_context_metadata: Option<GeminiUrlContextMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    #[serde(
        rename = "finishMessage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub finish_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiFinishReason {
    #[serde(rename = "FINISH_REASON_UNSPECIFIED")]
    FinishReasonUnspecified,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "MAX_TOKENS")]
    MaxTokens,
    #[serde(rename = "SAFETY")]
    Safety,
    #[serde(rename = "RECITATION")]
    Recitation,
    #[serde(rename = "LANGUAGE")]
    Language,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "BLOCKLIST")]
    Blocklist,
    #[serde(rename = "PROHIBITED_CONTENT")]
    ProhibitedContent,
    #[serde(rename = "SPII")]
    Spii,
    #[serde(rename = "MALFORMED_FUNCTION_CALL")]
    MalformedFunctionCall,
    #[serde(rename = "IMAGE_SAFETY")]
    ImageSafety,
    #[serde(rename = "IMAGE_PROHIBITED_CONTENT")]
    ImageProhibitedContent,
    #[serde(rename = "IMAGE_OTHER")]
    ImageOther,
    #[serde(rename = "NO_IMAGE")]
    NoImage,
    #[serde(rename = "IMAGE_RECITATION")]
    ImageRecitation,
    #[serde(rename = "UNEXPECTED_TOOL_CALL")]
    UnexpectedToolCall,
    #[serde(rename = "TOO_MANY_TOOL_CALLS")]
    TooManyToolCalls,
    #[serde(rename = "MISSING_THOUGHT_SIGNATURE")]
    MissingThoughtSignature,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiSafetyRating {
    pub category: GeminiHarmCategory,
    pub probability: GeminiHarmProbability,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiHarmProbability {
    #[serde(rename = "HARM_PROBABILITY_UNSPECIFIED")]
    HarmProbabilityUnspecified,
    #[serde(rename = "NEGLIGIBLE")]
    Negligible,
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGroundingAttribution {
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<GeminiAttributionSourceId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<GeminiContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiAttributionSourceId {
    #[serde(
        rename = "groundingPassage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_passage: Option<GeminiGroundingPassageId>,
    #[serde(
        rename = "semanticRetrieverChunk",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub semantic_retriever_chunk: Option<GeminiSemanticRetrieverChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiGroundingPassageId {
    #[serde(rename = "passageId")]
    pub passage_id: String,
    #[serde(rename = "partIndex")]
    pub part_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiSemanticRetrieverChunk {
    pub source: String,
    pub chunk: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGroundingMetadata {
    #[serde(
        rename = "groundingChunks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_chunks: Option<Vec<GeminiGroundingChunk>>,
    #[serde(
        rename = "groundingSupports",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_supports: Option<Vec<GeminiGroundingSupport>>,
    #[serde(
        rename = "webSearchQueries",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_search_queries: Option<Vec<String>>,
    #[serde(
        rename = "searchEntryPoint",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_entry_point: Option<GeminiSearchEntryPoint>,
    #[serde(
        rename = "retrievalMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retrieval_metadata: Option<GeminiRetrievalMetadata>,
    #[serde(
        rename = "googleMapsWidgetContextToken",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub google_maps_widget_context_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiSearchEntryPoint {
    #[serde(
        rename = "renderedContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub rendered_content: Option<String>,
    #[serde(rename = "sdkBlob", default, skip_serializing_if = "Option::is_none")]
    pub sdk_blob: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGroundingChunk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<GeminiWebGroundingChunk>,
    #[serde(
        rename = "retrievedContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retrieved_context: Option<GeminiRetrievedContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maps: Option<GeminiMapsGroundingChunk>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiWebGroundingChunk {
    pub uri: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiRetrievedContext {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(
        rename = "fileSearchStore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub file_search_store: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiMapsGroundingChunk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "placeId", default, skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    #[serde(
        rename = "placeAnswerSources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub place_answer_sources: Option<GeminiPlaceAnswerSources>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiPlaceAnswerSources {
    #[serde(
        rename = "reviewSnippets",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub review_snippets: Option<Vec<GeminiReviewSnippet>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeminiReviewSnippet {
    #[serde(rename = "reviewId")]
    pub review_id: String,
    #[serde(rename = "googleMapsUri")]
    pub google_maps_uri: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiGroundingSupport {
    #[serde(
        rename = "groundingChunkIndices",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub grounding_chunk_indices: Option<Vec<u32>>,
    #[serde(
        rename = "confidenceScores",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confidence_scores: Option<Vec<f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment: Option<GeminiSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiSegment {
    #[serde(rename = "partIndex", default, skip_serializing_if = "Option::is_none")]
    pub part_index: Option<u32>,
    #[serde(
        rename = "startIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<u32>,
    #[serde(rename = "endIndex", default, skip_serializing_if = "Option::is_none")]
    pub end_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiRetrievalMetadata {
    #[serde(
        rename = "googleSearchDynamicRetrievalScore",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub google_search_dynamic_retrieval_score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiLogprobsResult {
    #[serde(
        rename = "topCandidates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub top_candidates: Option<Vec<GeminiTopCandidates>>,
    #[serde(
        rename = "chosenCandidates",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chosen_candidates: Option<Vec<GeminiLogprobsCandidate>>,
    #[serde(
        rename = "logProbabilitySum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_probability_sum: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiTopCandidates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<GeminiLogprobsCandidate>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiLogprobsCandidate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "tokenId", default, skip_serializing_if = "Option::is_none")]
    pub token_id: Option<u64>,
    #[serde(
        rename = "logProbability",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_probability: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiUrlContextMetadata {
    #[serde(
        rename = "urlMetadata",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_metadata: Option<Vec<GeminiUrlMetadata>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiUrlMetadata {
    #[serde(
        rename = "retrievedUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub retrieved_url: Option<String>,
    #[serde(
        rename = "urlRetrievalStatus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub url_retrieval_status: Option<GeminiUrlRetrievalStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeminiUrlRetrievalStatus {
    #[serde(rename = "URL_RETRIEVAL_STATUS_UNSPECIFIED")]
    UrlRetrievalStatusUnspecified,
    #[serde(rename = "URL_RETRIEVAL_STATUS_SUCCESS")]
    UrlRetrievalStatusSuccess,
    #[serde(rename = "URL_RETRIEVAL_STATUS_ERROR")]
    UrlRetrievalStatusError,
    #[serde(rename = "URL_RETRIEVAL_STATUS_PAYWALL")]
    UrlRetrievalStatusPaywall,
    #[serde(rename = "URL_RETRIEVAL_STATUS_UNSAFE")]
    UrlRetrievalStatusUnsafe,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GeminiCitationMetadata {
    #[serde(
        rename = "citationSources",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub citation_sources: Option<Vec<GeminiCitationSource>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GeminiCitationSource {
    #[serde(
        rename = "startIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<u32>,
    #[serde(rename = "endIndex", default, skip_serializing_if = "Option::is_none")]
    pub end_index: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}
