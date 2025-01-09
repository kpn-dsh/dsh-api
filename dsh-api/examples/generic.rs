use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;

#[path = "common.rs"]
mod common;

const APPLICATION_ID: &str = "keyring-dev";

#[tokio::main]
async fn main() -> Result<(), String> {
  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  print_header("get application_configuration_by_tenant_by_appid");
  let application = client.get("get_application_configuration_by_tenant_by_appid", &[APPLICATION_ID]).await?;
  println!("{}", serde_json::to_string_pretty(&application).unwrap());

  print_header("get get_secret_by_tenant");
  let application = client.get("get_secret_by_tenant", &[]).await?;
  println!("{}", serde_json::to_string_pretty(&application).unwrap());

  print_header("put secret_by_tenant");
  let secret = serde_json::to_string("ABCDEF").unwrap();
  client.put("put_secret_by_tenant_by_id", &["abcdef"], &secret).await?;

  Ok(())
}
