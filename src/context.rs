use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::{Error, Result};

const HACKMD_API_BASE_URL: &str = "https://api.hackmd.io/v1";

pub struct Context {
    pub(crate) token: String,
    pub(crate) client: Client,
}

impl Context {
    /// Token is usually a alphanumberic string, like `5L3NBH6065203RT26UM8SCCCHJ5IHD15EXKDAMQKXOONLKH6D8`
    pub fn new(token: &str) -> Context {
        Context {
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub(crate) async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.client
            .get(Context::make_url(path))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await
            .map_err(Error::from)
    }

    pub(crate) async fn patch<T>(&self, path: &str, payload: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.client
            .patch(Context::make_url(path))
            .bearer_auth(&self.token)
            .json(payload)
            .send()
            .await
            .map(drop)
            .map_err(Error::from)
    }

    pub(crate) fn make_url(route: &str) -> String {
        format!("{HACKMD_API_BASE_URL}/{route}")
    }
}
