#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("list_volume_ids");
  for volume_id in client.get_volume_ids().await? {
    println!("{}", volume_id);
  }

  Ok(())
}
