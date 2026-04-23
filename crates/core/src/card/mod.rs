mod attribute;
mod banlist;
mod format;
mod frame_type;
mod race;
mod r#type;

use serde::{Deserialize, Serialize};
use url::Url;

pub use attribute::CardAttribute;
pub use banlist::{BanlistInfo, BanlistStatus};
pub use format::CardFormat;
pub use frame_type::CardFrameType;
pub use race::CardRace;
pub use r#type::CardType;

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct Card {
  pub archetype: Option<String>,
  pub atk: Option<i32>,
  pub attribute: Option<CardAttribute>,
  pub banlist_info: Option<BanlistInfo>,
  pub card_images: Vec<CardImage>,
  pub card_prices: Vec<CardPrice>,
  pub card_sets: Vec<CardSet>,
  pub def: Option<i32>,
  pub desc: Option<String>,
  #[serde(rename = "frameType")]
  pub frame_type: Option<CardFrameType>,
  #[serde(rename = "humanReadableCardType")]
  pub human_readable_card_type: Option<String>,
  pub id: Option<CardId>,
  pub level: Option<u8>,
  pub linkmarkers: Vec<String>,
  pub linkval: Option<u8>,
  pub misc_info: Vec<CardMisc>,
  pub monster_desc: Option<String>,
  pub name: Option<String>,
  pub pend_desc: Option<String>,
  pub r#type: Option<CardType>,
  pub race: Option<CardRace>,
  pub scale: Option<u8>,
  pub typeline: Vec<String>,
  pub ygoprodeck_url: Option<Url>,
}

impl Card {
  pub fn formats(&self) -> impl Iterator<Item = CardFormat> {
    self
      .misc_info
      .iter()
      .flat_map(|misc| misc.formats.iter())
      .copied()
  }
}

#[derive(
  Clone,
  Copy,
  Debug,
  Deserialize,
  Serialize,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  derive_more::Display,
  derive_more::From,
  derive_more::Into,
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct CardId(u32);

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct CardSet {
  pub set_code: Option<String>,
  pub set_name: Option<String>,
  pub set_price: Option<String>,
  pub set_rarity: Option<String>,
  pub set_rarity_code: Option<String>,
}

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct CardImage {
  pub id: Option<u32>,
  pub image_url: Option<Url>,
  pub image_url_cropped: Option<Url>,
  pub image_url_small: Option<Url>,
}

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct CardPrice {
  pub amazon_price: Option<String>,
  pub cardmarket_price: Option<String>,
  pub coolstuffinc_price: Option<String>,
  pub ebay_price: Option<String>,
  pub tcgplayer_price: Option<String>,
}

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct CardMisc {
  pub beta_id: Option<u32>,
  pub beta_name: Option<String>,
  pub downvotes: Option<u32>,
  pub formats: Vec<CardFormat>,
  pub has_effect: Option<u8>,
  pub konami_id: Option<i32>,
  pub md_rarity: Option<String>,
  pub ocg_date: Option<String>,
  pub question_atk: Option<u8>,
  pub staple: Option<String>,
  pub tcg_date: Option<String>,
  pub treated_as: Option<String>,
  pub upvotes: Option<u32>,
  pub views: Option<u32>,
  pub viewsweek: Option<u32>,
}
