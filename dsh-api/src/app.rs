//! # Manage apps in the App Catalog
//!
//! Module that contains functions to manage pre-packaged,
//! easily configured apps that you can select from the App Catalog.
//!
//! # API methods
//! * [`get_app_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_configuration)
//! * [`get_app_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_configurations)
//!
//! # Utility methods
//! * [`list_app_configurations() -> Vec<(String, AppCatalogApp)>`](DshApiClient::list_app_configurations)
//! * [`list_app_ids() -> Vec<String>`](DshApiClient::list_app_ids)
//!
//! # Utility functions
//! * [`application_from_app(app) -> (String, &Application)`](DshApiClient::application_from_app)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_app_actual_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_actual_configuration)")]
#![cfg_attr(feature = "actual", doc = "* [`get_app_actual_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_actual_configurations)")]
use crate::dsh_api_client::DshApiClient;
use std::collections::HashMap;

use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection};

/// # Manage apps in the App Catalog
///
/// Module that contains functions to manage pre-packaged,
/// easily configured apps that you can select from the App Catalog.
///
/// # API methods
/// * [`get_app_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_configuration)
/// * [`get_app_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_configurations)
///
/// # Utility methods
/// * [`list_app_configurations() -> Vec<(String, AppCatalogApp)>`](DshApiClient::list_app_configurations)
/// * [`list_app_ids() -> Vec<String>`](DshApiClient::list_app_ids)
///
/// # Utility functions
/// * [`application_from_app(app) -> (String, &Application)`](DshApiClient::application_from_app)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
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
          .get_appcatalogapp_actual_by_tenant_by_appcatalogappid(self.tenant_name(), app_id, self.token())
          .await,
      )
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
      .process(self.generated_client.get_appcatalogapp_actual_by_tenant(self.tenant_name(), self.token()).await)
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
          .get_appcatalogapp_configuration_by_tenant_by_appcatalogappid(self.tenant_name(), app_id, self.token())
          .await,
      )
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
          .get_appcatalogapp_configuration_by_tenant(self.tenant_name(), self.token())
          .await,
      )
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

  /// Get application resource from an App
  ///
  /// # Parameters
  /// * `app` - app to get the application resource from
  ///
  /// # Returns
  /// * `Some((resource_id, application))`
  ///   * `resource_id` - resource id from the app
  ///   * `application` - reference to the `Application`
  pub fn application_from_app(app: &AppCatalogApp) -> Option<(String, &Application)> {
    app.resources.iter().find_map(|(resource_id, resource)| match resource {
      AppCatalogAppResourcesValue::Application(application) => Some((resource_id.to_string(), application)),
      _ => None,
    })
  }

  /// Get apps that use a given secret injection
  ///
  /// # Parameters
  /// * `secret` - id of the secret to look for
  /// * `apps` - hashmap of all apps
  ///
  /// # Returns
  /// * `Vec<(app_id, app, application, injections)>` - vector of applications that use the secret
  ///   * `app_id` - application id of the application that uses the secret
  ///   * `app` - reference to the app
  ///   * `application_resource_id` - application resource id in the app
  ///   * `application` - reference to the application resource in the app
  ///   * `injections` - the injections of the secret in the application
  #[allow(clippy::type_complexity)]
  pub fn apps_with_secret_injections<'a>(secret: &str, apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, String, &'a Application, Vec<Injection>)> {
    let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
    app_ids.sort();
    let mut tuples: Vec<(String, &AppCatalogApp, String, &Application, Vec<Injection>)> = vec![];
    for app_id in app_ids {
      let app = apps.get(&app_id).unwrap();
      if let Some((application_resource_id, application)) = Self::application_from_app(app) {
        if !application.secrets.is_empty() {
          let mut injections = Vec::<Injection>::new();
          for application_secret in &application.secrets {
            if secret == application_secret.name {
              for application_secret_injection in &application_secret.injections {
                if let Some(env_injection) = application_secret_injection.get("env") {
                  injections.push(Injection::EnvVar(env_injection.to_string()));
                }
              }
            }
          }
          if !injections.is_empty() {
            tuples.push((app_id, app, application_resource_id, application, injections));
          }
        }
      }
    }
    tuples
  }

  /// Get apps that use any of a list of given secret injections
  ///
  /// # Parameters
  /// * `secrets` - ids of the secrets to look for
  /// * `apps` - hashmap of all apps
  ///
  /// # Returns
  /// * `Vec<(app_id, app, application, injections)>` - vector of applications that use the secret
  ///   * `app_id` - application id of the application that uses the secret
  ///   * `app` - reference to the app
  ///   * `application_resource_id` - application resource id in the app
  ///   * `application` - reference to the application resource in the app
  ///   * `injections` - the injections of the secret in the application
  #[allow(clippy::type_complexity)]
  pub fn apps_with_secrets_injections<'a>(
    secrets: &[String],
    apps: &'a HashMap<String, AppCatalogApp>,
  ) -> Vec<(String, &'a AppCatalogApp, String, &'a Application, HashMap<String, Vec<Injection>>)> {
    let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
    app_ids.sort();
    let mut tuples: Vec<(String, &AppCatalogApp, String, &Application, HashMap<String, Vec<Injection>>)> = vec![];
    for app_id in app_ids {
      let app = apps.get(&app_id).unwrap();
      if let Some((application_resource_id, application)) = Self::application_from_app(app) {
        if !application.secrets.is_empty() {
          let mut injections = HashMap::<String, Vec<Injection>>::new();
          for application_secret in &application.secrets {
            if secrets.contains(&application_secret.name) {
              let mut env_injections: Vec<Injection> = vec![];
              for application_secret_injection in &application_secret.injections {
                if let Some(env_injection) = application_secret_injection.get("env") {
                  env_injections.push(Injection::EnvVar(env_injection.to_string()));
                }
              }
              if !env_injections.is_empty() {
                injections.insert(application_secret.name.clone(), env_injections);
              }
            }
          }
          if !injections.is_empty() {
            tuples.push((app_id, app, application_resource_id, application, injections));
          }
        }
      }
    }
    tuples
  }
}
