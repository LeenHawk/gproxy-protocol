use crate::openai::create_response::response::OpenAiCreateResponseResponse;
use crate::openai::create_response::stream::ResponseStreamEvent;
use crate::openai::create_response::websocket::response::OpenAiCreateResponseWebSocketMessageResponse;
use crate::openai::create_response::websocket::types::{
    OpenAiCreateResponseWebSocketServerMessage, OpenAiCreateResponseWebSocketWrappedErrorEvent,
};
use crate::transform::openai::websocket::context::OpenAiWebsocketTransformContext;

const FALLBACK_WS_ERROR_CODE: &str = "websocket_error";
const FALLBACK_WS_ERROR_MESSAGE: &str = "websocket error";

fn wrapped_error_to_stream_event(
    event: OpenAiCreateResponseWebSocketWrappedErrorEvent,
    sequence_number: u64,
    ctx: &mut OpenAiWebsocketTransformContext,
) -> ResponseStreamEvent {
    if let Some(status) = event.status {
        ctx.push_warning(format!(
            "openai websocket to_http response: dropped wrapped error status={status}"
        ));
    }
    if let Some(headers) = event.headers.as_ref() {
        ctx.push_warning(format!(
            "openai websocket to_http response: dropped wrapped error headers ({} entries)",
            headers.len()
        ));
    }
    let payload = event.error.unwrap_or_default();
    ResponseStreamEvent::Error {
        error: crate::openai::create_response::stream::ResponseStreamErrorPayload {
            type_: payload
                .type_
                .or_else(|| payload.code.clone())
                .unwrap_or_else(|| FALLBACK_WS_ERROR_CODE.to_string()),
            code: payload.code,
            message: payload
                .message
                .unwrap_or_else(|| FALLBACK_WS_ERROR_MESSAGE.to_string()),
            param: payload.param,
        },
        sequence_number,
    }
}

fn api_error_to_stream_event(
    event: crate::openai::types::OpenAiApiErrorResponse,
    sequence_number: u64,
) -> ResponseStreamEvent {
    ResponseStreamEvent::Error {
        error: crate::openai::create_response::stream::ResponseStreamErrorPayload {
            type_: event.error.type_,
            code: event.error.code,
            message: event.error.message,
            param: event.error.param,
        },
        sequence_number,
    }
}

impl TryFrom<&[OpenAiCreateResponseWebSocketMessageResponse]> for OpenAiCreateResponseResponse {
    type Error = crate::transform::utils::TransformError;

    fn try_from(
        value: &[OpenAiCreateResponseWebSocketMessageResponse],
    ) -> Result<Self, crate::transform::utils::TransformError> {
        Ok(websocket_messages_to_openai_nonstream_with_context(value)?.0)
    }
}

impl TryFrom<Vec<OpenAiCreateResponseWebSocketMessageResponse>> for OpenAiCreateResponseResponse {
    type Error = crate::transform::utils::TransformError;

    fn try_from(
        value: Vec<OpenAiCreateResponseWebSocketMessageResponse>,
    ) -> Result<Self, crate::transform::utils::TransformError> {
        OpenAiCreateResponseResponse::try_from(value.as_slice())
    }
}

pub fn websocket_messages_to_openai_nonstream_with_context(
    value: &[OpenAiCreateResponseWebSocketMessageResponse],
) -> Result<
    (
        OpenAiCreateResponseResponse,
        OpenAiWebsocketTransformContext,
    ),
    crate::transform::utils::TransformError,
> {
    let (events, ctx) = websocket_messages_to_openai_stream_events_with_context(value)?;
    let response = OpenAiCreateResponseResponse::try_from(events)?;
    Ok((response, ctx))
}

pub fn websocket_messages_to_openai_stream_events_with_context(
    value: &[OpenAiCreateResponseWebSocketMessageResponse],
) -> Result<
    (Vec<ResponseStreamEvent>, OpenAiWebsocketTransformContext),
    crate::transform::utils::TransformError,
> {
    let mut ctx = OpenAiWebsocketTransformContext::default();
    let mut events = Vec::new();
    let mut next_sequence_number = 0_u64;

    for message in value.iter().cloned() {
        match message {
            OpenAiCreateResponseWebSocketServerMessage::StreamEvent(event) => {
                events.push(event);
            }
            OpenAiCreateResponseWebSocketServerMessage::Done(_) => {
                // Done marker is not a stream event; skip.
            }
            OpenAiCreateResponseWebSocketServerMessage::WrappedError(event) => {
                events.push(wrapped_error_to_stream_event(
                    event,
                    next_sequence_number,
                    &mut ctx,
                ));
                next_sequence_number = next_sequence_number.saturating_add(1);
            }
            OpenAiCreateResponseWebSocketServerMessage::ApiError(event) => {
                events.push(api_error_to_stream_event(event, next_sequence_number));
                next_sequence_number = next_sequence_number.saturating_add(1);
            }
            // No equivalent SSE event in OpenAI response stream schema.
            OpenAiCreateResponseWebSocketServerMessage::RateLimit(_) => {
                ctx.push_warning(
                    "openai websocket to_http response: dropped codex.rate_limits event"
                        .to_string(),
                );
            }
        }
    }

    Ok((events, ctx))
}
