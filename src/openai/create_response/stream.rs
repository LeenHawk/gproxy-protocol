use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::openai::create_response::response::ResponseBody as CreateResponseBody;
use crate::openai::create_response::types::{
    ResponseOutputItem, ResponseOutputRefusal, ResponseOutputText, ResponseReasoningTextContent,
    ResponseSummaryTextContent,
};

/// Parsed SSE stream body for `POST /responses` with `stream=true`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpenAiCreateResponseSseStreamBody {
    /// SSE events in receive order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<OpenAiCreateResponseSseEvent>,
}

/// A single SSE event frame.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenAiCreateResponseSseEvent {
    /// Optional SSE `event` field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// SSE `data` field payload.
    pub data: OpenAiCreateResponseSseData,
}

/// SSE `data` payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OpenAiCreateResponseSseData {
    /// A regular stream event object.
    Event(ResponseStreamEvent),
    /// Stream end marker (`[DONE]`).
    Done(String),
}

impl OpenAiCreateResponseSseData {
    pub fn is_done(&self) -> bool {
        matches!(self, Self::Done(marker) if marker == "[DONE]")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseStreamErrorPayload {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

impl ResponseStreamErrorPayload {
    pub fn code_or_type(&self) -> &str {
        self.code.as_deref().unwrap_or(self.type_.as_str())
    }
}

/// Stream event union documented by Responses API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseStreamEvent {
    #[serde(rename = "response.audio.delta")]
    AudioDelta {
        delta: String,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.audio.done")]
    AudioDone { sequence_number: u64 },
    #[serde(rename = "response.audio.transcript.delta")]
    AudioTranscriptDelta {
        delta: String,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.audio.transcript.done")]
    AudioTranscriptDone { sequence_number: u64 },

    #[serde(rename = "response.code_interpreter_call_code.delta")]
    CodeInterpreterCallCodeDelta {
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.code_interpreter_call_code.done")]
    CodeInterpreterCallCodeDone {
        code: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.code_interpreter_call.completed")]
    CodeInterpreterCallCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    CodeInterpreterCallInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    CodeInterpreterCallInterpreting {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.created")]
    Created {
        response: CreateResponseBody,
        sequence_number: u64,
    },
    #[serde(rename = "response.queued")]
    Queued {
        response: CreateResponseBody,
        sequence_number: u64,
    },
    #[serde(rename = "response.in_progress")]
    InProgress {
        response: CreateResponseBody,
        sequence_number: u64,
    },
    #[serde(rename = "response.failed")]
    Failed {
        response: CreateResponseBody,
        sequence_number: u64,
    },
    #[serde(rename = "response.incomplete")]
    Incomplete {
        response: CreateResponseBody,
        sequence_number: u64,
    },
    #[serde(rename = "response.completed")]
    Completed {
        response: CreateResponseBody,
        sequence_number: u64,
    },

    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        item: ResponseOutputItem,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        item: ResponseOutputItem,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        content_index: u64,
        item_id: String,
        output_index: u64,
        part: ResponseStreamContentPart,
        sequence_number: u64,
    },
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        content_index: u64,
        item_id: String,
        output_index: u64,
        part: ResponseStreamContentPart,
        sequence_number: u64,
    },

    #[serde(rename = "response.output_text.annotation.added")]
    OutputTextAnnotationAdded {
        annotation: Value,
        annotation_index: u64,
        content_index: u64,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        content_index: u64,
        delta: String,
        item_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<ResponseStreamTokenLogprob>>,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        content_index: u64,
        item_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<ResponseStreamTokenLogprob>>,
        output_index: u64,
        sequence_number: u64,
        text: String,
    },

    #[serde(rename = "response.refusal.delta")]
    RefusalDelta {
        content_index: u64,
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.refusal.done")]
    RefusalDone {
        content_index: u64,
        item_id: String,
        output_index: u64,
        refusal: String,
        sequence_number: u64,
    },

    #[serde(rename = "response.reasoning_text.delta")]
    ReasoningTextDelta {
        content_index: u64,
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.reasoning_text.done")]
    ReasoningTextDone {
        content_index: u64,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        text: String,
    },

    #[serde(rename = "response.reasoning_summary_part.added")]
    ReasoningSummaryPartAdded {
        item_id: String,
        output_index: u64,
        part: ResponseSummaryTextContent,
        sequence_number: u64,
        summary_index: u64,
    },
    #[serde(rename = "response.reasoning_summary_part.done")]
    ReasoningSummaryPartDone {
        item_id: String,
        output_index: u64,
        part: ResponseSummaryTextContent,
        sequence_number: u64,
        summary_index: u64,
    },
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ReasoningSummaryTextDelta {
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        summary_index: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.reasoning_summary_text.done")]
    ReasoningSummaryTextDone {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        summary_index: u64,
        text: String,
    },

    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        arguments: String,
        item_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.file_search_call.in_progress")]
    FileSearchCallInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.file_search_call.searching")]
    FileSearchCallSearching {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.file_search_call.completed")]
    FileSearchCallCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.web_search_call.in_progress")]
    WebSearchCallInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.web_search_call.searching")]
    WebSearchCallSearching {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.web_search_call.completed")]
    WebSearchCallCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.image_generation_call.in_progress")]
    ImageGenerationCallInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.image_generation_call.generating")]
    ImageGenerationCallGenerating {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.image_generation_call.partial_image")]
    ImageGenerationCallPartialImage {
        item_id: String,
        output_index: u64,
        partial_image_b64: String,
        partial_image_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.image_generation_call.completed")]
    ImageGenerationCallCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.mcp_call_arguments.delta")]
    McpCallArgumentsDelta {
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.mcp_call_arguments.done")]
    McpCallArgumentsDone {
        arguments: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.mcp_call.in_progress")]
    McpCallInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.mcp_call.completed")]
    McpCallCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.mcp_call.failed")]
    McpCallFailed {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.mcp_list_tools.in_progress")]
    McpListToolsInProgress {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.mcp_list_tools.completed")]
    McpListToolsCompleted {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },
    #[serde(rename = "response.mcp_list_tools.failed")]
    McpListToolsFailed {
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "response.custom_tool_call_input.delta")]
    CustomToolCallInputDelta {
        delta: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        obfuscation: Option<String>,
    },
    #[serde(rename = "response.custom_tool_call_input.done")]
    CustomToolCallInputDone {
        input: String,
        item_id: String,
        output_index: u64,
        sequence_number: u64,
    },

    #[serde(rename = "error")]
    Error {
        error: ResponseStreamErrorPayload,
        sequence_number: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseStreamContentPart {
    OutputText(ResponseOutputText),
    Refusal(ResponseOutputRefusal),
    ReasoningText(ResponseReasoningTextContent),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseStreamTokenLogprob {
    pub token: String,
    pub logprob: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<Vec<ResponseStreamTopLogprob>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResponseStreamTopLogprob {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprob: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::ResponseStreamEvent;

    #[test]
    fn function_call_arguments_done_name_is_optional_when_decoding() {
        let event = serde_json::from_str::<ResponseStreamEvent>(
            r#"{
                "type":"response.function_call_arguments.done",
                "arguments":"{}",
                "item_id":"fc_123",
                "output_index":0,
                "sequence_number":7
            }"#,
        )
        .unwrap();

        match event {
            ResponseStreamEvent::FunctionCallArgumentsDone { name, .. } => {
                assert_eq!(name, None);
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn error_event_accepts_nested_error_object() {
        let event = serde_json::from_str::<ResponseStreamEvent>(
            r#"{
                "type":"error",
                "error":{
                    "type":"invalid_request_error",
                    "code":"context_length_exceeded",
                    "message":"input too long",
                    "param":"input"
                },
                "sequence_number":2
            }"#,
        )
        .unwrap();

        match event {
            ResponseStreamEvent::Error { error, .. } => {
                assert_eq!(error.type_, "invalid_request_error");
                assert_eq!(error.code.as_deref(), Some("context_length_exceeded"));
                assert_eq!(error.message, "input too long");
                assert_eq!(error.param.as_deref(), Some("input"));
            }
            other => panic!("unexpected event: {other:?}"),
        }
    }

    #[test]
    fn error_event_rejects_flat_error_shape() {
        let err = serde_json::from_str::<ResponseStreamEvent>(
            r#"{
                "type":"error",
                "code":"context_length_exceeded",
                "message":"input too long",
                "param":"input",
                "sequence_number":2
            }"#,
        )
        .unwrap_err();

        assert!(err.to_string().contains("missing field `error`"));
    }
}
