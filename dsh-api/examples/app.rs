use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::AppCatalogApp;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), String> {
  env_logger::init();

  let app_catalog_id = "keyring-050";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let app_catalog_app: AppCatalogApp = client.get_appcatalogapp_appcatalogappid_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&app_catalog_app).unwrap());

  let app_catalog_apps: HashMap<String, AppCatalogApp> = client.get_appcatalogapp_configuration_map().await?;
  let mut keys = app_catalog_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  let deployed_apps: HashMap<String, AppCatalogApp> = client.get_appcatalogapp_actual_map().await?;
  let mut keys = deployed_apps.keys().into_iter().map(|k| k.to_string()).collect::<Vec<String>>();
  keys.sort();
  for key in keys {
    let app = app_catalog_apps.get(&key).unwrap();
    println!("{} -> {}", key, app.manifest_urn);
  }

  Ok(())
}
