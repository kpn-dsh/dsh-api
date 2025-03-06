#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;

#[tokio::main]
async fn main() -> Result<(), String> {
  use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  use dsh_api::types::{AllocationStatus, AppCatalogAppConfiguration};

  initialize_logger();

  let app_catalog_id = "keyring-dev-proxy";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let configuration: AppCatalogAppConfiguration = client.get_appcatalog_app_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&configuration).unwrap());

  let status: AllocationStatus = client.get_appcatalog_app_status(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&status).unwrap());

  Ok(())
}
