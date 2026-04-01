use crate::claude::count_tokens::types as ct;

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
