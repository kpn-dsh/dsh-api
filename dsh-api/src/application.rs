//! # Additional methods and functions to manage applications
//!
//! Module that contains methods and functions to manage applications.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//! * Functions - Functions that add extra capabilities but do not depend directly on the API.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`find_applications(predicate) -> [(id, application)]`](DshApiClient::find_applications)
//! * [`find_applications_that_use_env_value(query) -> [(id, application, envs)]`](DshApiClient::find_applications_that_use_env_value)
//! * [`find_applications_with_secrets(secret) -> [(id, application, injections)]`](DshApiClient::find_applications_with_secrets)
//! * [`list_application_allocation_statuses() -> [(id, allocation_status)]`](DshApiClient::list_application_allocation_statuses)
//! * [`list_application_ids() -> [id]`](DshApiClient::list_application_ids)
//! * [`list_applications() -> [(id, application)]`](DshApiClient::list_applications)
//! * [`list_applications_with_secrets() -> [(id, application, injections)]`](DshApiClient::list_applications_with_secrets)
use crate::dsh_api_client::DshApiClient;
use crate::query_processor::{Part, QueryProcessor};
use crate::types::{AllocationStatus, Application, ApplicationSecret, ApplicationVolumes, HealthCheck, Metrics, PortMapping};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection};
use futures::future::try_join_all;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// # Additional methods and functions to manage applications
///
/// Module that contains methods and functions to manage applications.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`find_applications(predicate) -> [(id, application)]`](DshApiClient::find_applications)
/// * [`find_applications_that_use_env_value(query) -> [(id, application, envs)]`](DshApiClient::find_applications_that_use_env_value)
/// * [`find_applications_with_secrets(secret) -> [(id, application, injections)]`](DshApiClient::find_applications_with_secrets)
/// * [`list_application_allocation_statuses() -> [(id, allocation_status)]`](DshApiClient::list_application_allocation_statuses)
/// * [`list_application_ids() -> [id]`](DshApiClient::list_application_ids)
/// * [`list_applications() -> [(id, application)]`](DshApiClient::list_applications)
/// * [`list_applications_with_secrets() -> [(id, application, injections)]`](DshApiClient::list_applications_with_secrets)
impl DshApiClient {
  /// # List application ids with the corresponding allocation status
  ///
  /// # Returns
  /// * `Ok<Vec<(String, `[`AllocationStatus`]`)>>` - list of application ids and allocation statuses
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_application_allocation_statuses(&self) -> DshApiResult<Vec<(String, AllocationStatus)>> {
    let application_ids: Vec<String> = self.list_application_ids().await?;
    let allocation_statuses = try_join_all(application_ids.iter().map(|application_id| self.get_application_status(application_id.as_str()))).await?;
    Ok(application_ids.into_iter().zip(allocation_statuses).collect::<Vec<_>>())
  }

  /// # List all application configurations with their ids
  ///
  /// # Returns
  /// * `Ok<Vec<(String, `[`Application`]`)>>` - list of application ids and configurations
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_applications(&self) -> DshApiResult<Vec<(String, Application)>> {
    self.find_applications(&|_| true).await
  }

  /// # Find all applications that match a predicate
  ///
  /// # Parameters
  /// * `predicate` - predicate that will be used to filter the applications
  ///
  /// # Returns
  /// * `Ok<Vec<(String, `[`Application`]`)>>` - list of id/application pairs, ordered by id,
  ///   for which the predicate returned `true`
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_applications(&self, predicate: &dyn Fn(&Application) -> bool) -> DshApiResult<Vec<(String, Application)>> {
    let mut matching_applications: Vec<(String, Application)> = self
      .get_application_configuration_map()
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
  /// # Returns
  /// * `Ok<Vec<String>>` - vector containing the sorted application ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_application_ids(&self) -> DshApiResult<Vec<String>> {
    let mut application_ids: Vec<String> = self
      .get_application_configuration_map()
      .await?
      .keys()
      .map(|application_id| application_id.to_string())
      .collect();
    application_ids.sort();
    Ok(application_ids)
  }

  /// # List applications with secret injections
  ///
  /// # Returns
  /// * `Vec<(String, `[`Application`]`, Vec(<String, Vec<String>)>)>` - list of tuples
  ///   that describe the applications with secret injections.
  ///   Each tuple consist of the application id, the `Application` and a list of secret ids
  ///   with the environment variables that the secrets are injected into.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_applications_with_secrets(&self) -> DshApiResult<Vec<(String, Application, Vec<(String, Vec<Injection>)>)>> {
    Ok(
      self
        .find_applications(&|application| !application.secrets.is_empty())
        .await?
        .into_iter()
        .map(|(id, application)| {
          let injections = secrets_from_application(&application);
          (id, application, injections)
        })
        .collect::<Vec<_>>(),
    )
  }

  /// # Find application that use an environment variable value
  ///
  /// # Parameters
  /// * `query_process` - `[`QueryProcessor`]` that is matched against all environment variables of all applications
  ///
  /// # Returns
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

  /// # Find applications that use a secret injection
  ///
  /// # Parameters
  /// * `secret_id` - the secret that is matched against all applications
  ///
  /// # Returns
  /// * `Vec<(String, `[`Application`]`, Vec<String>)>` - list of tuples
  ///   that describe the applications with secret injections.
  ///   Each tuple consist of the application id, the `Application` and a map of
  ///   environment variables that the secret is injected into.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn find_applications_with_secrets(&self, secret_id: &str) -> DshApiResult<Vec<(String, Application, Vec<Injection>)>> {
    Ok(
      self
        .find_applications(&|application| !application.secrets.is_empty())
        .await?
        .into_iter()
        .filter_map(|(id, application)| {
          secrets_from_application(&application)
            .into_iter()
            .find(|(sid, _)| sid == secret_id)
            .map(|(_, a)| (id, application, a))
        })
        .collect::<Vec<_>>(),
    )
  }
}

/// # Compare Applications
///
/// # Parameters
/// * `baseline` - baseline application to compare against
/// * `sample` - sample application that will be compared against the baseline
///
/// # Returns
/// * `[`ApplicationDiff`]` - struct that describes the differences between the two `[`Application`]`s
pub fn differences_between_applications(baseline: &Application, sample: &Application) -> ApplicationDiff {
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

/// # Get all secret injections in `Application`
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// * `Vec(<String, Vec<Injection>)>` - list of tuples that describe the secret injections.
///   Each tuple consist of the secret id and the environment variables
///   that the secret is injected into.
pub fn secrets_from_application(application: &Application) -> Vec<(String, Vec<Injection>)> {
  let mut injections: Vec<(String, Vec<Injection>)> = vec![];
  for application_secret in &application.secrets {
    let mut env_injections: Vec<Injection> = vec![];
    for application_secret_injection in &application_secret.injections {
      if let Some(env_injection) = application_secret_injection.get("env") {
        env_injections.push(Injection::EnvVar(env_injection.to_string()));
      }
    }
    if !env_injections.is_empty() {
      injections.push((application_secret.name.clone(), env_injections));
    }
  }
  injections.sort_by(|(secret_a, _), (secret_b, _)| secret_a.cmp(secret_b));
  injections
}

/// # Find topic injections in `Application`s
///
/// # Parameters
/// * `topic_id` - id of the topic to look for
/// * `applications` - hashmap of all application id/application pairs
///
/// # Returns
/// * `Vec<(application_id, application, injections)>` - vector of applications that use the topic
///   * `application_id` - application id of the application that uses the secret
///   * `application` - reference to the application
///   * `injections` - injections of the secret used in the application
pub fn find_applications_that_use_topic<'a>(topic_id: &str, applications: &'a HashMap<String, Application>) -> Vec<(String, &'a Application, Vec<Injection>)> {
  let mut application_ids: Vec<String> = applications.keys().map(|p| p.to_string()).collect();
  application_ids.sort();
  let mut tuples: Vec<(String, &Application, Vec<Injection>)> = vec![];
  for application_id in application_ids {
    let application = applications.get(&application_id).unwrap();
    if !application.env.is_empty() {
      let mut envs_that_contain_topic_id: Vec<String> = application.env.clone().into_iter().filter(|(_, v)| v.contains(topic_id)).map(|(k, _)| k).collect();
      if !envs_that_contain_topic_id.is_empty() {
        envs_that_contain_topic_id.sort();
        tuples.push((
          application_id.clone(),
          application,
          envs_that_contain_topic_id.into_iter().map(Injection::EnvVar).collect::<Vec<_>>(),
        ));
      }
    }
  }
  tuples
}

/// # Get all vhost injections from `Application`
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<(String, Injection)>` - list of tuples that describe the vhost injections.
/// Each tuple consist of
/// * vhost id
/// * vhost injection.
pub fn vhosts_from_application(application: &Application) -> Vec<(String, Injection)> {
  let mut vhost_injections: Vec<(String, Injection)> = vec![];
  for (port, port_mapping) in &application.exposed_ports {
    if let Some(vhost_string) = port_mapping.vhost.clone() {
      if let Some((vhost, a_zone)) = parse_vhost_string(&vhost_string) {
        vhost_injections.push((vhost, Injection::Vhost(port.clone(), a_zone)));
      }
    }
  }
  vhost_injections
}

/// # Get all vhost injections from `Application`s
///
/// # Parameters
/// * `applications` - hashmap containing all applications
///
/// # Returns
/// `Vec<(String, Vec<Injection>)>` - list of tuples that describe the vhosts. Each tuple consists of
/// * application id
/// * reference to the `Application`
/// * list of tuples that describe the vhost injections
///   * vhost id
///   * vhost injection
#[allow(clippy::type_complexity)]
pub fn vhosts_from_applications(applications: &HashMap<String, Application>) -> Vec<(&String, &Application, Vec<(String, Injection)>)> {
  let mut application_injections: Vec<(&String, &Application, Vec<(String, Injection)>)> = vec![];
  for (application_id, application) in applications {
    let injections = vhosts_from_application(application);
    if !injections.is_empty() {
      application_injections.push((application_id, application, injections));
    }
  }
  application_injections.sort_by(|(id_a, _, _), (id_b, _, _)| id_a.cmp(id_b));
  application_injections
}

/// # Find secret injections in `Application`
///
/// # Parameters
/// * `secret_id` - id of the secret to look for
/// * `application` - reference to toe `Application`
///
/// # Returns
/// * `Vec<(Injection)>` - injections of the secret used in the application
pub fn get_secret_from_application(secret_id: &str, application: &Application) -> Vec<Injection> {
  let mut injections = Vec::<Injection>::new();
  for application_secret in &application.secrets {
    if secret_id == application_secret.name {
      for application_secret_injection in &application_secret.injections {
        if let Some(env_injection) = application_secret_injection.get("env") {
          injections.push(Injection::EnvVar(env_injection.to_string()));
        }
      }
    }
  }
  injections
}

/// # Find secret injections in `Application`s
///
/// # Parameters
/// * `secret_id` - id of the secret to look for
/// * `applications` - hashmap of all application id/application pairs
///
/// # Returns
/// * `Vec<(application_id, application, injections)>` - vector of applications that use the secret
///   * `application_id` - application id of the application that uses the secret
///   * `application` - reference to the application
///   * `injections` - injections of the secret used in the application
pub fn find_applications_that_use_secret<'a>(secret_id: &str, applications: &'a HashMap<String, Application>) -> Vec<(String, &'a Application, Vec<Injection>)> {
  let mut application_ids: Vec<String> = applications.keys().map(|p| p.to_string()).collect();
  application_ids.sort();
  let mut tuples: Vec<(String, &Application, Vec<Injection>)> = vec![];
  for application_id in application_ids {
    let application = applications.get(&application_id).unwrap();
    let injections = get_secret_from_application(secret_id, application);
    if !injections.is_empty() {
      tuples.push((application_id, application, injections));
    }
  }
  tuples
}

/// # Get applications that use any of a list of given secret injections
///
/// # Parameters
/// * `secret_ids` - ids of the secrets to look for
/// * `applications` - hashmap of all applications
///
/// # Returns
/// * `Vec<(application_id, application, injections)>` - vector of applications that use the secret
///   * `application_id` - application id of the application that uses the secret
///   * `application` - reference to the application
///   * `injections` - the injections of the secret in the application
#[allow(clippy::type_complexity)]
pub fn find_applications_that_use_secrets<'a>(
  secret_ids: &[String],
  applications: &'a HashMap<String, Application>,
) -> Vec<(String, &'a Application, HashMap<String, Vec<Injection>>)> {
  let mut application_ids: Vec<String> = applications.keys().map(|p| p.to_string()).collect();
  application_ids.sort();
  let mut tuples: Vec<(String, &Application, HashMap<String, Vec<Injection>>)> = vec![];
  for application_id in application_ids {
    let application = applications.get(&application_id).unwrap();
    if !application.secrets.is_empty() {
      let mut injections = HashMap::<String, Vec<Injection>>::new();
      for application_secret in &application.secrets {
        if secret_ids.contains(&application_secret.name) {
          let mut env_injections: Vec<Injection> = vec![];
          for application_secret_injection in &application_secret.injections {
            if let Some(env_injection) = application_secret_injection.get("env") {
              env_injections.push(Injection::EnvVar(env_injection.to_string()));
            }
          }
          if !env_injections.is_empty() {
            injections.insert(application_secret.name.clone(), env_injections);
          }
        }
      }
      if !injections.is_empty() {
        tuples.push((application_id, application, injections));
      }
    }
  }
  tuples
}

/// # Get applications that use a given volume injection
///
/// # Parameters
/// * `volume_id` - id of the volume to look for
/// * `applications` - hashmap of all applications
///
/// # Returns
/// * `Vec<(application_id, application, injections)>` - vector of applications that use the secret
///   * `application_id` - application id of the application that uses the secret
///   * `application` - reference to the application
///   * `injections` - injections of the secret used in the application
pub fn find_applications_that_use_volume<'a>(volume_id: &str, applications: &'a HashMap<String, Application>) -> Vec<(String, &'a Application, Vec<Injection>)> {
  let mut application_ids: Vec<String> = applications.keys().map(|p| p.to_string()).collect();
  application_ids.sort();
  let mut tuples: Vec<(String, &Application, Vec<Injection>)> = vec![];
  for application_id in application_ids {
    let application = applications.get(&application_id).unwrap();
    let mut injections = Vec::<Injection>::new();
    for (path, application_volume) in &application.volumes {
      if application_volume.name.contains(volume_id) {
        injections.push(Injection::Path(path.to_string()));
      }
    }
    if !injections.is_empty() {
      tuples.push((application_id, application, injections));
    }
  }
  tuples
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
  /// # Check if there are any differences
  ///
  /// # Returns
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

  /// # List the differences
  ///
  /// If there are no differences, an empty list will be returned.
  ///
  /// # Returns
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

fn a_zone(a_zone_string: String) -> Option<String> {
  if a_zone_string.contains("'private'") {
    Some("private".to_string())
  } else if a_zone_string.contains("'public'") {
    Some("public".to_string())
  } else {
    None
  }
}

lazy_static! {
  static ref VHOST_REGEX: Regex = Regex::new(r"\{\s*vhost\(\s*'([a-zA-Z0-9_\.-]+)'\s*(,\s*'([a-zA-Z0-9_-]+)')?\s*\)\s*\}").unwrap();
}

pub(crate) fn parse_vhost_string(vhost_string: &str) -> Option<(String, Option<String>)> {
  VHOST_REGEX.captures(vhost_string).map(|captures| {
    (
      captures.get(1).map(|m| m.as_str().to_string()).unwrap_or_default(),
      captures.get(2).and_then(|m| a_zone(m.as_str().to_string())),
    )
  })
}

lazy_static! {
  static ref APP_CATALOG_IMAGE_REGEX: Regex =
    Regex::new(r"APPCATALOG_REGISTRY/dsh-appcatalog/tenant/([a-z0-9-_]+)/([0-9]+)/([0-9]+)/(release|draft)/(klarrio|kpn)/([a-zA-Z0-9-_:.]+)").unwrap();
  static ref REGISTRY_IMAGE_REGEX: Regex = Regex::new(r"registry.cp.kpn-dsh.com/([a-z0-9-_]+)/([a-zA-Z0-9-_:.]+)").unwrap();
}

/// # Parses an image string
///
/// # Parameters
/// * `image_string` - the string to be parsed
///
/// # Returns
/// When the provided string is valid, the method returns a 2-tuple containing:
/// * registry of the image
/// * image id
pub fn parse_image_string(image_string: &str) -> Result<(String, String), String> {
  match APP_CATALOG_IMAGE_REGEX.captures(image_string) {
    Some(app_catalog_captures) => Ok((
      format!(
        "app:{}:{}",
        app_catalog_captures.get(4).map(|m| m.as_str().to_string()).unwrap_or_default(),
        app_catalog_captures.get(5).map(|m| m.as_str().to_string()).unwrap_or_default()
      ),
      app_catalog_captures.get(6).map(|m| m.as_str().to_string()).unwrap_or_default(),
    )),
    None => match REGISTRY_IMAGE_REGEX.captures(image_string) {
      Some(registry_captures) => Ok(("registry".to_string(), registry_captures.get(2).map(|m| m.as_str().to_string()).unwrap_or_default())),
      None => Err(format!("unrecognized image string {}", image_string)),
    },
  }
}

#[test]
fn test_app_catalog_image_draft_kpn() {
  const APP_CATALOG_IMAGE: &str = "APPCATALOG_REGISTRY/dsh-appcatalog/tenant/greenbox-dev/1903/1903/draft/kpn/schema-store-proxy:0.2.3-0";
  assert_eq!(
    parse_image_string(APP_CATALOG_IMAGE).unwrap(),
    ("app:draft:kpn".to_string(), "schema-store-proxy:0.2.3-0".to_string())
  );
}

#[test]
fn test_app_catalog_image_release_klarrio() {
  const APP_CATALOG_IMAGE: &str = "APPCATALOG_REGISTRY/dsh-appcatalog/tenant/greenbox-dev/1903/1903/release/klarrio/whoami:1.6.1";
  assert_eq!(
    parse_image_string(APP_CATALOG_IMAGE).unwrap(),
    ("app:release:klarrio".to_string(), "whoami:1.6.1".to_string())
  );
}

#[test]
fn test_registry_image() {
  const REGISTRY_IMAGE: &str = "registry.cp.kpn-dsh.com/greenbox-dev/cck-ingestor:0.0.18";
  assert_eq!(
    parse_image_string(REGISTRY_IMAGE).unwrap(),
    ("registry".to_string(), "cck-ingestor:0.0.18".to_string())
  );
}
