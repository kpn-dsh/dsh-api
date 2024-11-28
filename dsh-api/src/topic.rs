//! # Manage Kafka topics
//!
//! Module that contains functions to manage Kafka topics.
//!
//! # API Methods
//! * [`create_topic(topic_id, configuration)`](DshApiClient::create_topic)
//! * [`delete_topic(topic_id)`](DshApiClient::delete_topic)
//! * [`get_topic(topic_id) -> TopicStatus`](DshApiClient::get_topic)
//! * [`get_topic_allocation_status(topic_id) -> AllocationStatus`](DshApiClient::get_topic_allocation_status)
//! * [`get_topic_configuration(topic_id) -> Topic`](DshApiClient::get_topic_configuration)
//! * [`get_topic_ids() -> Vec<String>`](DshApiClient::get_topic_ids)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_topic_actual_configuration(topic_id) -> Topic`](DshApiClient::get_topic_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, Topic, TopicStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage Kafka topics
///
/// Module that contains functions to manage Kafka topics.
///
/// # API Methods
/// * [`create_topic(topic_id, configuration)`](DshApiClient::create_topic)
/// * [`delete_topic(topic_id)`](DshApiClient::delete_topic)
/// * [`get_topic(topic_id) -> TopicStatus`](DshApiClient::get_topic)
/// * [`get_topic_allocation_status(topic_id) -> AllocationStatus`](DshApiClient::get_topic_allocation_status)
/// * [`get_topic_configuration(topic_id) -> Topic`](DshApiClient::get_topic_configuration)
/// * [`get_topic_ids() -> Vec<String>`](DshApiClient::get_topic_ids)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_topic_actual_configuration(topic_id) -> Topic`](DshApiClient::get_topic_actual_configuration)")]
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
  pub async fn get_topic_ids(&self) -> DshApiResult<Vec<String>> {
    let mut topic_ids: Vec<String> = self
      .process(self.generated_client.get_topic_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|secret_ids| secret_ids.iter().map(|secret_id| secret_id.to_string()).collect())?;
    topic_ids.sort();
    Ok(topic_ids)
  }
}
