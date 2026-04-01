pub const COMPACT_MAX_OUTPUT_TOKENS: u64 = 16_384;

pub const COMPACT_SYSTEM_INSTRUCTION_PREFIX: &str = "You are performing context compaction. Produce a concise, loss-minimized compacted state that preserves facts, decisions, constraints, tool outcomes, and unresolved tasks.";

pub const CLAUDE_COMPACT_SYSTEM_INSTRUCTION_PREFIX: &str = "Claude has native compaction support. Prefer Claude-native compaction behavior and preserve compaction semantics.";

fn prepend_prefix(prefix: &str, text: Option<String>) -> String {
    match text {
        Some(text) if !text.is_empty() => format!("{prefix}\n\n{text}"),
        _ => prefix.to_string(),
    }
}

pub fn compact_system_instruction(text: Option<String>) -> String {
    prepend_prefix(COMPACT_SYSTEM_INSTRUCTION_PREFIX, text)
}

pub fn claude_compact_system_instruction(text: Option<String>) -> String {
    let text = compact_system_instruction(text);
    prepend_prefix(CLAUDE_COMPACT_SYSTEM_INSTRUCTION_PREFIX, Some(text))
}
