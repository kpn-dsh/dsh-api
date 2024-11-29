//! # Manage volumes
//!
//! Module that contains functions to manage volumes.
//!
//! # API methods
//! * [`create_volume(volume_id, configuration)`](DshApiClient::create_volume)
//! * [`delete_volume(volume_id)`](DshApiClient::delete_volume)
//! * [`get_volume(volume_id) -> VolumeStatus`](DshApiClient::get_volume)
//! * [`get_volume_allocation_status(volume_id) -> AllocationStatus`](DshApiClient::get_volume_allocation_status)
//! * [`get_volume_configuration(volume_id) -> Volume`](DshApiClient::get_volume_configuration)
//! * [`get_volume_ids() -> Vec<String>`](DshApiClient::get_volume_ids)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_volume_actual_configuration(volume_id) -> Volume`](DshApiClient::get_volume_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, Volume, VolumeStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage volumes
///
/// Module that contains functions to manage volumes.
///
/// # API methods
/// * [`create_volume(volume_id, configuration)`](DshApiClient::create_volume)
/// * [`delete_volume(volume_id)`](DshApiClient::delete_volume)
/// * [`get_volume(volume_id) -> VolumeStatus`](DshApiClient::get_volume)
/// * [`get_volume_allocation_status(volume_id) -> AllocationStatus`](DshApiClient::get_volume_allocation_status)
/// * [`get_volume_configuration(volume_id) -> Volume`](DshApiClient::get_volume_configuration)
/// * [`get_volume_ids() -> Vec<String>`](DshApiClient::get_volume_ids)
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
          .put_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token(), configuration)
          .await,
      )
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
          .delete_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token())
          .await,
      )
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
      .process(self.generated_client.get_volume_by_tenant_by_id(self.tenant_name(), volume_id, self.token()).await)
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
          .get_volume_status_by_tenant_by_id(self.tenant_name(), volume_id, self.token())
          .await,
      )
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
          .get_volume_configuration_by_tenant_by_id(self.tenant_name(), volume_id, self.token())
          .await,
      )
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
          .get_volume_actual_by_tenant_by_id(self.tenant_name(), volume_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return sorted list of volume names
  ///
  /// API function: `GET /allocation/{tenant}/volume`
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - list of volume names
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_volume_ids(&self) -> DshApiResult<Vec<String>> {
    let mut volume_ids: Vec<String> = self
      .process(self.generated_client.get_volume_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    volume_ids.sort();
    Ok(volume_ids)
  }
}
