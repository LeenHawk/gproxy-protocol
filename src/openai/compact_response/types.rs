use serde::{Deserialize, Serialize};

pub use crate::openai::count_tokens::types::{
    JsonObject, ResponseApplyPatchCall, ResponseApplyPatchCallOutput,
    ResponseCodeInterpreterToolCall, ResponseCompactionItemParam, ResponseComputerCallOutput,
    ResponseComputerToolCall, ResponseComputerToolCallOutputScreenshot, ResponseCustomToolCall,
    ResponseCustomToolCallOutput, ResponseFileSearchToolCall, ResponseFunctionCallOutput,
    ResponseFunctionToolCall, ResponseFunctionWebSearch, ResponseInput, ResponseInputFile,
    ResponseInputImage, ResponseInputItem, ResponseInputText, ResponseItemReference,
    ResponseItemStatus, ResponseLocalShellCall, ResponseLocalShellCallOutput,
    ResponseMcpApprovalRequest, ResponseMcpApprovalResponse, ResponseMcpCall, ResponseMcpListTools,
    ResponseOutputRefusal, ResponseOutputText, ResponseReasoningItem, ResponseReasoningTextContent,
    ResponseShellCall, ResponseShellCallOutput, ResponseSummaryTextContent, ResponseToolSearchCall,
    ResponseToolSearchOutput,
};
pub use crate::openai::types::{
    HttpMethod, OpenAiApiError, OpenAiApiErrorResponse, OpenAiResponseHeaders,
};

/// Compacted output item union.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompactedResponseOutputItem {
    Message(CompactedResponseMessage),
    FileSearchToolCall(ResponseFileSearchToolCall),
    ComputerToolCall(ResponseComputerToolCall),
    ComputerCallOutput(ResponseComputerCallOutput),
    ToolSearchCall(ResponseToolSearchCall),
    ToolSearchOutput(ResponseToolSearchOutput),
    FunctionWebSearch(ResponseFunctionWebSearch),
    FunctionToolCall(ResponseFunctionToolCall),
    FunctionCallOutput(ResponseFunctionCallOutput),
    ReasoningItem(ResponseReasoningItem),
    CompactionItem(ResponseCompactionItemParam),
    CodeInterpreterToolCall(ResponseCodeInterpreterToolCall),
    LocalShellCall(ResponseLocalShellCall),
    LocalShellCallOutput(ResponseLocalShellCallOutput),
    ShellCall(ResponseShellCall),
    ShellCallOutput(ResponseShellCallOutput),
    ApplyPatchCall(ResponseApplyPatchCall),
    ApplyPatchCallOutput(ResponseApplyPatchCallOutput),
    McpListTools(ResponseMcpListTools),
    McpApprovalRequest(ResponseMcpApprovalRequest),
    McpApprovalResponse(ResponseMcpApprovalResponse),
    McpCall(ResponseMcpCall),
    CustomToolCallOutput(ResponseCustomToolCallOutput),
    CustomToolCall(ResponseCustomToolCall),
    ItemReference(ResponseItemReference),
}

/// Message item emitted in compacted output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompactedResponseMessage {
    pub id: String,
    pub content: Vec<CompactedResponseMessageContent>,
    pub role: CompactedResponseMessageRole,
    pub status: ResponseItemStatus,
    #[serde(rename = "type")]
    pub type_: CompactedResponseMessageType,
}

/// Message content item union in compacted output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompactedResponseMessageContent {
    InputText(ResponseInputText),
    OutputText(ResponseOutputText),
    Text(CompactedResponseTextContent),
    SummaryText(ResponseSummaryTextContent),
    ReasoningText(ResponseReasoningTextContent),
    Refusal(ResponseOutputRefusal),
    InputImage(ResponseInputImage),
    ComputerScreenshot(ResponseComputerToolCallOutputScreenshot),
    InputFile(ResponseInputFile),
}

/// Generic text content block (`type=text`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompactedResponseTextContent {
    pub text: String,
    #[serde(rename = "type")]
    pub type_: CompactedResponseTextContentType,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactedResponseTextContentType {
    #[serde(rename = "text")]
    Text,
}

/// Role discriminator for compacted message items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactedResponseMessageRole {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "critic")]
    Critic,
    #[serde(rename = "discriminator")]
    Discriminator,
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "tool")]
    Tool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompactedResponseMessageType {
    #[serde(rename = "message")]
    Message,
}

/// Token accounting for compaction responses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseUsage {
    pub input_tokens: u64,
    pub input_tokens_details: ResponseInputTokensDetails,
    pub output_tokens: u64,
    pub output_tokens_details: ResponseOutputTokensDetails,
    pub total_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseInputTokensDetails {
    pub cached_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseOutputTokensDetails {
    pub reasoning_tokens: u64,
}
