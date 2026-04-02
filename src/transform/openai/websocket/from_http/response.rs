use crate::openai::create_response::response::OpenAiCreateResponseResponse;
use crate::openai::create_response::stream::ResponseStreamEvent;
use crate::openai::create_response::websocket::response::OpenAiCreateResponseWebSocketMessageResponse;
use crate::openai::create_response::websocket::types::{
    OpenAiCreateResponseWebSocketDoneMarker, OpenAiCreateResponseWebSocketServerMessage,
};
use crate::transform::openai::websocket::context::OpenAiWebsocketTransformContext;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCreateResponseResponse> for Vec<OpenAiCreateResponseWebSocketMessageResponse> {
    type Error = TransformError;

    fn try_from(value: OpenAiCreateResponseResponse) -> Result<Self, TransformError> {
        Ok(openai_nonstream_response_to_websocket_messages_with_context(value)?.0)
    }
}

pub fn openai_nonstream_response_to_websocket_messages_with_context(
    value: OpenAiCreateResponseResponse,
) -> Result<
    (
        Vec<OpenAiCreateResponseWebSocketMessageResponse>,
        OpenAiWebsocketTransformContext,
    ),
    TransformError,
> {
    let events = Vec::<ResponseStreamEvent>::try_from(value)?;
    openai_stream_events_to_websocket_messages_with_context(&events)
}

pub fn openai_stream_events_to_websocket_messages_with_context(
    value: &[ResponseStreamEvent],
) -> Result<
    (
        Vec<OpenAiCreateResponseWebSocketMessageResponse>,
        OpenAiWebsocketTransformContext,
    ),
    TransformError,
> {
    let ctx = OpenAiWebsocketTransformContext::default();
    let mut messages = Vec::with_capacity(value.len() + 1);
    for event in value {
        messages.push(OpenAiCreateResponseWebSocketServerMessage::StreamEvent(
            event.clone(),
        ));
    }
    messages.push(OpenAiCreateResponseWebSocketServerMessage::Done(
        OpenAiCreateResponseWebSocketDoneMarker::Done,
    ));

    Ok((messages, ctx))
}
