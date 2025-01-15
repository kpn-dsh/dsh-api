//! # Manage secrets
//!
//! Module that contains methods and functions to manage secrets.
//! * API methods - DshApiClient methods that directly call the API.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # API methods
//!
//! [`DshApiClient`] methods that directly call the DSH resource management API.
//!
//! * [`create_secret(secret)`](DshApiClient::create_secret)
//! * [`delete_secret(id)`](DshApiClient::delete_secret)
//! * [`get_secret(id) -> bytes`](DshApiClient::get_secret)
//! * [`get_secret_allocation_status(id) -> allocation_status`](DshApiClient::get_secret_allocation_status)
//! * [`get_secret_configuration(id) -> ok`](DshApiClient::get_secret_configuration)
//! * [`get_secret_with_usage(id) -> [usage]`](DshApiClient::get_secret_with_usage)
//! * [`list_secret_ids() -> [id]`](DshApiClient::list_secret_ids)
//! * [`list_secrets_with_usage() -> [(id, usage)]`](DshApiClient::list_secrets_with_usage)
//! * [`update_secret(id, secret)`](DshApiClient::update_secret)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_secret_with_usage(secret_id) -> [used_by]`](DshApiClient::get_secret_with_usage)
//! * [`list_secrets_with_usage() -> [(secret_id, used_by)]`](DshApiClient::list_secrets_with_usage)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_secret_actual_configuration(id) -> Empty`](DshApiClient::get_secret_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::types::{AllocationStatus, Empty, Secret};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{app, application, DshApiResult, UsedBy};
use futures::try_join;

/// # Manage secrets
///
/// Module that contains methods and functions to manage secrets.
/// * API methods - DshApiClient methods that directly call the API.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # API methods
///
/// [`DshApiClient`] methods that directly call the DSH resource management API.
///
/// * [`create_secret(secret)`](DshApiClient::create_secret)
/// * [`delete_secret(id)`](DshApiClient::delete_secret)
/// * [`get_secret(id) -> bytes`](DshApiClient::get_secret)
/// * [`get_secret_allocation_status(id) -> allocation_status`](DshApiClient::get_secret_allocation_status)
/// * [`get_secret_configuration(id) -> ok`](DshApiClient::get_secret_configuration)
/// * [`get_secret_with_usage(id) -> [usage]`](DshApiClient::get_secret_with_usage)
/// * [`list_secret_ids() -> [id]`](DshApiClient::list_secret_ids)
/// * [`list_secrets_with_usage() -> [(id, usage)]`](DshApiClient::list_secrets_with_usage)
/// * [`update_secret(id, secret)`](DshApiClient::update_secret)
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_secret_with_usage(secret_id) -> [used_by]`](DshApiClient::get_secret_with_usage)
/// * [`list_secrets_with_usage() -> [(secret_id, used_by)]`](DshApiClient::list_secrets_with_usage)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_secret_actual_configuration(id) -> Empty`](DshApiClient::get_secret_actual_configuration)")]
impl DshApiClient<'_> {
  /// # Create secret
  ///
  /// API function: `POST /allocation/{tenant}/secret`
  ///
  /// # Parameters
  /// * `secret` - secret to be created, consisting of a key/value pair
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully created)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_secret(&self, secret: &Secret) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .post_secret_by_tenant(self.tenant_name(), self.token().await?.as_str(), secret)
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Delete secret
  ///
  /// API function: `DELETE /allocation/{tenant}/secret/{id}/configuration`
  ///
  /// # Parameters
  /// * `secret_id` - id of the secret to delete
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_secret(&self, secret_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_secret_configuration_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}`
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// # Returns
  /// * `Ok<String>` - secret
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret(&self, secret_id: &str) -> DshApiResult<String> {
    self
      .process_string(
        self
          .generated_client
          .get_secret_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str())
          .await,
      )
      .await
      .map(|(_, result)| result)
  }

  /// # Return actual state of secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/actual`
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// # Returns
  /// * `Ok<`[`Empty`]`>` - indicates that secret is ok, but the actual return value will be empty
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_secret_actual_configuration(&self, secret_id: &str) -> DshApiResult<Empty> {
    self
      .process(
        self
          .generated_client
          .get_secret_actual_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return secret allocation status
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/status`
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// # Returns
  /// * `Ok<`[`AllocationStatus`]`>` - allocation status of the secret
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_allocation_status(&self, secret_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_secret_status_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return configuration of secret
  ///
  /// API function: `GET /allocation/{tenant}/secret/{id}/configuration`
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// # Returns
  /// * `Ok<`[`Empty`]`>` - indicates that secret is ok, but the return value will be empty
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_configuration(&self, secret_id: &str) -> DshApiResult<Empty> {
    self
      .process(
        self
          .generated_client
          .get_secret_configuration_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str())
          // .secret_get_by_tenant_secret_by_id_configuration(self.tenant_name(), secret_id, self.token().await?.as_str())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Get secrets with usage
  ///
  /// Returns configuration and usage for a given secret.
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secrets
  ///
  /// # Returns
  /// * `Ok<Vec<UsedBy>>` - usage.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_secret_with_usage(&self, secret_id: &str) -> DshApiResult<Vec<UsedBy>> {
    let (applications, apps) = try_join!(self.get_applications(), self.get_app_configurations())?;
    let mut usages: Vec<UsedBy> = vec![];
    for (application_id, application, injections) in application::find_applications_that_use_secret(secret_id, &applications) {
      usages.push(UsedBy::Application(application_id, application.instances, injections));
    }
    for (app_id, _, resource_ids) in app::find_apps_that_use_secret(secret_id, &apps) {
      usages.push(UsedBy::App(app_id, resource_ids));
    }
    Ok(usages)
  }

  /// # Return sorted list of secret names
  ///
  /// API function: `GET /allocation/{tenant}/secret`
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - list of secret names
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_secret_ids(&self) -> DshApiResult<Vec<String>> {
    let mut secret_ids: Vec<String> = self
      .process(self.generated_client.get_secret_by_tenant(self.tenant_name(), self.token().await?.as_str()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    secret_ids.sort();
    Ok(secret_ids)
  }

  /// # List all secrets with usage
  ///
  /// Returns a list of all secrets together with the apps and applications that use them.
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<UsedBy>>>` - list of tuples
  ///   containing the secret id and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_secrets_with_usage(&self) -> DshApiResult<Vec<(String, Vec<UsedBy>)>> {
    let (secret_ids, applications, apps) = try_join!(self.list_secret_ids(), self.get_applications(), self.get_app_configurations())?;
    let mut secrets_with_usage: Vec<(String, Vec<UsedBy>)> = vec![];
    for secret_id in secret_ids {
      if !is_system_secret(secret_id.as_str()) {
        let mut usages: Vec<UsedBy> = vec![];
        for (application_id, application, injections) in application::find_applications_that_use_secret(secret_id.as_str(), &applications) {
          usages.push(UsedBy::Application(application_id, application.instances, injections));
        }
        for (app_id, _, resource_ids) in app::find_apps_that_use_secret(secret_id.as_str(), &apps) {
          usages.push(UsedBy::App(app_id, resource_ids));
        }
        secrets_with_usage.push((secret_id, usages));
      }
    }
    Ok(secrets_with_usage)
  }

  /// # Update secret value
  ///
  /// API function: `PUT /allocation/{tenant}/secret/{id}`
  ///
  /// # Parameters
  /// * `secret_id` - id of the secret to update
  /// * `secret` - new value of the secret
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the secret has been successfully updated)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn update_secret(&self, secret_id: &str, secret: String) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_secret_by_tenant_by_id(self.tenant_name(), secret_id, self.token().await?.as_str(), secret)
          .await,
      )
      .map(|(_, result)| result)
  }
}

/// # Checks if secret is a system secret
pub fn is_system_secret(secret_id: &str) -> bool {
  secret_id.contains('!')
}
