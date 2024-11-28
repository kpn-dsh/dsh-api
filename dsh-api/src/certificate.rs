//! # Manage certificates
//!
//! Module that contains functions to manage certificates.
//!
//! # API Methods
//! * [`create_certificate(certificate_id, certificate)`](DshApiClient::create_certificate)
//! * [`delete_certificate(certificate_id)`](DshApiClient::delete_certificate)
//! * [`get_certificate(certificate_id) -> certificate_status`](DshApiClient::get_certificate)
//! * [`get_certificate_allocation_status(certificate_id) -> allocation_status`](DshApiClient::get_certificate_allocation_status)
//! * [`get_certificate_configuration(certificate_id) -> certificate`](DshApiClient::get_certificate_configuration)
//! * [`get_certificate_ids() -> [id]`](DshApiClient::get_certificate_ids)
//! * [`get_certificate_with_usage(certificate_id) -> (certificate_status, [used_by])`](DshApiClient::get_certificate_with_usage)
//! * [`list_certificates_with_usage() -> (certificate_id, certificate_status, [used_by])`](DshApiClient::list_certificates_with_usage)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_certificate_actual_configuration(certificate_id) -> Certificate`](DshApiClient::get_certificate_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, Certificate, CertificateStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{DshApiResult, UsedBy};
use dsh_api_generated::types::{AppCatalogApp, Application};
use futures::future::try_join_all;
use futures::try_join;
use std::collections::HashMap;

/// # Manage certificates
///
/// Module that contains functions to manage certificates.
///
/// # API Methods
/// * [`create_certificate(certificate_id, certificate)`](DshApiClient::create_certificate)
/// * [`delete_certificate(certificate_id)`](DshApiClient::delete_certificate)
/// * [`get_certificate(certificate_id) -> CertificateStatus`](DshApiClient::get_certificate)
/// * [`get_certificate_allocation_status(certificate_id) -> AllocationStatus`](DshApiClient::get_certificate_allocation_status)
/// * [`get_certificate_configuration(certificate_id) -> Certificate`](DshApiClient::get_certificate_configuration)
/// * [`get_certificate_ids(&self) -> Vec<String>`](DshApiClient::get_certificate_ids)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "## Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_certificate_actual_configuration(certificate_id) -> Certificate`](DshApiClient::get_certificate_actual_configuration)")]
impl DshApiClient<'_> {
  /// # Create certificate
  ///
  /// API function: `PUT /allocation/{tenant}/certificate/{id}/configuration`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the certificate to update
  /// * `certificate` - new value of the certificate
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the certificate has been successfully updated)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn create_certificate(&self, certificate_id: &str, certificate: Certificate) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_certificate_configuration_by_tenant_by_id(self.tenant_name(), certificate_id, self.token(), &certificate)
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Delete certificate
  ///
  /// API function: `DELETE /allocation/{tenant}/certificate/{id}/configuration`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the certificate to delete
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the certificate has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_certificate(&self, certificate_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_certificate_configuration_by_tenant_by_id(self.tenant_name(), certificate_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return certificate
  ///
  /// API function: `GET /allocation/{tenant}/certificate/{id}`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<`[`CertificateStatus`]`>` - certificate
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_certificate(&self, certificate_id: &str) -> DshApiResult<CertificateStatus> {
    self
      .process_raw(
        self
          .generated_client
          .get_certificate_by_tenant_by_id(self.tenant_name(), certificate_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return actual state of certificate
  ///
  /// API function: `GET /allocation/{tenant}/certificate/{id}/actual`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<`[`Certificate`]`>` - indicates that certificate is ok
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_certificate_actual_configuration(&self, certificate_id: &str) -> DshApiResult<Certificate> {
    self
      .process(
        self
          .generated_client
          .get_certificate_actual_by_tenant_by_id(self.tenant_name(), certificate_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return certificate allocation status
  ///
  /// API function: `GET /allocation/{tenant}/certificate/{id}/status`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<`[`CertificateStatus`]`>` - allocation status of the certificate
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_certificate_allocation_status(&self, certificate_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_certificate_status_by_tenant_by_id(self.tenant_name(), certificate_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return certificate configuration
  ///
  /// API function: `GET /allocation/{tenant}/certificate/{id}/configuration`
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<`[`Certificate`]`>` - indicates that certificate is ok
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_certificate_configuration(&self, certificate_id: &str) -> DshApiResult<Certificate> {
    self
      .process(
        self
          .generated_client
          .get_certificate_configuration_by_tenant_by_id(self.tenant_name(), certificate_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # List all certificates with usage
  ///
  /// Returns a list of all certificate configurations,
  /// together with the apps and applications that use this certificate.
  ///
  /// # Returns
  /// * `Ok<Vec<(CertificateStatus, Vec<UsedBy>>>` - list of tuples
  ///   containing the certificate configuration and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_certificates_with_usage(&self) -> DshApiResult<Vec<(String, CertificateStatus, Vec<UsedBy>)>> {
    let certificate_ids = self.get_certificate_ids().await?;
    let certificates = try_join_all(certificate_ids.iter().map(|certificate_id| self.get_certificate(certificate_id.as_str()))).await?;
    let applications = self.get_applications().await?;
    let apps = self.get_app_configurations().await?;
    let mut certificates_with_usage: Vec<(String, CertificateStatus, Vec<UsedBy>)> = vec![];
    for (certificate_id, certificate_status) in certificate_ids.iter().zip(certificates) {
      let mut usages: Vec<UsedBy> = vec![];
      if let Some(ref configuration) = certificate_status.configuration {
        let secrets = match configuration.passphrase_secret {
          Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
          None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
        };
        for (application_id, application, secret_injections) in Self::applications_with_secrets_injections(&secrets, &applications) {
          for (_, injections) in secret_injections {
            usages.push(UsedBy::Application(application_id.clone(), application.instances, injections));
          }
        }
        for (app_id, _, application_resource_id, application, secret_injections) in Self::apps_with_secrets_injections(&secrets, &apps) {
          for (_, injections) in secret_injections {
            usages.push(UsedBy::App(app_id.clone(), application_resource_id.clone(), application.instances, injections));
          }
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
    let (certificate_status, applications, apps): (CertificateStatus, HashMap<String, Application>, HashMap<String, AppCatalogApp>) =
      try_join!(self.get_certificate(certificate_id), self.get_applications(), self.get_app_configurations())?;
    let mut used_by: Vec<UsedBy> = vec![];
    if let Some(ref configuration) = certificate_status.configuration {
      let secrets = match configuration.passphrase_secret {
        Some(ref passphrase_secret) => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone(), passphrase_secret.clone()],
        None => vec![configuration.cert_chain_secret.clone(), configuration.key_secret.clone()],
      };
      for (application_id, application, secret_injections) in Self::applications_with_secrets_injections(&secrets, &applications) {
        for (_, injections) in secret_injections {
          used_by.push(UsedBy::Application(application_id.clone(), application.instances, injections));
        }
      }
      for (app_id, _, application_resource_id, application, secret_injections) in Self::apps_with_secrets_injections(&secrets, &apps) {
        for (_, injections) in secret_injections {
          used_by.push(UsedBy::App(app_id.clone(), application_resource_id.clone(), application.instances, injections));
        }
      }
    }
    Ok((certificate_status, used_by))
  }

  /// # Return certificate ids
  ///
  /// API function: `GET /allocation/{tenant}/certificate`
  ///
  /// # Returns
  /// * `Ok<Vec<String>` - certificate ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_certificate_ids(&self) -> DshApiResult<Vec<String>> {
    let mut certificate_ids: Vec<String> = self
      .process(self.generated_client.get_certificate_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|certificate_ids| certificate_ids.iter().map(|certificate_id| certificate_id.to_string()).collect())?;
    certificate_ids.sort();
    Ok(certificate_ids)
  }
}
