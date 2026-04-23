use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs, EnumIter, EnumString, IntoStaticStr, VariantArray};

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct BanlistInfo {
  pub ban_ocg: Option<BanlistStatus>,
  pub ban_tcg: Option<BanlistStatus>,
}

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
pub enum BanlistStatus {
  #[serde(rename = "Forbidden")]
  #[strum(serialize = "Forbidden")]
  Forbidden,

  #[serde(rename = "Limited")]
  #[strum(serialize = "Limited")]
  Limited,

  #[serde(rename = "Semi-Limited")]
  #[strum(serialize = "Semi-Limited")]
  SemiLimited,
}
