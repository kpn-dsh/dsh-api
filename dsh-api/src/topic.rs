//! # Manage Kafka topics
//!
//! Module that contains functions to manage Kafka topics.
//!
//! # API methods
//! * [`create_topic(id, configuration)`](DshApiClient::create_topic)
//! * [`delete_topic(id)`](DshApiClient::delete_topic)
//! * [`get_topic(id) -> topic_status`](DshApiClient::get_topic)
//! * [`get_topic_allocation_status(id) -> allocation_status`](DshApiClient::get_topic_allocation_status)
//! * [`get_topic_configuration(id) -> topic`](DshApiClient::get_topic_configuration)
//! * [`list_topic_ids() -> [id]`](DshApiClient::list_topic_ids)
//! * [`list_topics_with_usage() -> [id, [usage]]`](DshApiClient::list_topics_with_usage)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_topic_actual_configuration(id) -> topic`](DshApiClient::get_topic_actual_configuration)")]

use crate::app::find_apps_that_use_topic;
use crate::application::find_applications_that_use_topic;
use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, AppCatalogApp, Application, Topic, TopicStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection, UsedBy};
use futures::try_join;

/// # Manage Kafka topics
///
/// Module that contains functions to manage Kafka topics.
///
/// # API methods
/// * [`create_topic(id, configuration)`](DshApiClient::create_topic)
/// * [`delete_topic(id)`](DshApiClient::delete_topic)
/// * [`get_topic(id) -> topic_status`](DshApiClient::get_topic)
/// * [`get_topic_allocation_status(id) -> allocation_status`](DshApiClient::get_topic_allocation_status)
/// * [`get_topic_configuration(id) -> topic`](DshApiClient::get_topic_configuration)
/// * [`list_topic_ids() -> [id]`](DshApiClient::list_topic_ids)
/// * [`list_topics_with_usage() -> [id, [usage]]`](DshApiClient::list_topics_with_usage)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_topic_actual_configuration(id) -> topic`](DshApiClient::get_topic_actual_configuration)")]
impl DshApiClient<'_> {
  /// # Create topic
  ///
  /// API function: `PUT /allocation/{tenant}/topic/{id}/configuration`
  ///
  /// # Parameters
  /// * `topic_id` - name of the created topic
  /// * `configuration` - configuration for the created topic
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the topic has been successfully created)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_topic(&self, topic_id: &str, configuration: &Topic) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_topic_configuration_by_tenant_by_id(self.tenant_name(), topic_id, self.token(), configuration)
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Delete topic
  ///
  /// API function: `DELETE /allocation/{tenant}/topic/{id}/configuration`
  ///
  /// # Parameters
  /// * `topic_id` - name of the topic to delete
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the topic has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_topic(&self, topic_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_topic_configuration_by_tenant_by_id(self.tenant_name(), topic_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return topic
  ///
  /// API function: `GET /allocation/{tenant}/topic/{id}`
  ///
  /// # Parameters
  /// * `topic_id` - name of the requested topic
  ///
  /// # Returns
  /// * `Ok<`[`TopicStatus`]`>` - topic status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_topic(&self, topic_id: &str) -> DshApiResult<TopicStatus> {
    self
      .process(self.generated_client.get_topic_by_tenant_by_id(self.tenant_name(), topic_id, self.token()).await)
      .map(|(_, result)| result)
  }

  /// # Return topic allocation status
  ///
  /// API function: `GET /allocation/{tenant}/topic/{id}/status`
  ///
  /// # Parameters
  /// * `topic_id` - name of the requested topic
  ///
  /// # Returns
  /// * `Ok<`[`AllocationStatus`]`>` - topic allocation status
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_topic_allocation_status(&self, topic_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_topic_status_by_tenant_by_id(self.tenant_name(), topic_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return topic configuration
  ///
  /// API function: `GET /allocation/{tenant}/topic/{id}/configuration`
  ///
  /// # Parameters
  /// * `topic_id` - name of the requested topic
  ///
  /// # Returns
  /// * `Ok<`[`Topic`]`>` - topic configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_topic_configuration(&self, topic_id: &str) -> DshApiResult<Topic> {
    self
      .process(
        self
          .generated_client
          .get_topic_configuration_by_tenant_by_id(self.tenant_name(), topic_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return actual topic configuration
  ///
  /// API function: `GET /allocation/{tenant}/topic/{id}/actual`
  ///
  /// # Parameters
  /// * `topic_id` - name of the requested topic
  ///
  /// # Returns
  /// * `Ok<`[`Topic`]`>` - topic configuration
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_topic_actual_configuration(&self, topic_id: &str) -> DshApiResult<Topic> {
    self
      .process(
        self
          .generated_client
          .get_topic_actual_by_tenant_by_id(self.tenant_name(), topic_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return sorted list of topic names
  ///
  /// API function: `GET /allocation/{tenant}/topic`
  ///
  /// # Returns
  /// * `Ok<Vec<String>>` - list of topic names
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_topic_ids(&self) -> DshApiResult<Vec<String>> {
    let mut topic_ids: Vec<String> = self
      .process(self.generated_client.get_topic_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    topic_ids.sort();
    Ok(topic_ids)
  }

  /// # List all topics with usage
  ///
  /// Returns a list of all topics together with the apps and applications that use them.
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<UsedBy>>>` - list of tuples
  ///   containing the topic id and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_topics_with_usage(&self) -> DshApiResult<Vec<(String, Vec<UsedBy>)>> {
    let (topic_ids, applications, apps) = try_join!(self.list_topic_ids(), self.get_applications(), self.get_app_configurations())?;
    let mut topics_with_usage: Vec<(String, Vec<UsedBy>)> = vec![];
    for topic_id in &topic_ids {
      let mut used_bys = vec![];
      let application_usages: Vec<(String, &Application, Vec<Injection>)> = find_applications_that_use_topic(topic_id, &applications);
      for (application_id, application, injections) in application_usages {
        if !injections.is_empty() {
          used_bys.push(UsedBy::Application(application_id, application.instances, injections));
        }
      }
      let app_usages: Vec<(String, &AppCatalogApp, Vec<String>)> = find_apps_that_use_topic(topic_id, &apps);
      for (app_id, _, injections) in app_usages {
        if !injections.is_empty() {
          used_bys.push(UsedBy::App(app_id, injections));
        }
      }
      topics_with_usage.push((topic_id.clone(), used_bys));
    }
    Ok(topics_with_usage)
  }
}
