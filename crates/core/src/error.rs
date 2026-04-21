use reqwest::StatusCode;
use strum::EnumIs;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[non_exhaustive]
#[derive(Debug, EnumIs, thiserror::Error)]
pub enum Error {
  #[error("{0}")]
  BadRequest(String),

  #[error("{reason}")]
  RequestFailed {
    status: Option<StatusCode>,
    reason: String,
  },
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    let status = error.status();
    let reason = error.to_string();
    Self::RequestFailed { status, reason }
  }
}
