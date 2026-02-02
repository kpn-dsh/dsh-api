#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::dsh_jwt::DshJwt;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let token = client.token().await?;

  if let Some(stripped_token) = token.strip_prefix("Bearer ") {
    let jwt = DshJwt::from_token(stripped_token.to_string())?;
    println!("{:#}", jwt);
    println!("{}", jwt.token());
    println!("{}", jwt.header());
    println!("{}", jwt.payload());
  }

  Ok(())
}
