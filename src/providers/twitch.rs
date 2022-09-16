use reqwest::header::HeaderMap;

lazy_static::lazy_static! {
    static ref TWITCH_CLIENT: reqwest::Client = {
        let mut headers = HeaderMap::new();
        headers.append("Client-Id", crate::dotenv_vars::CLIENT_ID.parse().unwrap());
        headers.append("Authorization", format!("Bearer {}", crate::dotenv_vars::CLIENT_SECRET).parse().unwrap());

        reqwest::Client::builder().default_headers(headers).build().unwrap()
    };
}

pub fn get_user_id(username: impl AsRef<str>) {
    let username = username.as_ref();
}
