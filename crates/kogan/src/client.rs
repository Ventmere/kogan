use std::env::var;
use crate::{KoganEnv, error::KoganError};
use crate::error::Result;
use reqwest::{Client, header::{HeaderMap, HeaderValue}};
use async_trait::async_trait;
pub use reqwest::{Method, RequestBuilder};
use serde::Deserialize;

#[derive(Debug)]
pub struct KoganClient {
    env: KoganEnv,
    client: Client,
    auth_headers: HeaderMap,
}

impl KoganClient {
    pub fn new(env: KoganEnv, seller_token: String, seller_id: String) -> Result<Self> {
        let mut auth_headers = HeaderMap::<_>::with_capacity(2);
        auth_headers.insert("SellerToken", HeaderValue::from_str(&seller_token).map_err(|_| KoganError::InvalidKoganCredential("SellerToken", seller_token.clone()))?);
        auth_headers.insert("SellerID", HeaderValue::from_str(&seller_id).map_err(|_| KoganError::InvalidKoganCredential("SellerID", seller_token.clone()))?);
        Ok(KoganClient {
            env,
            client: Client::new(),
            auth_headers
        })
    }

    pub fn from_env() -> Result<Self> {
        let env = var("KOGAN_ENV").expect("env KOGAN_ENV").parse().unwrap();
        let seller_token = var("KOGAN_SELLER_TOKEN").expect("env KOGAN_SELLER_TOKEN");
        let seller_id = var("KOGAN_SELLER_ID").expect("env KOGAN_SELLER_ID");
        Self::new(env, seller_token, seller_id)
    }

    pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = if path.starts_with("/") {
            format!("{}{}", self.env.base_api_url(), path)
        } else {
            path.to_string()
        };
        self.client
            .request(method, url)
            .headers(self.auth_headers.clone())
    }
}

#[async_trait]
pub trait KoganRequestBuilderExt {
    async fn send_json<T>(self) -> Result<T> where T: for<'de> Deserialize<'de>;
}

#[async_trait]
impl KoganRequestBuilderExt for RequestBuilder {
    async fn send_json<T>(self) -> Result<T> 
    where T: for<'de> Deserialize<'de>
    {
        let res = self.send().await?;
        let status = res.status();
        let text = res.text().await?;
        if !status.is_success() {
            Err(KoganError::RequestError(status, text))
        } else {
            let v = match serde_json::from_str(&text) {
                Ok(v) => v,
                Err(err) => {
                    tracing::error!("request error: status = {:?}, body = {}", status, text);
                    return Err(err.into())
                },
            };
            Ok(v)
        }
    }
}