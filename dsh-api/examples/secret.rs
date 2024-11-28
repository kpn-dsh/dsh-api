use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api::UsedBy;

#[path = "common.rs"]
mod common;

#[tokio::main]
async fn main() -> Result<(), String> {
  let secret_id = "boss-account-ids";
  let test_secret_id = "test_create_delete_update_secret";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  // let test_secret = Secret { name: test_secret_id.to_string(), value: "TEST_SECRET".to_string() };
  // let test_secret_update = "TEST_SECRET_UPDATE".to_string();
  // print_header("create_secret");
  // println!("{:?}", client.create_secret(&test_secret).await?);
  // print_header("delete_secret");
  // println!("{:?}", client.delete_secret(secret_id).await?);
  // print_header("update_secret");
  // println!("{:?}", client.update_secret(secret_id, test_secret_update).await?);

  print_header("list_secret_ids");
  for secret in client.list_secret_ids().await? {
    println!("{}", secret);
  }

  print_header("get_secret");
  println!("get_secret(greenbox_backend_password)\n{}", client.get_secret(secret_id).await?);

  #[cfg(feature = "actual")]
  {
    print_header("get_secret_actual_configuration");
    let secret_actual: dsh_api::types::Empty = client.get_secret_actual_configuration(test_secret_id).await?;
    println!("get_secret_actual({})\n{}", test_secret_id, serde_json::to_string_pretty(&secret_actual).unwrap());
  }

  print_header("get_secret_configuration");
  println!("{}", client.get_secret_configuration(test_secret_id).await?);

  print_header("get_secret_allocation_status");
  println!("{}", client.get_secret_allocation_status(test_secret_id).await?);

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
