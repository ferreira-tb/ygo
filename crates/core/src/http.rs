use crate::card::Card;
use crate::error::{Error, Result};
use crate::query::CardQuery;
use http::StatusCode;
use http::header::CONTENT_TYPE;
use reqwest::Client;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::borrow::Cow;
use std::error::Error as _;
use std::fmt;
use std::sync::LazyLock;

static HTTP: LazyLock<Client> = LazyLock::new(|| {
  let user_agent = format!("ygo-rs/{}", env!("CARGO_PKG_VERSION"));
  Client::builder()
    .use_rustls_tls()
    .https_only(true)
    .user_agent(user_agent)
    .build()
    .expect("failed to create http client")
});

pub enum Response {
  Data { data: Vec<Card> },
  Error { error: String },
}

impl<'de> Deserialize<'de> for Response {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_map(VisitResponse)
  }
}

struct VisitResponse;

impl<'de> Visitor<'de> for VisitResponse {
  type Value = Response;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("enum Response")
  }

  fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
  where
    V: MapAccess<'de>,
  {
    let mut data = None;
    let mut error = None;

    while let Some(key) = map.next_key::<Cow<'static, str>>()? {
      match key.as_ref() {
        "data" => data = Some(map.next_value()?),
        "error" => error = Some(map.next_value()?),
        _ => return Err(de::Error::unknown_field(&key, &["data", "error"])),
      }
    }

    match (data, error) {
      (Some(data), None) => Ok(Response::Data { data }),
      (None, Some(error)) => Ok(Response::Error { error }),
      _ => Err(de::Error::custom(
        "expected exactly one of `data` or `error`",
      )),
    }
  }
}

pub(crate) async fn send(query: CardQuery) -> Result<Vec<Card>> {
  let (status, raw) = HTTP
    .get(query.into_url())
    .send()
    .await
    .map(|raw| (raw.status(), raw))?;

  if status.is_success()
    || (matches!(status, StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND)
      && raw
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|it| it.to_str().ok())
        .map(str::to_ascii_lowercase)
        .is_some_and(|it| it.contains("application/json")))
  {
    match raw.json::<Response>().await {
      Ok(Response::Data { data }) => Ok(data),
      Ok(Response::Error { error }) => Err(Error::BadRequest(error)),
      Err(err) => {
        let reason = match err.source() {
          Some(source) => source.to_string(),
          None => err.to_string(),
        };

        Err(Error::RequestFailed { status: Some(status), reason })
      }
    }
  } else {
    Err(Error::RequestFailed {
      status: Some(status),
      reason: raw.text().await?,
    })
  }
}
