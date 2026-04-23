mod card;
mod error;
mod http;
mod query;

pub use crate::card::{
  BanlistInfo, BanlistStatus, Card, CardAttribute, CardFrameType, CardId, CardImage, CardMisc,
  CardPrice, CardRace, CardSet, CardType,
};
pub use crate::error::{Error, Result};
pub use crate::http::Response;
pub use crate::query::CardQuery;

pub async fn all() -> Result<Vec<Card>> {
  CardQuery::new().send().await
}

pub async fn all_with_misc() -> Result<Vec<Card>> {
  CardQuery::new().misc(true).send().await
}

pub async fn archetype(archetype: &str) -> Result<Vec<Card>> {
  CardQuery::new()
    .archetype(archetype)
    .send()
    .await
}

pub async fn fname(fname: &str) -> Result<Vec<Card>> {
  CardQuery::new().fname(fname).send().await
}

pub async fn id(id: impl Into<CardId>) -> Result<Vec<Card>> {
  CardQuery::new().id(id.into()).send().await
}

pub async fn name(name: &str) -> Result<Vec<Card>> {
  CardQuery::new().name(name).send().await
}

pub async fn staple() -> Result<Vec<Card>> {
  CardQuery::new().staple(true).send().await
}
