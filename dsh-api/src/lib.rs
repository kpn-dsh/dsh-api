#![doc(html_favicon_url = "https://teamkpn.kpnnet.org/static/images/favicon.svg")]
#![doc(html_logo_url = "https://teamkpn.kpnnet.org/static/images/favicon.svg")]
//! # DSH Resource Management API
//!
//! This crate contains functions and definitions that provide support for using the functions
//! of the DSH Resource Management API. The crate was originally developed as part of the
//! [dcli](https://github.com/kpn-dsh/dcli) tool, but has now been promoted to a separate library.
//!
//! ## Examples
//!
//! The first minimal example will print a list of all the applications that are deployed
//! in a tenant environment. This example requires that the tenant's name, group id, user id,
//! platform and API secret are configured via [environment variables](dsh_api_client_factory).
//!
//! ```no_run
//! use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
//!
//! # async fn hide() -> Result<(), String> {
//! let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
//! for (application_id, application) in client.list_applications()? {
//!   println!("{} -> {}", application_id, application);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! In the next, more elaborate example, these tenant parameters are given explicitly.
//! This example will list all the applications in the tenant environment that have been
//! configured to require a token in order to access the Kafka broker.
//! This is accomplished via the `find_applications()`
//! methods, that returns a list of all applications for which the provided predicate
//! evaluates to `true`.
//!
//!
//! ```no_run
//! use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//! use dsh_api::dsh_api_tenant::DshApiTenant;
//! use dsh_api::platform::DshPlatform;
//! use dsh_api::types::Application;
//!
//! # async fn hide() -> Result<(), String> {
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
//!
//!
//!

use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeJsonError;

#[cfg_attr(feature = "generated", doc = "## Functions generated from openapi file")]
#[cfg(feature = "generated")]
pub use dsh_api_generated::generated;

#[cfg(not(feature = "generated"))]
pub(crate) use dsh_api_generated::generated;

pub use dsh_api_generated::display;

/// # Types generated from openapi file
pub use dsh_api_generated::types;

pub mod app;
pub mod app_configuration;
pub mod app_manifest;
pub mod application;
pub mod bucket;
pub mod certificate;
pub mod dsh_api_client;
pub mod dsh_api_client_factory;
pub mod dsh_api_tenant;
pub mod platform;
pub mod proxy;
pub mod query_processor;
pub mod secret;
pub mod stream;
pub mod topic;
pub mod volume;

#[derive(Debug)]
pub enum DshApiError {
  NotAuthorized,
  NotFound,
  Unexpected(String),
}

pub type DshApiResult<T> = Result<T, DshApiError>;

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::NotAuthorized => write!(f, "not authorized"),
      DshApiError::NotFound => write!(f, "not found"),
      DshApiError::Unexpected(message) => write!(f, "unexpected error ({})", message),
    }
  }
}

impl From<SerdeJsonError> for DshApiError {
  fn from(error: SerdeJsonError) -> Self {
    DshApiError::Unexpected(error.to_string())
  }
}

impl From<ReqwestError> for DshApiError {
  fn from(error: ReqwestError) -> Self {
    DshApiError::Unexpected(error.to_string())
  }
}

impl From<Utf8Error> for DshApiError {
  fn from(error: Utf8Error) -> Self {
    DshApiError::Unexpected(error.to_string())
  }
}

impl From<String> for DshApiError {
  fn from(value: String) -> Self {
    DshApiError::Unexpected(value)
  }
}

impl From<&str> for DshApiError {
  fn from(value: &str) -> Self {
    DshApiError::Unexpected(value.to_string())
  }
}

impl From<DshApiError> for String {
  fn from(value: DshApiError) -> Self {
    value.to_string()
  }
}

pub const PLATFORM_ENVIRONMENT_VARIABLE: &str = "DSH_API_PLATFORM";
pub const TENANT_ENVIRONMENT_VARIABLE: &str = "DSH_API_TENANT";

pub fn secret_environment_variable(platform_name: &str, tenant_name: &str) -> String {
  format!(
    "DSH_API_SECRET_{}_{}",
    platform_name.to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

pub fn guid_environment_variable(tenant_name: &str) -> String {
  format!("DSH_API_GUID_{}", tenant_name.to_ascii_uppercase().replace('-', "_"))
}

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
