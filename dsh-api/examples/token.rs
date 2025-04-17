#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;
  let token_fetcher = client.token_fetcher();
  match token_fetcher.fetch_access_token_from_server().await {
    Ok(access_token) => println!("{:#?}", access_token),
    Err(error) => println!("{:#?}", error),
  }

  Ok(())
}
