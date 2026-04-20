use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Cli {
  name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Cli::parse();
  let cards = ygo_core::fname(&args.name).await?;
  println!("{}", serde_json::to_string_pretty(&cards)?);
  Ok(())
}
