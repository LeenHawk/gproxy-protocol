pub fn ensure_models_prefix(value: &str) -> String {
    if value.starts_with("models/") {
        value.to_string()
    } else {
        format!("models/{value}")
    }
}

pub fn strip_models_prefix(value: &str) -> String {
    value.strip_prefix("models/").unwrap_or(value).to_string()
}
