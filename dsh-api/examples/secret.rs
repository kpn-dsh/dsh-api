use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::UsedBy;

#[path = "common.rs"]
mod common;

#[tokio::main]
async fn main() -> Result<(), String> {
  env_logger::init();
  let secret_id = "boss-account-ids";
  let test_secret_id = "test_create_delete_update_secret";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("list_secret_ids");
  for secret in client.get_secret_ids().await? {
    println!("{}", secret);
  }

  print_header("get_secret");
  println!("get_secret(backend_password)\n{}", client.get_secret(secret_id).await?);

  print_header("get_secret_actual_configuration");
  let secret_actual: dsh_api::types::Empty = client.get_secret_actual(test_secret_id).await?;
  println!("get_secret_actual({})\n{}", test_secret_id, serde_json::to_string_pretty(&secret_actual).unwrap());

  print_header("get_secret_configuration");
  println!("{}", client.get_secret_configuration(test_secret_id).await?);

  print_header("get_secret_allocation_status");
  println!("{}", client.get_secret_status(test_secret_id).await?);

  print_header("list_secrets_with_usage");
  let secrets_with_usage: Vec<(String, Vec<UsedBy>)> = client.list_secrets_with_usage().await.unwrap();
  for (secret_id, usage) in secrets_with_usage {
    if !usage.is_empty() {
      println!("{}", secret_id);
      for used_by in usage {
        println!("  {}", used_by);
      }
    }
  }
  Ok(())
}
