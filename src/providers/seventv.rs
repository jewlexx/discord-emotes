use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SevenTvSet {
    #[serde(rename = "id")]
    id: Id,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "tags")]
    tags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "immutable")]
    immutable: bool,

    #[serde(rename = "privileged")]
    privileged: bool,

    #[serde(rename = "emotes")]
    emotes: Vec<Emote>,

    #[serde(rename = "capacity")]
    capacity: i64,

    #[serde(rename = "owner")]
    owner: Owner,
}

#[derive(Serialize, Deserialize)]
pub struct Emote {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "flags")]
    flags: i64,

    #[serde(rename = "Timestamp")]
    timestamp: String,

    #[serde(rename = "actor_id")]
    actor_id: Id,

    #[serde(rename = "data")]
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "flags")]
    flags: i64,

    #[serde(rename = "tags")]
    tags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "lifecycle")]
    lifecycle: i64,

    #[serde(rename = "listed")]
    listed: bool,

    #[serde(rename = "animated")]
    animated: bool,

    #[serde(rename = "owner")]
    owner: Owner,

    #[serde(rename = "host")]
    host: Host,
}

#[derive(Serialize, Deserialize)]
pub struct Host {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "files")]
    files: Vec<File>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "name")]
    name: Name,

    #[serde(rename = "width")]
    width: i64,

    #[serde(rename = "height")]
    height: i64,

    #[serde(rename = "frame_count")]
    frame_count: i64,

    #[serde(rename = "size")]
    size: i64,

    #[serde(rename = "format")]
    format: Format,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "id")]
    id: String,

    #[serde(rename = "username")]
    username: String,

    #[serde(rename = "display_name")]
    display_name: String,

    #[serde(rename = "roles")]
    roles: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "000000000000000000000000")]
    The000000000000000000000000,

    #[serde(rename = "61f638a2084cfa2e05d2569b")]
    The61F638A2084Cfa2E05D2569B,
}

#[derive(Serialize, Deserialize)]
pub enum Format {
    #[serde(rename = "AVIF")]
    Avif,

    #[serde(rename = "WEBP")]
    Webp,
}

#[derive(Serialize, Deserialize)]
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
