//! # Manage proxies
//!
//! Module that contains functions to manage Kafka proxies.
//!
//! * [`delete_proxy(proxy_id) -> ()`](DshApiClient::delete_proxy)
//! * [`get_proxy(proxy_id) -> Proxy`](DshApiClient::get_proxy)
//! * [`get_proxy_ids() -> Vec<String>`](DshApiClient::get_proxy_ids)
//! * [`update_proxy(proxy_id, proxy) -> ()`](DshApiClient::update_proxy)

use crate::dsh_api_client::DshApiClient;
#[allow(unused_imports)]
use crate::types::Empty;
use crate::types::KafkaProxy;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;

/// # Manage proxies
///
/// Module that contains functions to manage Kafka proxies.
///
/// * [`delete_proxy(proxy_id) -> ()`](DshApiClient::delete_proxy)
/// * [`get_proxy(proxy_id) -> Proxy`](DshApiClient::get_proxy)
/// * [`get_proxy_ids() -> Vec<String>`](DshApiClient::get_proxy_ids)
/// * [`update_proxy(proxy_id, proxy) -> ()`](DshApiClient::update_proxy)
impl DshApiClient<'_> {
  /// # Delete proxy
  ///
  /// API function: `DELETE /allocation/{tenant}/kafkaproxy/{id}/configuration`
  ///
  /// ## Parameters
  /// * `proxy_id` - id of the proxy to delete
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the proxy has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_proxy(&self, proxy_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_kafkaproxy_configuration_by_tenant_by_id(self.tenant_name(), proxy_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return proxy
  ///
  /// API function: `GET /allocation/{tenant}/kafkaproxy/{id}/configuration`
  ///
  /// ## Parameters
  /// * `proxy_id` - id of the requested proxy
  ///
  /// ## Returns
  /// * `Ok<KafkaProxy>` - proxy
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_proxy(&self, proxy_id: &str) -> DshApiResult<KafkaProxy> {
    self
      .process(
        self
          .generated_client
          .get_kafkaproxy_configuration_by_tenant_by_id(self.tenant_name(), proxy_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return sorted list of Kafka proxy ids
  ///
  /// API function: `GET /allocation/{tenant}/kafkaproxy`
  ///
  /// ## Returns
  /// * `Ok<Vec<String>>` - list of proxy ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_proxy_ids(&self) -> DshApiResult<Vec<String>> {
    let mut proxy_ids: Vec<String> = self
      .process(self.generated_client.get_kafkaproxy_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|proxy_ids| proxy_ids.iter().map(|proxy_id| proxy_id.to_string()).collect())?;
    proxy_ids.sort();
    Ok(proxy_ids)
  }

  /// # Update proxy configuration
  ///
  /// API function: `PUT /allocation/{tenant}/kafkaproxy/{id}/configuration`
  ///
  /// ## Parameters
  /// * `proxy_id` - id of the proxy to update
  /// * `proxy` - new configuration of the proxy
  ///
  /// ## Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the proxy has been successfully updated)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn update_proxy(&self, proxy_id: &str, proxy: KafkaProxy) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_kafkaproxy_configuration_by_tenant_by_id(self.tenant_name(), proxy_id, self.token(), &proxy)
          .await,
      )
      .map(|(_, result)| result)
  }
}
