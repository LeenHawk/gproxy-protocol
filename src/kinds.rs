use std::fmt;

use serde::{Deserialize, Serialize};

/// Protocol-agnostic operation family derived from an API route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OperationFamily {
    #[serde(rename = "model_list")]
    ModelList,
    #[serde(rename = "model_get")]
    ModelGet,
    #[serde(rename = "count_tokens")]
    CountToken,
    #[serde(rename = "compact")]
    Compact,
    #[serde(rename = "generate_content")]
    GenerateContent,
    #[serde(rename = "stream_generate_content")]
    StreamGenerateContent,
    #[serde(rename = "create_image")]
    CreateImage,
    #[serde(rename = "stream_create_image")]
    StreamCreateImage,
    #[serde(rename = "create_image_edit")]
    CreateImageEdit,
    #[serde(rename = "stream_create_image_edit")]
    StreamCreateImageEdit,
    #[serde(rename = "openai_response_websocket")]
    OpenAiResponseWebSocket,
    #[serde(rename = "gemini_live")]
    GeminiLive,
    #[serde(rename = "embeddings")]
    Embedding,
    #[serde(rename = "file_upload")]
    FileUpload,
    #[serde(rename = "file_list")]
    FileList,
    #[serde(rename = "file_get")]
    FileGet,
    #[serde(rename = "file_content")]
    FileContent,
    #[serde(rename = "file_delete")]
    FileDelete,
}

impl OperationFamily {
    pub const fn is_stream(self) -> bool {
        matches!(
            self,
            Self::StreamGenerateContent | Self::StreamCreateImage | Self::StreamCreateImageEdit
        )
    }

    pub const fn can_be_stream_driven(self) -> bool {
        matches!(
            self,
            Self::GenerateContent | Self::Compact | Self::CreateImage | Self::CreateImageEdit
        )
    }
}

impl fmt::Display for OperationFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::ModelList => "model_list",
            Self::ModelGet => "model_get",
            Self::CountToken => "count_tokens",
            Self::Compact => "compact",
            Self::GenerateContent => "generate_content",
            Self::StreamGenerateContent => "stream_generate_content",
            Self::CreateImage => "create_image",
            Self::StreamCreateImage => "stream_create_image",
            Self::CreateImageEdit => "create_image_edit",
            Self::StreamCreateImageEdit => "stream_create_image_edit",
            Self::OpenAiResponseWebSocket => "openai_response_websocket",
            Self::GeminiLive => "gemini_live",
            Self::Embedding => "embeddings",
            Self::FileUpload => "file_upload",
            Self::FileList => "file_list",
            Self::FileGet => "file_get",
            Self::FileContent => "file_content",
            Self::FileDelete => "file_delete",
        };
        f.write_str(label)
    }
}

impl TryFrom<&str> for OperationFamily {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "model_list" => Ok(Self::ModelList),
            "model_get" => Ok(Self::ModelGet),
            "count_tokens" => Ok(Self::CountToken),
            "compact" => Ok(Self::Compact),
            "generate_content" => Ok(Self::GenerateContent),
            "stream_generate_content" => Ok(Self::StreamGenerateContent),
            "create_image" => Ok(Self::CreateImage),
            "stream_create_image" => Ok(Self::StreamCreateImage),
            "create_image_edit" => Ok(Self::CreateImageEdit),
            "stream_create_image_edit" => Ok(Self::StreamCreateImageEdit),
            "openai_response_websocket" => Ok(Self::OpenAiResponseWebSocket),
            "gemini_live" => Ok(Self::GeminiLive),
            "embeddings" => Ok(Self::Embedding),
            "file_upload" => Ok(Self::FileUpload),
            "file_list" => Ok(Self::FileList),
            "file_get" => Ok(Self::FileGet),
            "file_content" => Ok(Self::FileContent),
            "file_delete" => Ok(Self::FileDelete),
            _ => Err("unknown operation family"),
        }
    }
}

impl TryFrom<String> for OperationFamily {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

/// Canonical protocol kind used across routing, transforms, and provider dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtocolKind {
    #[serde(rename = "openai")]
    OpenAi,
    #[serde(rename = "claude")]
    Claude,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "openai_chat_completions")]
    OpenAiChatCompletion,
    #[serde(rename = "gemini_ndjson")]
    GeminiNDJson,
    #[serde(rename = "openai_response")]
    OpenAiResponse,
}

impl ProtocolKind {
    pub const fn normalize_gemini_stream(self) -> Self {
        match self {
            Self::GeminiNDJson => Self::Gemini,
            _ => self,
        }
    }
}

impl fmt::Display for ProtocolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::OpenAi => "openai",
            Self::Claude => "claude",
            Self::Gemini => "gemini",
            Self::OpenAiChatCompletion => "openai_chat_completions",
            Self::GeminiNDJson => "gemini_ndjson",
            Self::OpenAiResponse => "openai_response",
        };
        f.write_str(label)
    }
}

impl TryFrom<&str> for ProtocolKind {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "openai" => Ok(Self::OpenAi),
            "claude" => Ok(Self::Claude),
            "gemini" => Ok(Self::Gemini),
            "openai_chat_completions" => Ok(Self::OpenAiChatCompletion),
            "gemini_ndjson" => Ok(Self::GeminiNDJson),
            "openai_response" => Ok(Self::OpenAiResponse),
            _ => Err("unknown protocol kind"),
        }
    }
}

impl TryFrom<String> for ProtocolKind {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
