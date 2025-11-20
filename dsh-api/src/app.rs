//! # Additional methods to manage apps in the app catalog
//!
//! Module that contains methods and functions to manage apps from the app catalog.
//!
//! # Generated methods
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_appcatalog_app_configuration(appcatalogappid)`](DshApiClient::delete_appcatalog_app_configuration)
//! * [`delete_application_configuration(appid)`](DshApiClient::delete_application_configuration)
//! * [`get_appcatalog_app_configuration(appcatalogappid) -> AppCatalogAppConfiguration`](DshApiClient::get_appcatalog_app_configuration)
//! * [`get_appcatalog_app_status(appcatalogappid) -> AllocationStatus`](DshApiClient::get_appcatalog_app_status)
//! * [`get_appcatalogapp_actual(appcatalogappid) -> AppCatalogApp`](DshApiClient::get_appcatalogapp_actual)
//! * [`get_appcatalogapp_actual_map() -> HashMap<id, AppCatalogApp>`](DshApiClient::get_appcatalogapp_actual_map)
//! * [`get_appcatalogapp_configuration(appcatalogappid) -> AppCatalogApp`](DshApiClient::get_appcatalogapp_configuration)
//! * [`get_appcatalogapp_configuration_map() -> HashMap<id, AppCatalogApp>`](DshApiClient::get_appcatalogapp_configuration_map)
//! * [`put_appcatalog_app_configuration(appcatalogappid, body)`](DshApiClient::put_appcatalog_app_configuration)
//!
//! # Derived methods
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`app_configuration(app_id) -> (app, configuration)`](DshApiClient::app_configuration)
//! * [`app_configurations() -> [(app id, app, configuration)]`](DshApiClient::app_configurations)
//! * [`app_ids() -> [app_id]`](DshApiClient::app_ids)
//! * [`apps_dependant_on_application(application_id) -> [app]`](DshApiClient::apps_dependant_on_application)
//! * [`apps_dependant_on_bucket(bucket_id) -> [app]`](DshApiClient::apps_dependant_on_bucket)
//! * [`apps_dependant_on_certificate(certificate_id) -> [app]`](DshApiClient::apps_dependant_on_certificate)
//! * [`apps_dependant_on_secret(secret_id) -> [app]`](DshApiClient::apps_dependant_on_secret)
//! * [`apps_dependant_on_topic(topic_id) -> [app]`](DshApiClient::apps_dependant_on_topic)
//! * [`apps_dependant_on_vhost(vhost_id) -> [app]`](DshApiClient::apps_dependant_on_vhost)
//! * [`apps_dependant_on_volume(volume_id) -> [app]`](DshApiClient::apps_dependant_on_volume)

use crate::application::application_resources_from_app;
use crate::bucket::bucket_resources_from_app;
use crate::certificate::certificate_resources_from_app;
use crate::dsh_api_client::DshApiClient;
use crate::secret::secret_resources_from_app;
use crate::topic::topic_resources_from_app;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue};
use crate::vhost::vhost_resources_from_app;
use crate::volume::volume_resources_from_app;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DependantApp, DshApiResult};
use itertools::Itertools;
use serde_json::from_str;
use std::collections::HashMap;

/// # Additional methods to manage apps in the app catalog
///
/// Module that contains methods and functions to manage apps from the app catalog.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`app_configuration(app_id) -> [app, configuration]`](DshApiClient::app_configuration)
/// * [`app_configurations() -> [(app id, app, configuration)]`](DshApiClient::app_configurations)
/// * [`app_ids() -> [app_id]`](DshApiClient::app_ids)
/// * [`apps_dependant_on_application(application_id) -> [app]`](DshApiClient::apps_dependant_on_application)
/// * [`apps_dependant_on_bucket(bucket_id) -> [app]`](DshApiClient::apps_dependant_on_bucket)
/// * [`apps_dependant_on_certificate(certificate_id) -> [app]`](DshApiClient::apps_dependant_on_certificate)
/// * [`apps_dependant_on_secret(secret_id) -> [app]`](DshApiClient::apps_dependant_on_secret)
/// * [`apps_dependant_on_topic(topic_id) -> [app]`](DshApiClient::apps_dependant_on_topic)
/// * [`apps_dependant_on_vhost(vhost_id) -> [app]`](DshApiClient::apps_dependant_on_vhost)
/// * [`apps_dependant_on_volume(volume_id) -> [app]`](DshApiClient::apps_dependant_on_volume)
impl DshApiClient {
  /// # Return app configurations
  ///
  /// # Parameters
  /// * `app_id` - Identifier of the app.
  ///
  /// # Returns
  /// * `Ok<(String, `[`AppCatalogApp`]`, HashMap)>` - Tuple containing the app configuration
  ///   and parsed configuration hashmap.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When the app could not be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn app_configuration(&self, app_id: &str) -> DshApiResult<(AppCatalogApp, Option<HashMap<String, String>>)> {
    match self.get_appcatalogapp_configuration(app_id).await {
      Ok(app_catalog_app) => {
        let configuration = app_catalog_app
          .configuration
          .clone()
          .map(|configuration| from_str::<HashMap<String, String>>(configuration.as_str()))
          .transpose()?;
        Ok((app_catalog_app.clone(), configuration.to_owned()))
      }
      Err(e) => Err(e),
    }
  }

  /// # Return app configurations
  ///
  /// # Parameters
  /// * `app_id` - Identifier of the app.
  ///
  /// # Returns
  /// * `Ok<(String, `[`AppCatalogApp`]`, HashMap)>` - Tuple containing the app configuration
  ///   and parsed configuration hashmap.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When the app could not be found.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn app_configurations(&self) -> DshApiResult<Vec<(String, AppCatalogApp, Option<HashMap<String, String>>)>> {
    let appcatalogapp_configuration_map = self.get_appcatalogapp_configuration_map().await?;
    match appcatalogapp_configuration_map
      .into_iter()
      .map(|(app_id, app_catalog_app)| {
        app_catalog_app
          .configuration
          .clone()
          .map(|configuration| from_str::<HashMap<String, String>>(configuration.as_str()))
          .transpose()
          .map(|c| (app_id, app_catalog_app.clone(), c.to_owned()))
      })
      .collect::<Result<Vec<_>, _>>()
    {
      Ok(mut app_configurations) => {
        app_configurations.sort_by(|(app_id_a, _, _), (app_id_b, _, _)| app_id_a.cmp(app_id_b));
        Ok(app_configurations)
      }
      Err(error) => Err(DshApiError::Unexpected("error parsing app configuration".to_string(), Some(error.to_string()))),
    }
  }

  /// # List all App ids
  ///
  /// If you also need the app configuration, use
  /// [`app_configurations()`](DshApiClient::app_configurations) instead.
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted app ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn app_ids(&self) -> DshApiResult<Vec<String>> {
    let mut app_ids: Vec<String> = self.get_appcatalogapp_configuration_map().await?.keys().map(|app_id| app_id.to_string()).collect();
    app_ids.sort();
    Ok(app_ids)
  }

  /// # Get all apps that depend on an application
  ///
  /// # Parameters
  /// * `application_id` - Identifier of the application.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the application.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_application(&self, application_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_application(application_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a bucket
  ///
  /// # Parameters
  /// * `bucket_id` - Identifier of the application.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the bucket.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_bucket(&self, bucket_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_resource(bucket_id, &self.get_appcatalogapp_configuration_map().await?, &bucket_resources_from_app)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a certificate
  ///
  /// # Parameters
  /// * `certificate_id` - Identifier of the certificate.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the certificate.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_certificate(&self, certificate_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_certificate(certificate_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a secret
  ///
  /// # Parameters
  /// * `secret_id` - Identifier of the secret.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the secret.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_secret(&self, secret_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_secret(secret_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a topic
  ///
  /// # Parameters
  /// * `topic_id` - Identifier of the topic.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the topic.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_topic(&self, topic_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_topic(topic_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a vhost
  ///
  /// # Parameters
  /// * `vhost_id` - Identifier of the vhost.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the vhost.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_vhost(&self, vhost_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_vhost(vhost_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a volume
  ///
  /// # Parameters
  /// * `volume_id` - Identifier of the volume.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the volume.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn apps_dependant_on_volume(&self, volume_id: &str) -> DshApiResult<Vec<DependantApp>> {
    Ok(
      apps_that_use_volume(volume_id, &self.get_appcatalogapp_configuration_map().await?)
        .into_iter()
        .map(|(app_id, _, resource_ids)| DependantApp::new(app_id.to_string(), resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec()))
        .collect_vec(),
    )
  }
}

/// Find apps that use an application
///
/// # Parameters
/// * `application_id` - Identifier of the application to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the application:
/// * `app_id` - App id of the app that uses the secret,
/// * `app` - Reference to the app,
/// * `resource_ids` - Application resources of the application in the app.
pub fn apps_that_use_application<'a>(application_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(application_id, apps, &application_resources_from_app)
}

/// Find apps that use a certificate
///
/// # Parameters
/// * `certificate_id` - Identifier of the certificate to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the certificate:
/// * `app_id` - App id of the app that uses the secret,
/// * `app` - Reference to the app,
/// * `resource_ids` - Certificate resources of the certificate in the app.
pub fn apps_that_use_certificate<'a>(certificate_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(certificate_id, apps, &certificate_resources_from_app)
}

/// Find apps that use a given secret
///
/// # Parameters
/// * `secret_id` - Identifier of the secret to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the secret:
/// * `app_id` - App id of the app that uses the secret,
/// * `app` - Reference to the app,
/// * `resource_ids` - Secret resources of the secret in the app.
pub fn apps_that_use_secret<'a>(secret_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(secret_id, apps, &secret_resources_from_app)
}

/// Find apps that use a given topic
///
/// # Parameters
/// * `topic_id` - Identifier of the topic to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the topic:
/// * `app_id` - App id of the app that uses the secret,
/// * `app` - Reference to the app,
/// * `resource_ids` - Topic resources of the topic in the app.
pub fn apps_that_use_topic<'a>(topic_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(topic_id, apps, &topic_resources_from_app)
}

/// Find apps that use a given vhost
///
/// # Parameters
/// * `volume_id` - Identifier of the vhost to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the vhost:
/// * `app_id` - App id of the app that uses the vhost,
/// * `app` - Reference to the app,
/// * `resource_ids` - Vhost resources of the vhost in the app.
pub fn apps_that_use_vhost<'a>(vhost_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(vhost_id, apps, &vhost_resources_from_app)
}

/// Find apps that use a given volume
///
/// # Parameters
/// * `volume_id` - Identifier of the volume to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// `Vec<(app_id, app, resource_ids)>` - Vector of apps that use the volume:
/// * `app_id` - App id of the app that uses the volume,
/// * `app` - Reference to the app,
/// * `resource_ids` - Volume resources of the volume in the app.
pub fn apps_that_use_volume<'a>(volume_id: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  apps_that_use_resource(volume_id, apps, &volume_resources_from_app)
}

/// Get resources of specified variant from `AppCatalogApp`
///
/// # Parameters
/// * `app` - Reference to the `AppCatalogApp` to get the resources from.
/// * `get_resource_variant` - Closure that takes a resource and must return `Some(variant)`
///   when the resource is of the requested variant. If the resource is of a different variant
///   the closure must return `None`. The variant must match the type parameter `T`.
///
/// # Returns
/// List of tuples describing the resource, sorted by resource id. Each tuple contains:
/// * Resource id,
/// * Reference to the `T`.
pub(crate) fn app_resources<'a, T>(app: &'a AppCatalogApp, get_resource_variant: &dyn Fn(&'a AppCatalogAppResourcesValue) -> Option<&'a T>) -> Vec<(&'a str, &'a T)> {
  let mut resources: Vec<(&str, &T)> = vec![];
  for (resource_id, resource) in &app.resources {
    if let Some(resource) = get_resource_variant(resource) {
      resources.push((resource_id, resource))
    }
  }
  resources.sort_by(|(resource_id_a, _), (resource_id_b, _)| resource_id_a.cmp(resource_id_b));
  resources
}

/// Find apps that use a given resource
///
/// # Parameters
/// * `resource_id` - Identifier of the resource to look for.
/// * `apps` - Hashmap of all apps.
/// * `get_resources_variants_from_app` - Closure that retrieves all resources from a specific variant from an app.
///
/// # Returns
/// Vector of tuples, sorted by `app_id`. Each tuple describes one app that uses the resource:
/// * `app_id` - App id of the app that uses the resource,
/// * `app` - Reference to the app,
/// * `resource_ids` - Resource ids of the matching resources in the app.
pub(crate) fn apps_that_use_resource<'a, T: 'a>(
  resource_id: &str,
  apps: &'a HashMap<String, AppCatalogApp>,
  get_resources_variants_from_app: &dyn Fn(&AppCatalogApp) -> Vec<(&str, &T)>,
) -> Vec<(&'a str, &'a AppCatalogApp, Vec<&'a str>)> {
  let mut tuples: Vec<(&str, &'a AppCatalogApp, Vec<&str>)> = vec![];
  for (app_id, app) in apps {
    for (resource_id_from_app, _resource) in get_resources_variants_from_app(app) {
      if resource_id_from_app == resource_id {
        tuples.push((app_id, app, vec![resource_id_from_app]));
      }
    }
  }
  tuples.sort_by(|(app_id_a, _, _), (app_id_b, _, _)| app_id_a.cmp(app_id_b));
  tuples
}
