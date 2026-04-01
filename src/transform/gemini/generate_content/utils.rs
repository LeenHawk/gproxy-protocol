use crate::gemini::types::JsonObject;

pub use crate::transform::gemini::model_list::claude::utils::gemini_error_response_from_claude;
pub use crate::transform::gemini::model_list::openai::utils::gemini_error_response_from_openai;
pub use crate::transform::gemini::utils::{gemini_content_to_text, strip_models_prefix};

pub fn parse_json_object_or_empty(input: &str) -> JsonObject {
    serde_json::from_str::<JsonObject>(input).unwrap_or_default()
}
