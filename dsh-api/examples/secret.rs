#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::secret::SecretInjection;
use dsh_api::Dependant;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
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
  let secrets_with_dependants: Vec<(String, Vec<Dependant<SecretInjection>>)> = client.secrets_with_dependants().await.unwrap();
  for (secret_id, dependants) in secrets_with_dependants {
    if !dependants.is_empty() {
      println!("{}", secret_id);
      for dependant in dependants {
        println!("  {}", dependant);
      }
    }
  }
  Ok(())
}
