//! # Manage apps in the App Catalog
//!
//! Module that contains functions to manage pre-packaged,
//! easily configured apps that you can select from the App Catalog.
//! * API methods - DshApiClient methods that directly call the API.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # API methods
//!
//! [`DshApiClient`] methods that directly call the DSH resource management API.
//!
//! * [`get_app_configuration(app_id) -> app`](DshApiClient::get_app_configuration)
//! * [`get_app_configurations() -> map<app_id, app>`](DshApiClient::get_app_configurations)
//! * [`list_app_configurations() -> [(app_id, app)]`](DshApiClient::list_app_configurations)
//! * [`list_app_ids() -> [app_id]`](DshApiClient::list_app_ids)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`list_app_configurations() -> [(app_id, app)]`](DshApiClient::list_app_configurations)
//! * [`list_app_ids() -> [app_id]`](DshApiClient::list_app_ids)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_app_actual_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_actual_configuration)")]
#![cfg_attr(feature = "actual", doc = "* [`get_app_actual_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_actual_configurations)")]
use crate::dsh_api_client::DshApiClient;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application, Bucket, Certificate, Secret, Topic, Vhost, Volume};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection};
use std::collections::HashMap;

/// # Manage apps in the App Catalog
///
/// Module that contains functions to manage pre-packaged,
/// easily configured apps that you can select from the App Catalog.
/// * API methods - DshApiClient methods that directly call the API.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # API methods
///
/// [`DshApiClient`] methods that directly call the DSH resource management API.
///
/// * [`get_app_configuration(app_id) -> app`](DshApiClient::get_app_configuration)
/// * [`get_app_configurations() -> map<app_id, app>`](DshApiClient::get_app_configurations)
/// * [`list_app_configurations() -> [(app_id, app)]`](DshApiClient::list_app_configurations)
/// * [`list_app_ids() -> [app_id]`](DshApiClient::list_app_ids)
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`list_app_configurations() -> [(app_id, app)]`](DshApiClient::list_app_configurations)
/// * [`list_app_ids() -> [app_id]`](DshApiClient::list_app_ids)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_app_actual_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_actual_configuration)")]
#[cfg_attr(feature = "actual", doc = "* [`get_app_actual_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_actual_configurations)")]
impl DshApiClient<'_> {
  /// # Return actual configuration of deployed App
  ///
  /// API function: `GET /allocation/{tenant}/appcatalogapp/{appcatalogappid}/actual`
  ///
  /// # Parameters
  /// * `app_id` - app id of the requested configuration
  ///
  /// # Returns
  /// * `Ok<`[`AppCatalogApp`]`>` - app configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_app_actual_configuration(&self, app_id: &str) -> DshApiResult<AppCatalogApp> {
    self
      .process(
        self
          .generated_client
          .get_appcatalogapp_actual_by_tenant_by_appcatalogappid(self.tenant_name(), app_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Get all actual configurations of deployed Apps
  ///
  /// API function: `GET /allocation/{tenant}/appcatalogapp/actual`
  ///
  /// # Returns
  /// * `Ok<HashMap<String, `[`AppCatalogApp`]`>>` - hashmap containing the app configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_app_actual_configurations(&self) -> DshApiResult<HashMap<String, AppCatalogApp>> {
    self
      .process(
        self
          .generated_client
          .get_appcatalogapp_actual_by_tenant(self.tenant_name(), self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return App configuration
  ///
  /// API function: `GET /allocation/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
  ///
  /// # Parameters
  /// * `app_id` - app id of the requested configuration
  ///
  /// # Returns
  /// * `Ok<`[`AppCatalogApp`]`>` - app configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_configuration(&self, app_id: &str) -> DshApiResult<AppCatalogApp> {
    self
      .process(
        self
          .generated_client
          .get_appcatalogapp_configuration_by_tenant_by_appcatalogappid(self.tenant_name(), app_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Get all App configurations
  ///
  /// API function: `GET /allocation/{tenant}/appcatalogapp/configuration`
  ///
  /// # Returns
  /// * `Ok<HashMap<String, `[`AppCatalogApp`]`>>` - hashmap containing the app configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_configurations(&self) -> DshApiResult<HashMap<String, AppCatalogApp>> {
    self
      .process(
        self
          .generated_client
          .get_appcatalogapp_configuration_by_tenant(self.tenant_name(), self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # List all App configurations
  ///
  /// # Returns
  /// * `Ok<Vec<(String, `[`AppCatalogApp`]`)>>` - list containing the app ids and configurations,
  ///   sorted by app id
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_configurations(&self) -> DshApiResult<Vec<(String, AppCatalogApp)>> {
    self.get_app_configurations().await.map(|mut app_configurations_map| {
      let mut app_ids: Vec<String> = app_configurations_map.keys().map(|app_id| app_id.to_string()).collect();
      app_ids.sort();
      app_ids
        .iter()
        .map(|app_id| (app_id.clone(), app_configurations_map.remove(app_id).unwrap()))
        .collect::<Vec<(_, _)>>()
    })
  }

  /// # List all App ids
  ///
  /// If you also need the app configuration, use
  /// [`list_app_configurations()`](Self::list_app_configurations) instead.
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted app ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_ids(&self) -> DshApiResult<Vec<String>> {
    let mut app_ids: Vec<String> = self.get_app_configurations().await?.keys().map(|app_id| app_id.to_string()).collect();
    app_ids.sort();
    Ok(app_ids)
  }
}

/// Get application resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the application resources from
///
/// # Returns
/// Either `None` when the `app` does not have any application resources,
/// or a `Some` that contains tuples describing the application resources:
/// * resource id
/// * reference to the `Application`
pub fn application_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Application)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Application(application) => Some(application),
    _ => None,
  })
}

/// Get bucket resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the bucket resources from
///
/// # Returns
/// Either `None` when the `app` does not have any bucket resources,
/// or a `Some` that contains tuples describing the bucket resources:
/// * resource id
/// * reference to the `Bucket`
pub fn bucket_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Bucket)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Bucket(bucket) => Some(bucket),
    _ => None,
  })
}

/// Get certificate resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the certificate resources from
///
/// # Returns
/// Either `None` when the `app` does not have any certificate resources,
/// or a `Some` that contains tuples describing the certificate resources:
/// * resource id
/// * reference to the `Certificate`
pub fn certificate_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Certificate)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Certificate(certificate) => Some(certificate),
    _ => None,
  })
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
pub fn secret_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Secret)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Secret(secret) => Some(secret),
    _ => None,
  })
}

/// Get topic resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the topic resources from
///
/// # Returns
/// Either `None` when the `app` does not have any topic resources,
/// or a `Some` that contains tuples describing the topic resources:
/// * resource id
/// * reference to the `Topic`
pub fn topic_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Topic)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Topic(topic) => Some(topic),
    _ => None,
  })
}

/// Get vhost resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the vhost resources from
///
/// # Returns
/// Either `None` when the `app` does not have any vhost resources,
/// or a `Some` that contains tuples describing the vhost resources:
/// * resource id
/// * reference to the `Vhost`
pub fn vhost_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Vhost)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Vhost(vhost) => Some(vhost),
    _ => None,
  })
}

/// Get volume resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the volume resources from
///
/// # Returns
/// Either `None` when the `app` does not have any volume resources,
/// or a `Some` that contains tuples describing the volume resources:
/// * resource id
/// * reference to the `Volume`
pub fn volume_resources_from_app(app: &AppCatalogApp) -> Option<Vec<(&String, &Volume)>> {
  resources_from_app(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Volume(volume) => Some(volume),
    _ => None,
  })
}

/// Find apps that use a given secret
///
/// # Parameters
/// * `secret_id` - id of the secret to look for
/// * `apps` - hashmap of all apps
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - vector of applications that use the secret
/// * `app_id` - app id of the app that uses the secret
/// * `app` - reference to the app
/// * `resource_ids` - secret resources of the secret in the app
pub fn find_apps_that_use_secret<'a>(secret_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &'a AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let app = apps.get(&app_id).unwrap();
    if let Some(secret_resources) = secret_resources_from_app(app) {
      for (secret_resource_id, secret) in secret_resources {
        if secret.name == secret_id {
          tuples.push((app_id.clone(), app, vec![secret_resource_id.to_string()]));
        }
      }
    }
  }
  tuples
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
pub fn find_apps_that_use_secrets<'a>(secrets: &[String], apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let mut resource_ids = vec![];
    let app = apps.get(&app_id).unwrap();
    if let Some(secret_resources) = secret_resources_from_app(app) {
      for (secret_resource_id, secret) in secret_resources {
        if secrets.contains(&secret.name) {
          resource_ids.push(secret_resource_id.to_string())
        }
      }
    }
    if !resource_ids.is_empty() {
      tuples.push((app_id, app, resource_ids));
    }
  }
  tuples
}

/// Find apps that use a given topic
///
/// # Parameters
/// * `topic_id` - id of the topic to look for
/// * `apps` - hashmap of all apps
///
/// # Returns
/// * `Vec<(app_id, app, resource_ids)>` - vector of applications that use the topic
///   * `app_id` - app id of the app that uses the topic
///   * `app` - reference to the app
///   * `resource_ids` - topic resources of the topic in the app
pub fn find_apps_that_use_topic<'a>(topic_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &'a AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let app = apps.get(&app_id).unwrap();
    if let Some(topic_resources) = topic_resources_from_app(app) {
      for (topic_resource_id, _) in topic_resources {
        if topic_resource_id.contains(topic_id) {
          tuples.push((app_id.clone(), app, vec![topic_resource_id.to_string()]));
        }
      }
    }
  }
  tuples
}

/// Find apps that use a given volume
///
/// # Parameters
/// * `volume_id` - id of the volume to look for
/// * `apps` - hashmap of all apps
///
/// # Returns
/// * `Vec<(app_id, app, application, injections)>` - vector of applications that use the secret
///   * `app_id` - application id of the app that uses the secret
///   * `app` - reference to the app
///   * `resource_ids` - topic resources of the volume in the app
pub fn find_apps_that_use_volume<'a>(volume_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let app = apps.get(&app_id).unwrap();
    if let Some(volume_resources) = volume_resources_from_app(app) {
      for (volume_resource_id, _) in volume_resources {
        if volume_resource_id.contains(volume_id) {
          tuples.push((app_id.clone(), app, vec![volume_resource_id.to_string()]));
        }
      }
    }
  }
  tuples
}

/// # Get all vhost injections from `AppCatalogApp`
///
/// # Parameters
/// * `app` - reference to the `AppCatalogApp`
///
/// # Returns
/// `Vec<(String, Injection)>` - list of tuples that describe the vhost injections.
/// Each tuple consist of
/// * vhost resource id
/// * vhost injection.
pub fn vhosts_from_app(app: &AppCatalogApp) -> Vec<(String, Injection)> {
  let mut injections: Vec<(String, Injection)> = vec![];
  if let Some(vhost_resources) = vhost_resources_from_app(app) {
    for (resource_id, vhost) in vhost_resources {
      injections.push((vhost.value.clone(), Injection::VhostResource(resource_id.clone())));
    }
  }
  injections
}

/// Get resources from App
fn resources_from_app<'a, T>(app: &'a AppCatalogApp, finder: &dyn Fn(&'a AppCatalogAppResourcesValue) -> Option<&'a T>) -> Option<Vec<(&'a String, &'a T)>> {
  let mut resources: Vec<(&String, &T)> = vec![];
  for (resource_id, resource) in &app.resources {
    if let Some(resource) = finder(resource) {
      resources.push((resource_id, resource))
    }
  }
  if resources.is_empty() {
    None
  } else {
    Some(resources)
  }
}
