use http::StatusCode;

use crate::gemini::count_tokens::types::{GeminiContentRole, GeminiFunctionCall, GeminiPart};
use crate::gemini::generate_content::response::GeminiGenerateContentResponse;
use crate::gemini::generate_content::response::ResponseBody as GeminiGenerateContentResponseBody;
use crate::gemini::generate_content::types::{
    GeminiCandidate, GeminiContent, GeminiFinishReason, GeminiUsageMetadata,
};
use crate::gemini::live::response::GeminiLiveMessageResponse;
use crate::gemini::live::types::GeminiBidiGenerateContentServerMessageType;
use crate::gemini::live::types::GeminiLiveUsageMetadata;
use crate::gemini::stream_generate_content::response::GeminiStreamGenerateContentResponse;
use crate::gemini::types::GeminiResponseHeaders;
use crate::transform::gemini::websocket::context::GeminiWebsocketTransformContext;
use crate::transform::utils::TransformError;

fn usage_live_to_generate(usage: Option<GeminiLiveUsageMetadata>) -> Option<GeminiUsageMetadata> {
    usage.map(|usage| GeminiUsageMetadata {
        prompt_token_count: usage.prompt_token_count,
        cached_content_token_count: usage.cached_content_token_count,
        candidates_token_count: usage.response_token_count,
        tool_use_prompt_token_count: usage.tool_use_prompt_token_count,
        thoughts_token_count: usage.thoughts_token_count,
        total_token_count: usage.total_token_count,
        prompt_tokens_details: usage.prompt_tokens_details,
        cache_tokens_details: usage.cache_tokens_details,
        candidates_tokens_details: usage.response_tokens_details,
        tool_use_prompt_tokens_details: usage.tool_use_prompt_tokens_details,
    })
}

fn server_message_to_chunk(
    message: crate::gemini::live::types::GeminiBidiGenerateContentServerMessage,
    ctx: &mut GeminiWebsocketTransformContext,
) -> Option<GeminiGenerateContentResponseBody> {
    let usage_metadata = usage_live_to_generate(message.usage_metadata);

    match message.message_type {
        GeminiBidiGenerateContentServerMessageType::SetupComplete { .. } => {
            ctx.push_warning(
                "gemini websocket to_http response: dropped setupComplete event".to_string(),
            );
            usage_metadata.map(|usage| GeminiGenerateContentResponseBody {
                usage_metadata: Some(usage),
                ..GeminiGenerateContentResponseBody::default()
            })
        }
        GeminiBidiGenerateContentServerMessageType::GoAway { go_away } => {
            ctx.push_warning(format!(
                "gemini websocket to_http response: dropped goAway event (timeLeft={})",
                go_away.time_left
            ));
            usage_metadata.map(|usage| GeminiGenerateContentResponseBody {
                usage_metadata: Some(usage),
                ..GeminiGenerateContentResponseBody::default()
            })
        }
        GeminiBidiGenerateContentServerMessageType::SessionResumptionUpdate { .. } => {
            ctx.push_warning(
                "gemini websocket to_http response: dropped sessionResumptionUpdate event"
                    .to_string(),
            );
            usage_metadata.map(|usage| GeminiGenerateContentResponseBody {
                usage_metadata: Some(usage),
                ..GeminiGenerateContentResponseBody::default()
            })
        }
        GeminiBidiGenerateContentServerMessageType::ToolCallCancellation { .. } => {
            ctx.push_warning(
                "gemini websocket to_http response: dropped toolCallCancellation event".to_string(),
            );
            usage_metadata.map(|usage| GeminiGenerateContentResponseBody {
                usage_metadata: Some(usage),
                ..GeminiGenerateContentResponseBody::default()
            })
        }
        GeminiBidiGenerateContentServerMessageType::ServerContent { server_content } => {
            if server_content.interrupted == Some(true) {
                ctx.push_warning(
                    "gemini websocket to_http response: dropped interrupted=true flag".to_string(),
                );
            }
            if server_content.input_transcription.is_some() {
                ctx.push_warning(
                    "gemini websocket to_http response: dropped inputTranscription".to_string(),
                );
            }
            if server_content.output_transcription.is_some() {
                ctx.push_warning(
                    "gemini websocket to_http response: dropped outputTranscription".to_string(),
                );
            }
            let candidates = server_content.model_turn.map(|model_turn| {
                vec![GeminiCandidate {
                    content: Some(model_turn),
                    finish_reason: if server_content.generation_complete == Some(true)
                        || server_content.turn_complete == Some(true)
                    {
                        Some(GeminiFinishReason::Stop)
                    } else {
                        None
                    },
                    grounding_metadata: server_content.grounding_metadata,
                    url_context_metadata: server_content.url_context_metadata,
                    index: Some(0),
                    ..GeminiCandidate::default()
                }]
            });

            if candidates.is_none() && usage_metadata.is_none() {
                return None;
            }

            Some(GeminiGenerateContentResponseBody {
                candidates,
                usage_metadata,
                ..GeminiGenerateContentResponseBody::default()
            })
        }
        GeminiBidiGenerateContentServerMessageType::ToolCall { tool_call } => {
            let calls = tool_call.function_calls.unwrap_or_default();
            if calls.is_empty() && usage_metadata.is_none() {
                return None;
            }

            let model_turn = GeminiContent {
                role: Some(GeminiContentRole::Model),
                parts: calls
                    .into_iter()
                    .map(|call| GeminiPart {
                        function_call: Some(GeminiFunctionCall {
                            id: call.id,
                            name: call.name,
                            args: call.args,
                        }),
                        ..GeminiPart::default()
                    })
                    .collect(),
            };

            Some(GeminiGenerateContentResponseBody {
                candidates: Some(vec![GeminiCandidate {
                    content: Some(model_turn),
                    index: Some(0),
                    ..GeminiCandidate::default()
                }]),
                usage_metadata,
                ..GeminiGenerateContentResponseBody::default()
            })
        }
    }
}

impl TryFrom<Vec<GeminiLiveMessageResponse>> for GeminiStreamGenerateContentResponse {
    type Error = TransformError;

    fn try_from(value: Vec<GeminiLiveMessageResponse>) -> Result<Self, TransformError> {
        Ok(gemini_live_messages_to_stream_response_with_context(value)?.0)
    }
}

impl TryFrom<Vec<GeminiLiveMessageResponse>> for GeminiGenerateContentResponse {
    type Error = TransformError;

    fn try_from(value: Vec<GeminiLiveMessageResponse>) -> Result<Self, TransformError> {
        Ok(gemini_live_messages_to_nonstream_response_with_context(value)?.0)
    }
}

pub fn gemini_live_messages_to_nonstream_response_with_context(
    value: Vec<GeminiLiveMessageResponse>,
) -> Result<
    (
        GeminiGenerateContentResponse,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let mut ctx = GeminiWebsocketTransformContext::default();
    let mut chunks = Vec::new();

    for message in value {
        match message {
            GeminiLiveMessageResponse::Error(body) => {
                return Ok((
                    GeminiGenerateContentResponse::Error {
                        stats_code: StatusCode::BAD_REQUEST,
                        headers: GeminiResponseHeaders::default(),
                        body,
                    },
                    ctx,
                ));
            }
            GeminiLiveMessageResponse::Message(server) => {
                if let Some(chunk) = server_message_to_chunk(server, &mut ctx) {
                    chunks.push(chunk);
                }
            }
        }
    }

    // Merge all chunks into a single response body.
    let mut merged = GeminiGenerateContentResponseBody::default();
    for chunk in chunks {
        if let Some(candidates) = chunk.candidates {
            merged
                .candidates
                .get_or_insert_with(Vec::new)
                .extend(candidates);
        }
        if chunk.usage_metadata.is_some() {
            merged.usage_metadata = chunk.usage_metadata;
        }
    }

    Ok((
        GeminiGenerateContentResponse::Success {
            stats_code: StatusCode::OK,
            headers: GeminiResponseHeaders::default(),
            body: merged,
        },
        ctx,
    ))
}

pub fn gemini_live_messages_to_stream_response_with_context(
    value: Vec<GeminiLiveMessageResponse>,
) -> Result<
    (
        GeminiStreamGenerateContentResponse,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let mut ctx = GeminiWebsocketTransformContext::default();

    for message in &value {
        if let GeminiLiveMessageResponse::Error(body) = message {
            return Ok((
                GeminiStreamGenerateContentResponse::Error {
                    stats_code: StatusCode::BAD_REQUEST,
                    headers: GeminiResponseHeaders::default(),
                    body: body.clone(),
                },
                ctx,
            ));
        }
    }

    // Validate chunks can be produced (side-effects into ctx).
    for message in value {
        if let GeminiLiveMessageResponse::Message(server) = message {
            let _ = server_message_to_chunk(server, &mut ctx);
        }
    }

    Ok((
        GeminiStreamGenerateContentResponse::Success {
            stats_code: StatusCode::OK,
            headers: GeminiResponseHeaders::default(),
        },
        ctx,
    ))
}
