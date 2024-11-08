//! # Manage the App Catalog
//!
//! Module that contains functions to configure apps you start from the App Catalog.
//!
//! ## API methods
//! * [`create_app_catalog_configuration(app_id, configuration) -> ()`](DshApiClient::create_app_catalog_configuration)
//! * [`delete_app_catalog_configuration(app_id) -> ()`](DshApiClient::delete_app_catalog_configuration)
//! * [`get_app_catalog_configuration_allocation_status(app_id) -> AllocationStatus`](DshApiClient::get_app_catalog_configuration_allocation_status)
//! * [`get_app_catalog_configuration(app_id) -> AppCatalogAppConfiguration`](DshApiClient::get_app_catalog_configuration)

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, AppCatalogAppConfiguration};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage the App Catalog
///
/// Module that contains functions to configure apps you start from the App Catalog.
///
/// ## API methods
/// * [`create_app_catalog_configuration(app_id, configuration) -> ()`](DshApiClient::create_app_catalog_configuration)
/// * [`delete_app_catalog_configuration(app_id) -> ()`](DshApiClient::delete_app_catalog_configuration)
/// * [`get_app_catalog_configuration_allocation_status(app_id) -> AllocationStatus`](DshApiClient::get_app_catalog_configuration_allocation_status)
/// * [`get_app_catalog_configuration(app_id) -> AppCatalogAppConfiguration`](DshApiClient::get_app_catalog_configuration)
impl DshApiClient<'_> {
  /// # Create or update a new App Catalog App
  ///
  /// API function: `PUT /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
  ///
  /// ## Parameters
  /// * `app_catalog_id` - id of the app that must be deleted
  /// * `configuration` - configuration of the app that must created or updated
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the app has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_app_catalog_configuration(&self, app_catalog_id: &str, body: &AppCatalogAppConfiguration) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_appcatalog_appcatalogapp_configuration_by_tenant_by_appcatalogappid(self.tenant_name(), app_catalog_id, self.token(), body)
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Delete an App Catalog App
  ///
  /// API function: `DELETE /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
  ///
  /// ## Parameters
  /// * `app_catalog_id` - id of the app that must be deleted
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the app has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_app_catalog_configuration(&self, app_catalog_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_appcatalog_appcatalogapp_configuration_by_tenant_by_appcatalogappid(self.tenant_name(), app_catalog_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Get an App Catalog App status
  ///
  /// API function: `GET /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/status`
  ///
  /// ## Parameters
  /// * `app_catalog_id` - id of the requested app
  ///
  /// ## Returns
  /// * `Ok<`[`AllocationStatus`]`>` - app status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_catalog_configuration_allocation_status(&self, app_catalog_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_appcatalog_appcatalogapp_status_by_tenant_by_appcatalogappid(self.tenant_name(), app_catalog_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Get an App Catalog App configuration
  ///
  /// API function: `GET /appcatalog/{tenant}/appcatalogapp/{appcatalogappid}/configuration`
  ///
  /// ## Parameters
  /// * `app_catalog_id` - id of the requested app
  ///
  /// ## Returns
  /// * `Ok<`[`AppCatalogAppConfiguration`]`>` - app configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_app_catalog_configuration(&self, app_catalog_id: &str) -> DshApiResult<AppCatalogAppConfiguration> {
    self
      .process(
        self
          .generated_client
          .get_appcatalog_appcatalogapp_configuration_by_tenant_by_appcatalogappid(self.tenant_name(), app_catalog_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }
}
