//! # Additional methods to manage the app catalog manifests
//!
//! Module that contains methods to query the App Catalog for all manifest files.
//!
//! # Derived methods
//! * [`get_app_catalog_manifest(id, version) -> [manifest]`](DshApiClient::get_app_catalog_manifest)
//! * [`get_app_catalog_manifest_latest(id, draft) -> [manifest]`](DshApiClient::get_app_catalog_manifest_latest)
//! * [`get_app_catalog_manifests(id) -> [manifest]`](DshApiClient::get_app_catalog_manifests)
//! * [`get_manifests() -> [manifest]`](DshApiClient::get_manifests)
//! * [`get_raw_manifest(id, version) -> [(json, draft)]`](DshApiClient::get_raw_manifest)
//! * [`get_raw_manifest_latest(id, allow_draft) -> [(version, json, draft)]`](DshApiClient::get_raw_manifest_latest)
//! * [`list_app_catalog_manifest_ids() -> [id]`](DshApiClient::list_app_catalog_manifest_ids)
//! * [`list_app_catalog_manifest_versions() -> [id, [(version, draft)]]`](DshApiClient::list_app_catalog_manifest_versions)
//! * [`list_app_catalog_manifests() -> [id, [manifest]]`](DshApiClient::list_app_catalog_manifests)
//! * [`list_app_catalog_manifests_latest(draft) -> [(id, manifest)]`](DshApiClient::list_app_catalog_manifests_latest)
use crate::dsh_api_client::DshApiClient;
use crate::types::AppCatalogManifest;
use crate::version::Version;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{epoch_milliseconds_to_string, DshApiResult};
use itertools::Itertools;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_str, Value};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

/// # Additional methods to manage the app catalog manifests
///
/// Module that contains methods and functions to query the App Catalog for all manifest files.
///
/// # Derived methods
/// * [`get_app_catalog_manifest(id, version) -> [manifest]`](DshApiClient::get_app_catalog_manifest)
/// * [`get_app_catalog_manifest_latest(id, draft) -> [manifest]`](DshApiClient::get_app_catalog_manifest_latest)
/// * [`get_app_catalog_manifests(id) -> [manifest]`](DshApiClient::get_app_catalog_manifests)
/// * [`get_manifests() -> [manifest]`](DshApiClient::get_manifests)
/// * [`get_raw_manifest(id, version) -> [(json, draft)]`](DshApiClient::get_raw_manifest)
/// * [`get_raw_manifest_latest(id, allow_draft) -> [(version, json, draft)]`](DshApiClient::get_raw_manifest_latest)
/// * [`list_app_catalog_manifest_ids() -> [id]`](DshApiClient::list_app_catalog_manifest_ids)
/// * [`list_app_catalog_manifest_versions() -> [id, [(version, draft)]]`](DshApiClient::list_app_catalog_manifest_versions)
/// * [`list_app_catalog_manifests() -> [id, [manifest]]`](DshApiClient::list_app_catalog_manifests)
/// * [`list_app_catalog_manifests_latest(draft) -> [(id, manifest)]`](DshApiClient::list_app_catalog_manifests_latest)
impl DshApiClient {
  /// # Return manifest by id and version
  ///
  /// Returns the manifest with the specified id and version.
  ///
  /// # Parameters
  /// * id - manifest id of the requested manifest
  /// * version - version of the requested manifest
  ///
  /// # Returns
  /// * `Ok<Manifest>` - manifest
  /// * `Err<`[`DshApiError::NotFound`]`>` - when the manifest could not be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_catalog_manifest(&self, id: &str, version: &Version) -> DshApiResult<Manifest> {
    self
      .get_manifests()
      .await?
      .iter()
      .find(|manifest| manifest.id == id && manifest.version == *version)
      .cloned()
      .ok_or(DshApiError::NotFound(None))
  }

  /// # Return latest version of a manifest by id
  ///
  /// Returns the latest version of the manifest with the specified id.
  ///
  /// # Parameters
  /// * id - manifest id of the requested manifest
  /// * allow_draft - whether the returned manifest can be a draft manifest or not
  ///
  /// # Returns
  /// * `Ok<Manifest>` - manifest
  /// * `Err<`[`DshApiError::NotFound`]`>` - when no manifest could be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_catalog_manifest_latest(&self, id: &str, allow_draft: bool) -> DshApiResult<Manifest> {
    match self.get_app_catalog_manifests(id).await {
      Ok(manifests) => match manifests.into_iter().filter(|manifest| !manifest.draft || allow_draft).next_back() {
        Some(latest_manifest) => Ok(latest_manifest),
        None => Err(DshApiError::NotFound(None)),
      },
      Err(_) => Err(DshApiError::NotFound(None)),
    }
  }

  /// # Return App Catalog manifests
  ///
  /// Returns a list of all versions of an app catalog manifest with the provided manifest_id.
  ///
  /// # Parameters
  /// * id - manifest id of the requested manifest
  ///
  /// # Returns
  /// * `Ok<Vec<manifest>>` - list of version/manifest pairs sorted by version
  /// * `Err<`[`DshApiError::NotFound`]`>` - when the manifest could not be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_catalog_manifests(&self, id: &str) -> DshApiResult<Vec<Manifest>> {
    let mut manifests: Vec<Manifest> = self.get_manifests().await?.into_iter().filter(|manifest| manifest.id == id).collect_vec();
    if manifests.is_empty() {
      Err(DshApiError::NotFound(None))
    } else {
      manifests.sort_by(|manifest_a, manifest_b| manifest_a.version.cmp(&manifest_b.version));
      Ok(manifests)
    }
  }

  /// # Return list of all Manifest objects
  ///
  /// # Returns
  /// * `Ok<Vec<Manifest>>` - vector containing Manifest objects in unspecified order
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_manifests(&self) -> DshApiResult<Vec<Manifest>> {
    self.get_appcatalog_manifests().await?.iter().map(Manifest::try_from).try_collect()
  }

  /// # Return raw manifest
  ///
  /// Returns the raw manifest as a json formatted string.
  ///
  /// # Parameters
  /// * id - manifest id of the requested manifest
  /// * version - version of the requested manifest
  ///
  /// # Returns
  /// * `Ok<(manifest, draft)>` - manifest as a json formatted string
  /// * `Err<`[`DshApiError::NotFound`]`>` - when the manifest could not be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_raw_manifest(&self, id: &str, version: &Version) -> DshApiResult<(String, bool)> {
    for app_catalog_manifest in self.get_appcatalog_manifests().await?.iter() {
      let payload = from_str::<HashMap<String, Value>>(app_catalog_manifest.payload.as_str())?;
      if payload.get("id").is_some_and(|payload_id| payload_id.as_str().unwrap() == id)
        && payload
          .get("version")
          .is_some_and(|version_value| Version::from_str(version_value.as_str().unwrap()).unwrap() == *version)
      {
        return Ok((serde_json::to_string_pretty(&payload)?, app_catalog_manifest.draft));
      }
    }
    Err(DshApiError::NotFound(None))
  }

  /// # Return raw manifest latest version
  ///
  /// Returns the latest version of raw manifest as a json formatted string.
  ///
  /// # Parameters
  /// * id - manifest id of the requested manifest
  /// * allow_draft - whether the returned raw manifest can be a draft manifest or not
  ///
  /// # Returns
  /// * `Ok<(version, manifest, draft)>` - version, draft and manifest as a json formatted string
  /// * `Err<`[`DshApiError::NotFound`]`>` - when the manifest could not be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_raw_manifest_latest(&self, id: &str, allow_draft: bool) -> DshApiResult<(Version, String, bool)> {
    let mut raw_manifests: Vec<(Version, bool, HashMap<String, Value>)> = self
      .get_appcatalog_manifests()
      .await?
      .iter()
      .filter(|manifest| !manifest.draft || allow_draft)
      .filter_map(|manifest| match from_str::<HashMap<String, Value>>(manifest.payload.as_str()) {
        Ok(payload) => {
          if payload.get("id").is_some_and(|payload_id| payload_id.as_str().unwrap() == id) {
            match payload.get("version").map(|version_value| Version::from_str(version_value.as_str().unwrap())) {
              Some(Ok(version)) => Some((version, manifest.draft, payload)),
              _ => None,
            }
          } else {
            None
          }
        }
        Err(_) => None,
      })
      .collect_vec();
    raw_manifests.sort_by(|(version_a, _, _), (version_b, _, _)| version_a.cmp(version_b));
    match raw_manifests.last() {
      Some((last_version, draft, last_payload)) => Ok((last_version.clone(), serde_json::to_string_pretty(&last_payload)?, *draft)),
      None => Err(DshApiError::NotFound(None)),
    }
  }

  /// # Return sorted list of all App Catalog manifest ids
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing all manifest ids sorted
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifest_ids(&self) -> DshApiResult<Vec<String>> {
    let mut ids: Vec<String> = self
      .get_manifests()
      .await?
      .iter()
      .map(|manifest| manifest.id.clone())
      .collect::<HashSet<_>>()
      .into_iter()
      .collect();
    ids.sort();
    Ok(ids)
  }

  /// # Return list of all available versions of all manifests
  ///
  /// # Returns
  /// * `Ok<Vec<(id, Vec<(version, draft)>)>>` - vector containing pairs of ids and versions,
  ///   sorted by id and version
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifest_versions(&self) -> DshApiResult<Vec<(String, Vec<(Version, bool)>)>> {
    let mut id_versions_map: HashMap<String, Vec<(Version, bool)>> = HashMap::new();
    for manifest in self.get_manifests().await? {
      id_versions_map
        .entry(manifest.id)
        .and_modify(|versions| versions.push((manifest.version.clone(), manifest.draft)))
        .or_insert(vec![(manifest.version, manifest.draft)]);
    }
    let mut id_versions_pairs: Vec<(String, Vec<(Version, bool)>)> = id_versions_map.iter().map(|(id, versions)| (id.to_string(), versions.clone())).collect();
    id_versions_pairs.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    for (_, versions) in id_versions_pairs.iter_mut() {
      versions.sort();
    }
    Ok(id_versions_pairs)
  }

  /// # Return list of all App Catalog manifests with available manifest versions
  ///
  /// # Returns
  /// * `Ok<Vec<(id, Vec<manifest>)>>` - vector containing pairs of ids and versions,
  ///   sorted by id and manifest version
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifests(&self) -> DshApiResult<Vec<(String, Vec<Manifest>)>> {
    let mut id_manifests: Vec<(String, Vec<Manifest>)> = self
      .get_manifests()
      .await?
      .into_iter()
      .map(|manifest| (manifest.id.clone(), manifest))
      .into_group_map()
      .into_iter()
      .collect_vec()
      .into_iter()
      .map(|(manifest_id, mut manifests)| {
        manifests.sort_by(|manifest_a, manifest_b| manifest_a.version.cmp(&manifest_b.version));
        (manifest_id, manifests)
      })
      .collect_vec();
    id_manifests.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    Ok(id_manifests)
  }

  /// # Return list of all latest versions of App Catalog manifests
  ///
  /// # Parameters
  /// * allow_draft - whether the returned latest manifests can be a draft manifests or not
  ///
  /// # Returns
  /// * `Ok<Vec<(id, manifest)>>` - vector containing tuples of ids, versions and
  ///   manifests, sorted by id
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_catalog_manifests_latest(&self, allow_draft: bool) -> DshApiResult<Vec<(String, Manifest)>> {
    let manifests_grouped_by_id: Vec<(String, Vec<Manifest>)> = self
      .get_manifests()
      .await?
      .into_iter()
      .map(|manifest| (manifest.id.clone(), manifest))
      .collect_vec()
      .into_iter()
      .into_group_map()
      .into_iter()
      .collect_vec();
    let mut latest_manifests: Vec<(String, Manifest)> = manifests_grouped_by_id
      .into_iter()
      .filter_map(|(id, manifests)| {
        manifests
          .into_iter()
          .filter(|manifest| !manifest.draft || allow_draft)
          .max_by_key(|manifest| manifest.version.clone())
          .map(|manifest| (id, manifest))
      })
      .collect_vec();
    latest_manifests.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    Ok(latest_manifests)
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Manifest {
  #[serde(skip_deserializing)]
  pub draft: bool,
  #[serde(skip_deserializing)]
  pub last_modified: String,
  pub id: String,
  pub name: String,
  pub version: Version,
  pub vendor: String,
  pub kind: Option<String>,
  #[serde(rename = "apiVersion")]
  pub api_version: Option<String>,
  pub description: Option<String>,
  #[serde(rename = "moreInfo")]
  pub more_info: Option<String>,
  pub contact: String,
  pub configuration: Option<Configuration>,
  #[serde(deserialize_with = "deserialize_resource_map")]
  pub resources: HashMap<String, Resource>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub r#type: String,
  pub properties: HashMap<String, Property>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Property {
  pub description: String,
  pub r#type: String,
  pub r#enum: Option<Vec<String>>,
  pub default: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Resource {
  Application(Box<ApplicationResource>),
  Bucket(Box<BucketResource>),
  Certificate(String), // TODO Replace string by a CertificateResource struct
  Database(Box<DatabaseResource>),
  Secret(String), // TODO Replace string by a SecretResource struct
  Topic(Box<TopicResource>),
  Vhost(String), // TODO Replace string by a VhostResource struct
  Volume(Box<VolumeResource>),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "Value", into = "Value")]
pub enum Numerical {
  Float(f64),
  Integer(i64),
  Template(String),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ApplicationResource {
  pub cpus: Numerical,
  pub env: HashMap<String, String>,
  #[serde(rename = "exposedPorts")]
  pub exposed_ports: Option<HashMap<String, ExposedPort>>,
  pub image: String,
  #[serde(rename = "imageConsole")]
  pub image_console: Option<String>,
  pub instances: Numerical,
  pub mem: Numerical,
  pub metrics: Option<Metrics>,
  pub name: String,
  #[serde(rename = "needsToken")]
  pub needs_token: bool,
  pub secrets: Option<Vec<Secret>>,
  #[serde(rename = "singleInstance")]
  pub single_instance: bool,
  pub user: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BucketResource {
  pub encrypted: bool,
  pub name: String,
  pub versioned: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DatabaseResource {
  pub cpus: Numerical,
  pub extensions: Vec<String>,
  pub instances: Numerical,
  pub mem: Numerical,
  pub name: String,
  #[serde(rename = "snapshotInterval")]
  pub snapshot_interval: Numerical,
  pub version: String,
  #[serde(rename = "volumeSize")]
  pub volume_size: Numerical,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicResource {
  #[serde(rename = "kafkaProperties")]
  pub kafka_properties: Option<HashMap<String, String>>,
  pub name: String,
  pub partitions: i64,
  #[serde(rename = "replicationFactor")]
  pub replication_factor: i64,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VolumeResource {
  pub name: String,
  pub size: Numerical,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExposedPort {
  pub auth: Option<String>,
  pub tls: Option<String>,
  pub vhost: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Injection {
  pub env: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Secret {
  pub injections: Vec<Injection>,
  pub name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Metrics {
  pub path: String,
  pub port: i64,
}

impl TryFrom<&AppCatalogManifest> for Manifest {
  type Error = DshApiError;

  fn try_from(app_catalog_manifest: &AppCatalogManifest) -> Result<Self, Self::Error> {
    from_str::<Manifest>(app_catalog_manifest.payload.as_str())
      .map(|payload| Manifest { draft: app_catalog_manifest.draft, last_modified: epoch_milliseconds_to_string(app_catalog_manifest.last_modified as i64), ..payload })
      .map_err(DshApiError::from)
  }
}

fn deserialize_resource_map<'de, D>(deserializer: D) -> Result<HashMap<String, Resource>, D::Error>
where
  D: Deserializer<'de>,
{
  HashMap::<String, Value>::deserialize(deserializer).and_then(|deserialized_map| {
    deserialized_map
      .iter()
      .map(|(key, value)| {
        let key_parts = key.split("/").collect_vec();
        match key_parts.get(2) {
          Some(resource_type) => match *resource_type {
            "application" => Resource::application(value),
            "bucket" => Resource::bucket(value),
            "certificate" => Resource::certificate(value),
            "database" => Resource::database(value),
            "secret" => Resource::secret(value),
            "topic" => Resource::topic(value),
            "vhost" => Resource::vhost(value),
            "volume" => Resource::volume(value),
            unknown_resource => Err(serde_json::Error::custom(format!("unknown resource type ({})", unknown_resource))),
          }
          .map(|resource| (key.to_string(), resource)),
          None => Err(serde_json::Error::custom(format!("illegal resource allocation ({})", key))),
        }
      })
      .try_collect()
      .map_err(D::Error::custom)
  })
}

impl Resource {
  fn application(value: &Value) -> Result<Resource, serde_json::Error> {
    ApplicationResource::deserialize(value).map(|application_resource| Resource::Application(Box::new(application_resource)))
  }

  fn bucket(value: &Value) -> Result<Resource, serde_json::Error> {
    BucketResource::deserialize(value).map(|bucket_resource| Resource::Bucket(Box::new(bucket_resource)))
  }

  fn certificate(value: &Value) -> Result<Resource, serde_json::Error> {
    Ok(Resource::Certificate(value.to_string()))
  }

  fn database(value: &Value) -> Result<Resource, serde_json::Error> {
    DatabaseResource::deserialize(value).map(|database_resource| Resource::Database(Box::new(database_resource)))
  }

  fn secret(value: &Value) -> Result<Resource, serde_json::Error> {
    Ok(Resource::Secret(value.to_string()))
  }

  fn topic(value: &Value) -> Result<Resource, serde_json::Error> {
    TopicResource::deserialize(value).map(|topic_resource| Resource::Topic(Box::new(topic_resource)))
  }

  fn vhost(value: &Value) -> Result<Resource, serde_json::Error> {
    Ok(Resource::Vhost(value.to_string()))
  }

  fn volume(value: &Value) -> Result<Resource, serde_json::Error> {
    VolumeResource::deserialize(value).map(|volume_resource| Resource::Volume(Box::new(volume_resource)))
  }
}

impl TryFrom<Value> for Numerical {
  type Error = String;

  fn try_from(value: Value) -> Result<Self, Self::Error> {
    match value.as_i64() {
      Some(integer) => Ok(Numerical::Integer(integer)),
      None => match value.as_f64() {
        Some(float) => Ok(Numerical::Float(float)),
        None => match value.as_str() {
          Some(string) => Ok(Numerical::Template(string.to_string())),
          None => Err(format!("could not parse '{}' value", value)),
        },
      },
    }
  }
}

impl From<Numerical> for Value {
  fn from(numerical: Numerical) -> Self {
    match numerical {
      Numerical::Float(float) => Value::from(float),
      Numerical::Integer(integer) => Value::from(integer),
      Numerical::Template(string) => Value::from(string),
    }
  }
}
