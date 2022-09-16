pub mod seventv;
pub mod twitch;

use std::{io::Write, path::Path};

use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use thiserror::Error as AsError;

#[derive(Debug, AsError)]
pub enum ProviderError {
    #[error("Error interacting with upstream API")]
    Request(#[from] reqwest::Error),
    #[error("Error parsing JSON")]
    Json(#[from] serde_json::Error),
    #[error("Error saving emotes")]
    Io(#[from] std::io::Error),
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
}

pub struct ProviderEmotes {
    pub provider: Providers,
    pub emotes: Vec<Emote>,
}

impl ProviderEmotes {
    pub fn download_to_dir(self, dir: impl AsRef<Path>) -> Result<(), ProviderError> {
        let dir: &Path = dir.as_ref();

        self.emotes
            .into_par_iter()
            .map(|emote| -> Result<(), ProviderError> {
                let file_name = format!("{}.{}", emote.name, emote.extension);

                trace!("Downloading emote {} from {}", emote.name, emote.url);

                let mut file = std::fs::File::create(dir.join(file_name))?;

                let bytes = reqwest::blocking::get(&emote.url)?.bytes()?;

                file.write_all(&bytes)?;

                Ok(())
            })
            .collect::<Result<Vec<()>, ProviderError>>()?;

        Ok(())
    }
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
