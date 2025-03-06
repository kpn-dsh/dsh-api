#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;

#[tokio::main]
async fn main() -> Result<(), String> {
  use dsh_api::app_manifest::{Manifest, API_VERSION, ID, KIND, NAME, VENDOR, VERSION};
  use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  use dsh_api::types::AppCatalogAppConfiguration;
  use dsh_api::types::AppCatalogManifest;
  use serde_json::de::from_str;
  use serde_json::Value;

  initialize_logger();

  let app_catalog_id = "keyring-050";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  println!("-------------------------------------------");
  println!("list_app_catalog_manifests");
  println!("-------------------------------------------");
  let manifests: Vec<AppCatalogManifest> = client.get_appcatalog_manifests().await?;
  for manifest in manifests {
    let payload = &manifest.payload;
    let des = from_str::<Value>(payload.as_str()).unwrap();
    let object = des.as_object().unwrap();
    println!("api version    {}", object.get(API_VERSION).unwrap());
    println!("id             {}", object.get(ID).unwrap().as_str().unwrap());
    println!("kind           {}", object.get(KIND).unwrap());
    println!("name           {}", object.get(NAME).unwrap());
    println!("vendor         {}", object.get(VENDOR).unwrap());
    println!("version        {}", object.get(VERSION).unwrap());
    println!("-------------------------------------------");
  }

  println!("\n-------------------------------------------");
  println!("list_app_catalog_manifest_ids");
  println!("-------------------------------------------");
  let manifest_ids: Vec<String> = client.list_app_catalog_manifest_ids().await?;
  for manifest_id in manifest_ids {
    println!("{}", manifest_id);
  }

  println!("\n-------------------------------------------");
  println!("list_app_catalog_manifest_ids_with_versions");
  println!("-------------------------------------------");
  let manifest_ids_with_versions: Vec<(String, Vec<String>)> = client.list_app_catalog_manifest_versions().await?;
  for manifest_id_with_versions in manifest_ids_with_versions {
    println!("{} -> {:?}", manifest_id_with_versions.0, manifest_id_with_versions.1);
  }

  println!("-------------------------------------------");
  println!("list_app_catalog_manifests_with_versions");
  println!("-------------------------------------------");
  let manifests_with_versions: Vec<(String, Vec<(String, Manifest)>)> = client.list_app_catalog_manifests().await?;
  for (manifest_id, versions) in manifests_with_versions {
    println!("{}", manifest_id);
    for (version, manifest) in versions {
      println!("  {} -> {}", version, manifest);
    }
  }

  let deployed_app: AppCatalogAppConfiguration = client.get_appcatalog_app_configuration(app_catalog_id).await?;
  println!("{}", serde_json::to_string_pretty(&deployed_app).unwrap());

  Ok(())
}
