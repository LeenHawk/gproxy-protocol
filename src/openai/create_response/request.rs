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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::RequestBody;
    use crate::openai::count_tokens::types::{ResponseInput, ResponseInputItem};

    #[test]
    fn request_body_accepts_reasoning_item_without_id() {
        let value = json!({
            "model": "gpt-5.3-codex",
            "stream": true,
            "input": [
                {
                    "type": "message",
                    "role": "developer",
                    "content": [
                        {
                            "type": "input_text",
                            "text": "developer prompt"
                        }
                    ]
                },
                {
                    "type": "reasoning",
                    "summary": [
                        {
                            "type": "summary_text",
                            "text": "plan"
                        }
                    ],
                    "content": null,
                    "encrypted_content": "abc"
                },
                {
                    "type": "function_call",
                    "name": "exec_command",
                    "arguments": "{\"cmd\":\"ls\"}",
                    "call_id": "call_1"
                },
                {
                    "type": "function_call_output",
                    "call_id": "call_1",
                    "output": "ok"
                },
                {
                    "type": "message",
                    "role": "assistant",
                    "phase": "commentary",
                    "content": [
                        {
                            "type": "output_text",
                            "text": "working"
                        }
                    ]
                }
            ]
        });

        let body: RequestBody = serde_json::from_value(value).expect("request body should parse");
        let Some(ResponseInput::Items(ref items)) = body.input else {
            panic!("expected input items");
        };
        let Some(ResponseInputItem::ReasoningItem(reasoning)) = items.get(1) else {
            panic!("expected reasoning item");
        };
        assert!(reasoning.id.is_none());

        let encoded = serde_json::to_value(body).expect("request body should serialize");
        assert!(encoded["input"][1].get("id").is_none());
        assert!(encoded["input"][4].get("id").is_none());
    }

    #[test]
    fn request_body_accepts_web_search_item_without_id() {
        let value = json!({
            "model": "gpt-5.3-codex",
            "stream": true,
            "input": [
                {
                    "type": "message",
                    "role": "user",
                    "content": [
                        {
                            "type": "input_text",
                            "text": "search docs"
                        }
                    ]
                },
                {
                    "type": "web_search_call",
                    "status": "completed",
                    "action": {
                        "type": "search",
                        "query": "site:docs.astro.build Astro v6 upgrade guide migration",
                        "queries": [
                            "site:docs.astro.build Astro v6 upgrade guide migration"
                        ]
                    }
                }
            ]
        });

        let body: RequestBody = serde_json::from_value(value).expect("request body should parse");
        let Some(ResponseInput::Items(ref items)) = body.input else {
            panic!("expected input items");
        };

        let Some(ResponseInputItem::FunctionWebSearch(web_search)) = items.get(1) else {
            panic!("expected web search item");
        };
        assert!(web_search.id.is_none());

        let encoded = serde_json::to_value(body).expect("request body should serialize");
        assert!(encoded["input"][1].get("id").is_none());
    }

    #[test]
    fn request_body_preserves_empty_reasoning_summary() {
        let value = json!({
            "model": "gpt-5.3-codex",
            "input": [
                {
                    "type": "reasoning",
                    "summary": [],
                    "encrypted_content": "abc"
                }
            ]
        });

        let body: RequestBody = serde_json::from_value(value).expect("request body should parse");
        let encoded = serde_json::to_value(body).expect("request body should serialize");
        assert!(encoded["input"][0].get("summary").is_some());
        assert_eq!(encoded["input"][0]["summary"], json!([]));
    }

    #[test]
    fn request_body_supports_tool_search_and_computer_tools() {
        let value = json!({
            "model": "gpt-5.4",
            "tool_choice": {
                "type": "computer"
            },
            "input": [
                {
                    "type": "message",
                    "role": "user",
                    "content": [
                        {
                            "type": "input_image",
                            "detail": "original",
                            "image_url": "https://example.com/screenshot.png"
                        },
                        {
                            "type": "input_file",
                            "detail": "low",
                            "file_url": "https://example.com/spec.pdf",
                            "filename": "spec.pdf"
                        }
                    ]
                }
            ],
            "tools": [
                {
                    "type": "function",
                    "name": "get_weather",
                    "parameters": {"type": "object"},
                    "strict": true,
                    "defer_loading": true
                },
                {
                    "type": "custom",
                    "name": "router",
                    "defer_loading": true,
                    "format": {"type": "text"}
                },
                {
                    "type": "namespace",
                    "name": "crm",
                    "description": "CRM tools",
                    "tools": [
                        {
                            "type": "function",
                            "name": "lookup",
                            "description": "Find records",
                            "parameters": {"type": "object"},
                            "strict": true
                        },
                        {
                            "type": "custom",
                            "name": "draft_email",
                            "defer_loading": true,
                            "format": {"type": "text"}
                        }
                    ]
                },
                {
                    "type": "tool_search",
                    "execution": "client",
                    "description": "Discover deferred tools",
                    "parameters": {"type": "object"}
                },
                {
                    "type": "computer"
                },
                {
                    "type": "web_search_preview",
                    "search_content_types": ["text", "image"],
                    "search_context_size": "medium",
                    "user_location": {
                        "type": "approximate",
                        "country": "US"
                    }
                }
            ]
        });

        let body: RequestBody = serde_json::from_value(value).expect("request body should parse");
        let encoded = serde_json::to_value(body).expect("request body should serialize");

        assert_eq!(encoded["tool_choice"]["type"], json!("computer"));
        assert_eq!(
            encoded["input"][0]["content"][0]["detail"],
            json!("original")
        );
        assert_eq!(encoded["input"][0]["content"][1]["detail"], json!("low"));
        assert_eq!(encoded["tools"][0]["defer_loading"], json!(true));
        assert_eq!(encoded["tools"][1]["defer_loading"], json!(true));
        assert_eq!(encoded["tools"][2]["type"], json!("namespace"));
        assert_eq!(encoded["tools"][3]["type"], json!("tool_search"));
        assert_eq!(encoded["tools"][4]["type"], json!("computer"));
        assert_eq!(
            encoded["tools"][5]["search_content_types"],
            json!(["text", "image"])
        );
    }
}
