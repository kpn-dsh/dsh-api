//! # Manage vhosts
//!
//! Module that contains methods and functions to manage vhosts.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # Derived methods
//! * [`list_vhosts_with_usage() -> [id, [usage]]`](DshApiClient::list_vhosts_with_usage)

use crate::app::vhosts_from_app;
use crate::application::vhosts_from_applications;

/// # Manage vhosts
///
/// Module that contains methods and functions to manage vhosts.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
/// * [`list_vhosts_with_usage() -> [id, [usage]]`](DshApiClient::list_vhosts_with_usage)
use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection, UsedBy};
use futures::try_join;
use std::collections::HashMap;

impl DshApiClient<'_> {
  //  /// # Get vhost with usage
  //  ///
  //  /// Returns usage for a given vhost.
  //  ///
  //  /// # Parameters
  //  /// * `vhost_id` - name of the requested vhost
  //  ///
  //  /// # Returns
  //  /// * `Ok<Vec<UsedBy>>` - vhost usage.
  //  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  // pub async fn get_vhost_usage(&self, _vhost_id: &str) -> DshApiResult<Vec<UsedBy>> {
  //   let (_applications, _apps) = try_join!(self.get_applications(), self.get_app_configurations())?;
  //   let usages: Vec<UsedBy> = vec![];
  // for (application_id, application) in &applications {
  //   let mut injections: Vec<Injection> = vec![];
  //   for (port, port_mapping) in &application.exposed_ports {
  //     if let Some(vhost_string) = port_mapping.vhost.clone() {
  //       if let Some((vhost, a_zone)) = parse_vhost_string(&vhost_string) {
  //         if vhost_id == vhost {
  //           injections.push(Injection::Vhost(port.clone(), vhost, a_zone));
  //           let used_by_application = UsedBy::Application(application_id.clone(), application.instances, injections);
  //         }
  //       }
  //     }
  //   }
  //   if !injections.is_empty() {
  //     usages.push(UsedBy::Application(application_id.clone(), application.instances, injections));
  //   }
  // }
  // for (app_id, app) in &apps {
  //   let mut injections: Vec<Injection> = vec![];
  //   for (port, port_mapping) in &application.exposed_ports {
  //     if let Some(vhost_string) = port_mapping.vhost.clone() {
  //       if let Some((vhost, a_zone)) = parse_vhost_string(&vhost_string) {
  //         if vhost_id == vhost {
  //           injections.push(Injection::Vhost(port.clone(), vhost, a_zone));
  //           let used_by_application = UsedBy::Application(application_id.clone(), application.instances);
  //         }
  //       }
  //     }
  //   }
  //   if !injections.is_empty() {
  //     usages.push(UsedBy::Application(application_id, application.instances, injections));
  //   }
  // }

  // {
  //         ...
  //         "exposedPorts": {
  //                 "1880": {
  //                         "vhost": "{ vhost('your-vhost-name','a-zone') }",
  //                         "auth": "app-realm:admin:$1$EZsDrd93$7g2osLFOay4.TzDgGo9bF/",
  //                         "mode": "http",
  //                         "whitelist": "56.59.120.23",
  //                         "paths": [
  //                           { "prefix": "/abc" }
  //                         ],
  //                         "serviceGroup": "mygroup"
  //                 }
  //         }
  // }

  // let applications_with_volume_injections: Vec<(String, &Application, Vec<Injection>)> = application::volume_injections_from_applications(volume_id, &applications);
  // for (application_id, application, injections) in applications_with_volume_injections {
  //   usages.push(UsedBy::Application(application_id, application.instances, injections));
  // }
  // let apps_with_volume_injections: Vec<(String, &AppCatalogApp, String, &Application, Vec<Injection>)> = app::volume_injections_from_apps(volume_id, &apps);
  // for (app_id, _, application_id, application, injections) in apps_with_volume_injections {
  //   usages.push(UsedBy::App(app_id, application_id, application.instances, injections));
  // }
  // Ok(usages)
  // }

  /// # Get all vhost injections from `Application`
  ///
  /// # Parameters
  /// * `application` - reference to the `Application`
  ///
  /// # Returns
  /// `Vec<(String, Vec<UsedBy>)>` - list of tuples that describe the vhost injections.
  /// Each tuple consist of
  /// * vhost id
  /// * vhost usage
  pub async fn list_vhosts_with_usage(&self) -> DshApiResult<Vec<(String, Vec<UsedBy>)>> {
    let (applications, apps) = try_join!(self.get_applications(), self.get_app_configurations())?;
    let mut vhosts_with_usage_map: HashMap<String, Vec<UsedBy>> = HashMap::new();
    for (application_id, application, vhost_injections) in vhosts_from_applications(&applications) {
      for (vhost, injection) in &vhost_injections {
        if let Injection::Vhost(_, _) = injection {
          vhosts_with_usage_map
            .entry(vhost.clone())
            .or_default()
            .push(UsedBy::Application(application_id.clone(), application.instances, vec![injection.clone()]))
        }
      }
    }
    let mut app_ids = apps.keys().collect::<Vec<_>>();
    app_ids.sort();
    for app_id in app_ids {
      let app = apps.get(app_id).unwrap();
      for (vhost, injection) in vhosts_from_app(app) {
        vhosts_with_usage_map
          .entry(vhost.clone())
          .or_default()
          .push(UsedBy::App(app_id.clone(), vec![injection.to_string()]))
      }
    }
    let mut vhosts_with_usage: Vec<(String, Vec<UsedBy>)> = Vec::from_iter(vhosts_with_usage_map.into_iter());
    vhosts_with_usage.sort_by(|(key_a, _), (key_b, _)| key_a.cmp(key_b));
    Ok(vhosts_with_usage)
  }
}
