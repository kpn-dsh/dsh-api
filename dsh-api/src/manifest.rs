//! # Additional methods to manage manifest
//!
//! Module that contains methods and functions to manage manifests.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`get_appcatalog_manifests() -> [AppCatalogManifest]`](DshApiClient::get_appcatalog_manifests)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`manifest(manifest id, version) -> [manifest]`](DshApiClient::manifest)
//! * [`manifest_latest_version(manifest id, draft) -> [manifest]`](DshApiClient::manifest_latest_version)
//! * [`manifest_all_versions(manifest id) -> [manifest]`](DshApiClient::manifest_all_versions)
//! * [`manifests() -> [manifest]`](DshApiClient::manifests)
//! * [`manifest_raw(manifest id, version) -> (raw, draft)`](DshApiClient::manifest_raw)
//! * [`manifest_raw_latest(manifest id, draft) -> (version, raw, draft)`](DshApiClient::manifest_raw_latest)
//! * [`manifest_ids() -> [manifest id]`](DshApiClient::manifest_ids)
//! * [`manifest_ids_versions() -> [(manifest id, [(version, draft)])]`](DshApiClient::manifest_ids_versions)
//! * [`manifest_all_versions() -> [(manifest id, [manifest])]`](DshApiClient::manifest_all_versions)
//! * [`manifests_latest_version(draft) -> [(manifest id, manifest)]`](DshApiClient::manifests_latest_version)
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
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

/// # Additional methods to manage the app catalog manifests
///
/// Module that contains methods and functions to query the App Catalog for all manifest files.
///
/// # Derived methods
/// * [`manifest(manifest id, version) -> [manifest]`](DshApiClient::manifest)
/// * [`manifest_latest_version(manifest id, draft) -> [manifest]`](DshApiClient::manifest_latest_version)
/// * [`manifest_all_versions(manifest id) -> [manifest]`](DshApiClient::manifest_all_versions)
/// * [`manifests() -> [manifest]`](DshApiClient::manifests)
/// * [`manifest_raw(manifest id, version) -> (raw, draft)`](DshApiClient::manifest_raw)
/// * [`manifest_raw_latest(manifest id, draft) -> (version, raw, draft)`](DshApiClient::manifest_raw_latest)
/// * [`manifest_ids() -> [manifest id]`](DshApiClient::manifest_ids)
/// * [`manifest_ids_versions() -> [(manifest id, [(version, draft)])]`](DshApiClient::manifest_ids_versions)
/// * [`manifest_all_versions() -> [(manifest id, [manifest])]`](DshApiClient::manifest_all_versions)
/// * [`manifests_latest_version(draft) -> [(manifest id, manifest)]`](DshApiClient::manifests_latest_version)
impl DshApiClient {
  /// # Return manifest by id and version
  ///
  /// Returns the manifest with the specified id and version.
  ///
  /// # Parameters
  /// * `manifest_id` - Identifier of the requested manifest.
  /// * `manifest_version` - Version of the requested manifest.
  ///
  /// # Returns
  /// * `Ok<Manifest>` - manifest
  /// * `Err<`[`DshApiError::NotFound`]`>` - when the manifest could not be found
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn manifest(&self, manifest_id: &str, manifest_version: &Version) -> DshApiResult<Manifest> {
    self
      .manifests()
      .await?
      .iter()
      .find(|manifest| manifest.id == manifest_id && manifest.version == *manifest_version)
      .cloned()
      .ok_or(DshApiError::NotFound(None))
  }

  /// # Return latest version of manifest by id
  ///
  /// Returns the latest version of the manifest with the specified id.
  ///
  /// # Parameters
  /// * `manifest_id` - Identifier of the requested manifest.
  /// * `allow_draft_version` - Whether the returned manifest can be a draft manifest or not.
  ///
  /// # Returns
  /// * `Ok<Manifest>` - Manifest.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When no manifest could be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_latest_version(&self, manifest_id: &str, allow_draft_version: bool) -> DshApiResult<Manifest> {
    match self.manifest_all_versions(manifest_id).await {
      Ok(manifests) => match manifests.into_iter().filter(|manifest| !manifest.draft || allow_draft_version).next_back() {
        Some(latest_manifest) => Ok(latest_manifest),
        None => Err(DshApiError::NotFound(None)),
      },
      Err(_) => Err(DshApiError::NotFound(None)),
    }
  }

  /// # Return all versions of manifest by id
  ///
  /// Returns a list of all versions of an app catalog manifest with the provided manifest_id.
  ///
  /// # Parameters
  /// * `manifest_id` - Identifier of the requested manifest.
  ///
  /// # Returns
  /// * `Ok<Vec<manifest>>` - List of version/manifest pairs sorted by version.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When the manifest could not be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_all_versions(&self, manifest_id: &str) -> DshApiResult<Vec<Manifest>> {
    let mut manifests: Vec<Manifest> = self.manifests().await?.into_iter().filter(|manifest| manifest.id == manifest_id).collect_vec();
    if manifests.is_empty() {
      Err(DshApiError::NotFound(None))
    } else {
      manifests.sort_by(|manifest_a, manifest_b| manifest_a.version.cmp(&manifest_b.version));
      Ok(manifests)
    }
  }

  /// # Return all manifests
  ///
  /// # Returns
  /// * `Ok<Vec<Manifest>>` - Vector containing Manifest objects in unspecified order.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifests(&self) -> DshApiResult<Vec<Manifest>> {
    self.get_appcatalog_manifests().await?.iter().map(Manifest::try_from).try_collect()
  }

  /// # Return raw manifest by id and version
  ///
  /// Returns the raw manifest as a json formatted string.
  ///
  /// # Parameters
  /// * `manifest_id` - Identifier of the requested manifest.
  /// * `manifest_version` - Version of the requested manifest.
  ///
  /// # Returns
  /// * `Ok<(manifest, draft)>` - Manifest as a json formatted string.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When the manifest could not be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_raw(&self, manifest_id: &str, manifest_version: &Version) -> DshApiResult<(String, bool)> {
    for app_catalog_manifest in self.get_appcatalog_manifests().await?.iter() {
      let payload = from_str::<HashMap<String, Value>>(app_catalog_manifest.payload.as_str())?;
      if payload.get("id").is_some_and(|payload_id| payload_id.as_str().unwrap() == manifest_id)
        && payload
          .get("version")
          .is_some_and(|version_value| Version::from_str(version_value.as_str().unwrap()).unwrap() == *manifest_version)
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
  /// * `manifest_id` - Identifier of the requested manifest.
  /// * `allow_draft_version` - Whether the returned raw manifest can be a draft manifest or not.
  ///
  /// # Returns
  /// * `Ok<(version, manifest, draft)>` - Tuple consisting of version, manifest as a json
  ///   formatted string and whether the manifest is a draft.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When the manifest could not be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_raw_latest(&self, manifest_id: &str, allow_draft_version: bool) -> DshApiResult<(Version, String, bool)> {
    let mut raw_manifests: Vec<(Version, bool, HashMap<String, Value>)> = self
      .get_appcatalog_manifests()
      .await?
      .iter()
      .filter(|manifest| !manifest.draft || allow_draft_version)
      .filter_map(|manifest| match from_str::<HashMap<String, Value>>(manifest.payload.as_str()) {
        Ok(payload) => {
          if payload.get("id").is_some_and(|payload_id| payload_id.as_str().unwrap() == manifest_id) {
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
  /// * `Ok<Vec<String>>` - Vector containing all manifest ids sorted.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_ids(&self) -> DshApiResult<Vec<String>> {
    let mut ids: Vec<String> = self
      .manifests()
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
  /// * `Ok<Vec<(id, Vec<(version, draft)>)>>` - Vector containing pairs of manifest ids and
  ///   lists of versions, sorted by id and version.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn manifest_ids_versions(&self) -> DshApiResult<Vec<(String, Vec<(Version, bool)>)>> {
    let mut id_versions_map: HashMap<String, Vec<(Version, bool)>> = HashMap::new();
    for manifest in self.manifests().await? {
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

  /// # Return list of all manifests with all available manifest versions
  ///
  /// # Returns
  /// * `Ok<Vec<(id, Vec<manifest>)>>` - vector containing pairs of ids and versions,
  ///   sorted by id and manifest version
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn manifests_all_versions(&self) -> DshApiResult<Vec<(String, Vec<Manifest>)>> {
    let mut id_manifests: Vec<(String, Vec<Manifest>)> = self
      .manifests()
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
  pub async fn manifests_latest_version(&self, allow_draft: bool) -> DshApiResult<Vec<(String, Manifest)>> {
    let manifests_grouped_by_id: Vec<(String, Vec<Manifest>)> = self
      .manifests()
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

/// Describes a manifest from the app catalog
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

impl Display for Manifest {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.id, self.version)
  }
}

impl TryFrom<&AppCatalogManifest> for Manifest {
  type Error = DshApiError;

  fn try_from(app_catalog_manifest: &AppCatalogManifest) -> Result<Self, Self::Error> {
    from_str::<Manifest>(app_catalog_manifest.payload.as_str())
      .map(|payload| Manifest { draft: app_catalog_manifest.draft, last_modified: epoch_milliseconds_to_string(app_catalog_manifest.last_modified as i64), ..payload })
      .map_err(DshApiError::from)
  }
}

/// Configuration of a manifest
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
  #[serde(rename = "$schema")]
  pub schema: String,
  #[serde(rename = "type")]
  pub kind: String,
  pub properties: HashMap<String, Property>,
}

/// Property of a manifest configuration
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Property {
  pub description: String,
  #[serde(rename = "type")]
  pub kind: PropertyKind,
  #[serde(rename = "enum")]
  pub enumeration: Option<Vec<String>>,
  pub default: Option<String>,
}

impl Display for Property {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    fn display_enumeration(enumeration: &[String], default: Option<&String>) -> String {
      enumeration
        .iter()
        .map(
          |enumeration_value| {
            if default.is_some_and(|default_value| default_value == enumeration_value) {
              format!("{}*", enumeration_value)
            } else {
              enumeration_value.to_string()
            }
          },
        )
        .join("|")
    }

    match self.kind {
      PropertyKind::DnsZone => write!(f, "dns-zone"),
      PropertyKind::Number => {
        if let Some(enumeration) = &self.enumeration {
          write!(f, "number:{}", display_enumeration(enumeration, Option::from(&self.default)))
        } else if let Some(default_value) = &self.default {
          write!(f, "number:default={}", default_value)
        } else {
          write!(f, "number")
        }
      }
      PropertyKind::String => {
        if let Some(enumeration) = &self.enumeration {
          write!(f, "string:{}", display_enumeration(enumeration, Option::from(&self.default)))
        } else if let Some(default_value) = &self.default {
          write!(f, "string:default=\"{}\"", default_value)
        } else {
          write!(f, "string")
        }
      }
    }
  }
}

/// Indicates the manifest configuration property type
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum PropertyKind {
  #[serde(rename = "dns-zone")]
  DnsZone,
  #[serde(rename = "number")]
  Number,
  #[serde(rename = "string")]
  String,
}

impl Display for PropertyKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      PropertyKind::DnsZone => write!(f, "dns-zone"),
      PropertyKind::Number => write!(f, "number"),
      PropertyKind::String => write!(f, "string"),
    }
  }
}

/// Resource that a manifest depends upon
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum Resource {
  Application(Box<ApplicationResource>),
  Bucket(Box<BucketResource>),
  Certificate(Box<CertificateResource>),
  Database(Box<DatabaseResource>),
  Secret(Box<SecretResource>),
  Topic(Box<TopicResource>),
  Vhost(Box<VhostResource>),
  Volume(Box<VolumeResource>),
}

impl Display for Resource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Application(application) => Display::fmt(&application, f),
      Self::Bucket(bucket) => Display::fmt(&bucket, f),
      Self::Certificate(certificate) => Display::fmt(&certificate, f),
      Self::Database(database) => Display::fmt(&database, f),
      Self::Secret(secret) => Display::fmt(&secret, f),
      Self::Topic(topic) => Display::fmt(&topic, f),
      Self::Vhost(vhost) => Display::fmt(&vhost, f),
      Self::Volume(volume) => Display::fmt(&volume, f),
    }
  }
}

/// Manifest configuration property data type
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(try_from = "Value", into = "Value")]
pub enum Numerical {
  Float(f64),
  Integer(i64),
  Template(String),
}

impl Display for Numerical {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Float(float) => write!(f, "{}", float),
      Self::Integer(integer) => write!(f, "{}", integer),
      Self::Template(template) => write!(f, "{}", template),
    }
  }
}

/// Manifest application dependency
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

impl Display for ApplicationResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.name, self.image)
  }
}

/// Manifest bucket dependency
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct BucketResource {
  pub encrypted: bool,
  pub name: String,
  pub versioned: bool,
}

impl Display for BucketResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)?;
    if self.encrypted {
      write!(f, ":encrypted")?;
    }
    if self.versioned {
      write!(f, ":versioned")?;
    }
    Ok(())
  }
}

// TODO Proper implementation when format is known
/// Manifest certificate dependency (plain string since this dependency is undocumented)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CertificateResource {
  pub unformatted_representation: String,
}

impl Display for CertificateResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.unformatted_representation)
  }
}

/// Manifest database dependency
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

impl Display for DatabaseResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.name, self.version)
  }
}

// TODO Proper implementation when format is known
/// Manifest secret dependency (plain string since this dependency is undocumented)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SecretResource {
  pub unformatted_representation: String,
}

impl Display for SecretResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.unformatted_representation)
  }
}

/// Manifest topic dependency
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct TopicResource {
  #[serde(rename = "kafkaProperties")]
  pub kafka_properties: Option<HashMap<String, String>>,
  pub name: String,
  pub partitions: i64,
  #[serde(rename = "replicationFactor")]
  pub replication_factor: i64,
}

impl Display for TopicResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}:{}", self.name, self.partitions, self.replication_factor)
  }
}

// TODO Proper implementation when format is known
/// Manifest vhost dependency (plain string since this dependency is undocumented)
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VhostResource {
  pub unformatted_representation: String,
}

impl Display for VhostResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.unformatted_representation)
  }
}

/// Manifest volume dependency
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VolumeResource {
  pub name: String,
  pub size: Numerical,
}

impl Display for VolumeResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.name, self.size)
  }
}

/// Used in application resource
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ExposedPort {
  pub auth: Option<String>,
  pub tls: Option<String>,
  pub vhost: String,
}

impl Display for ExposedPort {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.vhost)?;
    if let Some(auth) = &self.auth {
      write!(f, ":{}", auth)?;
    }
    if let Some(tls) = &self.tls {
      write!(f, ":{}", tls)?;
    }
    Ok(())
  }
}

/// Used in application resource
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Injection {
  pub env: String,
}

impl Display for Injection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.env)
  }
}

/// Used in application resource
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Secret {
  pub injections: Vec<Injection>,
  pub name: String,
}

impl Display for Secret {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)?;
    if self.injections.is_empty() {
      write!(f, ":{}", self.injections.iter().map(|injection| injection.to_string()).join(","))?;
    }
    Ok(())
  }
}

/// Used in application resource
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Metrics {
  pub path: String,
  pub port: i64,
}

impl Display for Metrics {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.path, self.port)
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
    Ok(Resource::Certificate(Box::new(CertificateResource {
      unformatted_representation: value.to_string(),
    })))
  }

  fn database(value: &Value) -> Result<Resource, serde_json::Error> {
    DatabaseResource::deserialize(value).map(|database_resource| Resource::Database(Box::new(database_resource)))
  }

  fn secret(value: &Value) -> Result<Resource, serde_json::Error> {
    Ok(Resource::Secret(Box::new(SecretResource { unformatted_representation: value.to_string() })))
  }

  fn topic(value: &Value) -> Result<Resource, serde_json::Error> {
    TopicResource::deserialize(value).map(|topic_resource| Resource::Topic(Box::new(topic_resource)))
  }

  fn vhost(value: &Value) -> Result<Resource, serde_json::Error> {
    Ok(Resource::Vhost(Box::new(VhostResource { unformatted_representation: value.to_string() })))
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
