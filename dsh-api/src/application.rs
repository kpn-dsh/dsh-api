//! # Additional methods to manage applications
//!
//! Module that contains methods and functions to manage applications.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_application_configuration(appid)`](DshApiClient::delete_application_configuration)
//! * [`get_application_actual(appid) -> Application`](DshApiClient::get_application_actual)
//! * [`get_application_actual_map() -> HashMap<id, Application>`](DshApiClient::get_application_actual_map)
//! * [`get_application_configuration(appid) -> Application`](DshApiClient::get_application_configuration)
//! * [`get_application_configuration_map() -> HashMap<id, Application>`](DshApiClient::get_application_configuration_map)
//! * [`get_application_status(appid) -> AllocationStatus`](DshApiClient::get_application_status)
//! * [`get_task(appid, id) -> TaskStatus`](DshApiClient::get_task)
//! * [`get_task_actual(appid, id) -> Task`](DshApiClient::get_task_actual)
//! * [`get_task_appid_ids(appid) -> [id]`](DshApiClient::get_task_appid_ids)
//! * [`get_task_ids() -> [id]`](DshApiClient::get_task_ids)
//! * [`get_task_status(appid, id) -> AllocationStatus`](DshApiClient::get_task_status)
//! * [`put_application_configuration(appid, body)`](DshApiClient::put_application_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`application_ids() -> [application]`](DshApiClient::application_ids)
//! * [`application_ids_with_allocation_statuses() -> [application]`](DshApiClient::application_ids_with_allocation_statuses)
//! * [`applications() -> [application]`](DshApiClient::applications)
//! * [`applications_dependant_on_bucket(bucket_id) -> [application]`](DshApiClient::applications_dependant_on_bucket)
//! * [`applications_dependant_on_secret(secret_id) -> [application]`](DshApiClient::applications_dependant_on_secret)
//! * [`applications_dependant_on_scratch_topic(topic_id) -> [application]`](DshApiClient::applications_dependant_on_scratch_topic)
//! * [`applications_dependant_on_vhost(secret_id) -> [application]`](DshApiClient::applications_dependant_on_vhost)
//! * [`applications_dependant_on_volume(secret_id) -> [application]`](DshApiClient::applications_dependant_on_volume)
//! * [`applications_filtered(predicate) -> [(id, application)]`](DshApiClient::applications_filtered)
//! * [`applications_that_use_env_value(query) -> [(id, app, [env])]`](DshApiClient::applications_that_use_env_value)
//! * [`guid() -> (gid, uid)`](DshApiClient::guid)

use crate::app::app_resources;
use crate::application_types::ApplicationValues;
use crate::bucket::BucketInjection;
use crate::dsh_api_client::DshApiClient;
use crate::platform::CloudProvider;
use crate::query_processor::{Match, QueryProcessor};
use crate::secret::SecretInjection;
use crate::topic::TopicInjection;
use crate::types::{AllocationStatus, AppCatalogApp, AppCatalogAppResourcesValue, Application};
use crate::vhost::VhostInjection;
use crate::volume::VolumeInjection;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiError::Unexpected;
use crate::{bucket, secret, topic, vhost, volume, DependantApplication, DshApiResult};
use futures::future::{join, try_join_all};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// # Additional methods to manage applications
///
/// Module that contains derived methods to manage applications.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`application_ids() -> [application]`](DshApiClient::application_ids)
/// * [`application_ids_with_allocation_statuses() -> [application]`](DshApiClient::application_ids_with_allocation_statuses)
/// * [`applications() -> [application]`](DshApiClient::applications)
/// * [`applications_dependant_on_bucket(bucket_id) -> [application]`](DshApiClient::applications_dependant_on_bucket)
/// * [`applications_dependant_on_secret(secret_id) -> [application]`](DshApiClient::applications_dependant_on_secret)
/// * [`applications_dependant_on_scratch_topic(topic_id) -> [application]`](DshApiClient::applications_dependant_on_scratch_topic)
/// * [`applications_dependant_on_vhost(secret_id) -> [application]`](DshApiClient::applications_dependant_on_vhost)
/// * [`applications_dependant_on_volume(secret_id) -> [application]`](DshApiClient::applications_dependant_on_volume)
/// * [`applications_filtered(predicate) -> [(id, application)]`](DshApiClient::applications_filtered)
/// * [`applications_that_use_env_value(query) -> [(id, app, [env])]`](DshApiClient::applications_that_use_env_value)
/// * [`guid() -> (gid, uid)`](DshApiClient::guid)
impl DshApiClient {
  /// # Return all application ids
  ///
  /// If you also need the application configuration, use
  /// [`applications()`](DshApiClient::applications) instead.
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted application ids
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn application_ids(&self) -> DshApiResult<Vec<String>> {
    let mut application_ids: Vec<String> = self
      .get_application_configuration_map()
      .await?
      .keys()
      .map(|application_id| application_id.to_string())
      .collect();
    application_ids.sort();
    Ok(application_ids)
  }

  /// # List application ids with the corresponding allocation status
  ///
  /// # Returns
  /// * `Ok<Vec<(String, \[AllocationStatus\])>>` - list of application ids and allocation statuses
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn application_ids_with_allocation_statuses(&self) -> DshApiResult<Vec<(String, AllocationStatus)>> {
    let application_ids: Vec<String> = self.application_ids().await?;
    let allocation_statuses = try_join_all(application_ids.iter().map(|application_id| self.get_application_status(application_id.as_str()))).await?;
    Ok(application_ids.into_iter().zip(allocation_statuses).collect_vec())
  }

  /// # List all application configurations with their ids
  ///
  /// # Returns
  /// * `Ok<Vec<(String, \[Application\])>>` - list of application ids and configurations
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications(&self) -> DshApiResult<Vec<(String, Application)>> {
    self.applications_filtered(&|_| true).await
  }

  /// # Get all application that depend on a given bucket
  ///
  /// # Parameters
  /// * `bucket_id` - Identifies the requested bucket.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApplication>>` - usage.
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_dependant_on_bucket(&self, bucket_id: &str) -> DshApiResult<Vec<DependantApplication<BucketInjection>>> {
    let (applications, bucket_name) = match self.platform().cloud_provider() {
      CloudProvider::AWS => (self.get_application_configuration_map().await?, None),
      CloudProvider::Azure => {
        let (applications, bucket_name) = join(self.get_application_configuration_map(), self.bucket_name(bucket_id)).await;
        (applications?, bucket_name.ok())
      }
    };
    Ok(
      bucket::bucket_injections_from_applications(bucket_id, bucket_name.as_deref(), &applications)
        .into_iter()
        .map(|application_values| {
          DependantApplication::new(
            application_values.id.to_string(),
            application_values.application.instances,
            application_values.values,
          )
        })
        .collect_vec(),
    )
  }

  /// # Get all application that depend on a given secret
  ///
  /// # Parameters
  /// * `secret_id` - id of the requested secret
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApplication>>` - usage.
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_dependant_on_secret(&self, secret_id: &str) -> DshApiResult<Vec<DependantApplication<SecretInjection>>> {
    let applications = self.get_application_configuration_map().await?;
    Ok(
      secret::secret_env_vars_from_applications(secret_id, &applications)
        .iter()
        .map(|application_values| {
          DependantApplication::new(
            application_values.id.to_string(),
            application_values.application.instances,
            application_values
              .values
              .iter()
              .map(|env_var| SecretInjection::EnvVar(env_var.to_string()))
              .collect_vec(),
          )
        })
        .collect_vec(),
    )
  }

  /// # Get all application that depend on a given vhost
  ///
  /// # Parameters
  /// * `vhost` - id of the requested vhost
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApplication>>` - usage.
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_dependant_on_scratch_topic(&self, topic: &str) -> DshApiResult<Vec<DependantApplication<TopicInjection>>> {
    let applications = self.get_application_configuration_map().await?;
    Ok(
      topic::topic_used_in_applications(topic, &applications)
        .iter()
        .map(|(application_id, application)| DependantApplication::new(application_id.to_string(), application.instances, vec![TopicInjection::Topic(topic.to_string())]))
        .collect_vec(),
    )
  }

  /// # Get all application that depend on a given vhost
  ///
  /// # Parameters
  /// * `vhost` - id of the requested vhost
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApplication>>` - usage.
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_dependant_on_vhost(&self, vhost: &str) -> DshApiResult<Vec<DependantApplication<VhostInjection>>> {
    let applications = self.get_application_configuration_map().await?;
    Ok(
      vhost::vhost_port_mappings_from_applications(vhost, &applications)
        .iter()
        .map(|application_values| {
          DependantApplication::new(
            application_values.id.to_string(),
            application_values.application.instances,
            application_values
              .values
              .iter()
              .map(|(port, port_mapping)| VhostInjection::Vhost(port.to_string(), Some(port_mapping.to_string())))
              .collect_vec(),
          )
        })
        .collect_vec(),
    )
  }

  /// # Get all application that depend on a given volume
  ///
  /// # Parameters
  /// * `volume` - id of the requested volume
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApplication>>` - usage.
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_dependant_on_volume(&self, volume: &str) -> DshApiResult<Vec<DependantApplication<VolumeInjection>>> {
    let applications = self.get_application_configuration_map().await?;
    Ok(
      volume::volume_paths_from_applications(volume, &applications)
        .iter()
        .map(|application_values| {
          DependantApplication::new(
            application_values.id.to_string(),
            application_values.application.instances,
            application_values.values.iter().map(|path| VolumeInjection::Volume(path.to_string())).collect_vec(),
          )
        })
        .collect_vec(),
    )
  }

  /// # Find all applications that match a predicate
  ///
  /// # Parameters
  /// * `predicate` - predicate that will be used to filter the applications
  ///
  /// # Returns
  /// * `Ok<Vec<(String, \[Application\])>>` - list of id/application tuples
  ///   for which the predicate returned `true`, sorted by id
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn applications_filtered(&self, predicate: &dyn Fn(&Application) -> bool) -> DshApiResult<Vec<(String, Application)>> {
    let mut matching_applications: Vec<(String, Application)> = self
      .get_application_configuration_map()
      .await?
      .into_iter()
      .filter(|(_, application)| predicate(application))
      .collect_vec();
    matching_applications.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    Ok(matching_applications)
  }

  /// # Find application that use an environment variable value
  ///
  /// # Parameters
  /// * `query_process` - \[QueryProcessor\] that is matched against all environment variables of all applications
  ///
  /// # Returns
  /// * `Vec<(String, \[Application\], Vec<(String, Vec<\[Part\]>)>)>` - list of tuples
  ///   that describe the matched environment variables.
  ///   Each tuple consist of
  ///   * `String` - id of the application that contains the value,
  ///   * `Application` - the application data and
  ///   * `Vec<(String, Vec<\[Part\]>)>` - list of environment variable key/value pairs that matched the query
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  #[allow(clippy::type_complexity)]
  pub async fn applications_that_use_env_value(&self, query_processor: &dyn QueryProcessor) -> DshApiResult<Vec<(String, Application, Vec<(String, Match)>)>> {
    let mut matches: Vec<(String, Application, Vec<(String, Match)>)> = vec![];
    let applications: Vec<(String, Application)> = self.applications().await?;
    for (application_id, application) in applications {
      let mut matching_envs: Vec<(String, Match)> = vec![];
      for (key, value) in &application.env {
        if let Some(matching) = query_processor.matching(value.as_str()) {
          matching_envs.push((key.to_string(), matching));
        }
      }
      if !matching_envs.is_empty() {
        matching_envs.sort_by(|(env_a, _), (env_b, _)| env_a.cmp(env_b));
        matches.push((application_id, application, matching_envs));
      }
    }
    Ok(matches)
  }

  /// # Get group and user id for the tenant
  ///
  /// Read the group and user id for the current tenant. These values are read from a random
  /// application configuration file, so the method will fail if no applications are deployed.
  ///
  /// # Returns
  /// * `Ok<(gid, uid)>` - group and user id for the tenant, which are always the same vale
  /// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
  pub async fn guid(&self) -> DshApiResult<(usize, usize)> {
    lazy_static! {
      static ref GUID_REGEX: Regex = Regex::new(r"^([0-9]+):([0-9]+)$").unwrap();
    }
    match self.get_application_configuration_map().await?.iter().take(1).last() {
      Some((_, application)) => match GUID_REGEX.captures(application.user.as_str()) {
        Some(captures) => Ok((
          captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
          captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        )),
        None => Err(Unexpected(format!("illegal user value {}", application.user), None)),
      },
      None => Err(Unexpected("no applications deployed".to_string(), None)),
    }
  }
}

/// # Match environment variables in an application against query
///
/// # Parameters
/// * `query_processor` - \[QueryProcessor\] that is matched against all environment variables in
///   the application.
/// * `application` - Reference to the `Application`.
///
/// # Returns
/// * `Vec<(String, Match)>` - List of tuples that describe the matched environment variables.
///   Each tuple consist of
///   * `String` - Name of the matching environment variable.
///   * `Match` - Type of the match.
pub fn application_environment_variables(query_processor: &dyn QueryProcessor, application: &Application) -> Vec<(String, Match)> {
  let mut matching_envs: Vec<(String, Match)> = vec![];
  for (key, value) in &application.env {
    if let Some(matching) = query_processor.matching(value.as_str()) {
      matching_envs.push((key.to_string(), matching));
    }
  }
  matching_envs.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
  matching_envs
}

/// Get application resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the application resources from
///
/// # Returns
/// Either `None` when the `app` does not have any application resources,
/// or a `Some` that contains tuples describing the application resources:
/// * resource id
/// * reference to the `Application`
pub fn application_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Application)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Application(application) => Some(application),
    _ => None,
  })
}

/// # Match all environment variables in all applications against query
///
/// # Parameters
/// * `query_process` - \[QueryProcessor\] that is matched against all environment variables in
///   all applications.
/// * `applications` - Hashmap containing the applications.
///
/// # Returns
/// * `Vec<(String, Vec<\[Part\]>)>` - list of tuples that describe the matched environment variables.
///   Each tuple consist of
///   * `String` - name of the matching environment variable
///   * `Vec<\[Part\]>` - list of environment variable key/value pairs that matched the query
/// * `Err<\[DshApiError\]>` - when the request could not be processed by the DSH
pub fn applications_environment_variables<'a>(query_processor: &dyn QueryProcessor, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, (String, Match)>> {
  let mut matching_applications: Vec<ApplicationValues<'a, (String, Match)>> = vec![];
  for (application_id, application) in applications {
    let matches: Vec<(String, Match)> = application_environment_variables(query_processor, application);
    if !matches.is_empty() {
      matching_applications.push(ApplicationValues::new(application_id, application, matches));
    }
  }
  matching_applications.sort();
  matching_applications
}
