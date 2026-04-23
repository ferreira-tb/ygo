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
pub enum CardFrameType {
  #[serde(rename = "effect")]
  #[strum(serialize = "effect")]
  Effect,

  #[serde(rename = "effect_pendulum")]
  #[strum(serialize = "effect_pendulum")]
  EffectPendulum,

  #[serde(rename = "fusion")]
  #[strum(serialize = "fusion")]
  Fusion,

  #[serde(rename = "fusion_pendulum")]
  #[strum(serialize = "fusion_pendulum")]
  FusionPendulum,

  #[serde(rename = "link")]
  #[strum(serialize = "link")]
  Link,

  #[serde(rename = "normal")]
  #[strum(serialize = "normal")]
  Normal,

  #[serde(rename = "normal_pendulum")]
  #[strum(serialize = "normal_pendulum")]
  NormalPendulum,

  #[serde(rename = "ritual")]
  #[strum(serialize = "ritual")]
  Ritual,

  #[serde(rename = "ritual_pendulum")]
  #[strum(serialize = "ritual_pendulum")]
  RitualPendulum,

  #[serde(rename = "skill")]
  #[strum(serialize = "skill")]
  Skill,

  #[serde(rename = "spell")]
  #[strum(serialize = "spell")]
  Spell,

  #[serde(rename = "synchro")]
  #[strum(serialize = "synchro")]
  Synchro,

  #[serde(rename = "synchro_pendulum")]
  #[strum(serialize = "synchro_pendulum")]
  SynchroPendulum,

  #[serde(rename = "token")]
  #[strum(serialize = "token")]
  Token,

  #[serde(rename = "trap")]
  #[strum(serialize = "trap")]
  Trap,

  #[serde(rename = "xyz")]
  #[strum(serialize = "xyz")]
  Xyz,

  #[serde(rename = "xyz_pendulum")]
  #[strum(serialize = "xyz_pendulum")]
  XyzPendulum,
}
