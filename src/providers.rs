pub mod seventv;
use std::path::Path;

use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum ProviderError {
    #[error("Error interacting with upstream API")]
    RequestError(#[from] reqwest::Error),
    #[error("Error parsing JSON")]
    JsonError(#[from] serde_json::Error),
}

unsafe impl Send for ProviderError {}
unsafe impl Sync for ProviderError {}

pub enum Providers {
    SevenTv,
    Bttv,
    Ffz,
    Ttv,
}

pub trait Provider: Send + Sync + Sized + Into<ProviderEmotes> {
    /// The API base url for getting the list of emotes
    const BASE_URL: &'static str;

    fn get(id: &str) -> Result<Self, ProviderError>;

    fn download_to_dir(emotes: &ProviderEmotes, dir: impl AsRef<Path>)
        -> Result<(), ProviderError>;
}

pub struct ProviderEmotes {
    pub provider: Providers,
    pub emotes: Vec<Emote>,
}

pub struct Emote {
    pub name: String,
    pub url: String,
    pub size: Size,
    pub extension: FileType,
}

pub enum Size {
    X1,
    X2,
    X3,
    X4,
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

impl From<Size> for u8 {
    fn from(size: Size) -> Self {
        match size {
            Size::X1 => 1,
            Size::X2 => 2,
            Size::X3 => 3,
            Size::X4 => 4,
        }
    }
}
