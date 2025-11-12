#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::manifest::Manifest;
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
  println!("manifest_ids");
  println!("-------------------------------------------");
  let manifest_ids: Vec<String> = client.manifest_ids().await?;
  for manifest_id in manifest_ids {
    println!("{}", manifest_id);
  }

  println!("\n-------------------------------------------");
  println!("manifest_ids_with_versions");
  println!("-------------------------------------------");
  let manifest_ids_with_versions: Vec<(String, Vec<Manifest>)> = client.manifests_all_versions().await?;
  for (manifest_id, manifest_versions) in manifest_ids_with_versions {
    println!("{}", manifest_id);
    for manifest_version in manifest_versions {
      println!("  {}", manifest_version.version);
    }
  }

  println!("-------------------------------------------");
  println!("manifests");
  println!("-------------------------------------------");
  let manifests: Vec<Manifest> = client.manifests().await?;
  for manifest in manifests {
    println!("-------------------------------------------");
    println!("{} -> {}", manifest.name, manifest.version);
  }

  println!("-------------------------------------------");
  println!("app_catalog_manifests");
  println!("-------------------------------------------");
  let manifest_latest_version: Manifest = client.manifest_latest_version(manifest_id, true).await?;
  println!("{}", manifest_latest_version);

  println!("-------------------------------------------");
  println!("app_catalog_manifest");
  println!("-------------------------------------------");
  let manifest = client.manifest(manifest_id, &manifest_version).await?;
  println!(
    "{}:{} -> {} : {}",
    manifest.id,
    manifest.version,
    manifest.name,
    manifest.description.unwrap_or_default()
  );

  println!("-------------------------------------------");
  println!("raw_manifest");
  println!("-------------------------------------------");
  let (manifest, draft) = client.manifest_raw(manifest_id, &manifest_version).await?;
  println!("{}", draft);
  println!("{}", manifest);

  Ok(())
}
