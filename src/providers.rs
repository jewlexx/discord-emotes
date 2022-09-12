mod seventv;
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
