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
  let secret_name = "boss-account-ids";
  // let test_secret_name = "test_create_delete_update_secret";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("list_secret_ids");
  for secret in client.get_secret_ids().await? {
    println!("{}", secret);
  }

  print_header("list_secret_names");
  for (secret_name, secret_id) in client.secret_names().await? {
    match secret_id {
      Some(id) => println!("{} -> {}", secret_name, id),
      None => println!("{}", secret_name),
    }
  }

  print_header("list_secret_names_non_system");
  for secret_name in client.secret_names_non_system().await? {
    println!("{}", secret_name);
  }

  print_header("list_secret_names_system");
  for (secret_name, secret_id) in client.secret_names_system().await? {
    println!("{} -> {}", secret_name, secret_id);
  }

  print_header("get_secret");
  println!("get_secret(backend_password)\n{}", client.get_secret(secret_name).await?);

  print_header("get_secret_actual_configuration");
  let secret_actual: dsh_api::types::Empty = client.get_secret_actual(secret_name).await?;
  println!("get_secret_actual({})\n{}", secret_name, serde_json::to_string_pretty(&secret_actual).unwrap());

  print_header("get_secret_configuration");
  println!("{}", client.get_secret_configuration(secret_name).await?);

  print_header("get_secret_allocation_status");
  println!("{}", client.get_secret_status(secret_name).await?);

  print_header("list_secrets_with_usage");
  let secrets_with_dependants: Vec<(String, Option<String>, Vec<Dependant<SecretInjection>>)> = client.secrets_with_dependants().await.unwrap();
  for (secret_name, secret_id, dependants) in secrets_with_dependants {
    if !dependants.is_empty() {
      match secret_id {
        Some(id) => println!("{} -> {}", secret_name, id),
        None => println!("{}", secret_name),
      }

      for dependant in dependants {
        match dependant {
          Dependant::App { app } => println!("  app -> {}", app),
          Dependant::Application { application } => println!("  service -> {}", application),
          Dependant::Certificate { certificate } => println!("  certificate -> {}", certificate),
          Dependant::Proxy { proxy } => println!("  proxy -> {}", proxy),
          Dependant::Trifonius { trifonius } => println!("  trifonius -> {}", trifonius),
        }
      }
    }
  }
  Ok(())
}
