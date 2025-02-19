//! # Additional method to manage Kafka topics
//!
//! Module that contains methods to manage Kafka topics.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`list_topics_with_usage() -> [id, [usage]]`](DshApiClient::list_topics_with_usage)

use crate::app::find_apps_that_use_topic;
use crate::application::find_applications_that_use_topic;
use crate::dsh_api_client::DshApiClient;
use crate::types::{AppCatalogApp, Application};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, Injection, UsedBy};
use futures::try_join;

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
/// * [`list_topics_with_usage() -> [id, [usage]]`](DshApiClient::list_topics_with_usage)
impl DshApiClient {
  /// # List all topics with usage
  ///
  /// Returns a list of all topics together with the apps and applications that use them.
  ///
  /// # Returns
  /// * `Ok<Vec<(String, Vec<UsedBy>>>` - list of tuples
  ///   containing the topic id and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_topics_with_usage(&self) -> DshApiResult<Vec<(String, Vec<UsedBy>)>> {
    let (topic_ids, applications, apps) = try_join!(
      self.get_topic_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
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
