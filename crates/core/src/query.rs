use crate::card::{Card, CardId, CardRace, CardType};
use crate::error::Result;
use std::borrow::Cow;
use std::sync::LazyLock;
use url::Url;

static CARD_URL: LazyLock<Url> = LazyLock::new(|| {
  let url = "https://db.ygoprodeck.com/api/v7/cardinfo.php";
  Url::parse(url).expect("failed to parse card url")
});

#[derive(Clone, Debug)]
pub struct CardQuery(Cow<'static, Url>);

impl CardQuery {
  pub fn new() -> Self {
    Self(Cow::Borrowed(&CARD_URL))
  }

  pub async fn send(self) -> Result<Vec<Card>> {
    crate::http::send(self).await
  }

  #[inline]
  pub fn into_url(self) -> Url {
    self.0.into_owned()
  }

  #[inline]
  #[must_use]
  pub fn archetype(self, archetype: &str) -> Self {
    self.append_pair("archetype", archetype)
  }

  #[inline]
  #[must_use]
  pub fn atk(self, atk: &str) -> Self {
    self.append_pair("atk", atk)
  }

  #[inline]
  #[must_use]
  pub fn attribute(self, attribute: &str) -> Self {
    self.append_pair("attribute", attribute)
  }

  #[inline]
  #[must_use]
  pub fn banlist(self, banlist: &str) -> Self {
    self.append_pair("banlist", banlist)
  }

  #[inline]
  #[must_use]
  pub fn cardset(self, set: &str) -> Self {
    self.append_pair("cardset", set)
  }

  #[inline]
  #[must_use]
  pub fn def(self, def: &str) -> Self {
    self.append_pair("def", def)
  }

  #[inline]
  #[must_use]
  pub fn id(self, id: CardId) -> Self {
    self.append_pair("id", &id.to_string())
  }

  #[inline]
  #[must_use]
  pub fn fname(self, fname: &str) -> Self {
    self.append_pair("fname", fname)
  }

  #[inline]
  #[must_use]
  pub fn format(self, format: &str) -> Self {
    self.append_pair("format", format)
  }

  #[inline]
  #[must_use]
  pub fn konami_id(self, id: u32) -> Self {
    self.append_pair("konami_id", &id.to_string())
  }

  #[inline]
  #[must_use]
  pub fn level(self, level: &str) -> Self {
    self.append_pair("level", level)
  }

  #[inline]
  #[must_use]
  pub fn link(self, value: u8) -> Self {
    self.append_pair("link", &value.to_string())
  }

  #[inline]
  #[must_use]
  pub fn linkmarker(self, marker: &str) -> Self {
    self.append_pair("linkmarker", marker)
  }

  #[inline]
  #[must_use]
  pub fn misc(self, yes: bool) -> Self {
    if yes {
      self.append_pair("misc", "yes")
    } else {
      self
    }
  }

  #[inline]
  #[must_use]
  pub fn name(self, name: &str) -> Self {
    self.append_pair("name", name)
  }

  #[inline]
  #[must_use]
  pub fn race(self, race: CardRace) -> Self {
    self.append_pair("race", race.into())
  }

  #[inline]
  #[must_use]
  pub fn scale(self, scale: u8) -> Self {
    self.append_pair("scale", &scale.to_string())
  }

  #[inline]
  #[must_use]
  pub fn sort(self, sort: &str) -> Self {
    self.append_pair("sort", sort)
  }

  #[inline]
  #[must_use]
  pub fn staple(self, yes: bool) -> Self {
    if yes {
      self.append_pair("staple", "yes")
    } else {
      self
    }
  }

  #[inline]
  #[must_use]
  pub fn r#type(self, r#type: CardType) -> Self {
    self.append_pair("type", r#type.into())
  }

  #[must_use]
  fn append_pair(mut self, name: &str, value: &str) -> Self {
    self
      .0
      .to_mut()
      .query_pairs_mut()
      .append_pair(name, value);

    self
  }
}

impl Default for CardQuery {
  fn default() -> Self {
    Self::new()
  }
}

impl From<CardQuery> for String {
  fn from(query: CardQuery) -> Self {
    query.0.to_string()
  }
}

impl From<CardQuery> for Url {
  fn from(query: CardQuery) -> Self {
    query.into_url()
  }
}
