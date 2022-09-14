pub(crate) mod seventv;
use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub(crate) enum ProviderError {
    #[error("Error interacting with upstream API")]
    RequestError(#[from] reqwest::Error),
    #[error("Error parsing JSON")]
    JsonError(#[from] serde_json::Error),
}

#[async_trait]
pub(crate) trait Provider: Send + Sync + Sized {
    /// The API base url for getting the list of emotes
    const BASE_URL: &'static str;

    async fn get(id: &str) -> Result<Self, ProviderError>;
}

pub(crate) struct ProviderEmotes {}

pub(crate) struct Emote {
    pub(crate) name: String,
    pub(crate) extension: FileType,
    pub(crate) url: String,
}

pub(crate) enum FileType {
    Avif,
    Webp,
    Png,
    Jpeg,
}

impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileType::Avif => write!(f, "avif"),
            FileType::Webp => write!(f, "webp"),
            FileType::Png => write!(f, "png"),
            FileType::Jpeg => write!(f, "jpeg"),
        }
    }
}
