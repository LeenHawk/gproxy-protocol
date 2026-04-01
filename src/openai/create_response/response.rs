use std::collections::BTreeMap;

use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::openai::create_response::types::{
    Metadata, Model, OpenAiApiErrorResponse, OpenAiResponseHeaders, ResponseConversationParam,
    ResponseError, ResponseIncompleteDetails, ResponseInput, ResponseObject, ResponseOutputItem,
    ResponsePrompt, ResponsePromptCacheRetention, ResponseReasoning, ResponseServiceTier,
    ResponseStatus, ResponseTextConfig, ResponseTool, ResponseToolChoice, ResponseTruncation,
    ResponseUsage,
};

/// Successful body returned by `responses.create`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseBody {
    pub id: String,
    pub created_at: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<ResponseInput>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: Metadata,
    pub model: Model,
    pub object: ResponseObject,
    pub output: Vec<ResponseOutputItem>,
    pub parallel_tool_calls: bool,
    pub temperature: f64,
    pub tool_choice: ResponseToolChoice,
    pub tools: Vec<ResponseTool>,
    pub top_p: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ResponseConversationParam>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
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
    pub status: Option<ResponseStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<ResponseTruncation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Full HTTP response for OpenAI `responses.create` endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiCreateResponseResponse {
    Success {
        /// HTTP status code returned by server (should be `200 OK`).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiResponseHeaders,
        /// Successful body.
        body: ResponseBody,
    },
    Error {
        /// HTTP status code returned by server (typically non-2xx).
        #[serde(with = "crate::openai::types::status_code_serde")]
        stats_code: StatusCode,
        /// Response headers.
        headers: OpenAiResponseHeaders,
        /// Error body.
        body: OpenAiApiErrorResponse,
    },
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::ResponseBody;

    #[test]
    fn response_body_supports_tool_search_output_items() {
        let value = json!({
            "id": "resp_123",
            "created_at": 1,
            "model": "gpt-5.4-pro",
            "object": "response",
            "output": [
                {
                    "type": "tool_search_call",
                    "id": "ts_1",
                    "call_id": "call_1",
                    "arguments": {"query": "weather"},
                    "execution": "client",
                    "status": "completed",
                    "created_by": "assistant"
                },
                {
                    "type": "tool_search_output",
                    "id": "tso_1",
                    "call_id": "call_1",
                    "execution": "client",
                    "status": "completed",
                    "created_by": "assistant",
                    "tools": [
                        {
                            "type": "function",
                            "name": "get_weather",
                            "parameters": {"type": "object"},
                            "strict": true,
                            "defer_loading": true
                        },
                        {
                            "type": "computer"
                        },
                        {
                            "type": "tool_search",
                            "execution": "server"
                        }
                    ]
                },
                {
                    "type": "compaction",
                    "id": "cmp_1",
                    "encrypted_content": "opaque",
                    "created_by": "assistant"
                }
            ],
            "parallel_tool_calls": true,
            "temperature": 1.0,
            "tool_choice": "auto",
            "tools": [
                {"type": "computer"},
                {"type": "tool_search", "execution": "server"}
            ],
            "top_p": 1.0
        });

        let body: ResponseBody = serde_json::from_value(value).expect("response body should parse");
        let encoded = serde_json::to_value(body).expect("response body should serialize");

        assert_eq!(encoded["output"][0]["type"], json!("tool_search_call"));
        assert_eq!(encoded["output"][1]["type"], json!("tool_search_output"));
        assert_eq!(
            encoded["output"][1]["tools"][0]["defer_loading"],
            json!(true)
        );
        assert_eq!(encoded["output"][1]["tools"][1]["type"], json!("computer"));
        assert_eq!(encoded["output"][2]["created_by"], json!("assistant"));
        assert_eq!(encoded["tools"][0]["type"], json!("computer"));
        assert_eq!(encoded["tools"][1]["type"], json!("tool_search"));
    }
}
