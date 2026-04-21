use crate::error::Result;
use crate::http::download;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIs, EnumString, IntoStaticStr};
use url::Url;

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct Card {
  pub archetype: Option<String>,
  pub atk: Option<u32>,
  pub attribute: Option<String>,
  pub banlist_info: Option<BanListInfo>,
  pub card_images: Vec<CardImage>,
  pub card_prices: Vec<CardPrice>,
  pub card_sets: Vec<CardSet>,
  pub def: Option<u32>,
  pub desc: Option<String>,
  #[serde(rename = "frameType")]
  pub frame_type: Option<CardFrameType>,
  #[serde(rename = "humanReadableCardType")]
  pub human_readable_card_type: Option<String>,
  pub id: Option<CardId>,
  pub level: Option<u8>,
  pub linkmarkers: Vec<String>,
  pub linkval: Option<u8>,
  pub misc_info: Option<CardMisc>,
  pub name: Option<String>,
  pub r#type: Option<CardType>,
  pub race: Option<CardRace>,
  pub typeline: Vec<String>,
  pub ygoprodeck_url: Option<Url>,
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
#[derive(
  Clone, Copy, Debug, Display, EnumIs, EnumString, IntoStaticStr, Deserialize, Serialize,
)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub enum CardRace {
  #[serde(rename = "Aqua")]
  #[strum(serialize = "Aqua")]
  Aqua,

  #[serde(rename = "Beast")]
  #[strum(serialize = "Beast")]
  Beast,

  #[serde(rename = "Beast-Warrior")]
  #[strum(serialize = "Beast-Warrior")]
  BeastWarrior,

  #[serde(rename = "Continuous")]
  #[strum(serialize = "Continuous")]
  Continuous,

  #[serde(rename = "Counter")]
  #[strum(serialize = "Counter")]
  Counter,

  #[serde(rename = "Creator-God")]
  #[strum(serialize = "Creator-God")]
  CreatorGod,

  #[serde(rename = "Cyberse")]
  #[strum(serialize = "Cyberse")]
  Cyberse,

  #[serde(rename = "Dinosaur")]
  #[strum(serialize = "Dinosaur")]
  Dinosaur,

  #[serde(rename = "Divine-Beast")]
  #[strum(serialize = "Divine-Beast")]
  DivineBeast,

  #[serde(rename = "Dragon")]
  #[strum(serialize = "Dragon")]
  Dragon,

  #[serde(rename = "Equip")]
  #[strum(serialize = "Equip")]
  Equip,

  #[serde(rename = "Fairy")]
  #[strum(serialize = "Fairy")]
  Fairy,

  #[serde(rename = "Field")]
  #[strum(serialize = "Field")]
  Field,

  #[serde(rename = "Fiend")]
  #[strum(serialize = "Fiend")]
  Fiend,

  #[serde(rename = "Fish")]
  #[strum(serialize = "Fish")]
  Fish,

  #[serde(rename = "Insect")]
  #[strum(serialize = "Insect")]
  Insect,

  #[serde(rename = "Machine")]
  #[strum(serialize = "Machine")]
  Machine,

  #[serde(rename = "Normal")]
  #[strum(serialize = "Normal")]
  Normal,

  #[serde(rename = "Plant")]
  #[strum(serialize = "Plant")]
  Plant,

  #[serde(rename = "Psychic")]
  #[strum(serialize = "Psychic")]
  Psychic,

  #[serde(rename = "Pyro")]
  #[strum(serialize = "Pyro")]
  Pyro,

  #[serde(rename = "Quick-Play")]
  #[strum(serialize = "Quick-Play")]
  QuickPlay,

  #[serde(rename = "Reptile")]
  #[strum(serialize = "Reptile")]
  Reptile,

  #[serde(rename = "Ritual")]
  #[strum(serialize = "Ritual")]
  Ritual,

  #[serde(rename = "Rock")]
  #[strum(serialize = "Rock")]
  Rock,

  #[serde(rename = "Sea Serpent")]
  #[strum(serialize = "Sea Serpent")]
  SeaSerpent,

  #[serde(rename = "Spellcaster")]
  #[strum(serialize = "Spellcaster")]
  Spellcaster,

  #[serde(rename = "Thunder")]
  #[strum(serialize = "Thunder")]
  Thunder,

  #[serde(rename = "Warrior")]
  #[strum(serialize = "Warrior")]
  Warrior,

  #[serde(rename = "Winged Beast")]
  #[strum(serialize = "Winged Beast")]
  WingedBeast,

  #[serde(rename = "Wyrm")]
  #[strum(serialize = "Wyrm")]
  Wyrm,

  #[serde(rename = "Zombie")]
  #[strum(serialize = "Zombie")]
  Zombie,
}

#[remain::sorted]
#[derive(
  Clone, Copy, Debug, Display, EnumIs, EnumString, IntoStaticStr, Deserialize, Serialize,
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

#[remain::sorted]
#[derive(
  Clone, Copy, Debug, Display, EnumIs, EnumString, IntoStaticStr, Deserialize, Serialize,
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

impl CardImage {
  async fn download(&self, url: Option<&Url>) -> Result<Option<Vec<u8>>> {
    if let Some(url) = url {
      Ok(Some(download(url.clone()).await?))
    } else {
      Ok(None)
    }
  }

  pub async fn download_image(&self) -> Result<Option<Vec<u8>>> {
    self.download(self.image_url.as_ref()).await
  }

  pub async fn download_cropped_image(&self) -> Result<Option<Vec<u8>>> {
    self
      .download(self.image_url_cropped.as_ref())
      .await
  }

  pub async fn download_small_image(&self) -> Result<Option<Vec<u8>>> {
    self
      .download(self.image_url_small.as_ref())
      .await
  }
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
  pub formats: Vec<String>,
  pub has_effect: Option<u8>,
  pub konami_id: Option<u32>,
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

#[remain::sorted]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(default)]
pub struct BanListInfo {
  pub ban_ocg: Option<String>,
  pub ban_tcg: Option<String>,
}
