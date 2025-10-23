#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::app_manifest::Manifest;
use dsh_api::version::Version;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
  use dsh_api::dsh_api_client_factory::DshApiClientFactory;

  initialize_logger();

  let manifest_id = "kpn/eavesdropper";
  let manifest_version = Version::from_str("0.9.2")?;

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
  let manifest_ids_with_versions: Vec<(String, Vec<(Version, bool)>)> = client.list_app_catalog_manifest_versions().await?;
  for (id, versions) in manifest_ids_with_versions {
    println!("{} -> {:?}", id, versions);
  }

  println!("-------------------------------------------");
  println!("list_app_catalog_manifests_with_versions");
  println!("-------------------------------------------");
  let manifests_with_versions: Vec<(String, Vec<Manifest>)> = client.list_app_catalog_manifests().await?;
  for (manifest_id, manifests) in manifests_with_versions {
    println!("-------------------------------------------");
    println!("{}", manifest_id);
    for manifest in manifests {
      println!("  {} -> {} : {}", manifest.version, manifest.name, manifest.description.unwrap_or_default());
    }
  }

  println!("-------------------------------------------");
  println!("get_app_catalog_manifests");
  println!("-------------------------------------------");
  let manifests: Vec<Manifest> = client.get_app_catalog_manifests(manifest_id).await?;
  for manifest in manifests {
    println!(
      "{}:{} -> {} : {}",
      manifest_id,
      manifest.version,
      manifest.name,
      manifest.description.unwrap_or_default()
    );
  }

  println!("-------------------------------------------");
  println!("get_app_catalog_manifest");
  println!("-------------------------------------------");
  let manifest = client.get_app_catalog_manifest(manifest_id, &manifest_version).await?;
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
  let (manifest, draft) = client.get_raw_manifest(manifest_id, &manifest_version).await?;
  println!("{}", draft);
  println!("{}", manifest);

  Ok(())
}
