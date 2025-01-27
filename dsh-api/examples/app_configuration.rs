use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api::types::{AllocationStatus, AppCatalogAppConfiguration};

#[tokio::main]
async fn main() -> Result<(), String> {
  env_logger::init();

  let app_catalog_id = "keyring-dev-proxy";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  let configuration: AppCatalogAppConfiguration = client.get_app_catalog_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&configuration).unwrap());

  let status: AllocationStatus = client.get_app_catalog_configuration_allocation_status(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&status).unwrap());

  Ok(())
}
