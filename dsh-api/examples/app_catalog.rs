use std::collections::HashMap;

use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api_raw::types::AppCatalogApp;

#[tokio::main]
async fn main() -> Result<(), String> {
  let app_catalog_id = "keyring-050";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  let app_catalog_app: AppCatalogApp = client.get_app_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&app_catalog_app).unwrap());

  let app_catalog_apps: HashMap<String, AppCatalogApp> = client.get_app_configurations().await?;
  let mut keys = app_catalog_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  let deployed_app: AppCatalogApp = client.get_app_actual_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&deployed_app).unwrap());

  let deployed_apps: HashMap<String, AppCatalogApp> = client.get_app_actual_configurations().await?;
  let mut keys = deployed_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  Ok(())
}
