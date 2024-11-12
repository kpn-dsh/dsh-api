//! # Manage apps in the App Catalog
//!
//! Module that contains functions to manage pre-packaged,
//! easily configured apps that you can select from the App Catalog.
//!
//! ## API methods
//! * [`get_app_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_configuration)
//! * [`get_app_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_configurations)
//!
//! ## Utility methods
//! * [`list_app_configurations() -> Vec<(String, AppCatalogApp)>`](DshApiClient::list_app_configurations)
//! * [`list_app_ids() -> Vec<String>`](DshApiClient::list_app_ids)
//!
//! ## Utility functions
//! * [`application_from_app(app) -> (String, &Application)`](DshApiClient::application_from_app)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = r##"## Actual configuration methods"##)]
#![cfg_attr(feature = "actual", doc = r##"* [`get_app_actual_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_actual_configuration)"##)]
#![cfg_attr(feature = "actual", doc = r##"* [`get_app_actual_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_actual_configurations)"##)]
use crate::dsh_api_client::DshApiClient;
use std::collections::HashMap;

use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage apps in the App Catalog
///
/// Module that contains functions to manage pre-packaged,
/// easily configured apps that you can select from the App Catalog.
///
/// ## API methods
/// * [`get_app_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_configuration)
/// * [`get_app_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_configurations)
///
/// ## Utility methods
/// * [`list_app_configurations() -> Vec<(String, AppCatalogApp)>`](DshApiClient::list_app_configurations)
/// * [`list_app_ids() -> Vec<String>`](DshApiClient::list_app_ids)
///
/// ## Utility functions
/// * [`application_from_app(app) -> (String, &Application)`](DshApiClient::application_from_app)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = r##"## Actual configuration methods"##)]
#[cfg_attr(feature = "actual", doc = r##"* [`get_app_actual_configuration(app_id) -> AppCatalogApp`](DshApiClient::get_app_actual_configuration)"]"##)]
#[cfg_attr(feature = "actual", doc = r##"* [`get_app_actual_configurations() -> HashMap<String, AppCatalogApp>`](DshApiClient::get_app_actual_configurations)"##)]
impl DshApiClient<'_> {
  /// # Return actual configuration of deployed App
  ///
  /// API function: `GET /allocation/{tenant}/appcatalogapp/{appcatalogappid}/actual`
  ///
  /// ## Parameters
  /// * `app_id` - app id of the requested configuration
  ///
  /// ## Returns
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
  /// ## Returns
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
  /// ## Parameters
  /// * `app_id` - app id of the requested configuration
  ///
  /// ## Returns
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
  /// ## Returns
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
  /// ## Returns
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
  /// ## Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted app ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_app_ids(&self) -> DshApiResult<Vec<String>> {
    let mut app_ids: Vec<String> = self.get_app_configurations().await?.keys().map(|app_id| app_id.to_string()).collect();
    app_ids.sort();
    Ok(app_ids)
  }

  /// Get application resource from an App
  ///
  /// ## Parameters
  /// * `app` - app to get the application resource from
  ///
  /// ## Returns
  /// * `Some((String, &`[`Application`]`))`
  pub fn application_from_app(app: &AppCatalogApp) -> Option<(String, &Application)> {
    app.resources.iter().find_map(|(resource_id, resource)| match resource {
      AppCatalogAppResourcesValue::Application(application) => Some((resource_id.to_string(), application)),
      _ => None,
    })
  }
}
