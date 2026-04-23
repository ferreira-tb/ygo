use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs, EnumIter, EnumString, IntoStaticStr, VariantArray};

#[remain::sorted]
#[derive(
  Clone,
  Copy,
  Debug,
  Display,
  EnumIs,
  EnumIter,
  EnumString,
  IntoStaticStr,
  VariantArray,
  Deserialize,
  Serialize,
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CardAttribute {
  #[serde(rename = "DARK")]
  #[strum(serialize = "DARK")]
  Dark,

  #[serde(rename = "DIVINE")]
  #[strum(serialize = "DIVINE")]
  Divine,

  #[serde(rename = "EARTH")]
  #[strum(serialize = "EARTH")]
  Earth,

  #[serde(rename = "FIRE")]
  #[strum(serialize = "FIRE")]
  Fire,

  #[serde(rename = "LIGHT")]
  #[strum(serialize = "LIGHT")]
  Light,

  #[serde(rename = "WATER")]
  #[strum(serialize = "WATER")]
  Water,

  #[serde(rename = "WIND")]
  #[strum(serialize = "WIND")]
  Wind,
}
