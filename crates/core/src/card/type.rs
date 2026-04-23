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
pub enum CardType {
  #[serde(rename = "Effect Monster")]
  #[strum(serialize = "Effect Monster")]
  EffectMonster,

  #[serde(rename = "Flip Effect Monster")]
  #[strum(serialize = "Flip Effect Monster")]
  FlipEffectMonster,

  #[serde(rename = "Flip Tuner Effect Monster")]
  #[strum(serialize = "Flip Tuner Effect Monster")]
  FlipTunerEffectMonster,

  #[serde(rename = "Fusion Monster")]
  #[strum(serialize = "Fusion Monster")]
  FusionMonster,

  #[serde(rename = "Gemini Monster")]
  #[strum(serialize = "Gemini Monster")]
  GeminiMonster,

  #[serde(rename = "Link Monster")]
  #[strum(serialize = "Link Monster")]
  LinkMonster,

  #[serde(rename = "Normal Monster")]
  #[strum(serialize = "Normal Monster")]
  NormalMonster,

  #[serde(rename = "Normal Tuner Monster")]
  #[strum(serialize = "Normal Tuner Monster")]
  NormalTunerMonster,

  #[serde(rename = "Pendulum Effect Fusion Monster")]
  #[strum(serialize = "Pendulum Effect Fusion Monster")]
  PendulumEffectFusionMonster,

  #[serde(rename = "Pendulum Effect Monster")]
  #[strum(serialize = "Pendulum Effect Monster")]
  PendulumEffectMonster,

  #[serde(rename = "Pendulum Effect Ritual Monster")]
  #[strum(serialize = "Pendulum Effect Ritual Monster")]
  PendulumEffectRitualMonster,

  #[serde(rename = "Pendulum Flip Effect Monster")]
  #[strum(serialize = "Pendulum Flip Effect Monster")]
  PendulumFlipEffectMonster,

  #[serde(rename = "Pendulum Normal Monster")]
  #[strum(serialize = "Pendulum Normal Monster")]
  PendulumNormalMonster,

  #[serde(rename = "Pendulum Tuner Effect Monster")]
  #[strum(serialize = "Pendulum Tuner Effect Monster")]
  PendulumTunerEffectMonster,

  #[serde(rename = "Ritual Effect Monster")]
  #[strum(serialize = "Ritual Effect Monster")]
  RitualEffectMonster,

  #[serde(rename = "Ritual Monster")]
  #[strum(serialize = "Ritual Monster")]
  RitualMonster,

  #[serde(rename = "Skill Card")]
  #[strum(serialize = "Skill Card")]
  SkillCard,

  #[serde(rename = "Spell Card")]
  #[strum(serialize = "Spell Card")]
  SpellCard,

  #[serde(rename = "Spirit Monster")]
  #[strum(serialize = "Spirit Monster")]
  SpiritMonster,

  #[serde(rename = "Synchro Monster")]
  #[strum(serialize = "Synchro Monster")]
  SynchroMonster,

  #[serde(rename = "Synchro Pendulum Effect Monster")]
  #[strum(serialize = "Synchro Pendulum Effect Monster")]
  SynchroPendulumEffectMonster,

  #[serde(rename = "Synchro Tuner Monster")]
  #[strum(serialize = "Synchro Tuner Monster")]
  SynchroTunerMonster,

  #[serde(rename = "Token")]
  #[strum(serialize = "Token")]
  Token,

  #[serde(rename = "Toon Monster")]
  #[strum(serialize = "Toon Monster")]
  ToonMonster,

  #[serde(rename = "Trap Card")]
  #[strum(serialize = "Trap Card")]
  TrapCard,

  #[serde(rename = "Tuner Monster")]
  #[strum(serialize = "Tuner Monster")]
  TunerMonster,

  #[serde(rename = "Union Effect Monster")]
  #[strum(serialize = "Union Effect Monster")]
  UnionEffectMonster,

  #[serde(rename = "XYZ Monster")]
  #[strum(serialize = "XYZ Monster")]
  XyzMonster,

  #[serde(rename = "XYZ Pendulum Effect Monster")]
  #[strum(serialize = "XYZ Pendulum Effect Monster")]
  XyzPendulumEffectMonster,
}
