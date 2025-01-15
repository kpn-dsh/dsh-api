use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use std::error::Error;

#[path = "common.rs"]
mod common;

const APPLICATION_ID: &str = "keyring-dev";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;

  print_header("get application_configuration_by_tenant_by_appid");
  let application = client.get("application-configuration", &[APPLICATION_ID]).await?;
  print_header("json");
  println!("{}", serde_json::to_string_pretty(&application)?);
  print_header("yaml");
  println!("{}", serde_yaml::to_string(&application)?);
  print_header("toml");
  println!("{}", toml::to_string_pretty(&application)?);

  print_header("get get_secret_by_tenant");
  let application = client.get("secret", &["abcdef"]).await?;
  println!("{}", serde_json::to_string_pretty(&application)?);

  print_header("put secret");
  let secret = serde_json::to_string("ABCDEF")?;
  client.put("secret", &["abcdef"], &secret).await?;

  Ok(())
}
