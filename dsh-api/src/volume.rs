//! # Additional methods to manage volumes
//!
//! Module that contains methods and functions to manage volumes.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_volume_configuration(id)`](DshApiClient::delete_volume_configuration)
//! * [`get_volume(id) -> VolumeStatus`](DshApiClient::get_volume)
//! * [`get_volume_actual(id) -> Volume`](DshApiClient::get_volume_actual)
//! * [`get_volume_configuration(id) -> Volume`](DshApiClient::get_volume_configuration)
//! * [`get_volume_ids() -> [id]`](DshApiClient::get_volume_ids)
//! * [`get_volume_status(id) -> AllocationStatus`](DshApiClient::get_volume_status)
//! * [`put_volume_configuration(id, body)`](DshApiClient::put_volume_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`volume_with_dependants(volume id) -> [volume status, [(id, [injection])]]`](DshApiClient::volume_with_dependants)
//! * [`volumes_with_dependant_applications() -> [volume id, [(application id, instances, [injection])]]`](DshApiClient::volumes_with_dependant_applications)
//! * [`volumes_with_dependant_apps() -> [volume id, [(app id, [resource])]]`](DshApiClient::volumes_with_dependant_apps)
//! * [`volumes_with_dependants() -> [volume id, [(id, [injection|resource])]]`](DshApiClient::volumes_with_dependants)

use crate::app::{app_resources, apps_that_use_volume};
use crate::application_types::ApplicationValues;
use crate::dsh_api_client::DshApiClient;
use crate::parse::parse_volume_string;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application, Volume, VolumeStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{Dependant, DependantApp, DependantApplication, DshApiResult};
use futures::try_join;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// # Describes an injection of a resource in an application
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum VolumeInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
  /// Path injection, where the value is the name of a directory in the container.
  #[serde(rename = "path")]
  Path(String),
  /// Variable function, where the values are the name of the function and the parameter.
  #[serde(rename = "variable")]
  Variable(String, String),
  /// Volume injection, where the value is the mount path
  #[serde(rename = "volume")]
  Volume(String),
}

impl Display for VolumeInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      VolumeInjection::EnvVar(env_var) => write!(f, "{}", env_var),
      VolumeInjection::Path(path) => write!(f, "path:{}", path),
      VolumeInjection::Variable(name, parameter) => write!(f, "{{ {}('{}') }}", name, parameter),
      VolumeInjection::Volume(mount_path) => write!(f, "mount:{}", mount_path),
    }
  }
}

/// # Additional methods to manage volumes
///
/// Module that contains methods to manage volumes.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`volume_with_dependants(volume id) -> [volume status, [(id, [injection])]]`](DshApiClient::volume_with_dependants)
/// * [`volumes_with_dependant_applications() -> [volume id, [(application id, instances, [injection])]]`](DshApiClient::volumes_with_dependant_applications)
/// * [`volumes_with_dependant_apps() -> [volume id, [(app id, [resource])]]`](DshApiClient::volumes_with_dependant_apps)
/// * [`volumes_with_dependants() -> [volume id, [(id, [injection|resource])]]`](DshApiClient::volumes_with_dependants)
impl DshApiClient {
  /// # Get volume with usage
  ///
  /// Returns configuration and usage for a given volume.
  ///
  /// # Parameters
  /// * `volume_id` - name of the requested volume
  ///
  /// # Returns
  /// * `Ok<(VolumeStatus, Vec<UsedBy>)>` - volume status and usage.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn volume_with_dependants(&self, volume_id: &str) -> DshApiResult<(VolumeStatus, Vec<Dependant<VolumeInjection>>)> {
    let (volume_status, applications, apps) = try_join!(
      self.get_volume(volume_id),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut dependants: Vec<Dependant<VolumeInjection>> = vec![];
    for ApplicationValues { id, application, values } in volume_paths_from_applications(volume_id, &applications) {
      dependants.push(Dependant::application(
        id.to_string(),
        application.instances,
        values.iter().map(|path| VolumeInjection::Volume(path.to_string())).collect_vec(),
      ));
    }
    for (app_id, _, resource_ids) in apps_that_use_volume(volume_id, &apps) {
      dependants.push(Dependant::app(
        app_id.to_string(),
        resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
      ));
    }
    Ok((volume_status, dependants))
  }

  /// # Returns all volumes with dependant applications
  ///
  /// Returns a sorted list of all volumes together with the applications that use them.
  pub async fn volumes_with_dependant_applications(&self) -> DshApiResult<Vec<(String, Vec<DependantApplication<VolumeInjection>>)>> {
    let (volume_ids, applications) = try_join!(self.get_volume_ids(), self.get_application_configuration_map())?;
    let mut volumes = Vec::<(String, Vec<DependantApplication<VolumeInjection>>)>::new();
    for volume_id in volume_ids {
      let mut dependant_applications: Vec<DependantApplication<VolumeInjection>> = vec![];
      for application in volume_paths_from_applications(volume_id.as_str(), &applications) {
        dependant_applications.push(DependantApplication::new(
          application.id.to_string(),
          application.application.instances,
          application.values.iter().map(|env_var| VolumeInjection::EnvVar(env_var.to_string())).collect_vec(),
        ));
      }
      volumes.push((volume_id, dependant_applications));
    }
    Ok(volumes)
  }

  /// # Returns all volumes with dependant apps
  ///
  /// Returns a sorted list of all volumes together with the apps that use them.
  pub async fn volumes_with_dependant_apps(&self) -> DshApiResult<Vec<(String, Vec<DependantApp>)>> {
    let (volume_ids, apps) = try_join!(self.get_volume_ids(), self.get_appcatalogapp_configuration_map())?;
    let mut volumes = Vec::<(String, Vec<DependantApp>)>::new();
    for volume_id in volume_ids {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      for (app_id, _, resource_ids) in apps_that_use_volume(volume_id.as_str(), &apps) {
        dependant_apps.push(DependantApp::new(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      volumes.push((volume_id, dependant_apps));
    }
    Ok(volumes)
  }

  /// # Returns all volumes with dependant applications and apps
  ///
  /// Returns a sorted list of all volumes together with the applications and apps that use them.
  pub async fn volumes_with_dependants(&self) -> DshApiResult<Vec<(String, Vec<Dependant<VolumeInjection>>)>> {
    let (volume_ids, applications, apps) = try_join!(
      self.get_volume_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut volumes = Vec::<(String, Vec<Dependant<VolumeInjection>>)>::new();
    for volume_id in volume_ids {
      let mut dependants: Vec<Dependant<VolumeInjection>> = vec![];
      for application in volume_paths_from_applications(volume_id.as_str(), &applications) {
        dependants.push(Dependant::application(
          application.id.to_string(),
          application.application.instances,
          application.values.iter().map(|env_var| VolumeInjection::EnvVar(env_var.to_string())).collect_vec(),
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_volume(volume_id.as_str(), &apps) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      volumes.push((volume_id, dependants));
    }
    Ok(volumes)
  }
}

pub fn volume_paths_from_applications<'a>(volume_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, &'a str>> {
  let mut application_ids = applications.keys().collect_vec();
  application_ids.sort();
  let mut tuples: Vec<ApplicationValues<&str>> = vec![];
  for application_id in application_ids {
    let application = applications.get(application_id).unwrap();
    let mut injections = Vec::<&str>::new();
    for (path, application_volume) in &application.volumes {
      if application_volume.name.contains(volume_id) {
        injections.push(path);
      }
    }
    if !injections.is_empty() {
      tuples.push(ApplicationValues::new(application_id, application, injections));
    }
  }
  tuples
}

/// # Get volumes from an application
///
/// Get all volumes with their mounting path from an `Application`.
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<(&str, &str)>` - list of tuples containing:
/// * volume id
/// * path where the volume will be mounted
///
/// The list is sorted by volume id.
pub fn volumes_from_application(application: &Application) -> Vec<(&str, &str)> {
  let mut volumes = application
    .volumes
    .iter()
    .filter_map(|(path, application_volumes)| {
      parse_volume_string(application_volumes.name.as_str())
        .map(|volume_id| (volume_id, path.as_str()))
        .ok()
    })
    .collect_vec();
  volumes.sort_by(|(volume_a, _), (volume_b, _)| volume_a.cmp(volume_b));
  volumes
}

/// # Get volumes from applications
///
/// Get all volumes with their mounting paths from all `Applications`.
///
/// # Parameters
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationValues<(&str, Path)>>` - list of tuples containing:
/// * application id
/// * application reference
/// * sorted list of pairs of volume ids and mounting paths
///
/// The list is sorted by application id.
pub fn volumes_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<(&str, &str)>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let volume_injections: Vec<(&str, &str)> = volumes_from_application(application);
      if !volume_injections.is_empty() {
        Some(ApplicationValues::new(application_id, application, volume_injections))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// Get volume resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - Reference to the app to get the volume resources from.
///
/// # Returns
/// List of tuples describing the volume resources. Each tuple contains:
/// * volume resource id,
/// * reference to the `Volume`,
pub fn volume_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Volume)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Volume(volume) => Some(volume),
    _ => None,
  })
}
