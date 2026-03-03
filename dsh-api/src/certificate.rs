//! Module that contains methods and functions to manage certificates.
//!
#![doc = mermaid!("diagrams/certificate.mmd")]
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_certificate_configuration(id)`](DshApiClient::delete_certificate_configuration)
//! * [`get_certificate(id) -> CertificateStatus`](DshApiClient::get_certificate)
//! * [`get_certificate_actual(id) -> Certificate`](DshApiClient::get_certificate_actual)
//! * [`get_certificate_configuration(id) -> Certificate`](DshApiClient::get_certificate_configuration)
//! * [`get_certificate_ids() -> Vec<String>`](DshApiClient::get_certificate_ids)
//! * [`get_certificate_status(id) -> AllocationStatus`](DshApiClient::get_certificate_status)
//! * [`put_certificate_configuration(id, body)`](DshApiClient::put_certificate_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`certificate_with_dependants(id) -> (CertificateStatus, Vec<DependantApp>)`](DshApiClient::certificate_with_dependant_apps)
//! * [`certificate_with_dependant_apps(id) -> (CertificateStatus, Vec<DependantApp>)`](DshApiClient::certificate_with_dependant_apps)
//! * [`certificate_with_dependant_proxies(id) -> (CertificateStatus, Vec<DependantProxy>)`](DshApiClient::certificate_with_dependant_apps)
//! * [`certificates() -> Vec<(String, CertificateStatus>`](DshApiClient::certificates)
//! * [`certificates_with_dependant_apps() -> Vec<(String, CertificateStatus, Vec<DependantApp>)>`](DshApiClient::certificates_with_dependant_apps)

use crate::app::app_resources;
use crate::dsh_api_client::DshApiClient;
use crate::error::DshApiResult;
use crate::secret::secrets_resources_from_apps;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Certificate, CertificateStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{CertificateSecretKind, Dependant, DependantApp, DependantCertificate, DependantProxy};
use futures::future::try_join_all;
use futures::try_join;
use itertools::Itertools;
use simple_mermaid::mermaid;

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
/// * [`certificate_with_dependant_apps(id) -> (CertificateStatus, Vec<DependantApp>)`](DshApiClient::certificate_with_dependant_apps)
/// * [`certificates_with_dependant_apps() -> Vec<(String, CertificateStatus, Vec<DependantApp>)>`](DshApiClient::certificates_with_dependant_apps)
impl DshApiClient {
  /// # Returns all certificates with dependant apps and proxies
  ///
  /// Returns a sorted list of all secrets together with the apps and proxies that use them.
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<(CertificateStatus, Vec<Dependant>>` - Tuple containing the certificate configuration
  ///   and a vector of dependants, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificate_with_dependants<T>(&self, certificate_id: &str) -> DshApiResult<(CertificateStatus, Vec<Dependant<T>>)> {
    let (certificate_status, appcatalogapp_configuration_map, proxies) =
      try_join!(self.get_certificate(certificate_id), self.get_appcatalogapp_configuration_map(), self.proxies())?;
    let mut dependants: Vec<Dependant<T>> = vec![];
    if let Some(certificate_configuration) = &certificate_status.configuration {
      for (app_id, app_catalog_app) in appcatalogapp_configuration_map {
        let certificate_resources = certificate_resources_from_app(&app_catalog_app)
          .iter()
          .filter(|(_, certificate_resource)| certificate_resource.cert_chain_secret == certificate_configuration.cert_chain_secret)
          .map(|(resource_id, _)| resource_id.to_string())
          .collect_vec();
        if !certificate_resources.is_empty() {
          dependants.push(Dependant::app(app_id, certificate_resources));
        }
      }
    } else {
      return Err(DshApiError::unexpected(format!("certificate {} has not configuration", certificate_id)));
    }
    for (proxy_id, proxy) in proxies {
      if proxy.certificate == certificate_id {
        dependants.push(Dependant::proxy(proxy_id, proxy.instances.get()));
      }
    }
    Ok((certificate_status, dependants))
  }

  /// # Return certificate with dependant apps
  ///
  /// Returns the certificate configuration for the provided certificate id,
  /// together with the apps that use this certificate.
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<(CertificateStatus, Vec<DependantApp>>` - tuple containing the certificate configuration
  ///   and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificate_with_dependant_apps(&self, certificate_id: &str) -> DshApiResult<(CertificateStatus, Vec<DependantApp>)> {
    let (certificate_status, appcatalogapp_configuration_map) = try_join!(self.get_certificate(certificate_id), self.get_appcatalogapp_configuration_map())?;
    if let Some(certificate_configuration) = &certificate_status.configuration {
      let mut dependants: Vec<DependantApp> = vec![];
      for (app_id, app_catalog_app) in appcatalogapp_configuration_map {
        let certificate_resources = certificate_resources_from_app(&app_catalog_app)
          .iter()
          .filter(|(_, certificate_resource)| certificate_resource.cert_chain_secret == certificate_configuration.cert_chain_secret)
          .map(|(resource_id, _)| resource_id.to_string())
          .collect_vec();
        if !certificate_resources.is_empty() {
          dependants.push(DependantApp::new(app_id, certificate_resources));
        }
      }
      Ok((certificate_status, dependants))
    } else {
      Err(DshApiError::unexpected(format!("certificate {} has not configuration", certificate_id)))
    }
  }

  /// # Return certificate with dependant proxies
  ///
  /// Returns the certificate configuration for the provided certificate id,
  /// together with the proxies that use this certificate.
  ///
  /// # Parameters
  /// * `certificate_id` - id of the requested certificate
  ///
  /// # Returns
  /// * `Ok<(CertificateStatus, Vec<DependantProxy>>` - Tuple containing the certificate
  ///   configuration and a vector of usages, which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificate_with_dependant_proxies(&self, certificate_id: &str) -> DshApiResult<(CertificateStatus, Vec<DependantProxy>)> {
    let (certificate_status, proxies) = try_join!(self.get_certificate(certificate_id), self.proxies())?;
    let dependant_proxies = proxies
      .iter()
      .filter(|(_, proxy)| proxy.certificate == certificate_id)
      .map(|(proxy_id, proxy)| DependantProxy::new(proxy_id.to_string(), proxy.instances.get()))
      .collect_vec();
    Ok((certificate_status, dependant_proxies))
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
  pub async fn certificates_with_dependants<T>(&self) -> DshApiResult<Vec<(String, CertificateStatus, Vec<Dependant<T>>)>> {
    let (certificate_ids, apps, proxies) = try_join!(self.get_certificate_ids(), self.get_appcatalogapp_configuration_map(), self.proxies())?;
    let certificates = try_join_all(certificate_ids.iter().map(|certificate_id| self.get_certificate(certificate_id.as_str()))).await?;

    let mut certificates_with_usage: Vec<(String, CertificateStatus, Vec<Dependant<T>>)> = vec![];

    for (certificate_id, certificate_status) in certificate_ids.iter().zip(certificates) {
      let mut dependants: Vec<Dependant<T>> = vec![];
      if let Some(certificate_configuration) = &certificate_status.configuration {
        for (app_id, app_catalog_app) in &apps {
          let certificate_resources = certificate_resources_from_app(app_catalog_app)
            .iter()
            .filter(|(_, certificate_resource)| certificate_resource.cert_chain_secret == certificate_configuration.cert_chain_secret)
            .map(|(resource_id, _)| resource_id.to_string())
            .collect_vec();
          if !certificate_resources.is_empty() {
            dependants.push(Dependant::app(app_id.to_string(), certificate_resources));
          }
        }
      } else {
        return Err(DshApiError::unexpected(format!("certificate {} has not configuration", certificate_id)));
      }
      for (proxy_id, proxy) in &proxies {
        if proxy.certificate == *certificate_id {
          dependants.push(Dependant::proxy(proxy_id.to_string(), proxy.instances.get()));
        }
      }
      certificates_with_usage.push((certificate_id.clone(), certificate_status, dependants));
    }
    Ok(certificates_with_usage)
  }

  /// # List all certificates
  ///
  /// Returns a list of all certificate ids and their configurations,
  ///
  /// # Returns
  /// * `Ok<Vec<(String, CertificateStatus, Vec<UsedBy>>>` - list of tuples
  ///   containing the certificate id, certificate configuration and a vector of usages,
  ///   which can be empty.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn certificates(&self) -> DshApiResult<Vec<(String, CertificateStatus)>> {
    let certificate_ids = self.get_certificate_ids().await?;
    let certificates = try_join_all(certificate_ids.iter().map(|certificate_id| self.get_certificate(certificate_id.as_str()))).await?;
    Ok(certificate_ids.into_iter().zip(certificates).collect_vec())
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
/// * `app` - App to get the certificate resources from.
///
/// # Returns
/// Either `None` when the `app` does not have any certificate resources,
/// or a `Some` that contains tuples describing the certificate resources:
/// * Resource id
/// * Reference to the `Certificate`
pub fn certificate_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Certificate)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Certificate(certificate) => Some(certificate),
    _ => None,
  })
}

/// Find certificates that use a given secret
///
/// # Parameters
/// * `secret_name` - Name of the secret to look for.
/// * `certificates` - Reference to a list of tuples describing all certificate.
///   Each tuple consists of:
///    * `String` - Certificate id.
///    * [`CertificateStatus`] - Certificate parameters.
///
/// # Returns
/// `Vec<DependantCertificate>` - List of certificates that use the secret.
pub fn certificates_that_use_secret(secret_name: &str, certificates: &[(String, CertificateStatus)]) -> Vec<DependantCertificate> {
  let mut dependant_certificates: Vec<DependantCertificate> = vec![];
  for (certificate_id, certificate_status) in certificates {
    if let Some(actual_certificate) = &certificate_status.actual {
      if actual_certificate.cert_chain_secret == secret_name {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::CertChainSecret))
      }
      if actual_certificate.key_secret == secret_name {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::KeySecret))
      }
      if actual_certificate
        .passphrase_secret
        .as_ref()
        .is_some_and(|passphrase_secret| passphrase_secret == secret_name)
      {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::PassphraseSecret))
      }
    } else if let Some(certificate) = &certificate_status.configuration {
      if certificate.cert_chain_secret == secret_name {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::CertChainSecret))
      }
      if certificate.key_secret == secret_name {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::KeySecret))
      }
      if certificate
        .passphrase_secret
        .as_ref()
        .is_some_and(|passphrase_secret| passphrase_secret == secret_name)
      {
        dependant_certificates.push(DependantCertificate::new(certificate_id.clone(), CertificateSecretKind::PassphraseSecret))
      }
    }
  }
  dependant_certificates
}
