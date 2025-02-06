//! # Additional methods and functions to manage secrets
//!
//! Module that contains methods and functions to manage secrets.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_secret_with_usage(secret_id) -> [used_by]`](DshApiClient::get_secret_with_usage)
//! * [`list_secrets_with_usage() -> [(secret_id, used_by)]`](DshApiClient::list_secrets_with_usage)

use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::types::{AllocationStatus, Empty, Secret};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{app, application, DshApiResult, UsedBy};
use futures::try_join;

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
/// * [`get_secret_with_usage(secret_id) -> [used_by]`](DshApiClient::get_secret_with_usage)
/// * [`list_secrets_with_usage() -> [(secret_id, used_by)]`](DshApiClient::list_secrets_with_usage)
impl DshApiClient<'_> {
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
    let (applications, apps) = try_join!(self.get_application_configuration_map(), self.get_appcatalogapp_configuration_map())?;
    let mut usages: Vec<UsedBy> = vec![];
    for (application_id, application, injections) in application::find_applications_that_use_secret(secret_id, &applications) {
      usages.push(UsedBy::Application(application_id, application.instances, injections));
    }
    for (app_id, _, resource_ids) in app::find_apps_that_use_secret(secret_id, &apps) {
      usages.push(UsedBy::App(app_id, resource_ids));
    }
    Ok(usages)
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
    let (secret_ids, applications, apps) = try_join!(
      self.get_secret_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
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
}

/// # Checks if secret is a system secret
pub fn is_system_secret(secret_id: &str) -> bool {
  secret_id.contains('!')
}
