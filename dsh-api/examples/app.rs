#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::AppCatalogApp;
use itertools::Itertools;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let app_catalog_id = "keyring-050";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let app_catalog_app: AppCatalogApp = client.get_appcatalogapp_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&app_catalog_app).unwrap());

  let app_catalog_apps: HashMap<String, AppCatalogApp> = client.get_appcatalogapp_configuration_map().await?;
  let mut keys = app_catalog_apps.keys().into_iter().map(|k| k.to_string()).collect_vec();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  let deployed_apps: HashMap<String, AppCatalogApp> = client.get_appcatalogapp_actual_map().await?;
  let mut keys = deployed_apps.keys().into_iter().map(|k| k.to_string()).collect_vec();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  Ok(())
}
