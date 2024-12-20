#![doc(html_favicon_url = "https://teamkpn.kpnnet.org/static/images/favicon.svg")]
#![doc(html_logo_url = "https://teamkpn.kpnnet.org/static/images/favicon.svg")]
//! # DSH resource management API
//!
//! This crate contains functions and definitions that provide support for using the functions
//! of the DSH resource management API. The crate was originally developed as part of the
//! [dcli](https://github.com/kpn-dsh/dcli) tool, but has now been promoted to a separate library.
//!
//! # Examples
//!
//! ## Minimal example
//!
//! The first minimal example will print a list of all the applications that are deployed
//! in a tenant environment. This example requires that the tenant's name, group id, user id,
//! platform and API secret are configured via [environment variables](dsh_api_client_factory).
//!
//! ```ignore
//! use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
//!
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
//! for (application_id, application) in client.list_applications()? {
//!   println!("{} -> {}", application_id, application);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## More elaborate example
//!
//! In the next, more elaborate example, these tenant parameters are given explicitly.
//! This example will list all the applications in the tenant environment that have been
//! configured to require a token in order to access the Kafka broker.
//! This is accomplished via the `find_applications()`
//! methods, that returns a list of all applications for which the provided predicate
//! evaluates to `true`.
//!
//!
//! ```ignore
//! use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//! use dsh_api::dsh_api_tenant::DshApiTenant;
//! use dsh_api::platform::DshPlatform;
//! use dsh_api::types::Application;
//!
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let tenant = DshApiTenant::new(
//!   "greenbox".to_string(),
//!   "2067:2067".to_string(),
//!   DshPlatform::NpLz
//! );
//! let secret = "...".to_string();
//! let client_factory = DshApiClientFactory::create(tenant, secret)?;
//! let client = client_factory.client().await?;
//! let predicate = |application: &Application| application.needs_token;
//! let applications = client.find_applications(&predicate).await?;
//! for (_, application) in applications {
//!   println!("{}", application);
//! }
//! # Ok(())
//! # }
//! ```

/// # Types generated from openapi file
pub use crate::generated::types;

pub(crate) mod generated {
  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub static OPENAPI_SPEC: &str = include_str!("../openapi_spec/open-api.json");

use dsh_sdk::error::DshRestTokenError;

use log::{debug, error};
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

pub mod app;
pub mod app_configuration;
pub mod app_manifest;
pub mod application;
pub mod bucket;
pub mod certificate;
pub mod display;
pub mod dsh_api_client;
pub mod dsh_api_client_factory;
pub mod dsh_api_tenant;
pub mod platform;
pub mod proxy;
pub mod query_processor;
pub mod secret;
pub mod stream;
pub mod topic;
pub mod vhost;
pub mod volume;

/// # Enumeration that denotes an injection of a resource
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Injection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
  /// Path injection, where the value is the name of a directory in the container.
  #[serde(rename = "path")]
  Path(String),
  /// Vhost injection, where the values are the exposed port and the a_zone
  #[serde(rename = "vhost")]
  Vhost(String, Option<String>),
  /// Vhost app resource injection, where the value is the resource name
  #[serde(rename = "vhost_resource")]
  VhostResource(String),
}

/// # Enumeration that denotes where a resource has been used
///
/// There are a number of methods that return where a certain resource (e.g. a secret,
/// a volume or an environment variable) has been used.
/// This enum represents one usage of the resource.
#[derive(Debug, Deserialize, Serialize)]
pub enum UsedBy {
  /// Resource is used in an [`AppCatalogApp`](types::AppCatalogApp).
  /// * Id of the `AppCatalogApp`.
  /// * Ids of the resources.
  App(String, Vec<String>),
  /// Resource is used in an [`Application`](types::Application).
  /// * Application id.
  /// * Number of instances.
  /// * Injections.
  Application(String, u64, Vec<Injection>),
}

#[derive(Debug)]
pub enum DshApiError {
  Configuration(String),
  NotAuthorized,
  NotFound,
  Unexpected(String, Option<Box<dyn StdError + Send + Sync>>),
}

pub type DshApiResult<T> = Result<T, DshApiError>;

impl Display for Injection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Injection::EnvVar(environment_variable) => write!(f, "{}", environment_variable),
      Injection::Path(path) => write!(f, "{}", path),
      Injection::Vhost(port, a_zone) => match a_zone {
        Some(a_zone) => write!(f, "{}:{}", port, a_zone),
        None => write!(f, "{}", port),
      },
      Injection::VhostResource(resource_name) => write!(f, "{}", resource_name),
    }
  }
}

impl Display for UsedBy {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      UsedBy::App(app_id, resource_ids) => {
        write!(f, "app: {}, resources: {}", app_id, resource_ids.join(", "))
      }
      UsedBy::Application(application_id, instances, usage_locations) => {
        write!(f, "application: {}, instances: {}", application_id, instances)?;
        if !usage_locations.is_empty() {
          write!(f, ", {}", usage_locations.iter().map(|inj| inj.to_string()).collect::<Vec<_>>().join(", "))?
        }
        Ok(())
      }
    }
  }
}

impl StdError for DshApiError {
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      DshApiError::Configuration(_) => None,
      DshApiError::NotAuthorized => None,
      DshApiError::NotFound => None,
      DshApiError::Unexpected(_, source) => source.as_deref()?.source(),
    }
  }
}

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::Configuration(message) => write!(f, "{}", message),
      DshApiError::NotAuthorized => write!(f, "not authorized"),
      DshApiError::NotFound => write!(f, "not found"),
      DshApiError::Unexpected(message, cause) => match cause {
        Some(cause) => write!(f, "unexpected error ({})", cause),
        None => write!(f, "unexpected error ({})", message),
      },
    }
  }
}

impl From<SerdeJsonError> for DshApiError {
  fn from(error: SerdeJsonError) -> Self {
    DshApiError::Unexpected(error.to_string(), Some(Box::new(error)))
  }
}

impl From<DshRestTokenError> for DshApiError {
  fn from(error: DshRestTokenError) -> Self {
    match error {
      DshRestTokenError::UnknownClientId => DshApiError::Unexpected("unknown client id".to_string(), Some(Box::new(error))),
      DshRestTokenError::UnknownClientSecret => DshApiError::Unexpected("unknown client secret".to_string(), Some(Box::new(error))),
      DshRestTokenError::FailureTokenFetch(_) => DshApiError::Unexpected("could not fetch token".to_string(), Some(Box::new(error))),
      DshRestTokenError::StatusCode { status_code, ref error_body } => {
        if status_code == 401 {
          DshApiError::NotAuthorized
        } else {
          let message = format!("unexpected error fetching token (status code {})", status_code);
          error!("{}", message);
          debug!("{:?}", error_body);
          DshApiError::Unexpected(message, Some(Box::new(error)))
        }
      }
      _ => DshApiError::Unexpected(format!("unrecognized error ({})", error), Some(Box::new(error))),
    }
  }
}

impl From<ReqwestError> for DshApiError {
  fn from(error: ReqwestError) -> Self {
    DshApiError::Unexpected(error.to_string(), Some(Box::new(error)))
  }
}

impl From<Utf8Error> for DshApiError {
  fn from(error: Utf8Error) -> Self {
    DshApiError::Unexpected(error.to_string(), Some(Box::new(error)))
  }
}

impl From<String> for DshApiError {
  fn from(value: String) -> Self {
    DshApiError::Unexpected(value, None)
  }
}

impl From<&str> for DshApiError {
  fn from(value: &str) -> Self {
    DshApiError::Unexpected(value.to_string(), None)
  }
}

impl From<DshApiError> for String {
  fn from(value: DshApiError) -> Self {
    value.to_string()
  }
}

/// Environment variable used to define the target platform.
pub const PLATFORM_ENVIRONMENT_VARIABLE: &str = "DSH_API_PLATFORM";
/// Environment variable used to define the target tenant.
pub const TENANT_ENVIRONMENT_VARIABLE: &str = "DSH_API_TENANT";

pub(crate) const SECRET_ENVIRONMENT_VARIABLE_PREFIX: &str = "DSH_API_SECRET";
pub(crate) const GUID_ENVIRONMENT_VARIABLE_PREFIX: &str = "DSH_API_GUID";

/// # Create target secret environment variable
///
/// This function creates the environment variable used to define the target's secret
/// from the platform name and the tenant name. The format of the environment variable is
/// `DSH_API_SECRET_[platform_name]_[tenant_name]`,
/// where the `platform_name` and the `tenant_name` will be converted to uppercase and
/// `-` will be replaced by `_`.
///
/// # Parameters
/// * `platform_name` - target's platform name
/// * `tenant_name` - target's tenant name
///
/// # Returns
/// Target secret environment variable.
///
/// # Example
/// ```
/// use dsh_api::secret_environment_variable;
///
/// let env_var = secret_environment_variable("nplz", "greenbox-dev");
/// assert_eq!(env_var, "DSH_API_SECRET_NPLZ_GREENBOX_DEV".to_string());
/// ```
pub fn secret_environment_variable(platform_name: &str, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    SECRET_ENVIRONMENT_VARIABLE_PREFIX,
    platform_name.to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

/// # Create target guid environment variable
///
/// This function creates the environment variable used to define the target's guid
/// from the tenant name. The format of the environment variable is
/// `DSH_API_GUID_[tenant_name]`,
/// where the `tenant_name` will be converted to uppercase and
/// `-` will be replaced by `_`.
///
/// # Parameters
/// * `tenant_name` - target's tenant name
///
/// # Returns
/// Target guid environment variable.
///
/// # Example
/// ```
/// use dsh_api::guid_environment_variable;
///
/// let env_var = guid_environment_variable("greenbox-dev");
/// assert_eq!(env_var, "DSH_API_GUID_GREENBOX_DEV".to_string());
/// ```
pub fn guid_environment_variable(tenant_name: &str) -> String {
  format!("{}_{}", GUID_ENVIRONMENT_VARIABLE_PREFIX, tenant_name.to_ascii_uppercase().replace('-', "_"))
}

#[test]
fn test_dsh_api_error_is_send() {
  fn assert_send<T: StdError + Send>() {}
  assert_send::<DshApiError>();
}

#[test]
fn test_dsh_api_error_is_sync() {
  fn assert_sync<T: Sync>() {}
  assert_sync::<DshApiError>();
}

// Function naming conventions
//
//                       parameter   returns
//
// find_Ys               predicate   zero or more Xs from one Y, that match a predicate
// find_Ys_that_use_X    x_id        find all Ys that use X
// find_Ys_that_use_Xs   x_ids       find all Ys that use one of Xs
// get_X_from_Y          x_id        optional X from Y, that matches x_id
// get_X_from_Ys         x_id        optional X from Ys, that matches x_id
// get_Xs_from_Y         x_ids       zero or more Xs from one Y, that match one of the x_ids
// get_Xs_from_Ys        x_ids       zero or more Xs from multiple Ys, that match one of the x_ids
// X_from_Y                          (optional) X from Y
// X_from_Ys                         (optional) X from Ys
// Xs_from_Y                         zero or more Xs from one Y
// Xs_from_Ys                        zero or more Xs from multiple Ys
//
// _with_Z                           result contains tuples (X, Z)
// _with_Zs                          result contains tuples (X, Zs)

// API naming convention
//
// Configuration is what was configured
// Actual is what is actual deployed
// Naming conventions
// create_SUBJECT                        SUBJECT_id?, CONFIG    create SUBJECT
// delete_SUBJECT                        SUBJECT_id             delete SUBJECT
// deploy_SUBJECT                        SUBJECT_id?, CONFIG    deploy SUBJECT
// get_SUBJECT                           SUBJECT_id             get all actual/current SUBJECT data, by SUBJECT_id
// get_SUBJECT_[SUB]_allocation_status   SUBJECT_id, SUB_id     get SUB allocation status, by SUBJECT_id and SUB_id
// get_SUBJECT_actual_configuration      SUBJECT_id             get actual/current configuration, by SUBJECT_id
// get_SUBJECT_actual_configurations                            get actual/current configurations, for all SUBJECTs
// get_SUBJECT_allocation_status         SUBJECT_id             get SUBJECT allocation status, by SUBJECT_id
// get_SUBJECT_configuration             SUBJECT_id             get configuration provided at creation, by SUBJECT_id
// get_SUBJECT_configurations                                   get configurations provided at creation, for all SUBJECTs
// get_SUBJECT_derived_task_ids          SUBJECT_id             get all taskIids for all derived tasks, by SUBJECT_id
// get_SUBJECT_ids                                              get all ids, for all SUBJECTs
// get_SUBJECT_ids_with_derived_tasks                           get ids for all SUBJECTs that have derived tasks
// get_SUBJECT_SPECIFIC                  SUBJECT_id             get SUBJECT specific data, by SUBJECT_id
// get_SUBJECT_SPECIFICs                 SUBJECT_id             get SUBJECT specific data, for all SUBJECTs
// get_SUBJECT_SUB_allocation_status     SUBJECT_id, SUB_id     get SUB allocation status, by SUBJECT_id and SUB_id
// get_SUBJECTs                                                 get all actual/current SUBJECT data, for all SUBJECTs
// undeploy_SUBJECT                      SUBJECT_id             undeploy SUBJECT, by SUBJECT_id
// update_SUBJECT                        SUBJECT_id, CONFIG     deploy SUBJECT, by SUBJECT_id
