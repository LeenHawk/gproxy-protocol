use std::collections::BTreeMap;

use crate::openai::count_tokens::types as ot;
use crate::openai::create_response::response::ResponseBody;
use crate::openai::create_response::stream::ResponseStreamEvent;
use crate::openai::create_response::types as rt;

pub fn next_sequence_number(next_sequence_number: &mut u64) -> u64 {
    let sequence_number = *next_sequence_number;
    *next_sequence_number = next_sequence_number.saturating_add(1);
    sequence_number
}

pub fn push_stream_event(
    out: &mut Vec<ResponseStreamEvent>,
    stream_event: ResponseStreamEvent,
) {
    out.push(stream_event);
}

pub fn push_done_event(_out: &mut Vec<ResponseStreamEvent>) {
    // No-op: the wrapper SSE envelope no longer exists.
    // Callers that previously appended a Done sentinel can simply stop.
}

pub fn response_usage_from_counts(
    input_tokens: u64,
    cached_tokens: u64,
    output_tokens: u64,
    reasoning_tokens: u64,
) -> rt::ResponseUsage {
    rt::ResponseUsage {
        input_tokens,
        input_tokens_details: rt::ResponseInputTokensDetails { cached_tokens },
        output_tokens,
        output_tokens_details: rt::ResponseOutputTokensDetails { reasoning_tokens },
        total_tokens: input_tokens.saturating_add(output_tokens),
    }
}

pub fn response_snapshot(
    id: &str,
    model: &str,
    status: Option<rt::ResponseStatus>,
    usage: Option<rt::ResponseUsage>,
    incomplete_reason: Option<rt::ResponseIncompleteReason>,
    error: Option<rt::ResponseError>,
    output_text: Option<String>,
) -> ResponseBody {
    ResponseBody {
        id: id.to_string(),
        created_at: 0,
        error,
        incomplete_details: incomplete_reason.map(|reason| rt::ResponseIncompleteDetails {
            reason: Some(reason),
        }),
        instructions: Some(ot::ResponseInput::Text(String::new())),
        metadata: BTreeMap::new(),
        model: model.to_string(),
        object: rt::ResponseObject::Response,
        output: Vec::new(),
        parallel_tool_calls: false,
        temperature: 1.0,
        tool_choice: ot::ResponseToolChoice::Options(ot::ResponseToolChoiceOptions::Auto),
        tools: Vec::new(),
        top_p: 1.0,
        background: None,
        completed_at: None,
        conversation: None,
        max_output_tokens: None,
        max_tool_calls: None,
        output_text: output_text.filter(|text| !text.is_empty()),
        previous_response_id: None,
        prompt: None,
        prompt_cache_key: None,
        prompt_cache_retention: None,
        reasoning: None,
        safety_identifier: None,
        service_tier: None,
        status,
        text: None,
        top_logprobs: None,
        truncation: None,
        usage,
        user: None,
    }
}
