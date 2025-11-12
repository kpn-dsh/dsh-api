//! # Additional methods to manage topics
//!
//! Module that contains methods and functions to manage topics.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_topic_configuration(id)`](DshApiClient::delete_topic_configuration)
//! * [`get_topic(id) -> TopicStatus`](DshApiClient::get_topic)
//! * [`get_topic_actual(id) -> Topic`](DshApiClient::get_topic_actual)
//! * [`get_topic_configuration(id) -> Topic`](DshApiClient::get_topic_configuration)
//! * [`get_topic_ids() -> [id]`](DshApiClient::get_topic_ids)
//! * [`get_topic_status(id) -> AllocationStatus`](DshApiClient::get_topic_status)
//! * [`put_topic_configuration(id, body)`](DshApiClient::put_topic_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`topic_dependant_applications(topic id) -> [application id, instances, [injection]]`](DshApiClient::topic_dependant_applications)
//! * [`topic_dependant_apps(topic id) -> [app id, [resource]]`](DshApiClient::topic_dependant_apps)
//! * [`topic_dependants(topic id) -> [id, [dependant]]`](DshApiClient::topic_dependants)
//! * [`topics_with_dependant_applications() -> [topic id, [application id, instances, [injection]]]`](DshApiClient::topics_with_dependant_applications)
//! * [`topics_with_dependant_apps() -> [topic id, [app id, [resource]]]`](DshApiClient::topics_with_dependant_apps)
//! * [`topics_with_dependants() -> [topic id, [dependant]]`](DshApiClient::topics_with_dependants)

use crate::app::{app_resources, apps_that_use_topic};
use crate::application_types::ApplicationValues;
use crate::dsh_api_client::DshApiClient;
use crate::parse::TopicString;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application, Topic};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{Dependant, DependantApp, DependantApplication, DshApiResult};
use futures::try_join;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// # Describes an injection of a resource in an application
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TopicInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
  /// Topic injection, where the value is the name of the topic.
  #[serde(rename = "topic")]
  Topic(String),
}

impl Display for TopicInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TopicInjection::EnvVar(env_var) => write!(f, "{}", env_var),
      TopicInjection::Topic(variable) => write!(f, "{{ topic('{}') }}", variable),
    }
  }
}

/// # Additional method to manage Kafka topics
///
/// Module that contains methods to manage Kafka topics.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`topic_dependant_applications(topic id) -> [application id, instances, [injection]]`](DshApiClient::topic_dependant_applications)
/// * [`topic_dependant_apps(topic id) -> [app id, [resource]]`](DshApiClient::topic_dependant_apps)
/// * [`topic_dependants(topic id) -> [id, [dependant]]`](DshApiClient::topic_dependants)
/// * [`topics_with_dependant_applications() -> [topic id, [application id, instances, [injection]]]`](DshApiClient::topics_with_dependant_applications)
/// * [`topics_with_dependant_apps() -> [topic id, [app id, [resource]]]`](DshApiClient::topics_with_dependant_apps)
/// * [`topics_with_dependants() -> [topic id, [dependant]]`](DshApiClient::topics_with_dependants)
impl DshApiClient {
  /// # Returns dependants of a topic
  ///
  /// # Parameters
  /// * `topic_id` - Identifies the scratch topic.
  ///
  /// Returns a sorted list of all topics together with the applications and apps that use them.
  pub async fn topic_dependant_applications(&self, topic_id: &str) -> DshApiResult<Vec<DependantApplication<TopicInjection>>> {
    let application_configuration_map = self.get_application_configuration_map().await?;
    let mut dependant_applications = Vec::<DependantApplication<TopicInjection>>::new();
    for application in topic_env_vars_from_applications(topic_id, &application_configuration_map) {
      dependant_applications.push(DependantApplication::new(
        application.id.to_string(),
        application.application.instances,
        application
          .values
          .iter()
          .map(|(env_var, _)| TopicInjection::EnvVar(env_var.to_string()))
          .collect_vec(),
      ));
    }
    Ok(dependant_applications)
  }

  /// # Returns dependants of a topic
  ///
  /// # Parameters
  /// * `topic_id` - Identifies the scratch topic.
  ///
  /// Returns a sorted list of all topics together with the applications and apps that use them.
  pub async fn topic_dependant_apps(&self, topic_id: &str) -> DshApiResult<Vec<DependantApp>> {
    let appcatalogapp_configuration_map = self.get_appcatalogapp_configuration_map().await?;
    let mut dependant_apps = Vec::<DependantApp>::new();
    for (app_id, _, resource_ids) in apps_that_use_topic(topic_id, &appcatalogapp_configuration_map) {
      dependant_apps.push(DependantApp::new(
        app_id.to_string(),
        resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
      ));
    }
    Ok(dependant_apps)
  }

  /// # Returns dependants of a topic
  ///
  /// # Parameters
  /// * `topic_id` - Identifies the scratch topic.
  ///
  /// Returns a sorted list of all topics together with the applications and apps that use them.
  pub async fn topic_dependants(&self, topic_id: &str) -> DshApiResult<Vec<Dependant<TopicInjection>>> {
    let (application_configuration_map, appcatalogapp_configuration_map) = try_join!(self.get_application_configuration_map(), self.get_appcatalogapp_configuration_map())?;
    let mut dependants = Vec::<Dependant<TopicInjection>>::new();
    for application in topic_injections_from_applications(topic_id, &application_configuration_map) {
      dependants.push(Dependant::application(
        application.id.to_string(),
        application.application.instances,
        application.values.iter().cloned().collect_vec(),
      ));
    }
    for (app_id, _, resource_ids) in apps_that_use_topic(topic_id, &appcatalogapp_configuration_map) {
      dependants.push(Dependant::app(
        app_id.to_string(),
        resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
      ));
    }
    dependants.sort();
    Ok(dependants)
  }

  /// # Returns all topics with dependant applications
  ///
  /// Returns a sorted list of all topics together with the applications that use them.
  pub async fn topics_with_dependant_applications(&self) -> DshApiResult<Vec<(String, Vec<DependantApplication<TopicInjection>>)>> {
    let (topic_ids, application_configuration_map) = try_join!(self.get_topic_ids(), self.get_application_configuration_map())?;
    let mut topics = Vec::<(String, Vec<DependantApplication<TopicInjection>>)>::new();
    for topic_id in topic_ids {
      let mut dependant_applications: Vec<DependantApplication<TopicInjection>> = vec![];
      for application in topic_env_vars_from_applications(topic_id.as_str(), &application_configuration_map) {
        dependant_applications.push(DependantApplication::new(
          application.id.to_string(),
          application.application.instances,
          application
            .values
            .iter()
            .map(|(env_var, _)| TopicInjection::EnvVar(env_var.to_string()))
            .collect_vec(),
        ));
      }
      topics.push((topic_id, dependant_applications));
    }
    Ok(topics)
  }

  /// # Returns all topics with dependant apps
  ///
  /// Returns a sorted list of all topics together with the apps that use them.
  pub async fn topics_with_dependant_apps(&self) -> DshApiResult<Vec<(String, Vec<DependantApp>)>> {
    let (topic_ids, apps) = try_join!(self.get_topic_ids(), self.get_appcatalogapp_configuration_map())?;
    let mut topics = Vec::<(String, Vec<DependantApp>)>::new();
    for topic_id in topic_ids {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      for (app_id, _, resource_ids) in apps_that_use_topic(topic_id.as_str(), &apps) {
        dependant_apps.push(DependantApp::new(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      topics.push((topic_id, dependant_apps));
    }
    Ok(topics)
  }

  /// # Returns all topics with dependant applications and apps
  ///
  /// Returns a sorted list of all topics together with the applications and apps that use them.
  pub async fn topics_with_dependants(&self) -> DshApiResult<Vec<(String, Vec<Dependant<TopicInjection>>)>> {
    let (topic_ids, application_configuration_map, appcatalogapp_configuration_map) = try_join!(
      self.get_topic_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut topics = Vec::<(String, Vec<Dependant<TopicInjection>>)>::new();
    for topic_id in topic_ids {
      let mut dependants: Vec<Dependant<TopicInjection>> = vec![];
      for application in topic_env_vars_from_applications(topic_id.as_str(), &application_configuration_map) {
        dependants.push(Dependant::application(
          application.id.to_string(),
          application.application.instances,
          application
            .values
            .iter()
            .map(|(env_var, _)| TopicInjection::EnvVar(env_var.to_string()))
            .collect_vec(),
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_topic(topic_id.as_str(), &appcatalogapp_configuration_map) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      topics.push((topic_id, dependants));
    }
    Ok(topics)
  }
}

/// # Get application environment variables referencing topic
///
/// Get all environment variables referencing topic `topic_id` from an `Application`.
/// When the application does not reference the topic, an empty list will be returned.
///
/// # Parameters
/// * `topic_name` - Identifies the topic to look for.
/// * `application` - Reference to the `Application`.
///
/// # Returns
/// `Vec<&str>` - List of all environment variables referencing topic `topic_id`
///
/// The list is sorted by environment variable key.
pub fn topic_env_vars_from_application<'a>(topic_name: &str, application: &'a Application) -> Vec<(&'a str, TopicString<'a>)> {
  let mut env_var_keys: Vec<(&str, TopicString)> = application
    .env
    .iter()
    .filter_map(|(env_key, env_value)| {
      TopicString::try_from(env_value.as_str())
        .ok()
        .and_then(|topic| if topic.name() == topic_name { Some((env_key.as_str(), topic)) } else { None })
    })
    .collect_vec();
  env_var_keys.sort_by_key(|(env_key, _)| *env_key);
  env_var_keys
}

/// # Get applications environment variables referencing topics
///
/// Get all environment variables referencing topic `topic_id` from multiple `Application`s.
/// Applications are only included if they reference topic `topic_id` at least once.
///
/// # Parameters
/// * `topic_id` - Identifier of the topic to look for.
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// `Vec<ApplicationValue<&str>>` - List of application values containing:
/// * application id,
/// * application reference,
/// * sorted list of environment variable keys that reference topic `topic_id`.
///
/// The list is sorted by application id.
pub fn topic_env_vars_from_applications<'a>(topic_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, (&'a str, TopicString<'a>)>> {
  let mut application_injections: Vec<ApplicationValues<(&'a str, TopicString<'a>)>> = vec![];
  for (application_id, application) in applications {
    let environment_variable_keys = topic_env_vars_from_application(topic_id, application);
    if !environment_variable_keys.is_empty() {
      application_injections.push(ApplicationValues::new(application_id, application, environment_variable_keys));
    }
  }
  application_injections.sort();
  application_injections
}

/// # Get application environment variables referencing topic
///
/// Get all environment variables referencing topic `topic_id` from an `Application`.
/// When the application does not reference the topic, an empty list will be returned.
///
/// # Parameters
/// * `topic_name` - Identifies the topic to look for.
/// * `application` - Reference to the `Application`.
///
/// # Returns
/// `Vec<&str>` - List of all topic injections referencing topic `topic_id`
///
/// The list is sorted by environment variable key.
pub fn topic_injections_from_application(topic_name: &str, application: &Application) -> Vec<TopicInjection> {
  let mut topic_injections: Vec<TopicInjection> = vec![];
  for (env_key, env_value) in &application.env {
    if let Ok(topic) = TopicString::try_from(env_value.as_str()) {
      if topic.name() == topic_name {
        topic_injections.push(TopicInjection::EnvVar(env_key.to_string()))
      }
    }
  }
  for application_topic in &application.topics {
    if let Ok(topic) = TopicString::try_from(application_topic.as_str()) {
      if topic.name() == topic_name {
        topic_injections.push(TopicInjection::Topic(application_topic.to_owned()))
      }
    }
  }
  topic_injections.sort();
  topic_injections
}

/// # Get applications environment variables referencing topics
///
/// Get all environment variables referencing topic `topic_id` from multiple `Application`s.
/// Applications are only included if they reference topic `topic_id` at least once.
///
/// # Parameters
/// * `topic_id` - Identifier of the topic to look for.
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// `Vec<ApplicationValue<&str>>` - List of application values containing:
/// * application id,
/// * application reference,
/// * sorted list of environment variable keys that reference topic `topic_id`.
///
/// The list is sorted by application id.
pub fn topic_injections_from_applications<'a>(topic_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, TopicInjection>> {
  let mut application_injections: Vec<ApplicationValues<TopicInjection>> = vec![];
  for (application_id, application) in applications {
    let environment_variable_keys = topic_injections_from_application(topic_id, application);
    if !environment_variable_keys.is_empty() {
      application_injections.push(ApplicationValues::new(application_id, application, environment_variable_keys));
    }
  }
  application_injections.sort();
  application_injections
}

/// Get topic resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the topic resources from
///
/// # Returns
/// Either `None` when the `app` does not have any topic resources,
/// or a `Some` that contains tuples describing the topic resources:
/// * resource id
/// * reference to the `Topic`
pub fn topic_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Topic)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Topic(topic) => Some(topic),
    _ => None,
  })
}

/// # Check whether `topic_id` in used in an `Application`
///
/// # Parameters
/// * `topic_id` - id of the topic to look for
/// * `application` - reference to the `Application`
///
/// # Returns
/// * `true` - topic `topic_id` is used in `application`
/// * `false` - topic `topic_id` is not used in `application`
pub fn topic_used_in_application(topic_id: &str, application: &Application) -> bool {
  application.topics.iter().any(|id| id == topic_id)
}

/// # Get all `Applications` that use scratch topic
///
/// Get all `Applications` that use the scratch topic with `topic_id`.
///
/// # Parameters
/// * `topic_id` - id of the scratch topic to look for
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<(&str, &application)>` - list of tuples containing:
/// * application id
/// * application reference
///
/// The list is sorted by application id.
pub fn topic_used_in_applications<'a>(topic_id: &str, applications: &'a HashMap<String, Application>) -> Vec<(&'a str, &'a Application)> {
  let mut application_tuples: Vec<(&str, &Application)> = applications
    .iter()
    .filter_map(|(application_id, application)| if topic_used_in_application(topic_id, application) { Some((application_id.as_str(), application)) } else { None })
    .collect_vec();
  application_tuples.sort_by(|(id_a, _), (id_b, _)| id_a.cmp(id_b));
  application_tuples
}

/// # Get all scratch topic ids from an `Application`
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<&str>` - sorted list of the ids of the scratch topics used in `application`.
pub fn topics_from_application(application: &Application) -> Vec<&str> {
  application.topics.iter().map(|topic_id| topic_id.as_str()).collect_vec()
}

/// # Get scratch topic ids from applications
///
/// Get all scratch topic ids from all `Application`s.
/// Application will only be included of they contain at least one scratch topic.
///
/// # Parameters
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationValues<&str>>` - list of tuples containing:
/// * application id
/// * application reference
/// * lists of scratch topic ids used in the applications, sorted by id
///
/// The list will be sorted by application id.
pub fn topics_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<&str>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let mut application_topics = topics_from_application(application);
      if !application_topics.is_empty() {
        application_topics.sort();
        Some(ApplicationValues::new(application_id, application, application_topics))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}
