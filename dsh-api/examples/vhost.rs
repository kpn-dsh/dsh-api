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
  for (vhost, used_bys) in client.list_vhosts_with_usage().await? {
    println!("{}", vhost);
    for used_by in used_bys {
      println!("  {}", used_by);
    }
  }
  Ok(())
}
