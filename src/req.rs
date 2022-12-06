
use http::HeaderMap;
use serde::{Serialize, Deserialize};
use reqwest::{Client,Method,Response};
use url::Url;
use anyhow::{Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestProfile {
    #[serde(with = "http_serde::method", default)]
    pub method: Method,

    pub url: Url,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<serde_json::Value>,

    #[serde(
        skip_serializing_if = "HeaderMap::is_empty",
        with = "http_serde::header_map",
        default
    )]
    pub headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<serde_json::Value>,
}

impl RequestProfile {
  pub async fn send(&self) -> Result<Response>{
    let client = Client::new();
    let mut req = client.request(self.method.clone(), self.url.clone());
    if let Some(params) = &self.params {
      req = req.query(params);
    }
    if let Some(body) = &self.body {
      req = req.body(body.to_string());
    }
    if self.headers.len() > 0{
      req = req.headers(self.headers.clone());
    }
    Ok(req.send().await?)
  }
}
