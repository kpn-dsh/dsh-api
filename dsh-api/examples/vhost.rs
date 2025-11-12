#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let _secret_id = "boss-account-ids";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("list_vhosts_with_usage");
  for (vhost, dependants) in client.vhosts_with_dependants().await? {
    println!("{}", vhost);
    for dependant in dependants {
      println!("  {}", dependant);
    }
  }
  Ok(())
}
