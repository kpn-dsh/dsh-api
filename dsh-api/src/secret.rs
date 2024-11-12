//! # Manage secrets
//!
//! Module that contains functions to manage secrets.
//!
//! ## API methods
//! * [`create_secret(secret)`](DshApiClient::create_secret)
//! * [`delete_secret(secret_id)`](DshApiClient::delete_secret)
//! * [`get_secret(secret_id) -> ByteStream`](DshApiClient::get_secret)
//! * [`get_secret_allocation_status(secret_id) -> AllocationStatus`](DshApiClient::get_secret_allocation_status)
//! * [`get_secret_configuration(secret_id) -> Empty`](DshApiClient::get_secret_configuration)
//! * [`get_secret_ids() -> Vec<String>`](DshApiClient::get_secret_ids)
//! * [`update_secret(secret_id, secret)`](DshApiClient::update_secret)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = r##"## Actual configuration methods"##)]
#![cfg_attr(feature = "actual", doc = r##"* [`get_secret_actual_configuration(secret_id) -> Empty`](DshApiClient::get_secret_actual_configuration)"##)]

// TODO Special treatment for system secrets
use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::types::{AllocationStatus, Empty, Secret};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage secrets
///
/// Module that contains functions to manage secrets.
///
/// ## API methods
/// * [`create_secret(secret)`](DshApiClient::create_secret)
/// * [`delete_secret(secret_id)`](DshApiClient::delete_secret)
/// * [`get_secret(secret_id) -> ByteStream`](DshApiClient::get_secret)
/// * [`get_secret_allocation_status(secret_id) -> AllocationStatus`](DshApiClient::get_secret_allocation_status)
/// * [`get_secret_configuration(secret_id) -> Empty`](DshApiClient::get_secret_configuration)
/// * [`get_secret_ids() -> Vec<String>`](DshApiClient::get_secret_ids)
/// * [`update_secret(secret_id, secret)`](DshApiClient::update_secret)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = r##"## Actual configuration methods"##)]
#[cfg_attr(feature = "actual", doc = r##"* [`get_secret_actual_configuration(secret_id) -> Empty`](DshApiClient::get_secret_actual_configuration)"##)]
impl DshApiClient<'_> {
  /// # Create secret
  ///
  /// API function: `POST /allocation/{tenant}/secret`
  ///
  /// ## Parameters
  /// * `secret` - secret to be created, consisting of a key/value pair
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully created)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_secret(&self, secret: &Secret) -> DshApiResult<()> {
    self
      .process(self.generated_client.post_secret_by_tenant(self.tenant_name(), self.token(), secret).await)
      .map(|(_, result)| result)
  }

  /// # Delete secret
  ///
  /// API function: `DELETE /allocation/{tenant}/secret/{id}/configuration`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the secret to delete
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_secret(&self, secret_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_secret_configuration_by_tenant_by_id(self.tenant_name(), secret_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// ## Returns
  /// * `Ok<String>` - secret
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret(&self, secret_id: &str) -> DshApiResult<String> {
    self
      .process_string(self.generated_client.get_secret_by_tenant_by_id(self.tenant_name(), secret_id, self.token()).await)
      .await
      .map(|(_, result)| result)
  }

  /// # Return actual state of secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/actual`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// ## Returns
  /// * `Ok<`[`Empty`]`>` - indicates that secret is ok, but the actual return value will be empty
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_secret_actual_configuration(&self, secret_id: &str) -> DshApiResult<Empty> {
    self
      .process(
        self
          .generated_client
          .get_secret_actual_by_tenant_by_id(self.tenant_name(), secret_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return secret allocation status
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/status`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// ## Returns
  /// * `Ok<`[`AllocationStatus`]`>` - allocation status of the secret
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_allocation_status(&self, secret_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_secret_status_by_tenant_by_id(self.tenant_name(), secret_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return configuration of secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/configuration`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// ## Returns
  /// * `Ok<`[`Empty`]`>` - indicates that secret is ok, but the return value will be empty
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_configuration(&self, secret_id: &str) -> DshApiResult<Empty> {
    self
      .process(
        self
          .generated_client
          .get_secret_configuration_by_tenant_by_id(self.tenant_name(), secret_id, self.token())
          // .secret_get_by_tenant_secret_by_id_configuration(self.tenant_name(), secret_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return sorted list of secret names
  ///
  /// API function: `GET /allocation/{tenant}/secret`
  ///
  /// ## Returns
  /// * `Ok<Vec<String>>` - list of secret names
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_ids(&self) -> DshApiResult<Vec<String>> {
    let mut secret_ids: Vec<String> = self
      .process(self.generated_client.get_secret_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    secret_ids.sort();
    Ok(secret_ids)
  }

  /// # Update secret value
  ///
  /// API function: `PUT /allocation/{tenant}/secret/{id}`
  ///
  /// ## Parameters
  /// * `secret_id` - id of the secret to update
  /// * `secret` - new value of the secret
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully updated)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn update_secret(&self, secret_id: &str, secret: String) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_secret_by_tenant_by_id(self.tenant_name(), secret_id, self.token(), secret)
          .await,
      )
      .map(|(_, result)| result)
  }
}
