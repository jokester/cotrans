use std::collections::HashMap;

use axum::body::Bytes;
use reqwest::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct R2Inner {
  client: reqwest::Client,
  base: String,
  public_base: String,
  secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Object {
  key: String,
  version: String,
  size: u32,
  etag: String,
  #[serde(rename = "httpEtag")]
  http_etag: String,
  uploaded: String,
  #[serde(rename = "httpMetadata")]
  http_metadata: R2HttpMetadata,
  #[serde(rename = "customMetadata")]
  custom_metadata: HashMap<String, String>,
  range: R2Range,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2HttpMetadata {
  #[serde(rename = "contentType")]
  content_type: Option<String>,
  #[serde(rename = "contentLanguage")]
  content_language: Option<String>,
  #[serde(rename = "contentDisposition")]
  content_disposition: Option<String>,
  #[serde(rename = "contentEncoding")]
  content_encoding: Option<String>,
  #[serde(rename = "cacheControl")]
  cache_control: Option<String>,
  #[serde(rename = "cacheExpiry")]
  cache_expiry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R2Range {
  pub offset: Option<u32>,
  pub length: Option<u32>,
  pub suffix: Option<u32>,
}

impl R2Inner {
  pub fn new(client: reqwest::Client, base: String, public_base: String, secret: String) -> Self {
    Self {
      client,
      base,
      public_base,
      secret,
    }
  }

  pub async fn head(&self, key: &str) -> Result<Option<R2Object>> {
    let res = self
      .client
      .head(&format!("{}/{}", self.base, key))
      .header("x-secret", &self.secret)
      .send()
      .await?
      .text()
      .await?;

    if res == "null" {
      Ok(None)
    } else {
      Ok(serde_json::from_str(&res).ok())
    }
  }

  pub async fn get(&self, key: &str) -> Result<Bytes> {
    self
      .client
      .get(&format!("{}/{}", self.base, key))
      .header("x-secret", &self.secret)
      .send()
      .await?
      .bytes()
      .await
  }

  pub async fn put(&self, key: &str, value: &Bytes) -> Result<()> {
    self
      .client
      .put(&format!("{}/{}", self.base, key))
      .header("x-secret", &self.secret)
      .body(value.clone())
      .send()
      .await?
      .error_for_status()?;
    Ok(())
  }

  pub async fn delete(&self, key: &str) -> Result<()> {
    self
      .client
      .delete(&format!("{}/{}", self.base, key))
      .header("x-secret", &self.secret)
      .send()
      .await?
      .error_for_status()?;
    Ok(())
  }

  pub fn public_url(&self, key: &str) -> String {
    format!("{}/{}", self.public_base, key)
  }
}

pub fn tweet_image_key(tweet_id: &str, image_id: &str) -> String {
  format!("twitter/{}/{}.png", tweet_id, image_id)
}

pub fn upload_image_key(sha: &str) -> String {
  format!("upload/{}.png", sha)
}

pub fn translation_mask_key(task_id: &str) -> String {
  format!("mask/{}.png", task_id)
}

pub fn pixiv_image_key(artwork_id: i64, page: i32) -> String {
  format!("pixiv/{}/{}.png", artwork_id, page)
}
