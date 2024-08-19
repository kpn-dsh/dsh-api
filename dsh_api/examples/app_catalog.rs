use std::collections::HashMap;

use trifonius_dsh_api::types::AppCatalogApp;
use trifonius_dsh_api::DEFAULT_DSH_API_CLIENT_FACTORY;

#[tokio::main]
async fn main() -> Result<(), String> {
  let app_catalog_id = "keyring-050";

  let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;

  let app_catalog_app: AppCatalogApp = client.get_app_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&app_catalog_app).unwrap());

  println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");

  let app_catalog_apps: HashMap<String, AppCatalogApp> = client.get_app_configurations().await?;
  let mut keys = app_catalog_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");

  let deployed_app: AppCatalogApp = client.get_app_actual_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&deployed_app).unwrap());

  println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>");

  let deployed_apps: HashMap<String, AppCatalogApp> = client.get_app_actual_configurations().await?;
  let mut keys = deployed_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  Ok(())
}
