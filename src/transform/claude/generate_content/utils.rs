use crate::claude::create_message::types::JsonObject;
use crate::claude::create_message::types::{BetaCacheCreation, BetaServiceTier, BetaUsage};
pub use crate::transform::claude::utils::{
    beta_message_content_to_text, beta_system_prompt_to_text, claude_model_to_string,
};

pub fn beta_usage_from_counts(
    input_tokens: u64,
    cached_input_tokens: u64,
    output_tokens: u64,
    service_tier: BetaServiceTier,
) -> BetaUsage {
    BetaUsage {
        cache_creation: BetaCacheCreation {
            ephemeral_1h_input_tokens: 0,
            ephemeral_5m_input_tokens: 0,
        },
        cache_creation_input_tokens: 0,
        cache_read_input_tokens: cached_input_tokens,
        inference_geo: "global".to_string(),
        input_tokens,
        output_tokens,
        server_tool_use: None,
        service_tier,
    }
}

pub fn parse_json_object_or_empty(input: &str) -> JsonObject {
    serde_json::from_str::<JsonObject>(input).unwrap_or_default()
}
