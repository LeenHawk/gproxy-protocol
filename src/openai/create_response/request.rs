use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_response::types::{
    HttpMethod, Metadata, Model, ResponseContextManagementEntry, ResponseConversation,
    ResponseIncludable, ResponseInput, ResponsePrompt, ResponsePromptCacheRetention,
    ResponseReasoning, ResponseServiceTier, ResponseStreamOptions, ResponseTextConfig,
    ResponseTool, ResponseToolChoice, ResponseTruncation,
};

/// Request descriptor for OpenAI `responses.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Path parameters.
    pub path: PathParameters,
    /// Query parameters.
    pub query: QueryParameters,
    /// Request headers.
    pub headers: RequestHeaders,
    /// Request body.
    pub body: RequestBody,
}

impl Default for OpenAiCreateResponseRequest {
    fn default() -> Self {
        Self {
            method: HttpMethod::Post,
            path: PathParameters::default(),
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody::default(),
        }
    }
}

/// `responses.create` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// `responses.create` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Body payload for `POST /responses`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_management: Option<Vec<ResponseContextManagementEntry>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ResponseConversation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<ResponseIncludable>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<ResponseInput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<Model>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<ResponsePromptCacheRetention>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ResponseReasoning>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ResponseServiceTier>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<ResponseStreamOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ResponseToolChoice>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<ResponseTool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<ResponseTruncation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}