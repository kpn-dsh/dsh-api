//! # Manage applications
//!
//! Module that contains a function to manage applications.
//!
//! ## API methods
//! * [`create_application(application_id, application)`](DshApiClient::create_application)
//! * [`delete_application(application_id)`](DshApiClient::delete_application)
//! * [`find_application_ids_with_derived_tasks() -> Vec<String>`](DshApiClient::find_application_ids_with_derived_tasks)
//! * [`get_application(application_id) -> Application`](DshApiClient::get_application)
//! * [`get_application_allocation_status(application_id) -> AllocationStatus`](DshApiClient::get_application_allocation_status)
//! * [`get_application_task(application_id, task_id) -> TaskStatus`](DshApiClient::get_application_task)
//! * [`get_application_task_allocation_status(application_id, task_id) -> AllocationStatus`](DshApiClient::get_application_task_allocation_status)
//! * [`get_applications() -> HashMap<String, Application>`](DshApiClient::get_applications)
//! * [`list_application_derived_task_ids(application_id) -> Vec<TaskId>`](DshApiClient::list_application_derived_task_ids)
//!
//! ## Utility methods
//! * [`find_applications(predicate) -> Vec<(String, Application)>`](DshApiClient::find_applications)
//! * [`find_applications_that_use_env_value(query) -> Vec<String>`](DshApiClient::find_applications_that_use_env_value)
//! * [`find_applications_with_secret_injection(secret) -> Vec<String>`](DshApiClient::find_applications_with_secret_injection)
//! * [`list_application_allocation_statuses() -> Vec<(String, AllocationStatus)>`](DshApiClient::list_application_allocation_statuses)
//! * [`list_application_ids() -> Vec<String>`](DshApiClient::list_application_ids)
//! * [`list_applications() -> Vec<(String, Application)>`](DshApiClient::list_applications)
//! * [`list_applications_with_secret_injections() -> Vec<String>`](DshApiClient::list_applications_with_secret_injections)
//!
//! ## Utility functions
//! * [`application_diff(baseline, sample) -> ApplicationDiff`](DshApiClient::application_diff)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_application_actual(application_id) -> Application`](DshApiClient::get_application_actual)")]
#![cfg_attr(feature = "actual", doc = "* [`get_applications_actual() -> HashMap<String, Application>`](DshApiClient::get_applications_actual)")]
#![cfg_attr(feature = "actual", doc = "* [`get_application_task_state(application_id, task_id) -> Task`](DshApiClient::get_application_task_state)")]
use crate::dsh_api_client::DshApiClient;
use crate::query_processor::{Part, QueryProcessor};
#[cfg(feature = "actual")]
use crate::types::Task;
use crate::types::{AllocationStatus, Application, ApplicationSecret, ApplicationVolumes, HealthCheck, Metrics, PortMapping, TaskStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;
use futures::future::try_join_all;
use std::collections::HashMap;

/// # Manage applications
///
/// Module that contains a function to manage applications.
///
/// ## API methods
/// * [`create_application(application_id, application)`](DshApiClient::create_application)
/// * [`delete_application(application_id)`](DshApiClient::delete_application)
/// * [`find_application_ids_with_derived_tasks() -> Vec<String>`](DshApiClient::find_application_ids_with_derived_tasks)
/// * [`get_application(application_id) -> Application`](DshApiClient::get_application)
/// * [`get_application_allocation_status(application_id) -> AllocationStatus`](DshApiClient::get_application_allocation_status)
/// * [`get_application_task(application_id, task_id) -> TaskStatus`](DshApiClient::get_application_task)
/// * [`get_application_task_allocation_status(application_id, task_id) -> AllocationStatus`](DshApiClient::get_application_task_allocation_status)
/// * [`get_applications() -> HashMap<String, Application>`](DshApiClient::get_applications)
/// * [`list_application_derived_task_ids(application_id) -> Vec<TaskId>`](DshApiClient::list_application_derived_task_ids)
///
/// ## Utility methods
/// * [`find_applications(predicate) -> Vec<(String, Application)>`](DshApiClient::find_applications)
/// * [`find_applications_with_secret_injection(secret) -> Vec<String>`](DshApiClient::find_applications_with_secret_injection)
/// * [`list_application_allocation_statuses() -> Vec<(String, AllocationStatus)>`](DshApiClient::list_application_allocation_statuses)
/// * [`list_application_ids() -> Vec<String>`](DshApiClient::list_application_ids)
/// * [`list_applications() -> Vec<(String, Application)>`](DshApiClient::list_applications)
/// * [`list_applications_with_secret_injections() -> Vec<String>`](DshApiClient::list_applications_with_secret_injections)
///
/// ## Utility functions
/// * [`application_diff(baseline, sample) -> ApplicationDiff`](DshApiClient::application_diff)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_application_actual(application_id) -> Application`](DshApiClient::get_application_actual)")]
#[cfg_attr(feature = "actual", doc = "* [`get_applications_actual() -> HashMap<String, Application>`](DshApiClient::get_applications_actual)")]
#[cfg_attr(feature = "actual", doc = "* [`get_application_task_state(application_id, task_id) -> Task`](DshApiClient::get_application_task_state)")]
impl DshApiClient<'_> {
  /// # Create application
  ///
  /// API function: `PUT /allocation/{tenant}/application/{appid}/configuration`
  ///
  /// ## Parameters
  /// * `application_id` - application name used when deploying the application
  /// * `configuration` - configuration used when deploying the application
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the application has been successfully
  ///              deployed)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_application(&self, application_id: &str, configuration: Application) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_application_configuration_by_tenant_by_appid(self.tenant_name(), application_id, self.token(), &configuration)
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Delete application
  ///
  /// API function: `DELETE /allocation/{tenant}/application/{appid}/configuration`
  ///
  /// ## Parameters
  /// * `application_id` - application name of the application to undeploy
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the application has been successfully
  ///              undeployed)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_application(&self, application_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_application_configuration_by_tenant_by_appid(self.tenant_name(), application_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return configuration of deployed application
  ///
  /// API function: `GET /allocation/{tenant}/application/{appid}/actual`
  ///
  /// ## Parameters
  /// * `application_id` - application id of the requested application
  ///
  /// ## Returns
  /// * `Ok<`[`Application`]`>` - application configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_application_actual(&self, application_id: &str) -> DshApiResult<Application> {
    self
      .process(
        self
          .generated_client
          .get_application_actual_by_tenant_by_appid(self.tenant_name(), application_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return all deployed applications with their configurations
  ///
  /// API function: `GET /allocation/{tenant}/application/actual`
  ///
  /// ## Returns
  /// * `Ok<HashMap<String, `[`Application`]`>>` - hashmap containing the application configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_applications_actual(&self) -> DshApiResult<HashMap<String, Application>> {
    self
      .process(self.generated_client.get_application_actual_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
  }

  /// # Return allocation status of application
  ///
  /// API function: `GET /allocation/{tenant}/application/{appid}/status`
  ///
  /// ## Parameters
  /// * `application_id` - application id of the requested application
  ///
  /// ## Returns
  /// * `Ok<`[`AllocationStatus`]`>` - application allocation status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_application_allocation_status(&self, application_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_application_status_by_tenant_by_appid(self.tenant_name(), application_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return application configuration
  ///
  /// API function: `GET /allocation/{tenant}/application/{appid}/configuration`
  ///
  /// ## Parameters
  /// * `application_id` - application id of the requested application
  ///
  /// ## Returns
  /// * `Ok<`[`Application`]`>` - application configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_application(&self, application_id: &str) -> DshApiResult<Application> {
    self
      .process(
        self
          .generated_client
          .get_application_configuration_by_tenant_by_appid(self.tenant_name(), application_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return all applications with their configuration
  ///
  /// API function: `GET /allocation/{tenant}/application/configuration`
  ///
  /// ## Returns
  /// * `Ok<HashMap<String, `[`Application`]`>>` - hashmap containing the application configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_applications(&self) -> DshApiResult<HashMap<String, Application>> {
    self
      .process(
        self
          .generated_client
          .get_application_configuration_by_tenant(self.tenant_name(), self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return all derived task ids for an application
  ///
  /// API function: `GET /allocation/{tenant}/task{appid}`
  ///
  /// ## Parameters
  /// * `application_id` - application name for which the tasks will be returned
  ///
  /// ## Returns
  /// * `Ok<Vec<String>>` - vector containing names of all derived tasks for the application
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_application_derived_task_ids(&self, application_id: &str) -> DshApiResult<Vec<String>> {
    let mut task_ids: Vec<String> = self
      .process(
        self
          .generated_client
          .get_task_by_tenant_by_appid(self.tenant_name(), application_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
      .map(|task_ids| task_ids.iter().map(|task_id| task_id.to_string()).collect())?;
    task_ids.sort();
    Ok(task_ids)
  }

  /// # Return ids of all applications that have derived tasks
  ///
  /// API function: `GET /allocation/{tenant}/task`
  ///
  /// ## Returns
  /// * `Ok<Vec<String>>` - vector containing names of all application that have derived tasks
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_application_ids_with_derived_tasks(&self) -> DshApiResult<Vec<String>> {
    let mut application_ids: Vec<String> = self
      .process(self.generated_client.get_task_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    application_ids.sort();
    Ok(application_ids)
  }

  /// # Return status of derived task
  ///
  /// API function: `GET /allocation/{tenant}/task{appid}/{id}`
  ///
  /// ## Parameters
  /// * `application_id` - application name of the requested application
  /// * `task_id` - id of the requested task
  ///
  /// ## Returns
  /// * `Ok<`[`TaskStatus`]`>` - application task status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_application_task(&self, application_id: &str, task_id: &str) -> DshApiResult<TaskStatus> {
    self
      .process(
        self
          .generated_client
          .get_task_by_tenant_by_appid_by_id(self.tenant_name(), application_id, task_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return task allocation status
  ///
  /// API function: `GET /allocation/{tenant}/task{appid}/{id}/status`
  ///
  /// ## Parameters
  /// * `application_id` - application name of the requested application
  /// * `task_id` - id of the requested task
  ///
  /// ## Returns
  /// * `Ok<`[`AllocationStatus`]`>` - application task allocation status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_application_task_allocation_status(&self, application_id: &str, task_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_task_status_by_tenant_by_appid_by_id(self.tenant_name(), application_id, task_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return task actual state
  ///
  /// API function: `GET /allocation/{tenant}/task{appid}/{id}/actual`
  ///
  /// ## Parameters
  /// * `application_id` - application name of the requested application
  /// * `task_id` - id of the requested task
  ///
  /// ## Returns
  /// * `Ok<`[`Task`]`>` - actual application task status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_application_task_state(&self, application_id: &str, task_id: &str) -> DshApiResult<Task> {
    self
      .process(
        self
          .generated_client
          .get_task_actual_by_tenant_by_appid_by_id(self.tenant_name(), application_id, task_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # List application ids with the corresponding allocation status
  ///
  /// ## Returns
  /// * `Ok<Vec<(String, `[`AllocationStatus`]`)>>` - list of application ids and allocation statuses
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_application_allocation_statuses(&self) -> DshApiResult<Vec<(String, AllocationStatus)>> {
    let application_ids: Vec<String> = self.list_application_ids().await?;
    let allocation_statuses = try_join_all(
      application_ids
        .iter()
        .map(|application_id| self.get_application_allocation_status(application_id.as_str())),
    )
    .await?;
    Ok(application_ids.into_iter().zip(allocation_statuses).collect::<Vec<_>>())
  }

  /// # List all application configurations with their ids
  ///
  /// ## Returns
  /// * `Ok<Vec<(String, `[`Application`]`)>>` - list of application ids and configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_applications(&self) -> DshApiResult<Vec<(String, Application)>> {
    self.find_applications(&|_| true).await
  }

  /// # Find all applications that match a predicate
  ///
  /// ## Parameters
  /// * `predicate` - predicate that will be used to filter the applications
  ///
  /// ## Returns
  /// * `Ok<Vec<(String, `[`Application`]`)>>` - list of id/application pairs, ordered by id,
  ///   for which the predicate returned `true`
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_applications(&self, predicate: &dyn Fn(&Application) -> bool) -> DshApiResult<Vec<(String, Application)>> {
    let mut matching_applications: Vec<(String, Application)> = self
      .get_applications()
      .await?
      .into_iter()
      .filter(|(_, application)| predicate(application))
      .collect::<Vec<_>>();
    matching_applications.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
    Ok(matching_applications)
  }

  /// # Return all application ids
  ///
  /// If you also need the application configuration, use
  /// [`list_applications()`](Self::list_applications) instead.
  ///
  /// ## Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted application ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_application_ids(&self) -> DshApiResult<Vec<String>> {
    let mut application_ids: Vec<String> = self.get_applications().await?.keys().map(|application_id| application_id.to_string()).collect();
    application_ids.sort();
    Ok(application_ids)
  }

  /// Get secret injections from an application
  ///
  /// ## Parameters
  /// * `application` - reference to the `Application`
  ///
  /// ## Returns
  /// * `Vec(<String, Vec<String>)>` - list of tuples that describe the secret injections.
  ///   Each tuple consist of the secret id and the environment variables
  ///   that the secret is injected into.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  fn all_secret_injections(application: &Application) -> Vec<(String, Vec<String>)> {
    let mut injections = Vec::<(String, Vec<String>)>::new();
    for application_secret in &application.secrets {
      let mut env_injections = vec![];
      for application_secret_injection in &application_secret.injections {
        if let Some(env_injection) = application_secret_injection.get("env") {
          env_injections.push(env_injection.to_string());
        }
      }
      if !env_injections.is_empty() {
        env_injections.sort();
        injections.push((application_secret.name.clone(), env_injections));
      }
    }
    injections.sort();
    injections
  }

  /// List applications with secret injections
  ///
  /// ## Returns
  /// * `Vec<(String, `[`Application`]`, Vec(<String, Vec<String>)>)>` - list of tuples
  ///   that describe the applications with secret injections.
  ///   Each tuple consist of the application id, the `Application` and a list of secret ids
  ///   with the environment variables that the secrets are injected into.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_applications_with_secret_injections(&self) -> DshApiResult<Vec<(String, Application, Vec<(String, Vec<String>)>)>> {
    Ok(
      self
        .find_applications(&|application| !application.secrets.is_empty())
        .await?
        .into_iter()
        .map(|(id, application)| {
          let injections = Self::all_secret_injections(&application);
          (id, application, injections)
        })
        .collect::<Vec<_>>(),
    )
  }

  /// Find application that use an environment variable value
  ///
  /// ## Parameters
  /// * `query_process` - `[`QueryProcessor`]` that is matched against all environment variables of all applications
  ///
  /// ## Returns
  /// * `Vec<(String, `[`Application`]`, Vec<(String, Vec<`[`Part`]`>)>)>` - list of tuples
  ///   that describe the matched environment variables.
  ///   Each tuple consist of
  ///   * `String` - id of the application that contains the value,
  ///   * `Application` - the application data and
  ///   * `Vec<(String, Vec<`[`Part`]`>)>` - list of environment variable key/value pairs that matched the query
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_applications_that_use_env_value(&self, query_processor: &dyn QueryProcessor) -> DshApiResult<Vec<(String, Application, Vec<(String, Vec<Part>)>)>> {
    #[allow(clippy::type_complexity)]
    let mut matches: Vec<(String, Application, Vec<(String, Vec<Part>)>)> = vec![];
    let applications: Vec<(String, Application)> = self.list_applications().await?;
    for (application_id, application) in applications {
      let mut matching_envs: Vec<(String, Vec<Part>)> = vec![];
      for (key, value) in &application.env {
        if let Some(parts) = query_processor.matching_parts(value.as_str()) {
          matching_envs.push((key.to_string(), parts));
        }
      }
      if !matching_envs.is_empty() {
        matching_envs.sort_by(|(env_a, _), (env_b, _)| env_a.cmp(env_b));
        matches.push((application_id, application, matching_envs));
      }
    }
    Ok(matches)
  }

  /// Find applications that use a secret injection
  ///
  /// ## Parameters
  /// * `secret` - the secret that is matched against all applications
  ///
  /// ## Returns
  /// * `Vec<(String, `[`Application`]`, Vec<String>)>` - list of tuples
  ///   that describe the applications with secret injections.
  ///   Each tuple consist of the application id, the `Application` and a list of
  ///   environment variables that the secrets are injected into.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_applications_with_secret_injection(&self, secret: &str) -> DshApiResult<Vec<(String, Application, Vec<String>)>> {
    Ok(
      self
        .find_applications(&|application| !application.secrets.is_empty())
        .await?
        .into_iter()
        .filter_map(|(id, application)| {
          Self::all_secret_injections(&application)
            .into_iter()
            .find(|(secret_id, _)| secret_id == secret)
            .map(|(_, a)| (id, application, a))
        })
        .collect::<Vec<_>>(),
    )
  }

  /// Find applications with secret injections
  ///
  /// ## Parameters
  /// * `application_id` - application name of the requested application
  /// * `task_id` - id of the requested task
  ///
  /// ## Returns
  /// * `Vec<(String, u64, HashMap<String, Vec<String>>)>` - application task allocation status
  pub fn applications_with_secrets_injections(secrets: &[String], applications: &HashMap<String, Application>) -> Vec<(String, u64, HashMap<String, Vec<String>>)> {
    let mut application_ids: Vec<String> = applications.keys().map(|p| p.to_string()).collect();
    application_ids.sort();
    let mut pairs: Vec<(String, u64, HashMap<String, Vec<String>>)> = vec![];
    for application_id in application_ids {
      let application = applications.get(&application_id).unwrap();
      if !application.secrets.is_empty() {
        let mut injections = HashMap::<String, Vec<String>>::new();
        for application_secret in &application.secrets {
          if secrets.contains(&application_secret.name) {
            let mut env_injections = vec![];
            for application_secret_injection in &application_secret.injections {
              if let Some(env_injection) = application_secret_injection.get("env") {
                env_injections.push(env_injection.to_string());
              }
            }
            if !env_injections.is_empty() {
              injections.insert(application_secret.name.clone(), env_injections);
            }
          }
        }
        if !injections.is_empty() {
          pairs.push((application_id.clone(), application.instances, injections));
        }
      }
    }
    pairs
  }

  /// # Compare Applications
  ///
  /// ## Parameters
  /// * `baseline` - baseline application to compare against
  /// * `sample` - sample application that will be compared against the baseline
  ///
  /// ## Returns
  /// * `[`ApplicationDiff`]` - struct that describes the differences between the two `[`Application`]`s
  pub fn application_diff(baseline: &Application, sample: &Application) -> ApplicationDiff {
    ApplicationDiff {
      cpus: if baseline.cpus == sample.cpus { None } else { Some((baseline.cpus, sample.cpus)) },
      env: if baseline.env == sample.env { None } else { Some((baseline.env.clone(), sample.env.clone())) },
      exposed_ports: if baseline.exposed_ports == sample.exposed_ports.clone() { None } else { Some((baseline.exposed_ports.clone(), sample.exposed_ports.clone())) },
      health_check: if baseline.health_check == sample.health_check { None } else { Some((baseline.health_check.clone(), sample.health_check.clone())) },
      image: if baseline.image == sample.image.clone() { None } else { Some((baseline.image.clone(), sample.image.clone())) },
      instances: if baseline.instances == sample.instances { None } else { Some((baseline.instances, sample.instances)) },
      mem: if baseline.mem == sample.mem { None } else { Some((baseline.mem, sample.mem)) },
      metrics: if baseline.metrics == sample.metrics { None } else { Some((baseline.metrics.clone(), sample.metrics.clone())) },
      needs_token: if baseline.needs_token == sample.needs_token { None } else { Some((baseline.needs_token, sample.needs_token)) },
      readable_streams: if baseline.readable_streams == sample.readable_streams { None } else { Some((baseline.readable_streams.clone(), sample.readable_streams.clone())) },
      secrets: if baseline.secrets == sample.secrets { None } else { Some((baseline.secrets.clone(), sample.secrets.clone())) },
      single_instance: if baseline.single_instance == sample.single_instance { None } else { Some((baseline.single_instance, sample.single_instance)) },
      spread_group: if baseline.spread_group == sample.spread_group { None } else { Some((baseline.spread_group.clone(), sample.spread_group.clone())) },
      topics: if baseline.topics == sample.topics { None } else { Some((baseline.topics.clone(), sample.topics.clone())) },
      user: if baseline.user == sample.user { None } else { Some((baseline.user.clone(), sample.user.clone())) },
      volumes: if baseline.volumes == sample.volumes { None } else { Some((baseline.volumes.clone(), sample.volumes.clone())) },
      writable_streams: if baseline.writable_streams == sample.writable_streams { None } else { Some((baseline.writable_streams.clone(), sample.writable_streams.clone())) },
    }
  }
}

/// Structure that contains the differences between two `Application`s
#[derive(Debug)]
pub struct ApplicationDiff {
  pub cpus: Option<(f64, f64)>,
  pub env: Option<(HashMap<String, String>, HashMap<String, String>)>,
  pub exposed_ports: Option<(HashMap<String, PortMapping>, HashMap<String, PortMapping>)>,
  pub health_check: Option<(Option<HealthCheck>, Option<HealthCheck>)>,
  pub image: Option<(String, String)>,
  pub instances: Option<(u64, u64)>,
  pub mem: Option<(u64, u64)>,
  pub metrics: Option<(Option<Metrics>, Option<Metrics>)>,
  pub needs_token: Option<(bool, bool)>,
  pub readable_streams: Option<(Vec<String>, Vec<String>)>,
  pub secrets: Option<(Vec<ApplicationSecret>, Vec<ApplicationSecret>)>,
  pub single_instance: Option<(bool, bool)>,
  pub spread_group: Option<(Option<String>, Option<String>)>,
  pub topics: Option<(Vec<String>, Vec<String>)>,
  pub user: Option<(String, String)>,
  pub volumes: Option<(HashMap<String, ApplicationVolumes>, HashMap<String, ApplicationVolumes>)>,
  pub writable_streams: Option<(Vec<String>, Vec<String>)>,
}

impl ApplicationDiff {
  /// Check if there are any differences
  ///
  /// ## Returns
  /// * `true` - struct does not contain any differences
  /// * `false` - struct does contain differences
  pub fn is_empty(&self) -> bool {
    self.cpus.is_none()
      && self.env.is_none()
      && self.exposed_ports.is_none()
      && self.health_check.is_none()
      && self.image.is_none()
      && self.instances.is_none()
      && self.mem.is_none()
      && self.metrics.is_none()
      && self.needs_token.is_none()
      && self.readable_streams.is_none()
      && self.secrets.is_none()
      && self.single_instance.is_none()
      && self.spread_group.is_none()
      && self.topics.is_none()
      && self.user.is_none()
      && self.volumes.is_none()
      && self.writable_streams.is_none()
  }

  /// List the differences
  ///
  /// If there are no differences, an empty list will be returned.
  ///
  /// ## Returns
  /// * `Vec<(String, String)>` - list of key/value pairs describing all differences
  pub fn differences(&self) -> Vec<(String, String)> {
    vec![
      self.env.as_ref().map(|value| ("env".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .exposed_ports
        .as_ref()
        .map(|value| ("exposed ports".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .health_check
        .as_ref()
        .map(|value| ("healt check".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.image.as_ref().map(|value| ("image".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .instances
        .map(|value| ("number of instances".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.mem.map(|value| ("memory".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.metrics.as_ref().map(|value| ("metrics".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.needs_token.map(|value| ("needs token".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .readable_streams
        .as_ref()
        .map(|value| ("readable streams".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.secrets.as_ref().map(|value| ("secrets".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .single_instance
        .map(|value| ("single instance".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .spread_group
        .as_ref()
        .map(|value| ("spread group".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.topics.as_ref().map(|value| ("topics".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.user.as_ref().map(|value| ("user".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self.volumes.as_ref().map(|value| ("volumes".to_string(), format!("{:?} / {:?}", value.0, value.1))),
      self
        .writable_streams
        .as_ref()
        .map(|value| ("writable streams".to_string(), format!("{:?} / {:?}", value.0, value.1))),
    ]
    .iter()
    .flatten()
    .collect::<Vec<_>>()
    .iter()
    .map(|p| p.to_owned().to_owned())
    .collect::<Vec<_>>()
  }
}
