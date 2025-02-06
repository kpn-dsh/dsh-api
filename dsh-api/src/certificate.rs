//! # Additional methods to manage certificates
//!
//! Module that contains methods to manage certificates.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_certificate_with_usage(certificate_id) -> (certificate_status, [used_by])`](DshApiClient::get_certificate_with_usage)
//! * [`list_certificates_with_usage() -> (certificate_id, certificate_status, [used_by])`](DshApiClient::list_certificates_with_usage)

use crate::dsh_api_client::DshApiClient;
use crate::types::{AppCatalogApp, Application, CertificateStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{app, application, DshApiResult, UsedBy};
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
/// * [`get_certificate_with_usage(certificate_id) -> (certificate_status, [used_by])`](DshApiClient::get_certificate_with_usage)
/// * [`list_certificates_with_usage() -> (certificate_id, certificate_status, [used_by])`](DshApiClient::list_certificates_with_usage)
impl DshApiClient<'_> {
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
  pub async fn list_certificates_with_usage(&self) -> DshApiResult<Vec<(String, CertificateStatus, Vec<UsedBy>)>> {
    let certificate_ids = self.get_certificate_ids().await?;
    let certificates = try_join_all(certificate_ids.iter().map(|certificate_id| self.get_certificate(certificate_id.as_str()))).await?;
    let (applications, apps) = try_join!(self.get_application_configuration_map(), self.get_appcatalogapp_configuration_map())?;
    let mut certificates_with_usage: Vec<(String, CertificateStatus, Vec<UsedBy>)> = vec![];
    for (certificate_id, certificate_status) in certificate_ids.iter().zip(certificates) {
      let mut usages: Vec<UsedBy> = vec![];
      if let Some(ref configuration) = certificate_status.configuration {
        let secrets = match configuration.passphrase_secret {
          Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
          None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
        };
        for (application_id, application, secret_injections) in application::find_applications_that_use_secrets(&secrets, &applications) {
          for (_, injections) in secret_injections {
            usages.push(UsedBy::Application(application_id.clone(), application.instances, injections));
          }
        }
        for (app_id, _, secret_resources) in app::find_apps_that_use_secrets(&secrets, &apps) {
          usages.push(UsedBy::App(app_id.clone(), secret_resources));
        }
      }
      certificates_with_usage.push((certificate_id.clone(), certificate_status, usages));
    }
    Ok(certificates_with_usage)
  }

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
  pub async fn get_certificate_with_usage(&self, certificate_id: &str) -> DshApiResult<(CertificateStatus, Vec<UsedBy>)> {
    let (certificate_status, applications, apps): (CertificateStatus, HashMap<String, Application>, HashMap<String, AppCatalogApp>) = try_join!(
      self.get_certificate(certificate_id),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut used_by: Vec<UsedBy> = vec![];
    if let Some(ref configuration) = certificate_status.configuration {
      let secrets = match configuration.passphrase_secret {
        Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
        None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
      };
      for (application_id, application, secret_injections) in application::find_applications_that_use_secrets(&secrets, &applications) {
        for (_, injections) in secret_injections {
          used_by.push(UsedBy::Application(application_id.clone(), application.instances, injections));
        }
      }
      for (app_id, _, secret_resources) in app::find_apps_that_use_secrets(&secrets, &apps) {
        used_by.push(UsedBy::App(app_id.clone(), secret_resources));
      }
    }
    Ok((certificate_status, used_by))
  }
}
