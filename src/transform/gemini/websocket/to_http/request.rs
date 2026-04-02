use crate::gemini::count_tokens::types::{GeminiContent, GeminiContentRole, GeminiPart};
use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::live::request::GeminiLiveConnectRequest;
use crate::gemini::live::types::{
    GeminiBidiGenerateContentClientMessage, GeminiBidiGenerateContentClientMessageType,
};
use crate::gemini::stream_generate_content::request::{
    AltQueryParameter, GeminiStreamGenerateContentRequest, PathParameters, QueryParameters,
    RequestBody,
};
use crate::transform::gemini::model_get::utils::ensure_models_prefix;
use crate::transform::gemini::websocket::context::GeminiWebsocketTransformContext;
use crate::transform::utils::TransformError;

const UNSUPPORTED_REALTIME_INPUT: &str =
    "cannot convert Gemini realtimeInput websocket frame to streamGenerateContent request";
const MISSING_SETUP_MODEL: &str =
    "cannot convert Gemini websocket frames to streamGenerateContent request without setup model";
const FALLBACK_MODEL: &str = "models/unknown";

pub fn gemini_live_client_messages_to_stream_request_with_context(
    value: &[GeminiBidiGenerateContentClientMessage],
) -> Result<
    (
        GeminiStreamGenerateContentRequest,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let mut ctx = GeminiWebsocketTransformContext::default();
    let mut model = None::<String>;
    let mut generation_config = None;
    let mut system_instruction = None;
    let mut tools = None;
    let mut contents = Vec::<GeminiContent>::new();

    for message in value {
        match &message.message_type {
            GeminiBidiGenerateContentClientMessageType::Setup { setup } => {
                model = Some(ensure_models_prefix(&setup.model));
                generation_config = setup.generation_config.clone();
                system_instruction = setup.system_instruction.clone();
                tools = setup.tools.clone();
                if let Some(prefix_turns) = &setup.prefix_turns {
                    contents.extend(prefix_turns.clone());
                }
            }
            GeminiBidiGenerateContentClientMessageType::ClientContent { client_content } => {
                if let Some(turns) = &client_content.turns {
                    contents.extend(turns.clone());
                }
            }
            GeminiBidiGenerateContentClientMessageType::ToolResponse { tool_response } => {
                if let Some(function_responses) = &tool_response.function_responses {
                    let parts = function_responses
                        .iter()
                        .cloned()
                        .map(|response| GeminiPart {
                            function_response: Some(response),
                            ..GeminiPart::default()
                        })
                        .collect::<Vec<_>>();
                    if !parts.is_empty() {
                        contents.push(GeminiContent {
                            parts,
                            role: Some(GeminiContentRole::User),
                        });
                    }
                }
            }
            GeminiBidiGenerateContentClientMessageType::RealtimeInput { .. } => {
                ctx.push_warning(UNSUPPORTED_REALTIME_INPUT.to_string());
            }
        }
    }

    let model = model.unwrap_or_else(|| {
        ctx.push_warning(MISSING_SETUP_MODEL.to_string());
        FALLBACK_MODEL.to_string()
    });

    Ok((
        GeminiStreamGenerateContentRequest {
            path: PathParameters { model },
            query: QueryParameters {
                alt: Some(AltQueryParameter::Sse),
            },
            body: RequestBody {
                contents,
                tools,
                tool_config: None,
                safety_settings: None,
                system_instruction,
                generation_config,
                cached_content: None,
                store: None,
            },
            ..GeminiStreamGenerateContentRequest::default()
        },
        ctx,
    ))
}

pub fn gemini_live_connect_to_stream_request_with_context(
    value: &GeminiLiveConnectRequest,
) -> Result<
    (
        GeminiStreamGenerateContentRequest,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let Some(frame) = value.body.as_ref() else {
        let mut ctx = GeminiWebsocketTransformContext::default();
        ctx.push_warning(
            "cannot convert Gemini live connect request without initial body; downgraded to empty streamGenerateContent request"
                .to_string(),
        );
        return Ok((
            GeminiStreamGenerateContentRequest {
                path: PathParameters {
                    model: FALLBACK_MODEL.to_string(),
                },
                query: QueryParameters {
                    alt: Some(AltQueryParameter::Sse),
                },
                ..GeminiStreamGenerateContentRequest::default()
            },
            ctx,
        ));
    };
    gemini_live_client_messages_to_stream_request_with_context(std::slice::from_ref(frame))
}

pub fn gemini_live_client_messages_to_nonstream_request_with_context(
    value: &[GeminiBidiGenerateContentClientMessage],
) -> Result<
    (
        GeminiGenerateContentRequest,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let (stream_request, ctx) = gemini_live_client_messages_to_stream_request_with_context(value)?;
    let request = GeminiGenerateContentRequest::try_from(stream_request)?;
    Ok((request, ctx))
}

pub fn gemini_live_connect_to_nonstream_request_with_context(
    value: &GeminiLiveConnectRequest,
) -> Result<
    (
        GeminiGenerateContentRequest,
        GeminiWebsocketTransformContext,
    ),
    TransformError,
> {
    let (stream_request, ctx) = gemini_live_connect_to_stream_request_with_context(value)?;
    let request = GeminiGenerateContentRequest::try_from(stream_request)?;
    Ok((request, ctx))
}

impl TryFrom<&GeminiBidiGenerateContentClientMessage> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiBidiGenerateContentClientMessage) -> Result<Self, TransformError> {
        Ok(
            gemini_live_client_messages_to_stream_request_with_context(std::slice::from_ref(
                value,
            ))?
            .0,
        )
    }
}

impl TryFrom<&[GeminiBidiGenerateContentClientMessage]> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &[GeminiBidiGenerateContentClientMessage]) -> Result<Self, TransformError> {
        Ok(gemini_live_client_messages_to_stream_request_with_context(value)?.0)
    }
}

impl TryFrom<&GeminiLiveConnectRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiLiveConnectRequest) -> Result<Self, TransformError> {
        Ok(gemini_live_connect_to_stream_request_with_context(value)?.0)
    }
}

impl TryFrom<GeminiLiveConnectRequest> for GeminiStreamGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: GeminiLiveConnectRequest) -> Result<Self, TransformError> {
        GeminiStreamGenerateContentRequest::try_from(&value)
    }
}

impl TryFrom<&GeminiBidiGenerateContentClientMessage> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiBidiGenerateContentClientMessage) -> Result<Self, TransformError> {
        Ok(
            gemini_live_client_messages_to_nonstream_request_with_context(std::slice::from_ref(
                value,
            ))?
            .0,
        )
    }
}

impl TryFrom<&[GeminiBidiGenerateContentClientMessage]> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &[GeminiBidiGenerateContentClientMessage]) -> Result<Self, TransformError> {
        Ok(gemini_live_client_messages_to_nonstream_request_with_context(value)?.0)
    }
}

impl TryFrom<&GeminiLiveConnectRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: &GeminiLiveConnectRequest) -> Result<Self, TransformError> {
        Ok(gemini_live_connect_to_nonstream_request_with_context(value)?.0)
    }
}

impl TryFrom<GeminiLiveConnectRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: GeminiLiveConnectRequest) -> Result<Self, TransformError> {
        GeminiGenerateContentRequest::try_from(&value)
    }
}

