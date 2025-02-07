use std::error::Error;

#[path = "common.rs"]
mod common;

#[cfg(not(feature = "generic"))]
fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}
#[cfg(feature = "generic")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  use crate::common::print_header;
  use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
  use dsh_api::types::{LimitValue, LimitValueCpu, LimitValueCpuName, LimitValueMem, LimitValueMemName};
  env_logger::init();

  const APPLICATION_ID: &str = "keyring-dev";

  let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;

  print_header("get application_configuration_by_tenant_by_appid");
  let application = client.get("application-configuration", &[APPLICATION_ID]).await?;
  print_header("json");
  println!("{}", serde_json::to_string_pretty(&application)?);

  print_header("yaml");
  println!("{}", serde_yaml::to_string(&application)?);
  print_header("toml");
  println!("{}", toml::to_string_pretty(&application)?);

  print_header("get secret");
  let application = client.get("secret", &["aaaa"]).await?;
  println!("{}", serde_json::to_string_pretty(&application)?);

  print_header("post secret");
  let secret = r#"{"name": "secret-name","value": "secret-value"}"#.to_string();
  println!("{:#?}", client.post("secret", &[], Some(secret)).await?);

  print_header("put secret");
  let secret = serde_json::to_string("ABCDEF")?;
  client.put("secret", &["abcdef"], Some(secret)).await?;

  let limit_values: Vec<LimitValue> =
    vec![LimitValue::Cpu(LimitValueCpu { name: LimitValueCpuName::Cpu, value: 2.0 }), LimitValue::Mem(LimitValueMem { name: LimitValueMemName::Mem, value: 1000 })];
  let body = serde_json::to_string(&limit_values)?;
  client.patch("manage-tenant-limit", &["my-tenant"], Some(body)).await?;

  Ok(())
}
