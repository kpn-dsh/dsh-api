#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::app_manifest::Manifest;

#[tokio::main]
async fn main() -> Result<(), String> {
  use dsh_api::dsh_api_client_factory::DshApiClientFactory;

  initialize_logger();

  let manifest_id = "kpn/eavesdropper";
  let manifest_version = "0.9.2";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

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
    println!("-------------------------------------------");
    println!("{}", manifest_id);
    for (version, manifest) in versions {
      println!("  {} -> {} : {}", version, manifest.name, manifest.description.unwrap_or_default());
    }
  }

  println!("-------------------------------------------");
  println!("get_app_catalog_manifests");
  println!("-------------------------------------------");
  let manifests: Vec<(String, Manifest)> = client.get_app_catalog_manifests(manifest_id).await?;
  for (version, manifest) in manifests {
    println!("{}:{} -> {} : {}", manifest_id, version, manifest.name, manifest.description.unwrap_or_default());
  }

  println!("-------------------------------------------");
  println!("get_app_catalog_manifest");
  println!("-------------------------------------------");
  let manifest = client.get_app_catalog_manifest(manifest_id, manifest_version).await?;
  println!(
    "{}:{} -> {} : {}",
    manifest.id,
    manifest.version,
    manifest.name,
    manifest.description.unwrap_or_default()
  );

  println!("-------------------------------------------");
  println!("get_raw_manifest");
  println!("-------------------------------------------");
  let manifest = client.get_raw_manifest(manifest_id, manifest_version).await?;
  println!("{}", manifest);

  Ok(())
}
