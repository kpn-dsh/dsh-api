//! # Manage volumes
//!
//! Module that contains methods and functions to manage volumes.
//! * API methods - DshApiClient methods that directly call the API.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # API methods
//!
//! [`DshApiClient`] methods that directly call the DSH resource management API.
//!
//! * [`create_volume(id, configuration)`](DshApiClient::create_volume)
//! * [`delete_volume(id)`](DshApiClient::delete_volume)
//! * [`get_volume(id) -> volume_status`](DshApiClient::get_volume)
//! * [`get_volume_allocation_status(id) -> allocation_status`](DshApiClient::get_volume_allocation_status)
//! * [`get_volume_configuration(id) -> volume`](DshApiClient::get_volume_configuration)
//! * [`list_volume_ids() -> [id]`](DshApiClient::list_volume_ids)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_volume_with_usage(id) -> [volume_status, [usage]]`](DshApiClient::get_volume_with_usage)
//! * [`list_volumes_with_usage() -> [id, [usage]]`](DshApiClient::list_volumes_with_usage)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_volume_actual_configuration(volume_id) -> Volume`](DshApiClient::get_volume_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, Volume, VolumeStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{app, application, DshApiResult, UsedBy};
use futures::try_join;

/// # Manage volumes
///
/// Module that contains methods and functions to manage volumes.
/// * API methods - DshApiClient methods that directly call the API.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # API methods
///
/// [`DshApiClient`] methods that directly call the DSH resource management API.
///
/// * [`create_volume(id, configuration)`](DshApiClient::create_volume)
/// * [`delete_volume(id)`](DshApiClient::delete_volume)
/// * [`get_volume(id) -> volume_status`](DshApiClient::get_volume)
/// * [`get_volume_allocation_status(id) -> allocation_status`](DshApiClient::get_volume_allocation_status)
/// * [`get_volume_configuration(id) -> volume`](DshApiClient::get_volume_configuration)
/// * [`list_volume_ids() -> [id]`](DshApiClient::list_volume_ids)
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_volume_with_usage(id) -> [volume_status, [usage]]`](DshApiClient::get_volume_with_usage)
/// * [`list_volumes_with_usage() -> [id, [usage]]`](DshApiClient::list_volumes_with_usage)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_volume_actual_configuration(volume_id) -> Volume`](DshApiClient::get_volume_actual_configuration)")]
impl DshApiClient<'_> {
  /// # Create volume
  ///
  /// API function: `PUT /allocation/{tenant}/volume/{id}/configuration`
  ///
  /// # Parameters
  /// * `volume_id` - name of the created volume
  /// * `configuration` - configuration for the created volume
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the volume has been successfully created)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_volume(&self, volume_id: &str, configuration: &Volume) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str(), configuration)
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Delete volume
  ///
  /// API function: `DELETE /allocation/{tenant}/volume/{id}/configuration`
  ///
  /// # Parameters
  /// * `volume_id` - name of the volume to delete
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the volume has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_volume(&self, volume_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return volume
  ///
  /// API function: `GET /allocation/{tenant}/volume/{id}`
  ///
  /// # Parameters
  /// * `volume_id` - name of the requested volume
  ///
  /// # Returns
  /// * `Ok<`[`VolumeStatus`]`>` - volume status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_volume(&self, volume_id: &str) -> DshApiResult<VolumeStatus> {
    self
      .process(
        self
          .generated_client
          .get_volume_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return volume allocation status
  ///
  /// API function: `GET /allocation/{tenant}/volume/{id}/status`
  ///
  /// # Parameters
  /// * `volume_id` - name of the requested volume
  ///
  /// # Returns
  /// * `Ok<`[`AllocationStatus`]`>` - volume allocation status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_volume_allocation_status(&self, volume_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_volume_status_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return volume configuration
  ///
  /// API function: `GET /allocation/{tenant}/volume/{id}/configuration`
  ///
  /// # Parameters
  /// * `volume_id` - name of the requested volume
  ///
  /// # Returns
  /// * `Ok<`[`Volume`]`>` - volume configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_volume_configuration(&self, volume_id: &str) -> DshApiResult<Volume> {
    self
      .process(
        self
          .generated_client
          .get_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return actual volume configuration
  ///
  /// API function: `GET /allocation/{tenant}/volume/{id}/actual`
  ///
  /// # Parameters
  /// * `volume_id` - name of the requested volume
  ///
  /// # Returns
  /// * `Ok<`[`Volume`]`>` - volume configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_volume_actual_configuration(&self, volume_id: &str) -> DshApiResult<Volume> {
    self
      .process(
        self
          .generated_client
          .get_volume_actual_by_tenant_by_id(self.tenant_name(), volume_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return sorted list of volume names
  ///
  /// API function: `GET /allocation/{tenant}/volume`
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - list of volume names
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_volume_ids(&self) -> DshApiResult<Vec<String>> {
    let mut volume_ids: Vec<String> = self
      .process(self.generated_client.get_volume_by_tenant(self.tenant_name(), self.token().await?.as_str()).await)
      .await
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    volume_ids.sort();
    Ok(volume_ids)
  }

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
  pub async fn get_volume_with_usage(&self, volume_id: &str) -> DshApiResult<(VolumeStatus, Vec<UsedBy>)> {
    let (volume_status, applications, apps) = try_join!(self.get_volume(volume_id), self.get_applications(), self.get_app_configurations())?;
    let mut usages: Vec<UsedBy> = vec![];
    for (application_id, application, injections) in application::find_applications_that_use_volume(volume_id, &applications) {
      usages.push(UsedBy::Application(application_id, application.instances, injections));
    }
    for (app_id, _, resource_ids) in app::find_apps_that_use_volume(volume_id, &apps) {
      usages.push(UsedBy::App(app_id, resource_ids));
    }
    Ok((volume_status, usages))
  }

  /// # List all volumes with usage
  ///
  /// Returns a list of all volumes together with the apps and applications that use them.
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<UsedBy>>>` - list of tuples
  ///   containing the secret id and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_volumes_with_usage(&self) -> DshApiResult<Vec<(String, Vec<UsedBy>)>> {
    let (volume_ids, applications, apps) = try_join!(self.list_volume_ids(), self.get_applications(), self.get_app_configurations())?;
    let mut volumes_with_usage: Vec<(String, Vec<UsedBy>)> = vec![];
    for volume_id in volume_ids {
      let mut usages: Vec<UsedBy> = vec![];
      for (application_id, application, injections) in application::find_applications_that_use_volume(volume_id.as_str(), &applications) {
        usages.push(UsedBy::Application(application_id, application.instances, injections));
      }
      for (app_id, _, resource_ids) in app::find_apps_that_use_volume(volume_id.as_str(), &apps) {
        usages.push(UsedBy::App(app_id, resource_ids));
      }
      volumes_with_usage.push((volume_id, usages));
    }
    Ok(volumes_with_usage)
  }
}
