use crate::gemini::count_tokens::types as gt;
use crate::gemini::generate_content::request::{
    GeminiGenerateContentRequest, PathParameters, QueryParameters, RequestBody, RequestHeaders,
};
use crate::gemini::generate_content::types::HttpMethod as GeminiHttpMethod;
use crate::openai::create_response::request::OpenAiCreateResponseRequest;
use crate::transform::gemini::model_get::utils::ensure_models_prefix;
use crate::transform::openai::count_tokens::gemini::utils::{
    openai_generation_config, openai_input_items_to_gemini_contents, openai_tool_choice_to_gemini,
    openai_tools_to_gemini,
};
use crate::transform::openai::count_tokens::utils::openai_input_to_items;
use crate::transform::utils::TransformError;

impl TryFrom<OpenAiCreateResponseRequest> for GeminiGenerateContentRequest {
    type Error = TransformError;

    fn try_from(value: OpenAiCreateResponseRequest) -> Result<Self, TransformError> {
        let body = value.body;
        let model = ensure_models_prefix(&body.model.unwrap_or_default());

        let contents = openai_input_items_to_gemini_contents(openai_input_to_items(body.input));

        let (tools, has_function_calling_tools) = body
            .tools
            .map(openai_tools_to_gemini)
            .unwrap_or((None, false));

        let tool_config =
            openai_tool_choice_to_gemini(body.tool_choice, has_function_calling_tools);
        let generation_config = openai_generation_config(
            body.reasoning,
            body.text,
            body.max_output_tokens,
            body.temperature,
            body.top_p,
            body.top_logprobs,
        );
        let system_instruction = body.instructions.and_then(|text| {
            if text.is_empty() {
                None
            } else {
                Some(gt::GeminiContent {
                    parts: vec![gt::GeminiPart {
                        text: Some(text),
                        ..gt::GeminiPart::default()
                    }],
                    role: None,
                })
            }
        });

        Ok(GeminiGenerateContentRequest {
            method: GeminiHttpMethod::Post,
            path: PathParameters { model },
            query: QueryParameters::default(),
            headers: RequestHeaders::default(),
            body: RequestBody {
                contents,
                tools,
                tool_config,
                safety_settings: None,
                system_instruction,
                generation_config,
                cached_content: None,
                store: None,
            },
        })
    }
}
