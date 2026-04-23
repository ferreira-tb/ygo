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
pub enum CardFormat {
  #[serde(rename = "Common Charity")]
  #[strum(serialize = "Common Charity")]
  CommonCharity,

  #[serde(rename = "Duel Links")]
  #[strum(serialize = "Duel Links")]
  DuelLinks,

  #[serde(rename = "Edison")]
  #[strum(serialize = "Edison")]
  Edison,

  #[serde(rename = "GOAT")]
  #[strum(serialize = "GOAT")]
  Goat,

  #[serde(rename = "Master Duel")]
  #[strum(serialize = "Master Duel")]
  MasterDuel,

  #[serde(rename = "OCG")]
  #[strum(serialize = "OCG")]
  OCG,

  #[serde(rename = "OCG GOAT")]
  #[strum(serialize = "OCG GOAT")]
  OCGGoat,

  #[serde(rename = "Speed Duel")]
  #[strum(serialize = "Speed Duel")]
  SpeedDuel,

  #[serde(rename = "TCG")]
  #[strum(serialize = "TCG")]
  TCG,
}
