use std::collections::BTreeMap;

use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::GeminiCandidate;
use crate::gemini::stream_generate_content::request::GeminiStreamGenerateContentRequest;
use crate::transform::utils::TransformError;

fn merge_candidate(target: &mut GeminiCandidate, incoming: GeminiCandidate, index: u32) {
    target.index = Some(index);

    if let Some(content) = incoming.content {
        if let Some(target_content) = target.content.as_mut() {
            if target_content.role.is_none() {
                target_content.role = content.role;
            }
            target_content.parts.extend(content.parts);
        } else {
            target.content = Some(content);
        }
    }

    if incoming.finish_reason.is_some() {
        target.finish_reason = incoming.finish_reason;
    }
    if incoming.safety_ratings.is_some() {
        target.safety_ratings = incoming.safety_ratings;
    }
    if incoming.citation_metadata.is_some() {
        target.citation_metadata = incoming.citation_metadata;
    }
    if incoming.token_count.is_some() {
        target.token_count = incoming.token_count;
    }
    if let Some(grounding_attributions) = incoming.grounding_attributions {
        if let Some(existing) = target.grounding_attributions.as_mut() {
            existing.extend(grounding_attributions);
        } else {
            target.grounding_attributions = Some(grounding_attributions);
        }
    }
    if incoming.grounding_metadata.is_some() {
        target.grounding_metadata = incoming.grounding_metadata;
    }
    if incoming.avg_logprobs.is_some() {
        target.avg_logprobs = incoming.avg_logprobs;
    }
    if incoming.logprobs_result.is_some() {
        target.logprobs_result = incoming.logprobs_result;
    }
    if incoming.url_context_metadata.is_some() {
        target.url_context_metadata = incoming.url_context_metadata;
    }
    if incoming.finish_message.is_some() {
        target.finish_message = incoming.finish_message;
    }
}

pub fn merge_chunk(
    merged: &mut GeminiGenerateContentResponseBody,
    candidate_map: &mut BTreeMap<u32, GeminiCandidate>,
    chunk: GeminiGenerateContentResponseBody,
) {
    if let Some(candidates) = chunk.candidates {
        for (pos, candidate) in candidates.into_iter().enumerate() {
            let index = candidate.index.unwrap_or(pos as u32);
            let entry = candidate_map
                .entry(index)
                .or_insert_with(|| GeminiCandidate {
                    index: Some(index),
                    ..GeminiCandidate::default()
                });
            merge_candidate(entry, candidate, index);
        }
    }

    if chunk.prompt_feedback.is_some() {
        merged.prompt_feedback = chunk.prompt_feedback;
    }
    if chunk.usage_metadata.is_some() {
        merged.usage_metadata = chunk.usage_metadata;
    }
    if chunk.model_version.is_some() {
        merged.model_version = chunk.model_version;
    }
    if chunk.response_id.is_some() {
        merged.response_id = chunk.response_id;
    }
    if chunk.model_status.is_some() {
        merged.model_status = chunk.model_status;
    }
}

pub fn finalize_body(
    mut merged: GeminiGenerateContentResponseBody,
    candidate_map: BTreeMap<u32, GeminiCandidate>,
) -> GeminiGenerateContentResponseBody {
    if candidate_map.is_empty() {
        merged.candidates = None;
    } else {
        merged.candidates = Some(candidate_map.into_values().collect());
    }
    merged
}

impl TryFrom<&GeminiStreamGenerateContentRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiStreamGenerateContentRequest) -> Result<Self, TransformError> {
        Ok(GeminiGenerateContentRequest {
            method: value.method,
            path: crate::gemini::generate_content::request::PathParameters {
                model: value.path.model.clone(),
            },
            query: crate::gemini::generate_content::request::QueryParameters::default(),
            headers: crate::gemini::generate_content::request::RequestHeaders::default(),
            body: value.body.clone(),
        })
    }
}

impl TryFrom<GeminiStreamGenerateContentRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: GeminiStreamGenerateContentRequest) -> Result<Self, TransformError> {
        GeminiGenerateContentRequest::try_from(&value)
    }
}
