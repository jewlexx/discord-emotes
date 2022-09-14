pub mod seventv;
use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum ProviderError {
    #[error("Error interacting with upstream API")]
    RequestError(#[from] reqwest::Error),
    #[error("Error parsing JSON")]
    JsonError(#[from] serde_json::Error),
}

pub enum Providers {
    SevenTv,
    Bttv,
    Ffz,
    Ttv,
}

#[async_trait]
pub trait Provider: Send + Sync + Sized {
    /// The API base url for getting the list of emotes
    const BASE_URL: &'static str;

    async fn get(id: &str) -> Result<Self, ProviderError>;
}

pub struct ProviderEmotes {
    pub provider: Providers,
}

pub struct Emote {
    pub name: String,
    pub extension: FileType,
    pub url: String,
}

pub enum FileType {
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
