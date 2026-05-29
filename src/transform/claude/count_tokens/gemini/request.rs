use crate::claude::count_tokens::request::ClaudeCountTokensRequest;
use crate::claude::count_tokens::types::{
    BetaContentBlockParam, BetaMessageContent, BetaMessageRole,
};
use crate::gemini::count_tokens::request::{
    GeminiCountTokensRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::count_tokens::types::{
    GeminiContent, GeminiContentRole, GeminiGenerateContentRequest, GeminiGenerationConfig,
    GeminiPart, HttpMethod,
};
use crate::transform::claude::count_tokens::utils::{
    beta_message_content_to_text, beta_mid_conversation_system_block_to_text,
    claude_model_to_string,
};
use crate::transform::claude::generate_content::gemini::utils::{
    gemini_system_instruction_from_claude, gemini_thinking_config_from_claude,
    gemini_tool_config_from_claude, gemini_tools_from_claude,
};
use crate::transform::claude::model_list::gemini::utils::ensure_models_prefix;
use crate::transform::utils::TransformError;

fn gemini_text_content(role: GeminiContentRole, text: String) -> Option<GeminiContent> {
    if text.is_empty() {
        return None;
    }

    Some(GeminiContent {
        parts: vec![GeminiPart {
            text: Some(text),
            ..GeminiPart::default()
        }],
        role: Some(role),
    })
}

impl TryFrom<ClaudeCountTokensRequest> for GeminiCountTokensRequest {
    type Error = TransformError;

    fn try_from(value: ClaudeCountTokensRequest) -> Result<Self, TransformError> {
        let model = ensure_models_prefix(&claude_model_to_string(&value.body.model));
        let contents = value
            .body
            .messages
            .into_iter()
            .flat_map(|message| {
                let role = match message.role {
                    BetaMessageRole::User => GeminiContentRole::User,
                    BetaMessageRole::Assistant => GeminiContentRole::Model,
                    BetaMessageRole::System => GeminiContentRole::User,
                };

                match message.content {
                    BetaMessageContent::Text(text) => gemini_text_content(role, text)
                        .into_iter()
                        .collect::<Vec<_>>(),
                    BetaMessageContent::Blocks(blocks) => {
                        let fallback_text = beta_message_content_to_text(
                            &BetaMessageContent::Blocks(blocks.clone()),
                        );
                        let mut contents = Vec::new();
                        let mut text_parts = Vec::new();

                        for block in blocks {
                            match block {
                                BetaContentBlockParam::MidConversationSystem(block) => {
                                    if let Some(content) =
                                        gemini_text_content(role.clone(), text_parts.join("\n"))
                                    {
                                        contents.push(content);
                                    }
                                    text_parts.clear();
                                    if let Some(content) = gemini_text_content(
                                        GeminiContentRole::User,
                                        beta_mid_conversation_system_block_to_text(&block),
                                    ) {
                                        contents.push(content);
                                    }
                                }
                                other => {
                                    let text = beta_message_content_to_text(
                                        &BetaMessageContent::Blocks(vec![other]),
                                    );
                                    if !text.is_empty() {
                                        text_parts.push(text);
                                    }
                                }
                            }
                        }

                        if let Some(content) =
                            gemini_text_content(role.clone(), text_parts.join("\n"))
                        {
                            contents.push(content);
                        }
                        if contents.is_empty()
                            && let Some(content) = gemini_text_content(role, fallback_text)
                        {
                            contents.push(content);
                        }

                        contents
                    }
                }
            })
            .collect::<Vec<_>>();
        let tools = gemini_tools_from_claude(value.body.tools, false);
        let tool_config = gemini_tool_config_from_claude(value.body.tool_choice);
        let thinking_config = gemini_thinking_config_from_claude(
            value.body.thinking,
            value
                .body
                .output_config
                .as_ref()
                .and_then(|config| config.effort.as_ref()),
        );
        let system_instruction = gemini_system_instruction_from_claude(value.body.system);
        let json_output_requested = value
            .body
            .output_config
            .as_ref()
            .and_then(|config| config.format.as_ref())
            .is_some();
        let generation_config = if thinking_config.is_some() || json_output_requested {
            Some(GeminiGenerationConfig {
                response_mime_type: if json_output_requested {
                    Some("application/json".to_string())
                } else {
                    None
                },
                thinking_config,
                ..GeminiGenerationConfig::default()
            })
        } else {
            None
        };

        Ok(Self {
            method: HttpMethod::Post,
            path: PathParameters {
                model: model.clone(),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                contents: None,
                generate_content_request: Some(GeminiGenerateContentRequest {
                    model,
                    contents,
                    tools,
                    tool_config,
                    safety_settings: None,
                    system_instruction,
                    generation_config,
                    cached_content: None,
                }),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claude::count_tokens::request as claude_request;
    use crate::claude::count_tokens::types as ct;

    #[test]
    fn mid_conversation_system_block_becomes_gemini_count_user_content() {
        let request = ClaudeCountTokensRequest {
            method: ct::HttpMethod::Post,
            path: claude_request::PathParameters::default(),
            query: claude_request::QueryParameters::default(),
            headers: claude_request::RequestHeaders::default(),
            body: claude_request::RequestBody {
                messages: vec![ct::BetaMessageParam {
                    role: ct::BetaMessageRole::User,
                    content: ct::BetaMessageContent::Blocks(vec![
                        ct::BetaContentBlockParam::Text(ct::BetaTextBlockParam {
                            text: "First user turn".to_string(),
                            type_: ct::BetaTextBlockType::Text,
                            cache_control: None,
                            citations: None,
                        }),
                        ct::BetaContentBlockParam::MidConversationSystem(
                            ct::BetaMidConversationSystemBlockParam {
                                content: vec![ct::BetaTextBlockParam {
                                    text: "Apply the new policy now.".to_string(),
                                    type_: ct::BetaTextBlockType::Text,
                                    cache_control: None,
                                    citations: None,
                                }],
                                type_: ct::BetaMidConversationSystemBlockType::MidConvSystem,
                                cache_control: None,
                            },
                        ),
                        ct::BetaContentBlockParam::Text(ct::BetaTextBlockParam {
                            text: "Second user turn".to_string(),
                            type_: ct::BetaTextBlockType::Text,
                            cache_control: None,
                            citations: None,
                        }),
                    ]),
                }],
                model: ct::Model::Custom("claude-test".to_string()),
                context_management: None,
                mcp_servers: None,
                cache_control: None,
                output_config: None,
                speed: None,
                system: None,
                thinking: None,
                tool_choice: None,
                tools: None,
            },
        };

        let converted = GeminiCountTokensRequest::try_from(request).expect("request converts");
        let contents = converted
            .body
            .generate_content_request
            .expect("generate content request")
            .contents;

        assert_eq!(contents.len(), 3);
        assert_eq!(
            contents[0].parts[0].text.as_deref(),
            Some("First user turn")
        );
        assert_eq!(
            contents[1].parts[0].text.as_deref(),
            Some("Apply the new policy now.")
        );
        assert_eq!(contents[1].role, Some(GeminiContentRole::User));
        assert_eq!(
            contents[2].parts[0].text.as_deref(),
            Some("Second user turn")
        );
        assert_eq!(contents[2].role, Some(GeminiContentRole::User));
    }
}
