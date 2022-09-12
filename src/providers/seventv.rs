use serde::{Deserialize, Serialize};

use super::Provider;

#[async_trait]
impl Provider for SevenTvSet {
    const BASE_URL: &'static str = "https://7tv.io/v3/emote-sets/";

    async fn get(id: &str) -> Result<Self, super::ProviderError> {
        let url = format!("{}{}", Self::BASE_URL, id);
        debug!("Fetching emotes from {}", url);
        let resp = reqwest::get(&url).await?.json::<Self>().await?;

        Ok(resp)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SevenTvSet {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) tags: Vec<Option<serde_json::Value>>,
    pub(crate) immutable: bool,
    pub(crate) privileged: bool,
    pub(crate) emotes: Vec<Emote>,
    pub(crate) capacity: i64,
    pub(crate) owner: Owner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Emote {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) flags: i64,
    #[serde(rename = "Timestamp")]
    pub(crate) timestamp: String,
    pub(crate) actor_id: Id,
    pub(crate) data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) flags: i64,
    pub(crate) tags: Vec<Option<serde_json::Value>>,
    pub(crate) lifecycle: i64,
    pub(crate) listed: bool,
    pub(crate) animated: bool,
    pub(crate) owner: Owner,
    pub(crate) host: Host,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Host {
    pub(crate) url: String,
    pub(crate) files: Vec<File>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub(crate) name: Name,
    pub(crate) width: i64,
    pub(crate) height: i64,
    pub(crate) frame_count: i64,
    pub(crate) size: i64,
    pub(crate) format: Format,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Owner {
    pub(crate) id: String,
    pub(crate) username: String,
    pub(crate) display_name: String,
    pub(crate) roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "000000000000000000000000")]
    The000000000000000000000000,
    #[serde(rename = "61f638a2084cfa2e05d2569b")]
    The61F638A2084Cfa2E05D2569B,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "AVIF")]
    Avif,
    #[serde(rename = "WEBP")]
    Webp,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Name {
    #[serde(rename = "1x")]
    The1X,
    #[serde(rename = "1x.avif")]
    The1XAvif,
    #[serde(rename = "2x")]
    The2X,
    #[serde(rename = "2x.avif")]
    The2XAvif,
    #[serde(rename = "3x")]
    The3X,
    #[serde(rename = "3x.avif")]
    The3XAvif,
    #[serde(rename = "4x")]
    The4X,
    #[serde(rename = "4x.avif")]
    The4XAvif,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = serde_json::to_string(self).unwrap().replace('"', "");
        write!(f, "{}", name)
    }
}
