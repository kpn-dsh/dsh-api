//! # Additional methods to manage volumes
//!
//! Module that contains methods to manage volumes.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_volume_with_usage(id) -> [volume_status, [usage]]`](DshApiClient::get_volume_with_usage)
//! * [`list_volumes_with_usage() -> [id, [usage]]`](DshApiClient::list_volumes_with_usage)

use crate::dsh_api_client::DshApiClient;
use crate::types::VolumeStatus;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{app, application, DshApiResult, UsedBy};
use futures::try_join;

/// # Additional methods to manage volumes
///
/// Module that contains methods to manage volumes.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_volume_with_usage(id) -> [volume_status, [usage]]`](DshApiClient::get_volume_with_usage)
/// * [`list_volumes_with_usage() -> [id, [usage]]`](DshApiClient::list_volumes_with_usage)
impl DshApiClient<'_> {
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
    let (volume_status, applications, apps) = try_join!(
      self.get_volume(volume_id),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
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
    let (volume_ids, applications, apps) = try_join!(
      self.get_volume_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
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
