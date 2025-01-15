//! # View the App Catalog manifests
//!
//! Module that contains a function to query the App Catalog for all manifest files.
//!
//! # API methods
//! * [`list_app_catalog_manifests() -> [manifest]`](DshApiClient::list_app_catalog_manifests)
//!
//! # Derived methods
//! * [`list_app_catalog_manifest_ids() -> [id]`](DshApiClient::list_app_catalog_manifest_ids)
//! * [`list_app_catalog_manifest_ids_with_versions() -> [id, [version]]`](DshApiClient::list_app_catalog_manifest_ids_with_versions)
//! * [`list_app_catalog_manifests_with_versions() -> [id, [(version, manifest)]]`](DshApiClient::list_app_catalog_manifests_with_versions)
use crate::dsh_api_client::DshApiClient;
use crate::types::AppCatalogManifest;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;
use chrono::{TimeZone, Utc};
use itertools::Itertools;
use serde_json::{from_str, Value};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub const API_VERSION: &str = "apiVersion";
pub const CONFIGURATION: &str = "configuration";
pub const CONTACT: &str = "contact";
pub const DESCRIPTION: &str = "description";
pub const ID: &str = "id";
pub const KIND: &str = "kind";
pub const MORE_INFO: &str = "moreInfo";
pub const NAME: &str = "name";
pub const RESOURCES: &str = "resources";
pub const VENDOR: &str = "vendor";
pub const VERSION: &str = "version";

/// # View the App Catalog manifests
///
/// Module that contains a function to query the App Catalog for all manifest files.
///
/// # API methods
/// * [`list_app_catalog_manifests() -> [manifest]`](DshApiClient::list_app_catalog_manifests)
///
/// # Derived methods
/// * [`list_app_catalog_manifest_ids() -> [id]`](DshApiClient::list_app_catalog_manifest_ids)
/// * [`list_app_catalog_manifest_ids_with_versions() -> [id, [version]]`](DshApiClient::list_app_catalog_manifest_ids_with_versions)
/// * [`list_app_catalog_manifests_with_versions() -> [id, [(version, manifest)]]`](DshApiClient::list_app_catalog_manifests_with_versions)
impl DshApiClient<'_> {
  /// # Return a list of all App Catalog manifests
  ///
  /// API function: `GET /appcatalog/{tenant}/manifest`
  ///
  /// # Returns
  /// * `Ok<Vec`[`AppCatalogManifest`]`>` - vector containing all app manifests
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifests(&self) -> DshApiResult<Vec<AppCatalogManifest>> {
    self
      .process(
        self
          .generated_client
          .get_appcatalog_manifest_by_tenant(self.tenant_name(), self.token().await?.as_str())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return sorted list of all App Catalog manifest ids
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing all manifest ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifest_ids(&self) -> DshApiResult<Vec<String>> {
    let mut unique_ids: HashSet<String> = HashSet::new();
    for manifest in self.list_app_catalog_manifests().await? {
      match from_str::<Value>(manifest.payload.as_str())?.as_object() {
        Some(payload_object) => {
          if let Some(id) = payload_object.get(ID).and_then(|id| id.as_str().map(|id| id.to_string())) {
            unique_ids.insert(id);
          }
        }
        None => return Err(DshApiError::from("payload is not a json object")),
      }
    }
    let mut ids: Vec<String> = unique_ids.iter().map(|id| id.to_string()).collect();
    ids.sort();
    Ok(ids)
  }

  /// # Return list of all App Catalog manifest ids with available version numbers
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<String>)>>` - vector containing pairs of ids and versions
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifest_ids_with_versions(&self) -> DshApiResult<Vec<(String, Vec<String>)>> {
    let mut id_versions: HashMap<String, Vec<String>> = HashMap::new();
    for manifest in self.list_app_catalog_manifests().await? {
      match from_str::<Value>(manifest.payload.as_str())?.as_object() {
        Some(payload_object) => {
          if let Some(id) = payload_object.get(ID).and_then(|id| id.as_str().map(|id| id.to_string())) {
            if let Some(version) = payload_object.get(VERSION).and_then(|version| version.as_str().map(|version| version.to_string())) {
              id_versions.entry(id).and_modify(|versions| versions.push(version.clone())).or_insert(vec![version]);
            }
          }
        }
        None => return Err(DshApiError::from("payload is not a json object")),
      }
    }
    let mut id_versions_pairs: Vec<(String, Vec<String>)> = id_versions.iter().map(|(k, v)| (k.to_string(), v.clone())).collect();
    id_versions_pairs.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    for (_, versions) in id_versions_pairs.iter_mut() {
      versions.sort(); // TODO Sort as semver instead of lexicographically
    }
    Ok(id_versions_pairs)
  }

  /// # Return list of all App Catalog manifests with available manifest versions
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<(String, Manifest)>)>>` - vector containing pairs of ids and versions
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifests_with_versions(&self) -> DshApiResult<Vec<(String, Vec<(String, Manifest)>)>> {
    let manifests: Vec<(String, Manifest)> = self
      .list_app_catalog_manifests()
      .await?
      .iter()
      .map(|app_catalog_manifest| Manifest::try_from(app_catalog_manifest).map(|manifest| (manifest.manifest_id.clone(), manifest)))
      .collect::<Result<_, _>>()?;
    let mut manifests_grouped_vec: Vec<(String, Vec<Manifest>)> = manifests.into_iter().into_group_map().into_iter().collect::<Vec<_>>();
    manifests_grouped_vec.sort_by(|(manifest_id_a, _), (manifest_id_b, _)| manifest_id_a.cmp(manifest_id_b));
    let manifests_with_available_versions: Vec<(String, Vec<(String, Manifest)>)> = manifests_grouped_vec
      .into_iter()
      .map(|(manifest_id, manifests)| {
        (manifest_id, {
          let mut version_manifest = manifests.into_iter().map(|manifest| (manifest.version.clone(), manifest)).collect::<Vec<_>>();
          version_manifest.sort_by(|(version_a, _), (version_b, _)| version_a.cmp(version_b)); // TODO Sort as semver instead of lexicographically
          version_manifest
        })
      })
      .collect::<Vec<_>>();
    Ok(manifests_with_available_versions)
  }
}

#[derive(Debug)]
pub struct Manifest {
  pub manifest_id: String,
  pub contact: String,
  pub draft: bool,
  pub last_modified: String,
  pub name: String,
  pub vendor: String,
  pub version: String,
}

impl TryFrom<&AppCatalogManifest> for Manifest {
  type Error = String;

  fn try_from(value: &AppCatalogManifest) -> Result<Self, Self::Error> {
    match from_str::<Value>(value.payload.as_str()) {
      Ok(payload_value) => match payload_value.as_object() {
        Some(payload_object) => Ok(Manifest {
          manifest_id: payload_object
            .get(&ID.to_string())
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_default(),
          contact: payload_object
            .get(&CONTACT.to_string())
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_default(),
          draft: value.draft,
          last_modified: Utc
            .timestamp_opt(value.last_modified as i64 / 1000, 0)
            .single()
            .map(|g| g.to_string())
            .unwrap_or_default(),
          name: payload_object
            .get(&NAME.to_string())
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_default(),
          vendor: payload_object
            .get(&VENDOR.to_string())
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_default(),
          version: payload_object
            .get(&VERSION.to_string())
            .and_then(|v| v.as_str())
            .map(|v| v.to_string())
            .unwrap_or_default(),
        }),
        None => Err("payload is not a json object".to_string()),
      },
      Err(_) => Err("could not parse payload".to_string()),
    }
  }
}

impl Display for Manifest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "id: {}, contact: {}, draft: {}, last_modified: {}, name: {}, vendor: {}, version: {}",
      self.manifest_id, self.contact, self.draft, self.last_modified, self.name, self.vendor, self.version
    )
  }
}
