//! # Additional methods to manage secrets
//!
//! Module that contains methods and functions to manage secrets.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_secret_configuration(id)`](DshApiClient::delete_secret_configuration)
//! * [`get_secret(id) -> String`](DshApiClient::get_secret)
//! * [`get_secret_actual(id) -> Empty`](DshApiClient::get_secret_actual)
//! * [`get_secret_configuration(id) -> Empty`](DshApiClient::get_secret_configuration)
//! * [`get_secret_ids() -> [id]`](DshApiClient::get_secret_ids)
//! * [`get_secret_status(id) -> AllocationStatus`](DshApiClient::get_secret_status)
//! * [`post_secret(body)`](DshApiClient::post_secret)
//! * [`put_secret(id, body)`](DshApiClient::put_secret)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`secrets_with_dependants() -> [id, [application|app]]`](DshApiClient::secrets_with_dependants)
//! * [`secrets_with_dependant_applications() -> [id, [application]]`](DshApiClient::secrets_with_dependant_applications)
//! * [`secrets_with_dependant_apps() -> [id, [app]]`](DshApiClient::secrets_with_dependant_apps)

use crate::app::{app_resources, apps_that_use_secret};
use crate::application_types::{ApplicationValues, EnvVarInjection};
use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::types::{AllocationStatus, Empty, Secret};
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application};
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
pub enum SecretInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
}

impl Display for SecretInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SecretInjection::EnvVar(env_var) => write!(f, "{}", env_var),
    }
  }
}

/// # Additional methods and functions to manage secrets
///
/// Module that contains methods and functions to manage secrets.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`secrets_with_dependants() -> [id, [application|app]]`](DshApiClient::secrets_with_dependants)
/// * [`secrets_with_dependant_applications() -> [id, [application]]`](DshApiClient::secrets_with_dependant_applications)
/// * [`secrets_with_dependant_apps() -> [id, [app]]`](DshApiClient::secrets_with_dependant_apps)
impl DshApiClient {
  /// # Returns all secrets with dependant applications and apps
  ///
  /// Returns a sorted list of all secrets together with the applications and apps that use them.
  pub async fn secrets_with_dependants(&self) -> DshApiResult<Vec<(String, Vec<Dependant<SecretInjection>>)>> {
    let (secret_ids, applications, apps) = try_join!(
      self.get_secret_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut secrets = Vec::<(String, Vec<Dependant<SecretInjection>>)>::new();
    for secret_id in secret_ids {
      let mut dependants: Vec<Dependant<SecretInjection>> = vec![];
      for application in secret_env_vars_from_applications(secret_id.as_str(), &applications) {
        dependants.push(Dependant::application(
          application.id.to_string(),
          application.application.instances,
          application.values.iter().map(|env_var| SecretInjection::EnvVar(env_var.to_string())).collect_vec(),
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_secret(secret_id.as_str(), &apps) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      secrets.push((secret_id, dependants));
    }
    Ok(secrets)
  }

  /// # Returns all secrets with dependant applications
  ///
  /// Returns a sorted list of all secrets together with the applications that use them.
  pub async fn secrets_with_dependant_applications(&self) -> DshApiResult<Vec<(String, Vec<DependantApplication<SecretInjection>>)>> {
    let (secret_ids, applications) = try_join!(self.get_secret_ids(), self.get_application_configuration_map())?;
    let mut secrets = Vec::<(String, Vec<DependantApplication<SecretInjection>>)>::new();
    for secret_id in secret_ids {
      let mut dependant_applications: Vec<DependantApplication<SecretInjection>> = vec![];
      for application in secret_env_vars_from_applications(secret_id.as_str(), &applications) {
        dependant_applications.push(DependantApplication::new(
          application.id.to_string(),
          application.application.instances,
          application.values.iter().map(|env_var| SecretInjection::EnvVar(env_var.to_string())).collect_vec(),
        ));
      }
      secrets.push((secret_id, dependant_applications));
    }
    Ok(secrets)
  }

  /// # Returns all secrets with dependant apps
  ///
  /// Returns a sorted list of all secrets together with the apps that use them.
  pub async fn secrets_with_dependant_apps(&self) -> DshApiResult<Vec<(String, Vec<DependantApp>)>> {
    let (secret_ids, apps) = try_join!(self.get_secret_ids(), self.get_appcatalogapp_configuration_map())?;
    let mut secrets = Vec::<(String, Vec<DependantApp>)>::new();
    for secret_id in secret_ids {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      for (app_id, _, resource_ids) in apps_that_use_secret(secret_id.as_str(), &apps) {
        dependant_apps.push(DependantApp::new(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      secrets.push((secret_id, dependant_apps));
    }
    Ok(secrets)
  }
}

/// # Get application environment variables referencing secret
///
/// Get all environment variables from `application` referencing secret with `secret_id`.
///
/// # Parameters
/// * `secret_id` - id of the secret to look for
/// * `application` - reference to the `Application`
///
/// # Returns
/// * `Vec<EnvVarKey>` - list of all environment variables referencing secret `secret_id`
///
/// The list is sorted by environment variable key.
pub fn secret_env_vars_from_application<'a>(secret_id: &str, application: &'a Application) -> Vec<&'a str> {
  let mut secret_environment_variables = application
    .secrets
    .iter()
    .filter_map(|secret| {
      if secret_id == secret.name {
        let secret_injections = secret
          .injections
          .iter()
          .filter_map(|injection| injection.get("env").map(|secret_injection| secret_injection.as_str()))
          .collect_vec();
        if secret_injections.is_empty() {
          None
        } else {
          Some(secret_injections)
        }
      } else {
        None
      }
    })
    .flatten()
    .collect_vec();
  secret_environment_variables.sort();
  secret_environment_variables
}

/// # Get applications environment variables referencing secret
///
/// Get all environment variables from multiple `Application`s referencing secret with `secret_id`.
/// Applications are only included if they reference secret `secret_id` at least once.
///
/// # Parameters
/// * `secret_id` - id of the secret to look for
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationTuple<EnvVarKey>>` - list of tuples containing:
/// * application id
/// * reference to application
/// * list of environment variables referencing the secret `secret_id`, sorted by environment variable key
///
/// The list is sorted by application id.
pub fn secret_env_vars_from_applications<'a>(secret_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, &'a str>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let injections = secret_env_vars_from_application(secret_id, application);
      if !injections.is_empty() {
        Some(ApplicationValues::new(application_id, application, injections))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// # Checks if secret is a system secret
pub fn secret_is_system(secret_id: &str) -> bool {
  secret_id.contains('!')
}

/// Get secret resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the secret resources from
///
/// # Returns
/// Either `None` when the `app` does not have any secret resources,
/// or a `Some` that contains tuples describing the secret resources:
/// * resource id
/// * reference to the `Secret`
pub fn secret_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Secret)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Secret(secret) => Some(secret),
    _ => None,
  })
}

/// # Get application environment variables referencing secrets
///
/// Get all environment variables from an `Application` that reference secrets.
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<EnvInjection>` - list of tuples containing:
/// * secret id
/// * lists of environment variables that reference the secret
///
/// The list is sorted by secret id.
pub fn secrets_from_application(application: &Application) -> Vec<EnvVarInjection> {
  let mut grouped_injections: Vec<(&String, Vec<&str>)> = application
    .secrets
    .iter()
    .filter_map(|secret| {
      secret
        .injections
        .iter()
        .filter_map(|injection| injection.get("env").map(|key| key.as_str()))
        .collect_vec()
        .first()
        .map(|env_injection| (&secret.name, *env_injection))
    })
    .into_group_map()
    .into_iter()
    .collect_vec();
  grouped_injections.iter_mut().for_each(|(_, injections)| injections.sort());
  grouped_injections.sort();
  grouped_injections
    .into_iter()
    .map(|(secret_id, injections)| EnvVarInjection::new(secret_id, injections))
    .collect_vec()
}

/// # Get applications environment variables referencing secrets
///
/// Get all environment variables referencing secrets from all `Applications`
///
/// # Parameters
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationTuple<EnvInjection>>` - list of tuples containing:
/// * application id
/// * application reference
/// * sorted list of pairs of secret ids and lists of environment variables referencing those secrets
///
/// The list is sorted by application id.
pub fn secrets_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<EnvVarInjection>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let secret_injections = secrets_from_application(application);
      if !secret_injections.is_empty() {
        Some(ApplicationValues::new(application_id, application, secret_injections))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// Find apps that use any of a list of given secret
///
/// # Parameters
/// * `secrets` - ids of the secrets to look for
/// * `apps` - hashmap of all apps
///
/// # Returns
/// * `Vec<(app_id, app, resource_ids)>` - vector of apps that use the secret
///   * `app_id` - app id of the app that uses the secret
///   * `app` - reference to the app
///   * `resource_ids` - application secret resource ids
pub fn secrets_resources_from_apps<'a>(secrets: &[String], apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let mut resource_ids = vec![];
    let app = apps.get(&app_id).unwrap();
    for (secret_resource_id, secret) in secret_resources_from_app(app) {
      if secrets.contains(&secret.name) {
        resource_ids.push(secret_resource_id.to_string())
      }
    }
    if !resource_ids.is_empty() {
      tuples.push((app_id, app, resource_ids));
    }
  }
  tuples
}
