use crate::gemini::count_tokens::types::GeminiFunctionResponse;
use crate::gemini::generate_content::types as gt;
use crate::openai::create_chat_completions::types as ct;

pub fn parse_tool_use_args(input: String) -> gt::JsonObject {
    serde_json::from_str::<gt::JsonObject>(&input).unwrap_or_else(|_| {
        let escaped = serde_json::to_string(&input).unwrap_or_else(|_| "\"\"".to_string());
        serde_json::from_str::<gt::JsonObject>(&format!(r#"{{"input":{escaped}}}"#))
            .unwrap_or_default()
    })
}

pub fn prompt_feedback_refusal_text(feedback: Option<&gt::GeminiPromptFeedback>) -> Option<String> {
    let reason = feedback.and_then(|feedback| feedback.block_reason.as_ref())?;
    Some(match reason {
        gt::GeminiBlockReason::Safety => "blocked_by_safety".to_string(),
        gt::GeminiBlockReason::Other => "blocked".to_string(),
        gt::GeminiBlockReason::Blocklist => "blocked_by_blocklist".to_string(),
        gt::GeminiBlockReason::ProhibitedContent => "blocked_by_prohibited_content".to_string(),
        gt::GeminiBlockReason::ImageSafety => "blocked_by_image_safety".to_string(),
        gt::GeminiBlockReason::BlockReasonUnspecified => String::new(),
    })
}

pub fn json_object_to_string(value: &gt::JsonObject) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
}

pub fn gemini_function_response_to_text(function_response: GeminiFunctionResponse) -> String {
    let mut lines = vec![json_object_to_string(&function_response.response)];
    if let Some(parts) = function_response.parts {
        for part in parts {
            if let Some(inline_data) = part.inline_data {
                lines.push(format!(
                    "data:{};base64,{}",
                    inline_data.mime_type, inline_data.data
                ));
            }
        }
    }

    lines
        .into_iter()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn gemini_citation_annotations(
    citation_metadata: Option<&gt::GeminiCitationMetadata>,
) -> Vec<ct::ChatCompletionAnnotation> {
    citation_metadata
        .and_then(|metadata| metadata.citation_sources.as_ref())
        .map(|sources| {
            sources
                .iter()
                .filter_map(|source| {
                    let url = source.uri.clone()?;
                    Some(ct::ChatCompletionAnnotation {
                        type_: ct::ChatCompletionAnnotationType::UrlCitation,
                        url_citation: ct::ChatCompletionUrlCitation {
                            start_index: source.start_index.unwrap_or(0) as u64,
                            end_index: source.end_index.unwrap_or(0) as u64,
                            title: url.clone(),
                            url,
                        },
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn gemini_logprobs(
    result: Option<&gt::GeminiLogprobsResult>,
) -> Option<ct::ChatCompletionLogprobs> {
    let result = result?;
    let chosen_candidates = result.chosen_candidates.as_ref()?;
    let top_candidates = result.top_candidates.as_ref();

    let mut content = Vec::new();
    for (index, chosen) in chosen_candidates.iter().enumerate() {
        let token = chosen.token.clone().unwrap_or_default();
        if token.is_empty() {
            continue;
        }

        let top_logprobs = top_candidates
            .and_then(|list| list.get(index))
            .and_then(|item| item.candidates.as_ref())
            .map(|candidates| {
                candidates
                    .iter()
                    .filter_map(|candidate| {
                        let token = candidate.token.clone()?;
                        Some(ct::ChatCompletionTopLogprob {
                            token,
                            bytes: None,
                            logprob: candidate.log_probability.unwrap_or_default(),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        content.push(ct::ChatCompletionTokenLogprob {
            token,
            bytes: None,
            logprob: chosen.log_probability.unwrap_or_default(),
            top_logprobs,
        });
    }

    if content.is_empty() {
        None
    } else {
        Some(ct::ChatCompletionLogprobs {
            content: Some(content),
            refusal: None,
        })
    }
}
