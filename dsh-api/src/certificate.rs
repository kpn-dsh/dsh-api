//! # Additional methods to manage certificates
//!
//! Module that contains methods and functions to manage certificates.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_certificate_configuration(id)`](DshApiClient::delete_certificate_configuration)
//! * [`get_certificate(id) -> CertificateStatus`](DshApiClient::get_certificate)
//! * [`get_certificate_actual(id) -> Certificate`](DshApiClient::get_certificate_actual)
//! * [`get_certificate_configuration(id) -> Certificate`](DshApiClient::get_certificate_configuration)
//! * [`get_certificate_ids() -> [id]`](DshApiClient::get_certificate_ids)
//! * [`get_certificate_status(id) -> AllocationStatus`](DshApiClient::get_certificate_status)
//! * [`put_certificate_configuration(id, body)`](DshApiClient::put_certificate_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`certificate_with_dependant_apps(certificate id) -> (certificate status, [app])`](DshApiClient::certificate_with_dependant_apps)
//! * [`certificates_with_dependant_apps() -> [(certificate id, certificate status, [app])]`](DshApiClient::certificates_with_dependant_apps)

use crate::app::app_resources;
use crate::dsh_api_client::DshApiClient;
use crate::secret::secrets_resources_from_apps;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Certificate, CertificateStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DependantApp, DshApiResult};
use futures::future::try_join_all;
use futures::try_join;
use std::collections::HashMap;

/// # Additional methods to manage certificates
///
/// Module that contains methods to manage certificates.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`certificate_with_dependant_apps(certificate id) -> (certificate status, [app])`](DshApiClient::certificate_with_dependant_apps)
/// * [`certificates_with_dependant_apps() -> [(certificate id, certificate status, [app])]`](DshApiClient::certificates_with_dependant_apps)
impl DshApiClient {
  /// # Return certificate with usage
  ///
  /// Returns the certificate configuration for the provided certificate id,
  /// together with the apps and applications that use this certificate.
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<(CertificateStatus, Vec<UsedBy>>` - tuple containing the certificate configuration
  ///   and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificate_with_dependant_apps(&self, certificate_id: &str) -> DshApiResult<(CertificateStatus, Vec<DependantApp>)> {
    let (certificate_status, appcatalogapp_configuration_map): (CertificateStatus, HashMap<String, AppCatalogApp>) =
      try_join!(self.get_certificate(certificate_id), self.get_appcatalogapp_configuration_map())?;
    let mut dependants: Vec<DependantApp> = vec![];
    if let Some(ref configuration) = certificate_status.configuration {
      let secrets = match configuration.passphrase_secret {
        Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
        None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
      };
      for (app_id, _, secret_resources) in secrets_resources_from_apps(&secrets, &appcatalogapp_configuration_map) {
        dependants.push(DependantApp::new(app_id.clone(), secret_resources));
      }
    }
    Ok((certificate_status, dependants))
  }

  /// # List all certificates with usage
  ///
  /// Returns a list of all certificate configurations,
  /// together with the apps and applications that use this certificate.
  ///
  /// # Returns
  /// * `Ok<Vec<(String, CertificateStatus, Vec<UsedBy>>>` - list of tuples
  ///   containing the certificate id, certificate configuration and a vector of usages,
  ///   which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificates_with_dependant_apps(&self) -> DshApiResult<Vec<(String, CertificateStatus, Vec<DependantApp>)>> {
    let certificate_ids = self.get_certificate_ids().await?;
    let certificates = try_join_all(certificate_ids.iter().map(|certificate_id| self.get_certificate(certificate_id.as_str()))).await?;
    let apps = self.get_appcatalogapp_configuration_map().await?;
    let mut certificates_with_usage: Vec<(String, CertificateStatus, Vec<DependantApp>)> = vec![];
    for (certificate_id, certificate_status) in certificate_ids.iter().zip(certificates) {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      if let Some(ref configuration) = certificate_status.configuration {
        let secrets = match configuration.passphrase_secret {
          Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
          None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
        };
        for (app_id, _, secret_resources) in secrets_resources_from_apps(&secrets, &apps) {
          dependant_apps.push(DependantApp::new(app_id.clone(), secret_resources));
        }
      }
      certificates_with_usage.push((certificate_id.clone(), certificate_status, dependant_apps));
    }
    Ok(certificates_with_usage)
  }
}

/// Get certificate resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the certificate resources from
///
/// # Returns
/// Either `None` when the `app` does not have any certificate resources,
/// or a `Some` that contains tuples describing the certificate resources:
/// * resource id
/// * reference to the `Certificate`
pub fn certificate_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Certificate)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Certificate(certificate) => Some(certificate),
    _ => None,
  })
}
