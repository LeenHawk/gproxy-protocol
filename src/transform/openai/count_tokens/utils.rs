use crate::openai::count_tokens::types::{
    ResponseFunctionCallOutputContent, ResponseInput, ResponseInputContent, ResponseInputItem,
    ResponseInputMessage, ResponseInputMessageContent, ResponseInputMessageRole,
    ResponseInputMessageType, ResponseSummaryTextContent,
};

pub fn openai_input_to_items(input: Option<ResponseInput>) -> Vec<ResponseInputItem> {
    match input {
        Some(ResponseInput::Items(items)) => items,
        Some(ResponseInput::Text(text)) => vec![ResponseInputItem::Message(ResponseInputMessage {
            content: ResponseInputMessageContent::Text(text),
            role: ResponseInputMessageRole::User,
            phase: None,
            status: None,
            type_: Some(ResponseInputMessageType::Message),
        })],
        None => Vec::new(),
    }
}

pub fn openai_input_content_to_text(content: &ResponseInputContent) -> String {
    match content {
        ResponseInputContent::Text(part) => part.text.clone(),
        ResponseInputContent::Image(part) => part
            .image_url
            .clone()
            .or(part.file_id.clone())
            .unwrap_or_else(|| "[input_image]".to_string()),
        ResponseInputContent::File(part) => part
            .file_url
            .clone()
            .or(part.file_id.clone())
            .or(part.filename.clone())
            .or(part.file_data.clone())
            .unwrap_or_else(|| "[input_file]".to_string()),
    }
}

pub fn openai_message_content_to_text(content: &ResponseInputMessageContent) -> String {
    match content {
        ResponseInputMessageContent::Text(text) => text.clone(),
        ResponseInputMessageContent::List(parts) => parts
            .iter()
            .map(openai_input_content_to_text)
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn openai_function_call_output_content_to_text(
    content: &ResponseFunctionCallOutputContent,
) -> String {
    match content {
        ResponseFunctionCallOutputContent::Text(text) => text.clone(),
        ResponseFunctionCallOutputContent::Content(parts) => parts
            .iter()
            .map(openai_input_content_to_text)
            .collect::<Vec<_>>()
            .join("\n"),
    }
}

pub fn openai_reasoning_summary_to_text(summary: &[ResponseSummaryTextContent]) -> String {
    summary
        .iter()
        .map(|item| item.text.clone())
        .filter(|text| !text.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}
