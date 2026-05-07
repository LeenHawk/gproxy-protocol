use crate::claude::count_tokens::types as ct;
use crate::openai::create_chat_completions::types as oct;

pub fn text_block(text: String) -> ct::BetaContentBlockParam {
    ct::BetaContentBlockParam::Text(ct::BetaTextBlockParam {
        text,
        type_: ct::BetaTextBlockType::Text,
        cache_control: None,
        citations: None,
    })
}

pub fn system_text_block(text: String) -> ct::BetaTextBlockParam {
    ct::BetaTextBlockParam {
        text,
        type_: ct::BetaTextBlockType::Text,
        cache_control: None,
        citations: None,
    }
}

pub fn parse_tool_use_input(input: String) -> ct::JsonObject {
    serde_json::from_str::<ct::JsonObject>(&input).unwrap_or_else(|_| {
        let escaped = serde_json::to_string(&input).unwrap_or_else(|_| "\"\"".to_string());
        serde_json::from_str::<ct::JsonObject>(&format!(r#"{{"input":{escaped}}}"#))
            .unwrap_or_default()
    })
}

pub fn server_tool_name(name: &ct::BetaServerToolUseName) -> String {
    match name {
        ct::BetaServerToolUseName::WebSearch => "web_search".to_string(),
        ct::BetaServerToolUseName::WebFetch => "web_fetch".to_string(),
        ct::BetaServerToolUseName::CodeExecution => "code_execution".to_string(),
        ct::BetaServerToolUseName::BashCodeExecution => "bash_code_execution".to_string(),
        ct::BetaServerToolUseName::TextEditorCodeExecution => {
            "text_editor_code_execution".to_string()
        }
        ct::BetaServerToolUseName::ToolSearchToolRegex => "tool_search_tool_regex".to_string(),
        ct::BetaServerToolUseName::ToolSearchToolBm25 => "tool_search_tool_bm25".to_string(),
    }
}

pub fn stdout_stderr_text(stdout: String, stderr: String) -> String {
    if stderr.is_empty() {
        stdout
    } else if stdout.is_empty() {
        stderr
    } else {
        format!("stdout: {stdout}\nstderr: {stderr}")
    }
}

fn non_empty(value: Option<String>) -> Option<String> {
    value.filter(|text| !text.is_empty())
}

fn reasoning_detail_signature(detail: &oct::ChatCompletionReasoningDetail) -> Option<String> {
    non_empty(detail.signature.clone()).or_else(|| non_empty(detail.id.clone()))
}

fn reasoning_detail_to_claude_block(
    detail: oct::ChatCompletionReasoningDetail,
) -> Option<ct::BetaContentBlockParam> {
    match detail.type_ {
        oct::ChatCompletionReasoningDetailType::ReasoningEncrypted => {
            non_empty(detail.data).map(|data| {
                ct::BetaContentBlockParam::RedactedThinking(ct::BetaRedactedThinkingBlockParam {
                    data,
                    type_: ct::BetaRedactedThinkingBlockType::RedactedThinking,
                })
            })
        }
        oct::ChatCompletionReasoningDetailType::ReasoningSummary
        | oct::ChatCompletionReasoningDetailType::ReasoningText => {
            let signature = reasoning_detail_signature(&detail)?;
            let thinking = non_empty(detail.text)?;
            Some(ct::BetaContentBlockParam::Thinking(
                ct::BetaThinkingBlockParam {
                    signature,
                    thinking,
                    type_: ct::BetaThinkingBlockType::Thinking,
                },
            ))
        }
    }
}

pub fn chat_reasoning_to_claude_blocks(
    reasoning_content: Option<String>,
    reasoning_details: Option<Vec<oct::ChatCompletionReasoningDetail>>,
) -> Vec<ct::BetaContentBlockParam> {
    let content_signature = reasoning_details
        .as_ref()
        .and_then(|details| details.iter().find_map(reasoning_detail_signature));
    let mut blocks = Vec::new();

    if let (Some(thinking), Some(signature)) = (non_empty(reasoning_content), content_signature) {
        blocks.push(ct::BetaContentBlockParam::Thinking(
            ct::BetaThinkingBlockParam {
                signature,
                thinking,
                type_: ct::BetaThinkingBlockType::Thinking,
            },
        ));
    }

    if let Some(reasoning_details) = reasoning_details {
        blocks.extend(
            reasoning_details
                .into_iter()
                .filter_map(reasoning_detail_to_claude_block),
        );
    }

    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_reasoning_details_map_to_claude_thinking_and_redacted_blocks() {
        let blocks = chat_reasoning_to_claude_blocks(
            Some("visible chain".to_string()),
            Some(vec![
                oct::ChatCompletionReasoningDetail {
                    type_: oct::ChatCompletionReasoningDetailType::ReasoningText,
                    id: Some("reasoning_1".to_string()),
                    data: None,
                    text: None,
                    signature: Some("sig_text".to_string()),
                    index: Some(0),
                },
                oct::ChatCompletionReasoningDetail {
                    type_: oct::ChatCompletionReasoningDetailType::ReasoningEncrypted,
                    id: Some("enc_1".to_string()),
                    data: Some("ciphertext".to_string()),
                    text: None,
                    signature: None,
                    index: Some(1),
                },
            ]),
        );

        assert!(matches!(
            &blocks[0],
            ct::BetaContentBlockParam::Thinking(block)
                if block.signature == "sig_text" && block.thinking == "visible chain"
        ));
        assert!(matches!(
            &blocks[1],
            ct::BetaContentBlockParam::RedactedThinking(block)
                if block.data == "ciphertext"
        ));
    }
}
