#[cfg(not(feature = "appcatalog"))]
use std::error::Error;
#[cfg(not(feature = "appcatalog"))]
fn main() -> Result<(), Box<dyn Error>> {
  Ok(())
}
#[cfg(feature = "appcatalog")]
#[tokio::main]
async fn main() -> Result<(), String> {
  use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  use dsh_api::types::{AllocationStatus, AppCatalogAppConfiguration};

  env_logger::init();

  let app_catalog_id = "keyring-dev-proxy";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let configuration: AppCatalogAppConfiguration = client.get_appcatalog_appcatalogapp_appcatalogappid_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&configuration).unwrap());

  let status: AllocationStatus = client.get_appcatalog_appcatalogapp_appcatalogappid_status(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&status).unwrap());

  Ok(())
}
