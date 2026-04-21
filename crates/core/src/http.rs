use crate::card::Card;
use crate::error::{Error, Result};
use crate::query::CardQuery;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use url::Url;

static HTTP: LazyLock<Client> = LazyLock::new(|| {
  let user_agent = format!("ygo-rs/{}", env!("CARGO_PKG_VERSION"));
  Client::builder()
    .use_rustls_tls()
    .https_only(true)
    .user_agent(user_agent)
    .build()
    .expect("failed to create http client")
});

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
  Data { data: Vec<Card> },
  Error { error: String },
}

pub(crate) async fn send(query: CardQuery) -> Result<Vec<Card>> {
  let (status, raw) = HTTP
    .get(query.into_url())
    .send()
    .await
    .map(|raw| (raw.status(), raw))?;

  if status.is_success()
    || (matches!(status, StatusCode::BAD_REQUEST)
      && raw
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|it| it.to_str().ok())
        .map(str::to_ascii_lowercase)
        .is_some_and(|it| it.contains("application/json")))
  {
    match raw.json::<Response>().await? {
      Response::Data { data } => Ok(data),
      Response::Error { error } => Err(Error::BadRequest(error)),
    }
  } else {
    Err(Error::RequestFailed {
      status: Some(status),
      reason: raw.text().await?,
    })
  }
}

pub(crate) async fn download(url: Url) -> Result<Vec<u8>> {
  let (status, raw) = HTTP
    .get(url)
    .send()
    .await
    .map(|raw| (raw.status(), raw))?;

  if status.is_success() {
    Ok(raw.bytes().await?.to_vec())
  } else {
    Err(Error::RequestFailed {
      status: Some(status),
      reason: raw.text().await?,
    })
  }
}
