use anyhow::Result;
use clap::Parser;
use ygo::{CardRace, CardType};
use ygo_core::CardQuery;

#[derive(Parser)]
struct Cli {
  name: String,

  #[arg(long)]
  archetype: Option<String>,

  #[arg(long)]
  atk: Option<String>,

  #[arg(long)]
  attribute: Option<String>,

  #[arg(long)]
  banlist: Option<String>,

  #[arg(long)]
  cardset: Option<String>,

  #[arg(long)]
  def: Option<String>,

  #[arg(long)]
  id: Option<u32>,

  #[arg(long)]
  format: Option<String>,

  #[arg(long)]
  konami_id: Option<u32>,

  #[arg(long)]
  level: Option<String>,

  #[arg(long)]
  link: Option<u8>,

  #[arg(long)]
  linkmarker: Option<String>,

  #[arg(long)]
  misc: bool,

  #[arg(long)]
  race: Option<CardRace>,

  #[arg(long)]
  scale: Option<u8>,

  #[arg(long)]
  sort: Option<String>,

  #[arg(long)]
  staple: bool,

  #[arg(long)]
  r#type: Option<CardType>,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Cli::parse();
  let mut query = CardQuery::new()
    .fname(&args.name)
    .misc(args.misc)
    .staple(args.staple);

  if let Some(archetype) = args.archetype {
    query = query.archetype(&archetype);
  }

  if let Some(atk) = args.atk {
    query = query.atk(&atk);
  }

  if let Some(attribute) = args.attribute {
    query = query.attribute(&attribute);
  }

  if let Some(banlist) = args.banlist {
    query = query.banlist(&banlist);
  }

  if let Some(cardset) = args.cardset {
    query = query.cardset(&cardset);
  }

  if let Some(def) = args.def {
    query = query.def(&def);
  }

  if let Some(id) = args.id {
    query = query.id(id.into());
  }

  if let Some(format) = args.format {
    query = query.format(&format);
  }

  if let Some(konami_id) = args.konami_id {
    query = query.konami_id(konami_id);
  }

  if let Some(level) = args.level {
    query = query.level(&level);
  }

  if let Some(link) = args.link {
    query = query.link(link);
  }

  if let Some(linkmarker) = args.linkmarker {
    query = query.linkmarker(&linkmarker);
  }

  if let Some(race) = args.race {
    query = query.race(race);
  }

  if let Some(scale) = args.scale {
    query = query.scale(scale);
  }

  if let Some(sort) = args.sort {
    query = query.sort(&sort);
  }

  if let Some(r#type) = args.r#type {
    query = query.r#type(r#type);
  }

  let cards = query.send().await?;
  println!("{}", serde_json::to_string_pretty(&cards)?);

  Ok(())
}
