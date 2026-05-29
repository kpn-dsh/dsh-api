//! # Additional methods to manage secrets
//!
//! Module that contains methods and functions to manage secrets.
//!
//! When you need a list of the available secrets you should preferably use the derived method
//! [`secret_names()`](DshApiClient::secret_names) instead of the generated method
//! [`get_secret_ids()`](DshApiClient::get_secret_ids), since the latter method returns internal
//! ids for the system secrets, which cannot be used with any of the other secret functions.
//! The method `secret_names()` converts these internal ids to secret names, which can be used
//! with the other secret methods and functions.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_secret_configuration(name)`](DshApiClient::delete_secret_configuration)
//! * [`get_secret(name) -> String`](DshApiClient::get_secret)
//! * [`get_secret_actual(name) -> Empty`](DshApiClient::get_secret_actual)
//! * [`get_secret_configuration(name) -> Empty`](DshApiClient::get_secret_configuration)
//! * [`get_secret_ids() -> [id]`](DshApiClient::get_secret_ids)
//! * [`get_secret_status(name) -> AllocationStatus`](DshApiClient::get_secret_status)
//! * [`post_secret(body)`](DshApiClient::post_secret)
//! * [`put_secret(name, body)`](DshApiClient::put_secret)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`secret_dependants(name) -> Vec<Dependant>`](DshApiClient::secret_dependants)
//! * [`secret_name(id/name) -> (name, id)`](normalize_secret_name)
//! * [`secret_names() -> Vec<(String, bool, Option<String>)>`](DshApiClient::secret_names)
//! * [`secret_names_non_system() -> Vec<String>`](DshApiClient::secret_names_non_system)
//! * [`secret_names_system() -> Vec<(String, Option<String>)>`](DshApiClient::secret_names_system)
//! * [`secrets_with_dependants() -> Vec<(String, Vec<Dependant)>`](DshApiClient::secrets_with_dependants)
//! * [`secrets_with_dependant_applications() -> Vec<(String, Vec<DependantApplication)>`](DshApiClient::secrets_with_dependant_applications)
//! * [`secrets_with_dependant_apps() -> Vec<(String, Vec<DependantApp)>`](DshApiClient::secrets_with_dependant_apps)
//! * [`secrets_with_dependant_proxies() -> Vec<(String, Vec<DependantProxy)>`](DshApiClient::secrets_with_dependant_proxies)

use crate::app::{app_resources, apps_that_use_secret};
use crate::application_types::{ApplicationValues, EnvVarInjection};
use crate::certificate::certificates_that_use_secret;
use crate::dsh_api_client::DshApiClient;
use crate::error::DshApiResult;
use crate::proxy::proxies_that_use_secret;
#[allow(unused_imports)]
use crate::types::{AllocationStatus, Empty, Secret};
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{Dependant, DependantApp, DependantApplication, DependantCertificate, DependantProxy};
use futures::try_join;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub const BUCKET_ACCESS_IDENTIFIER: &str = "system/bucketaccess/bucket-id/identifier";
pub const BUCKET_ACCESS_SECRET: &str = "system/bucketaccess/bucket-id/secret";
pub const DBAAS_DATABASE_PASSWORD_SECRET: &str = "system/dbaas/database_password";
pub const DBAAS__PASSWORD: &str = "system/dbaas/trifoniusdb_password";
pub const OBJECT_STORE_ACCESS_KEY_ID: &str = "system/objectstore/access_key_id";
pub const OBJECT_STORE_SECRET_ACCESS_KEY: &str = "system/objectstore/secret_access_key";
pub const ROBOT_SECRET: &str = "system/rest-api-client";
pub const VPN_SECRET: &str = "system/vpn-password";

/// Describes an injection of a `Secret` in an application.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SecretInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar { env_var_name: String },
  /// Certificate cert chain secret.
  #[serde(rename = "cert-chain-secret")]
  CertChainSecret,
  /// Certificate key secret.
  #[serde(rename = "key-secret")]
  KeySecret,
  /// Certificate passphrase secret.
  #[serde(rename = "passphrase-secret")]
  PassphraseSecret,
}

impl SecretInjection {
  pub(crate) fn env_var<T>(env_var_name: T) -> Self
  where
    T: Into<String>,
  {
    Self::EnvVar { env_var_name: env_var_name.into() }
  }
}

impl Display for SecretInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::EnvVar { env_var_name } => write!(f, "{}", env_var_name),
      Self::CertChainSecret => write!(f, "cert-chain-secret"),
      Self::KeySecret => write!(f, "key-secret"),
      Self::PassphraseSecret => write!(f, "passphrase-secret"),
    }
  }
}

/// # Additional methods and functions to manage secrets
///
/// Module that contains methods and functions to manage secrets.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
/// * Functions - Functions that add extra capabilities but do not depend directly on the API.
///
/// When you need a list of the available secrets you should preferably use the derived method
/// [`secret_names()`](DshApiClient::secret_names) instead of the generated method
/// [`get_secret_ids()`](DshApiClient::get_secret_ids), since the latter method returns internal
/// ids for the system secrets, which cannot be used with any of the other secret functions.
/// The method `secret_names()` converts these internal ids to secret names, which can be used
/// with the other secret methods and functions.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`secret_dependants(name) -> Vec<Dependant>`](DshApiClient::secret_dependants)
/// * [`secret_name(id/name) -> (name, id)`](normalize_secret_name)
/// * [`secret_names() -> Vec<(String, bool, Option<String>)>`](DshApiClient::secret_names)
/// * [`secret_names_non_system() -> Vec<String>`](DshApiClient::secret_names_non_system)
/// * [`secret_names_system() -> Vec<(String, Option<String>)>`](DshApiClient::secret_names_system)
/// * [`secrets_with_dependants() -> Vec<(String, Vec<Dependant)>`](DshApiClient::secrets_with_dependants)
/// * [`secrets_with_dependant_applications() -> Vec<(String, Vec<DependantApplication)>`](DshApiClient::secrets_with_dependant_applications)
/// * [`secrets_with_dependant_apps() -> Vec<(String, Vec<DependantApp)>`](DshApiClient::secrets_with_dependant_apps)
/// * [`secrets_with_dependant_proxies() -> Vec<(String, Vec<DependantProxy)>`](DshApiClient::secrets_with_dependant_proxies)
impl DshApiClient {
  /// Returns secret value and allocation status.
  ///
  /// # Parameters
  /// * `secret_name` - Name of the secret.
  ///
  /// # Returns
  /// Tuple containing the secret value and the allocation status.
  pub async fn secret_with_status(&self, secret_name: &str) -> DshApiResult<(String, AllocationStatus)> {
    let (secret_value, allocation_status) = try_join!(self.get_secret(secret_name), self.get_secret_status(secret_name),)?;
    Ok((secret_value, allocation_status))
  }

  /// Returns dependant applications, apps and proxies.
  ///
  /// # Parameters
  /// * `secret_name` - Name of the secret to get the dependants for.
  ///
  /// Returns the applications, apps and proxies that use the provided secret.
  pub async fn secret_dependants(&self, secret_name: &str) -> DshApiResult<Vec<Dependant<SecretInjection>>> {
    let (applications, apps, proxies, certificates) = try_join!(
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map(),
      self.proxies(),
      self.certificates()
    )?;
    let mut dependants: Vec<Dependant<SecretInjection>> = vec![];
    for application in secret_env_vars_from_applications(secret_name, &applications) {
      dependants.push(Dependant::service(
        application.id,
        application.application.instances,
        application
          .values
          .iter()
          .map(|env_var| SecretInjection::EnvVar { env_var_name: env_var.to_string() })
          .collect_vec(),
      ));
    }
    for (app_id, _, resource_ids) in apps_that_use_secret(secret_name, &apps) {
      dependants.push(Dependant::app(
        app_id.to_string(),
        resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
      ));
    }
    for (proxy_id, proxy) in proxies_that_use_secret(secret_name, &proxies) {
      dependants.push(Dependant::proxy(proxy_id.to_string(), proxy.instances.get()));
    }
    for certificate in certificates_that_use_secret(secret_name, &certificates) {
      dependants.push(Dependant::certificate(certificate.certificate_id, certificate.secret_kind));
    }
    Ok(dependants)
  }

  /// Returns all secret names.
  ///
  /// Returns a sorted list of all secret names, including the secret id when the secret is a
  /// system secret.
  ///
  /// In most cases this method should be used instead of the generated method `get_secret_ids()`,
  /// since that method returns internal ids for the system secrets which cannot be used with
  /// any of the other secret functions. The method `secret_names()` converts these internal ids
  /// to secret names, which can be used with the other secret methods and functions.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  pub async fn secret_names(&self) -> DshApiResult<Vec<(String, Option<String>)>> {
    let mut secret_names = self.get_secret_ids().await?.into_iter().map(normalize_secret_name).collect_vec();
    secret_names.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
    Ok(secret_names)
  }

  /// Returns names of non-system secrets.
  ///
  /// Returns a sorted list of all non-system secret names.
  ///
  /// # Returns
  /// List of secret names of non system secrets.
  pub async fn secret_names_non_system(&self) -> DshApiResult<Vec<String>> {
    Ok(self.get_secret_ids().await?.into_iter().filter(|secret_id| !is_system_id(secret_id)).collect_vec())
  }

  /// Returns names of system secrets.
  ///
  /// Returns a sorted list of all system secret names with the secret id.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Secret name.
  /// * `String` - Secret id.
  pub async fn secret_names_system(&self) -> DshApiResult<Vec<(String, String)>> {
    let mut secret_names = self
      .secret_names()
      .await?
      .into_iter()
      .flat_map(|(secret_name, secret_id)| secret_id.map(|id| (secret_name, id)))
      .collect_vec();
    secret_names.sort_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
    Ok(secret_names)
  }

  /// Returns all secrets with dependant applications, apps and proxies.
  ///
  /// Returns a sorted list of all secrets together with the applications, apps and proxies that
  /// use them.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  /// * `Vec<Dependant>` - List of dependants.
  pub async fn secrets_with_dependants(&self) -> DshApiResult<Vec<(String, Option<String>, Vec<Dependant<SecretInjection>>)>> {
    let (secret_names, certificates, applications, apps, proxies) = try_join!(
      self.secret_names(),
      self.certificates(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map(),
      self.proxies()
    )?;
    let mut secrets = Vec::<(String, Option<String>, Vec<Dependant<SecretInjection>>)>::new();
    for (secret_name, secret_id) in secret_names {
      let mut dependants: Vec<Dependant<SecretInjection>> = vec![];
      for application in secret_env_vars_from_applications(&secret_name, &applications) {
        dependants.push(Dependant::service(
          application.id,
          application.application.instances,
          application
            .values
            .iter()
            .map(|env_var| SecretInjection::EnvVar { env_var_name: env_var.to_string() })
            .collect_vec(),
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_secret(&secret_name, &apps) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      for dependant_certificate in certificates_that_use_secret(&secret_name, &certificates) {
        dependants.push(Dependant::Certificate { certificate: dependant_certificate })
      }
      for (proxy_id, proxy) in proxies_that_use_secret(&secret_name, &proxies) {
        dependants.push(Dependant::proxy(proxy_id.to_string(), proxy.instances.get()));
      }
      secrets.push((secret_name, secret_id, dependants));
    }
    Ok(secrets)
  }

  /// Returns all secrets with dependant applications.
  ///
  /// Returns a sorted list of all secrets together with the applications that use them.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  /// * `Vec<DependantApplication>` - List of dependant applications.
  pub async fn secrets_with_dependant_applications(&self) -> DshApiResult<Vec<(String, Option<String>, Vec<DependantApplication<SecretInjection>>)>> {
    let (secret_names, applications) = try_join!(self.secret_names(), self.get_application_configuration_map())?;
    let mut secrets = Vec::<(String, Option<String>, Vec<DependantApplication<SecretInjection>>)>::new();
    for (secret_name, secret_id) in secret_names {
      let mut dependant_applications: Vec<DependantApplication<SecretInjection>> = vec![];
      for application in secret_env_vars_from_applications(secret_name.as_str(), &applications) {
        dependant_applications.push(DependantApplication::new(
          application.id.to_string(),
          application.application.instances,
          application
            .values
            .iter()
            .map(|env_var| SecretInjection::EnvVar { env_var_name: env_var.to_string() })
            .collect_vec(),
        ));
      }
      secrets.push((secret_name, secret_id, dependant_applications));
    }
    Ok(secrets)
  }

  /// Returns all secrets with dependant apps.
  ///
  /// Returns a sorted list of all secrets together with the apps that use them.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  /// * `Vec<DependantApp>` - List of dependant apps.
  pub async fn secrets_with_dependant_apps(&self) -> DshApiResult<Vec<(String, Option<String>, Vec<DependantApp>)>> {
    let (secret_names, apps) = try_join!(self.secret_names(), self.get_appcatalogapp_configuration_map())?;
    let mut secrets = Vec::<(String, Option<String>, Vec<DependantApp>)>::new();
    for (secret_name, secret_id) in secret_names {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      for (app_id, _, resource_ids) in apps_that_use_secret(secret_name.as_str(), &apps) {
        dependant_apps.push(DependantApp::new(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      secrets.push((secret_name, secret_id, dependant_apps));
    }
    Ok(secrets)
  }

  /// Returns all secrets with dependant certificates.
  ///
  /// Returns a sorted list of all secrets together with the certificates that use them.
  ///
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  /// * `Vec<DependantCertificate>` - List of dependant certificates.
  pub async fn secrets_with_dependant_certificates(&self) -> DshApiResult<Vec<(String, Option<String>, Vec<DependantCertificate>)>> {
    let (secret_names, certificates) = try_join!(self.secret_names(), self.certificates())?;
    let mut secrets = Vec::<(String, Option<String>, Vec<DependantCertificate>)>::new();
    for (secret_name, secret_id) in &secret_names {
      let mut dependant_certificates: Vec<DependantCertificate> = vec![];
      for dependant_certificate in certificates_that_use_secret(secret_name, &certificates) {
        dependant_certificates.push(dependant_certificate);
      }
      secrets.push((secret_name.clone(), secret_id.clone(), dependant_certificates));
    }
    Ok(secrets)
  }

  /// Returns all secrets with dependant proxies.
  ///
  /// Returns a sorted list of all secrets together with the proxies that use them.
  /// # Returns
  /// List of tuples (sorted by secret name) where each tuple consists of:
  /// * `String` - Contains the secret name.
  /// * `Option<String>` - Secret id if the secret is a system secret, else empty.
  /// * `Vec<DependantProxy>` - List of dependant proxies.
  pub async fn secrets_with_dependant_proxies(&self) -> DshApiResult<Vec<(String, Option<String>, Vec<DependantProxy>)>> {
    let (secret_names, proxies) = try_join!(self.secret_names(), self.proxies())?;
    let mut secrets = Vec::<(String, Option<String>, Vec<DependantProxy>)>::new();
    for (secret_name, secret_id) in secret_names {
      let mut dependant_proxies: Vec<DependantProxy> = vec![];
      for (proxy_id, proxy) in proxies_that_use_secret(secret_name.as_str(), &proxies) {
        dependant_proxies.push(DependantProxy::new(proxy_id.to_string(), proxy.instances.get()));
      }
      secrets.push((secret_name, secret_id, dependant_proxies));
    }
    Ok(secrets)
  }
}

/// Get application environment variables referencing secret.
///
/// Get all environment variables from `application` referencing secret with `secret_name`.
///
/// # Parameters
/// * `secret_name` - Name of the secret to look for.
/// * `application` - Reference to the `Application`.
///
/// # Returns
/// * `Vec<EnvVarKey>` - list of all environment variables referencing secret `secret_name` The
///   list is sorted by environment variable key.
pub fn secret_env_vars_from_application<'a>(secret_name: &str, application: &'a Application) -> Vec<&'a str> {
  let mut secret_environment_variables = application
    .secrets
    .iter()
    .filter_map(|secret| {
      if secret_name == secret.name {
        let secret_injections = secret
          .injections
          .iter()
          .filter_map(|injection| injection.get("env").map(|secret_injection| secret_injection.as_str()))
          .collect_vec();
        if secret_injections.is_empty() {
          None
        } else {
          Some(secret_injections)
        }
      } else {
        None
      }
    })
    .flatten()
    .collect_vec();
  secret_environment_variables.sort();
  secret_environment_variables
}

/// Get applications environment variables referencing secret.
///
/// Get all environment variables from multiple `Application`s referencing secret with `secret_name`.
/// Applications are only included if they reference secret `secret_name` at least once.
///
/// # Parameters
/// * `secret_name` - Name of the secret to look for.
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// List of `ApplicationValue`s (sorted by application id).
pub fn secret_env_vars_from_applications<'a>(secret_name: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, &'a str>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let injections = secret_env_vars_from_application(secret_name, application);
      if !injections.is_empty() {
        Some(ApplicationValues::new(application_id, application, injections))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// Checks if secret id is a system secret.
///
/// Deprecated, use [is_system_id].
#[deprecated]
pub fn secret_is_system(secret_id: &str) -> bool {
  is_system_id(secret_id)
}

/// Checks if secret id is a system secret id.
pub fn is_system_id(secret_id_name: &str) -> bool {
  secret_id_name.contains('!')
}

/// Checks if secret id is a system secret name.
pub fn is_system_name(secret_id_name: &str) -> bool {
  secret_id_name.starts_with("system/")
}

/// Converts secret id to secret name.
///
/// This function is deprecated, use [`secret_name()`](normalize_secret_name) instead.
///
/// When the secret is a system secret this function will convert the secret id to a secret name.
/// For non-system secrets the secret id and the secret name are the same.
///
/// # Parameters
/// `secret_id` - Secret id to be converted.
///
/// # Returns
/// * `Cow::Borrowed` - Secret id was already in the proper format.
/// * `Cow::Owned` - Secret id was not in the proper format.
#[deprecated]
pub fn secret_id_to_secret_name(secret_id: &String) -> Cow<String> {
  if is_system_id(secret_id) {
    Cow::Owned(format!("system{}", secret_id.replace("!", "/")))
  } else {
    Cow::Borrowed(secret_id)
  }
}

/// Normalize secret id or name.
///
/// * When the provided secret id or name is a system secret id, this function will convert the
///   system secret id to a system secret name and return both.
/// * When the provided secret id or name is a system secret name, this function will convert
///   the secret name to a system secret id and return both.
/// * When the provided secret id or name is a non-system secret name, only the secret name
///   will be returned.
///
/// # Parameters
/// `secret_id_name` - Secret id or name to be normalized.
///
/// # Returns
/// Tuple consisting of:
/// * `String` - Normalized secret name, can be a non-system secret name or a system secret name.
/// * `Option<String>` - Secret id when the `secret_id_name` was a system secret id or system
///   secret name, empty otherwise.
pub fn normalize_secret_name(secret_id_name: String) -> (String, Option<String>) {
  if is_system_id(&secret_id_name) {
    (format!("system{}", secret_id_name.replace("!", "/")), Some(secret_id_name))
  } else if let Some(stripped_system_name) = &secret_id_name.strip_prefix("system/") {
    (secret_id_name.clone(), Some(format!("!{}", stripped_system_name.replace("/", "!"))))
  } else {
    (secret_id_name, None)
  }
}

/// Get secret resources from `AppCatalogApp`.
///
/// # Parameters
/// * `app` - App to get the secret resources from.
///
/// # Returns
/// Either `None` when the `app` does not have any secret resources,
/// or a `Some` that contains tuples describing the secret resources:
/// * resource id
/// * reference to the `Secret`
pub fn secret_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Secret)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Secret(secret) => Some(secret),
    _ => None,
  })
}

/// Get application environment variables referencing secrets.
///
/// Get all environment variables from an `Application` that reference secrets.
///
/// # Parameters
/// * `application` - Reference to the `Application`.
///
/// # Returns
/// List of `EnvVarInjection`s containing:
/// * secret name
/// * lists of environment variables that reference the secret
///
/// The list is sorted by secret name.
pub fn secrets_from_application(application: &Application) -> Vec<EnvVarInjection> {
  let mut grouped_injections: Vec<(&String, Vec<&str>)> = application
    .secrets
    .iter()
    .filter_map(|secret| {
      secret
        .injections
        .iter()
        .filter_map(|injection| injection.get("env").map(|key| key.as_str()))
        .collect_vec()
        .first()
        .map(|env_injection| (&secret.name, *env_injection))
    })
    .into_group_map()
    .into_iter()
    .collect_vec();
  grouped_injections.iter_mut().for_each(|(_, injections)| injections.sort());
  grouped_injections.sort();
  grouped_injections
    .into_iter()
    .map(|(secret_name, injections)| EnvVarInjection::new(secret_name, injections))
    .collect_vec()
}

/// Get applications environment variables referencing secrets.
///
/// Get all environment variables referencing secrets from all `Applications`
///
/// # Parameters
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// List of `ApplicationValues` containing:
/// * application id
/// * application reference
/// * sorted list of pairs of secret names and lists of environment variables referencing those secrets
///
/// The list is sorted by application id.
pub fn secrets_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<EnvVarInjection>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| {
      let secret_injections = secrets_from_application(application);
      if !secret_injections.is_empty() {
        Some(ApplicationValues::new(application_id, application, secret_injections))
      } else {
        None
      }
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// Find apps that use any of a list of given secret.
///
/// # Parameters
/// * `secret_names` - Names of the secrets to look for.
/// * `apps` - Hashmap of all apps.
///
/// # Returns
/// List of tuples containing:
///   * `app_id` - app id of the app that uses the secret
///   * `app` - reference to the app
///   * `resource_ids` - application secret resource ids
pub fn secrets_resources_from_apps<'a>(secret_names: &[String], apps: &'a HashMap<String, AppCatalogApp>) -> Vec<(String, &'a AppCatalogApp, Vec<String>)> {
  let mut app_ids: Vec<String> = apps.keys().map(|p| p.to_string()).collect();
  app_ids.sort();
  let mut tuples: Vec<(String, &AppCatalogApp, Vec<String>)> = vec![];
  for app_id in app_ids {
    let mut resource_ids = vec![];
    let app = apps.get(&app_id).unwrap();
    for (secret_resource_id, secret) in secret_resources_from_app(app) {
      if secret_names.contains(&secret.name) {
        resource_ids.push(secret_resource_id.to_string())
      }
    }
    if !resource_ids.is_empty() {
      tuples.push((app_id, app, resource_ids));
    }
  }
  tuples
}
