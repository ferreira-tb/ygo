use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
  let cards = ygo_core::fname("Dark Magician").await?;
  println!("{}", serde_json::to_string_pretty(&cards)?);
  Ok(())
}
