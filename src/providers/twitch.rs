use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use super::ProviderError;

lazy_static::lazy_static! {
    static ref TWITCH_CLIENT: reqwest::blocking::Client = {
        let mut headers = HeaderMap::new();
        headers.append("Client-Id", crate::dotenv_vars::CLIENT_ID.parse().unwrap());
        headers.append("Authorization", format!("Bearer {}", crate::dotenv_vars::CLIENT_SECRET).parse().unwrap());

        reqwest::blocking::Client::builder().default_headers(headers).build().unwrap()
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    data: Vec<Datum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Datum {
    id: String,
    login: String,
    display_name: String,
    #[serde(rename = "type")]
    datum_type: String,
    broadcaster_type: String,
    description: String,
    profile_image_url: String,
    offline_image_url: String,
    view_count: i64,
    email: String,
    created_at: String,
}

pub fn get_user_id(username: impl AsRef<str>) -> Result<String, ProviderError> {
    let username = username.as_ref();

    let base_url = "https://api.twitch.tv/helix/users";
    let url = format!("{}/?login={}", base_url, username);

    let mut resp: UserInfo = TWITCH_CLIENT.get(&url).send()?.json()?;

    let user_info = resp.data.remove(0);

    Ok(user_info.id)
}
