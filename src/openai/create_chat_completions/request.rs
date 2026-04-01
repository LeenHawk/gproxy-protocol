use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::openai::create_chat_completions::types::{
    self, ChatCompletionClaudeThinkingConfig, ChatCompletionGeminiExtraThinkingConfig, HttpMethod,
};

/// Request descriptor for OpenAI `chat.completions.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiChatCompletionsRequest {
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

impl Default for OpenAiChatCompletionsRequest {
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

/// `chat.completions.create` does not define path params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PathParameters {}

/// `chat.completions.create` does not define query params.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QueryParameters {}

/// Proxy-side request model does not carry auth headers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequestHeaders {
    #[serde(flatten, default, skip_serializing_if = "BTreeMap::is_empty")]
    pub extra: BTreeMap<String, String>,
}

/// Body payload for `POST /chat/completions`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RequestBody {
    /// A list of messages comprising the conversation so far.
    pub messages: Vec<types::ChatCompletionMessageParam>,
    /// Model identifier.
    pub model: types::Model,
    /// Audio output configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<types::ChatCompletionAudioParam>,
    /// Frequency penalty in range [-2.0, 2.0].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// Deprecated function-call control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function_call: Option<types::ChatCompletionFunctionCallOptionParam>,
    /// Deprecated function definitions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<types::ChatCompletionLegacyFunction>>,
    /// Token-level logit bias.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<types::LogitBias>,
    /// Whether to return logprobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    /// Upper bound of generated tokens including reasoning tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<u64>,
    /// Deprecated maximum generated tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u64>,
    /// Request metadata key-value map.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<types::Metadata>,
    /// Output modalities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<types::ChatCompletionModality>>,
    /// Number of choices to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    /// Enable parallel tool calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Predicted output content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<types::ChatCompletionPredictionContent>,
    /// Presence penalty in range [-2.0, 2.0].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// Prompt cache bucketing key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    /// Prompt cache retention policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<types::ChatCompletionPromptCacheRetention>,
    /// Reasoning effort level.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<types::ChatCompletionReasoningEffort>,
    /// Output format control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<types::ChatCompletionResponseFormat>,
    /// Stable safety identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    /// Best-effort deterministic seed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    /// Requested processing tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<types::ChatCompletionServiceTier>,
    /// Stop sequence(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<types::ChatCompletionStop>,
    /// Whether to store output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Whether to stream with SSE.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Streaming options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<types::ChatCompletionStreamOptions>,
    /// Sampling temperature in range [0, 2].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Tool selection policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<types::ChatCompletionToolChoiceOption>,
    /// Available tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<types::ChatCompletionTool>>,
    /// Number of top candidate tokens in logprobs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    /// Nucleus sampling probability mass.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Deprecated user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Verbosity hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<types::ChatCompletionVerbosity>,
    /// Provider-specific OpenAI-compatible extension payload (flattened).
    /// Claude-compatible extended thinking control.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ChatCompletionClaudeThinkingConfig>,
    #[serde(
        rename = "thinking_config",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thinking_config: Option<ChatCompletionGeminiExtraThinkingConfig>,
    #[serde(
        rename = "cached_content",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cached_content: Option<String>,
    /// Web-search tool options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<types::ChatCompletionWebSearchOptions>,
}
