use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;

#[path = "common.rs"]
mod common;

#[tokio::main]
async fn main() -> Result<(), String> {
  let _secret_id = "boss-account-ids";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
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
