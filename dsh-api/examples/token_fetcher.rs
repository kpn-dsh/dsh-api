#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::token_fetcher::TokenFetcher;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let token_fetcher = TokenFetcher::try_default()?;

  let bearer_token = token_fetcher.get_bearer_token().await?;
  println!("bearer token -> {}", bearer_token);

  let raw_token = token_fetcher.get_raw_token().await?;
  println!("raw token -> {}", raw_token);

  let jwt = token_fetcher.get_jwt().await?;
  println!("jwt -> {}", jwt);

  let fresh_bearer_token = token_fetcher.get_fresh_bearer_token().await?;
  println!("fresh bearer token -> {}", fresh_bearer_token);

  let fresh_raw_token = token_fetcher.get_fresh_raw_token().await?;
  println!("fresh raw token -> {}", fresh_raw_token);

  let fresh_jwt = token_fetcher.get_fresh_jwt().await?;
  println!("fresh jwt -> {:#?}", fresh_jwt);

  Ok(())
}
