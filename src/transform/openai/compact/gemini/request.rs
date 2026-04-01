use crate::gemini::count_tokens::types as gt;
use crate::gemini::generate_content::request::GeminiGenerateContentRequest;
use crate::gemini::generate_content::request::{
    PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::openai::compact_response::request::OpenAiCompactRequest;
use crate::transform::gemini::model_get::utils::ensure_models_prefix;
use crate::transform::openai::compact::utils::{
    COMPACT_MAX_OUTPUT_TOKENS, compact_system_instruction,
};
use crate::transform::openai::count_tokens::gemini::utils::{
    openai_generation_config, openai_input_items_to_gemini_contents,
};
use crate::transform::openai::count_tokens::utils::openai_input_to_items;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCompactRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiCompactRequest) -> Result<Self, TransformError> {
        let body = value.body;
        let contents = openai_input_items_to_gemini_contents(openai_input_to_items(body.input));

        Ok(GeminiGenerateContentRequest {
            method: gt::HttpMethod::Post,
            path: PathParameters {
                model: ensure_models_prefix(&body.model),
            },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                contents,
                tools: None,
                tool_config: None,
                safety_settings: None,
                system_instruction: Some(gt::GeminiContent {
                    parts: vec![gt::GeminiPart {
                        text: Some(compact_system_instruction(body.instructions)),
                        ..gt::GeminiPart::default()
                    }],
                    role: None,
                }),
                generation_config: openai_generation_config(
                    None,
                    None,
                    Some(COMPACT_MAX_OUTPUT_TOKENS),
                    None,
                    None,
                    None,
                ),
                cached_content: None,
                store: None,
            },
        })
    }
}
