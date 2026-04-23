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
pub enum CardRace {
  #[serde(rename = "Abidos the Th")]
  #[strum(serialize = "Abidos the Th")]
  AbidosTheTh,

  #[serde(rename = "Adrian Gecko")]
  #[strum(serialize = "Adrian Gecko")]
  AdrianGecko,

  #[serde(rename = "Alexis Rhodes")]
  #[strum(serialize = "Alexis Rhodes")]
  AlexisRhodes,

  #[serde(rename = "Amnael")]
  #[strum(serialize = "Amnael")]
  Amnael,

  #[serde(rename = "Andrew")]
  #[strum(serialize = "Andrew")]
  Andrew,

  #[serde(rename = "Aqua")]
  #[strum(serialize = "Aqua")]
  Aqua,

  #[serde(rename = "Arkana")]
  #[strum(serialize = "Arkana")]
  Arkana,

  #[serde(rename = "Aster Phoenix")]
  #[strum(serialize = "Aster Phoenix")]
  AsterPhoenix,

  #[serde(rename = "Axel Brodie")]
  #[strum(serialize = "Axel Brodie")]
  AxelBrodie,

  #[serde(rename = "Bastion Misaw")]
  #[strum(serialize = "Bastion Misaw")]
  BastionMisaw,

  #[serde(rename = "Beast")]
  #[strum(serialize = "Beast")]
  Beast,

  #[serde(rename = "Beast-Warrior")]
  #[strum(serialize = "Beast-Warrior")]
  BeastWarrior,

  #[serde(rename = "Bonz")]
  #[strum(serialize = "Bonz")]
  Bonz,

  #[serde(rename = "Camula")]
  #[strum(serialize = "Camula")]
  Camula,

  #[serde(rename = "Chazz Princet")]
  #[strum(serialize = "Chazz Princet")]
  ChazzPrincet,

  #[serde(rename = "Christine")]
  #[strum(serialize = "Christine")]
  Christine,

  #[serde(rename = "Chumley Huffi")]
  #[strum(serialize = "Chumley Huffi")]
  ChumleyHuffi,

  #[serde(rename = "Continuous")]
  #[strum(serialize = "Continuous")]
  Continuous,

  #[serde(rename = "Counter")]
  #[strum(serialize = "Counter")]
  Counter,

  #[serde(rename = "Creator God")]
  #[strum(serialize = "Creator God")]
  CreatorGod,

  #[serde(rename = "Cyberse")]
  #[strum(serialize = "Cyberse")]
  Cyberse,

  #[serde(rename = "David")]
  #[strum(serialize = "David")]
  David,

  #[serde(rename = "Dinosaur")]
  #[strum(serialize = "Dinosaur")]
  Dinosaur,

  #[serde(rename = "Divine-Beast")]
  #[strum(serialize = "Divine-Beast")]
  DivineBeast,

  #[serde(rename = "Don Zaloog")]
  #[strum(serialize = "Don Zaloog")]
  DonZaloog,

  #[serde(rename = "Dragon")]
  #[strum(serialize = "Dragon")]
  Dragon,

  #[serde(rename = "Dr. Vellian C")]
  #[strum(serialize = "Dr. Vellian C")]
  DrVellianC,

  #[serde(rename = "Emma")]
  #[strum(serialize = "Emma")]
  Emma,

  #[serde(rename = "Equip")]
  #[strum(serialize = "Equip")]
  Equip,

  #[serde(rename = "Espa Roba")]
  #[strum(serialize = "Espa Roba")]
  EspaRoba,

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

  #[serde(rename = "Illusion")]
  #[strum(serialize = "Illusion")]
  Illusion,

  #[serde(rename = "Insect")]
  #[strum(serialize = "Insect")]
  Insect,

  #[serde(rename = "Ishizu")]
  #[strum(serialize = "Ishizu")]
  Ishizu,

  #[serde(rename = "Ishizu Ishtar")]
  #[strum(serialize = "Ishizu Ishtar")]
  IshizuIshtar,

  #[serde(rename = "Jaden Yuki")]
  #[strum(serialize = "Jaden Yuki")]
  JadenYuki,

  #[serde(rename = "Jesse Anderso")]
  #[strum(serialize = "Jesse Anderso")]
  JesseAnderso,

  #[serde(rename = "Joey")]
  #[strum(serialize = "Joey")]
  Joey,

  #[serde(rename = "Joey Wheeler")]
  #[strum(serialize = "Joey Wheeler")]
  JoeyWheeler,

  #[serde(rename = "Kagemaru")]
  #[strum(serialize = "Kagemaru")]
  Kagemaru,

  #[serde(rename = "Kaiba")]
  #[strum(serialize = "Kaiba")]
  Kaiba,

  #[serde(rename = "Keith")]
  #[strum(serialize = "Keith")]
  Keith,

  #[serde(rename = "Lumis and Umb")]
  #[strum(serialize = "Lumis and Umb")]
  LumisAndUmb,

  #[serde(rename = "Lumis Umbra")]
  #[strum(serialize = "Lumis Umbra")]
  LumisUmbra,

  #[serde(rename = "Machine")]
  #[strum(serialize = "Machine")]
  Machine,

  #[serde(rename = "Mai")]
  #[strum(serialize = "Mai")]
  Mai,

  #[serde(rename = "Mai Valentine")]
  #[strum(serialize = "Mai Valentine")]
  MaiValentine,

  #[serde(rename = "Mako")]
  #[strum(serialize = "Mako")]
  Mako,

  #[serde(rename = "Nightshroud")]
  #[strum(serialize = "Nightshroud")]
  Nightshroud,

  #[serde(rename = "")]
  #[strum(serialize = "")]
  None,

  #[serde(rename = "Normal")]
  #[strum(serialize = "Normal")]
  Normal,

  #[serde(rename = "Odion")]
  #[strum(serialize = "Odion")]
  Odion,

  #[serde(rename = "Paradox Broth")]
  #[strum(serialize = "Paradox Broth")]
  ParadoxBroth,

  #[serde(rename = "Pegasus")]
  #[strum(serialize = "Pegasus")]
  Pegasus,

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

  #[serde(rename = "Rex")]
  #[strum(serialize = "Rex")]
  Rex,

  #[serde(rename = "Ritual")]
  #[strum(serialize = "Ritual")]
  Ritual,

  #[serde(rename = "Rock")]
  #[strum(serialize = "Rock")]
  Rock,

  #[serde(rename = "Sea Serpent")]
  #[strum(serialize = "Sea Serpent")]
  SeaSerpent,

  #[serde(rename = "Seto Kaiba")]
  #[strum(serialize = "Seto Kaiba")]
  SetoKaiba,

  #[serde(rename = "Spellcaster")]
  #[strum(serialize = "Spellcaster")]
  Spellcaster,

  #[serde(rename = "Syrus Truesda")]
  #[strum(serialize = "Syrus Truesda")]
  SyrusTruesda,

  #[serde(rename = "Tania")]
  #[strum(serialize = "Tania")]
  Tania,

  #[serde(rename = "Tea Gardner")]
  #[strum(serialize = "Tea Gardner")]
  TeaGardner,

  #[serde(rename = "Thelonious Vi")]
  #[strum(serialize = "Thelonious Vi")]
  TheloniousVi,

  #[serde(rename = "The Supreme K")]
  #[strum(serialize = "The Supreme K")]
  TheSupremeK,

  #[serde(rename = "Thunder")]
  #[strum(serialize = "Thunder")]
  Thunder,

  #[serde(rename = "Titan")]
  #[strum(serialize = "Titan")]
  Titan,

  #[serde(rename = "Tyranno Hassl")]
  #[strum(serialize = "Tyranno Hassl")]
  TyrannoHassl,

  #[serde(rename = "Warrior")]
  #[strum(serialize = "Warrior")]
  Warrior,

  #[serde(rename = "Weevil")]
  #[strum(serialize = "Weevil")]
  Weevil,

  #[serde(rename = "Winged Beast")]
  #[strum(serialize = "Winged Beast")]
  WingedBeast,

  #[serde(rename = "Wyrm")]
  #[strum(serialize = "Wyrm")]
  Wyrm,

  #[serde(rename = "Yami Bakura")]
  #[strum(serialize = "Yami Bakura")]
  YamiBakura,

  #[serde(rename = "Yami Marik")]
  #[strum(serialize = "Yami Marik")]
  YamiMarik,

  #[serde(rename = "Yami Yugi")]
  #[strum(serialize = "Yami Yugi")]
  YamiYugi,

  #[serde(rename = "Yubel")]
  #[strum(serialize = "Yubel")]
  Yubel,

  #[serde(rename = "Yugi")]
  #[strum(serialize = "Yugi")]
  Yugi,

  #[serde(rename = "Zane Truesdal")]
  #[strum(serialize = "Zane Truesdal")]
  ZaneTruesdal,

  #[serde(rename = "Zombie")]
  #[strum(serialize = "Zombie")]
  Zombie,
}
