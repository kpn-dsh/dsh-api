//! # Additional methods to manage proxies
//!
//! Module that contains methods and functions to manage proxies.
//!
//! # Generated methods
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_kafkaproxy_configuration(id)`](DshApiClient::delete_kafkaproxy_configuration)
//! * [`get_kafkaproxy_actual(id) -> KafkaProxy`](DshApiClient::get_kafkaproxy_actual)
//! * [`get_kafkaproxy_configuration(id) -> KafkaProxy`](DshApiClient::get_kafkaproxy_configuration)
//! * [`get_kafkaproxy_ids() -> Vec<String>`](DshApiClient::get_kafkaproxy_ids)
//! * [`get_kafkaproxy_status(id) -> AllocationStatus`](DshApiClient::get_kafkaproxy_status)
//! * [`put_kafkaproxy_configuration(id, proxy)`](DshApiClient::put_kafkaproxy_configuration)
//!
//! # Derived methods
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`proxies() -> Vec<(String, KafkaProxy)>`](DshApiClient::proxies)
//! * [`proxies_dependant_on_certificate(id) -> Vec<DependantProxy>`](DshApiClient::proxies_dependant_on_certificate)
//! * [`proxies_dependant_on_secret(id) -> Vec<DependantProxy>`](DshApiClient::proxies_dependant_on_secret)

use crate::dsh_api_client::DshApiClient;
use crate::types::KafkaProxy;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DependantProxy, DshApiResult};
use futures::future::try_join_all;
use itertools::Itertools;

/// # Additional methods to manage proxies
///
/// Module that contains methods and functions to manage proxies.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`proxies() -> Vec<(String, KafkaProxy)>`](DshApiClient::proxies)
/// * [`proxies_dependant_on_certificate(id) -> Vec<DependantProxy>`](DshApiClient::proxies_dependant_on_certificate)
/// * [`proxies_dependant_on_secret(id) -> Vec<DependantProxy>`](DshApiClient::proxies_dependant_on_secret)
impl DshApiClient {
  /// # Return proxy configurations
  ///
  /// # Returns
  /// * `Ok<(String, KafkaProxy)>` - Tuple containing the proxy ids and proxy configurations.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn proxies(&self) -> DshApiResult<Vec<(String, KafkaProxy)>> {
    let proxy_ids = self.get_kafkaproxy_ids().await?;
    let proxies = try_join_all(proxy_ids.iter().map(|proxy_id| self.get_kafkaproxy_configuration(proxy_id))).await?;
    Ok(proxy_ids.into_iter().zip(proxies).collect_vec())
  }

  /// # Get all proxies that depend on a certificate
  ///
  /// # Parameters
  /// * `certificate_id` - Identifier of the certificate.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the certificate.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn proxies_dependant_on_certificate(&self, certificate_id: &str) -> DshApiResult<Vec<DependantProxy>> {
    Ok(
      proxies_that_use_certificate(certificate_id, &self.proxies().await?)
        .into_iter()
        .map(|(proxy_id, proxy)| DependantProxy::new(proxy_id.to_string(), proxy.instances.get()))
        .collect_vec(),
    )
  }

  /// # Get all apps that depend on a secret
  ///
  /// # Parameters
  /// * `secret_id` - Identifier of the secret.
  ///
  /// # Returns
  /// * `Ok<Vec<DependantApp>>` - Apps that depend on the secret.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn proxies_dependant_on_secret(&self, secret_id: &str) -> DshApiResult<Vec<DependantProxy>> {
    Ok(
      proxies_that_use_secret(secret_id, &self.proxies().await?)
        .into_iter()
        .map(|(proxy_id, proxy)| DependantProxy::new(proxy_id.to_string(), proxy.instances.get()))
        .collect_vec(),
    )
  }
}

/// Find proxies that use a certificate
///
/// # Parameters
/// * `certificate_id` - Identifier of the certificate to look for.
/// * `proxies` - List of all proxies.
///
/// # Returns
/// `Vec<(proxy_id, proxy)>` - Vector of proxies that use the certificate:
/// * `proxy_id` - Proxy id of the proxy that uses the certificates.
/// * `proxy` - Reference to the proxy.
pub fn proxies_that_use_certificate<'a>(certificate_id: &str, proxies: &'a [(String, KafkaProxy)]) -> Vec<(&'a str, &'a KafkaProxy)> {
  proxies
    .iter()
    .filter(|(_, proxy)| proxy.certificate == certificate_id)
    .map(|(proxy_id, proxy)| (proxy_id.as_str(), proxy))
    .collect_vec()
}

/// Find proxies that use a secret
///
/// # Parameters
/// * `secret_id` - Identifier of the secret to look for.
/// * `proxies` - List of all proxies.
///
/// # Returns
/// `Vec<(proxy_id, proxy)>` - Vector of proxies that use the secret:
/// * `proxy_id` - Proxy id of the proxy that uses the secret.
/// * `proxy` - Reference to the proxy.
pub fn proxies_that_use_secret<'a>(secret_id: &str, proxies: &'a [(String, KafkaProxy)]) -> Vec<(&'a str, &'a KafkaProxy)> {
  proxies
    .iter()
    .filter(|(_, proxy)| proxy.secret_name_ca_chain == secret_id)
    .map(|(proxy_id, proxy)| (proxy_id.as_str(), proxy))
    .collect_vec()
}
