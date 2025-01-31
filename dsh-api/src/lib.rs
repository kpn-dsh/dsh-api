#![doc(
  html_favicon_url = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeD0iMHB4IiB5PSIwcHgiCiAgICAgdmlld0JveD0iMCAwIDE3MS4zIDE4Mi45IiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCAxNzEuMyAxODIuOTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPgogICAgPHN0eWxlPgoJCSNrcG5fbG9nbyB7CgkJCWZpbGw6IGJsYWNrOwoJCX0KCgkJQG1lZGlhIChwcmVmZXJzLWNvbG9yLXNjaGVtZTogZGFyaykgewoJCQkja3BuX2xvZ28gewoJCQkJZmlsbDogd2hpdGU7CgkJCX0KCQl9Cgk8L3N0eWxlPgogICAgPGcgaWQ9Imtwbl9sb2dvIj4KCQk8cGF0aCBkPSJNMTYxLjcsNzIuMWMtNS40LTUuNC0xNS4zLTExLjgtMzIuMi0xMS44Yy0zLjEsMC02LjIsMC4yLTkuMSwwLjZsLTAuOSwwLjFsMC4zLDAuOWMwLjgsMi42LDEuNCw1LjUsMS44LDguNGwwLjEsMC44CgkJCWwwLjgtMC4xYzIuNC0wLjMsNC43LTAuNCw3LTAuNGMxMy40LDAsMjEsNC44LDI1LDguOGM0LjIsNC4yLDYuNSw5LjYsNi41LDE1YzAsNi45LTMuNiwxNS42LTcuMiwyNC4xYy0xLjcsNC4yLTQuOSwxMi4zLTYuNywxOS4yCgkJCWMtMy4zLDEzLjEtOC44LDM1LTIxLjksMzVjLTQuMywwLTkuNC0yLjQtMTUuNS03LjJjLTMuMywxLjktNi44LDMuNC0xMC41LDQuNmM5LjgsOC43LDE4LjEsMTIuOCwyNiwxMi44CgkJCWMyMS4yLDAsMjguMS0yNy44LDMxLjgtNDIuN2MxLjEtNC42LDMuMy0xMC44LDYuMi0xNy43YzMuOS05LjQsOC0xOS4xLDgtMjhDMTcxLjMsODYuMywxNjcuOCw3OC4yLDE2MS43LDcyLjF6Ii8+CgkJPHBhdGggZD0iTTExNiw1Mi4ybDAuOS0wLjJjMi45LTAuNSw1LjktMC44LDkuMS0xYzAuMywwLDAuNiwwLDAuOSwwQzExMi45LDE3LjcsNzcuMiwwLDU2LjcsMEMyOS42LDAsMjAsMjcuNiwyMCw1My40CgkJCWMwLDEyLDQuMSwyNC42LDcuNSwzM2wwLjMsMC44bDAuOC0wLjNjMi40LTEuMSw1LTIuMSw4LTMuMmwwLjgtMC4zTDM3LDgyLjZjLTQuMy0xMC42LTYuOC0yMS4zLTYuOC0yOS4yYzAtMTYuNSw0LTMwLDExLjEtMzcKCQkJYzQuMS00LjEsOS4xLTYuMSwxNS40LTYuMUM3Mi44LDEwLjMsMTAzLDI1LjIsMTE2LDUyLjJ6Ii8+CgkJPHBhdGggZD0iTTk0LjksMTUxLjNsLTAuNC0wLjRsLTAuNSwwLjJjLTUuNSwyLTExLjEsMi45LTE3LjIsMi45Yy0yMCwwLTQxLjgtOC45LTU1LjYtMjIuOGMtNi45LTYuOS0xMC45LTE0LjMtMTAuOS0yMC4yCgkJCWMwLTguMSwzLTE0LjEsOS40LTE5Yy0xLjItMi45LTIuNi02LjMtMy44LTkuOUM1LjIsODkuMiwwLDk4LjcsMCwxMTFjMCw4LjcsNC45LDE4LjUsMTMuOSwyNy41YzEyLjQsMTIuNSwzNS41LDI1LjgsNjIuOSwyNS44CgkJCWM4LjYsMCwxNi44LTEuNywyNC40LTVsMS4xLTAuNWwtMC44LTAuOEM5OS4xLDE1NS43LDk2LjksMTUzLjQsOTQuOSwxNTEuM3oiLz4KCQk8cGF0aCBkPSJNODMuMiw3OS45di05QzgxLDcwLjMsNzguNSw3MCw3NS45LDcwYy0xMC41LDAtMTUuNiw3LjEtMTUuNiwxNC4yYzAsNi44LDIuNSwxMy4zLDExLjksMjcuOWMzLjgtMC41LDcuNi0wLjgsMTEuNC0wLjkKCQkJYy04LjItMTUuMi0xMC4yLTIwLjYtMTAuMi0yNC41YzAtNC41LDIuNi02LjgsNy45LTYuOEM4Miw3OS44LDgyLjYsNzkuOSw4My4yLDc5Ljl6Ii8+CgkJPHBhdGggZD0iTTU0LjcsOTMuMWMtMC44LTItMS4zLTUuMy0xLjYtNy43Yy04LjMtMC4zLTE0LjYsNC41LTE0LjYsMTEuMmMwLDUuNCwyLjgsMTAuMiwxNC4yLDE5LjljMi45LTEsNi44LTIuMSwxMC4xLTIuOAoJCQljLTExLjItMTAuNS0xMy0xMy4zLTEzLTE2LjRDNTAsOTUuMSw1MS42LDkzLjYsNTQuNyw5My4xeiIvPgoJCTxwYXRoIGQ9Ik05MC45LDc5Ljl2LTljMi4xLTAuNiw0LjctMC45LDcuMy0wLjljMTAuNCwwLDE1LjYsNy4xLDE1LjYsMTQuMmMwLDYuOC0yLjUsMTMuMy0xMS45LDI3LjljLTMuOC0wLjUtNy42LTAuOC0xMS40LTAuOQoJCQljOC4yLTE1LjIsMTAuMi0yMC42LDEwLjItMjQuNWMwLTQuNS0yLjYtNi44LTcuOS02LjhDOTIsNzkuOCw5MS40LDc5LjksOTAuOSw3OS45eiIvPgoJCTxwYXRoIGQ9Ik0xMTkuMyw5My4xYzAuOC0yLDEuMy01LjMsMS42LTcuN2M4LjMtMC4zLDE0LjYsNC41LDE0LjYsMTEuMmMwLDUuNC0yLjgsMTAuMi0xNC4yLDE5LjljLTIuOS0xLTYuOC0yLjEtMTAuMS0yLjgKCQkJYzExLjItMTAuNSwxMy0xMy4zLDEzLTE2LjRDMTI0LjEsOTUuMSwxMjIuNSw5My42LDExOS4zLDkzLjF6Ii8+CgkJPHBhdGggZD0iTTg3LDEzMC4yYzguNCwwLDE3LDEuMSwyNS45LDMuOGwzLTEwYy0xMC0zLTE5LjgtNC4yLTI5LTQuMmMtOS4yLDAtMTguOSwxLjItMjksNC4ybDMsMTBDNzAsMTMxLjMsNzguNiwxMzAuMiw4NywxMzAuMnoiCgkJLz4KCQk8cmVjdCB4PSI4MC41IiB5PSI0OS4zIiB0cmFuc2Zvcm09Im1hdHJpeCgwLjcwNzIgLTAuNzA3MSAwLjcwNzEgMC43MDcyIC0xMy45OTkyIDc3Ljg3NDQpIiB3aWR0aD0iMTMuMSIKCQkJICBoZWlnaHQ9IjEzLjEiLz4KCTwvZz4KPC9zdmc+Cg=="
)]
#![doc(
  html_logo_url = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeD0iMHB4IiB5PSIwcHgiCiAgICAgdmlld0JveD0iMCAwIDE3MS4zIDE4Mi45IiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCAxNzEuMyAxODIuOTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPgogICAgPHN0eWxlPgoJCSNrcG5fbG9nbyB7CgkJCWZpbGw6IGJsYWNrOwoJCX0KCgkJQG1lZGlhIChwcmVmZXJzLWNvbG9yLXNjaGVtZTogZGFyaykgewoJCQkja3BuX2xvZ28gewoJCQkJZmlsbDogd2hpdGU7CgkJCX0KCQl9Cgk8L3N0eWxlPgogICAgPGcgaWQ9Imtwbl9sb2dvIj4KCQk8cGF0aCBkPSJNMTYxLjcsNzIuMWMtNS40LTUuNC0xNS4zLTExLjgtMzIuMi0xMS44Yy0zLjEsMC02LjIsMC4yLTkuMSwwLjZsLTAuOSwwLjFsMC4zLDAuOWMwLjgsMi42LDEuNCw1LjUsMS44LDguNGwwLjEsMC44CgkJCWwwLjgtMC4xYzIuNC0wLjMsNC43LTAuNCw3LTAuNGMxMy40LDAsMjEsNC44LDI1LDguOGM0LjIsNC4yLDYuNSw5LjYsNi41LDE1YzAsNi45LTMuNiwxNS42LTcuMiwyNC4xYy0xLjcsNC4yLTQuOSwxMi4zLTYuNywxOS4yCgkJCWMtMy4zLDEzLjEtOC44LDM1LTIxLjksMzVjLTQuMywwLTkuNC0yLjQtMTUuNS03LjJjLTMuMywxLjktNi44LDMuNC0xMC41LDQuNmM5LjgsOC43LDE4LjEsMTIuOCwyNiwxMi44CgkJCWMyMS4yLDAsMjguMS0yNy44LDMxLjgtNDIuN2MxLjEtNC42LDMuMy0xMC44LDYuMi0xNy43YzMuOS05LjQsOC0xOS4xLDgtMjhDMTcxLjMsODYuMywxNjcuOCw3OC4yLDE2MS43LDcyLjF6Ii8+CgkJPHBhdGggZD0iTTExNiw1Mi4ybDAuOS0wLjJjMi45LTAuNSw1LjktMC44LDkuMS0xYzAuMywwLDAuNiwwLDAuOSwwQzExMi45LDE3LjcsNzcuMiwwLDU2LjcsMEMyOS42LDAsMjAsMjcuNiwyMCw1My40CgkJCWMwLDEyLDQuMSwyNC42LDcuNSwzM2wwLjMsMC44bDAuOC0wLjNjMi40LTEuMSw1LTIuMSw4LTMuMmwwLjgtMC4zTDM3LDgyLjZjLTQuMy0xMC42LTYuOC0yMS4zLTYuOC0yOS4yYzAtMTYuNSw0LTMwLDExLjEtMzcKCQkJYzQuMS00LjEsOS4xLTYuMSwxNS40LTYuMUM3Mi44LDEwLjMsMTAzLDI1LjIsMTE2LDUyLjJ6Ii8+CgkJPHBhdGggZD0iTTk0LjksMTUxLjNsLTAuNC0wLjRsLTAuNSwwLjJjLTUuNSwyLTExLjEsMi45LTE3LjIsMi45Yy0yMCwwLTQxLjgtOC45LTU1LjYtMjIuOGMtNi45LTYuOS0xMC45LTE0LjMtMTAuOS0yMC4yCgkJCWMwLTguMSwzLTE0LjEsOS40LTE5Yy0xLjItMi45LTIuNi02LjMtMy44LTkuOUM1LjIsODkuMiwwLDk4LjcsMCwxMTFjMCw4LjcsNC45LDE4LjUsMTMuOSwyNy41YzEyLjQsMTIuNSwzNS41LDI1LjgsNjIuOSwyNS44CgkJCWM4LjYsMCwxNi44LTEuNywyNC40LTVsMS4xLTAuNWwtMC44LTAuOEM5OS4xLDE1NS43LDk2LjksMTUzLjQsOTQuOSwxNTEuM3oiLz4KCQk8cGF0aCBkPSJNODMuMiw3OS45di05QzgxLDcwLjMsNzguNSw3MCw3NS45LDcwYy0xMC41LDAtMTUuNiw3LjEtMTUuNiwxNC4yYzAsNi44LDIuNSwxMy4zLDExLjksMjcuOWMzLjgtMC41LDcuNi0wLjgsMTEuNC0wLjkKCQkJYy04LjItMTUuMi0xMC4yLTIwLjYtMTAuMi0yNC41YzAtNC41LDIuNi02LjgsNy45LTYuOEM4Miw3OS44LDgyLjYsNzkuOSw4My4yLDc5Ljl6Ii8+CgkJPHBhdGggZD0iTTU0LjcsOTMuMWMtMC44LTItMS4zLTUuMy0xLjYtNy43Yy04LjMtMC4zLTE0LjYsNC41LTE0LjYsMTEuMmMwLDUuNCwyLjgsMTAuMiwxNC4yLDE5LjljMi45LTEsNi44LTIuMSwxMC4xLTIuOAoJCQljLTExLjItMTAuNS0xMy0xMy4zLTEzLTE2LjRDNTAsOTUuMSw1MS42LDkzLjYsNTQuNyw5My4xeiIvPgoJCTxwYXRoIGQ9Ik05MC45LDc5Ljl2LTljMi4xLTAuNiw0LjctMC45LDcuMy0wLjljMTAuNCwwLDE1LjYsNy4xLDE1LjYsMTQuMmMwLDYuOC0yLjUsMTMuMy0xMS45LDI3LjljLTMuOC0wLjUtNy42LTAuOC0xMS40LTAuOQoJCQljOC4yLTE1LjIsMTAuMi0yMC42LDEwLjItMjQuNWMwLTQuNS0yLjYtNi44LTcuOS02LjhDOTIsNzkuOCw5MS40LDc5LjksOTAuOSw3OS45eiIvPgoJCTxwYXRoIGQ9Ik0xMTkuMyw5My4xYzAuOC0yLDEuMy01LjMsMS42LTcuN2M4LjMtMC4zLDE0LjYsNC41LDE0LjYsMTEuMmMwLDUuNC0yLjgsMTAuMi0xNC4yLDE5LjljLTIuOS0xLTYuOC0yLjEtMTAuMS0yLjgKCQkJYzExLjItMTAuNSwxMy0xMy4zLDEzLTE2LjRDMTI0LjEsOTUuMSwxMjIuNSw5My42LDExOS4zLDkzLjF6Ii8+CgkJPHBhdGggZD0iTTg3LDEzMC4yYzguNCwwLDE3LDEuMSwyNS45LDMuOGwzLTEwYy0xMC0zLTE5LjgtNC4yLTI5LTQuMmMtOS4yLDAtMTguOSwxLjItMjksNC4ybDMsMTBDNzAsMTMxLjMsNzguNiwxMzAuMiw4NywxMzAuMnoiCgkJLz4KCQk8cmVjdCB4PSI4MC41IiB5PSI0OS4zIiB0cmFuc2Zvcm09Im1hdHJpeCgwLjcwNzIgLTAuNzA3MSAwLjcwNzEgMC43MDcyIC0xMy45OTkyIDc3Ljg3NDQpIiB3aWR0aD0iMTMuMSIKCQkJICBoZWlnaHQ9IjEzLjEiLz4KCTwvZz4KPC9zdmc+Cg=="
)]
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
//! platform and API password are configured via [environment variables](dsh_api_client_factory).
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
//!   "my-tenant".to_string(),
//!   1234,
//!   DshPlatform::try_from("np-aws-lz-dsh")?
//! );
//! let password = "...".to_string();
//! let client_factory = DshApiClientFactory::create(tenant, password)?;
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
//! ## Features
//!
//! The following features are defined:
//!
//! * `actual` - When this feature is enabled the library will include all the "actual"
//!   method versions of the REST API. By default, these methods will not be included.
//! * `generic` - When this feature is enabled the library will also include the generic methods.
//!   This feature is disabled by default.

/// # Types generated from openapi file
pub use crate::generated::types;

pub(crate) mod generated {
  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

/// Openapi specification version 1.9.0
pub static OPENAPI_SPEC: &str = include_str!(concat!(env!("OUT_DIR"), "/openapi.json"));

/// Specification of default platforms
pub static DEFAULT_PLATFORMS: &str = include_str!("../default-platforms.json");

use dsh_sdk::error::DshRestTokenError;

use crate::platform::DshPlatform;
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
#[cfg(feature = "generic")]
pub mod generic;
pub mod platform;
pub mod proxy;
pub mod query_processor;
pub mod secret;
// pub mod stream;
pub mod topic;
pub mod vhost;
pub mod volume;

/// # Returns the version of the lib crate
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::crate_version(), "0.4.0");
/// ```
pub fn crate_version() -> &'static str {
  "0.4.0"
}

/// # Returns the version of the openapi spec
///
/// Version number of the open api file that the crate has been generated from.
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::api_version(), "1.9.0");
/// ```
pub fn api_version() -> &'static str {
  generated::Client::new("").api_version()
}

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

/// Enumeration of the recognized api errors
#[derive(Debug)]
pub enum DshApiError {
  Configuration(String),
  NotAuthorized,
  NotFound,
  Parameter(String),
  Unexpected(String, Option<Box<dyn StdError + Send + Sync>>),
}

/// Generic result type
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
      Self::Configuration(_) => None,
      Self::NotAuthorized => None,
      Self::NotFound => None,
      Self::Parameter(_) => None,
      Self::Unexpected(_, source) => source.as_deref()?.source(),
    }
  }
}

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::Configuration(message) => write!(f, "{}", message),
      DshApiError::NotAuthorized => write!(f, "not authorized"),
      DshApiError::NotFound => write!(f, "not found"),
      DshApiError::Parameter(message) => write!(f, "{}", message),
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

/// Environment variable used to specify the name of a file with an alternative list of platforms
pub const ENV_VAR_PLATFORMS_FILE_NAME: &str = "DSH_API_PLATFORMS_FILE";

/// Environment variable used to define the target platform
pub const ENV_VAR_PLATFORM: &str = "DSH_API_PLATFORM";

/// Environment variable used to define the client tenant
pub const ENV_VAR_TENANT: &str = "DSH_API_TENANT";

pub(crate) const ENV_VAR_PREFIX_PASSWORD: &str = "DSH_API_PASSWORD";
pub(crate) const ENV_VAR_PREFIX_PASSWORD_FILE: &str = "DSH_API_PASSWORD_FILE";
pub(crate) const ENV_VAR_PREFIX_GUID: &str = "DSH_API_GUID";

/// # Create client password environment variable
///
/// This function creates the environment variable used to define the client tenant's password
/// from the platform name and the tenant name. The format of the environment variable is
/// `DSH_API_PASSWORD_[platform_name]_[tenant_name]`,
/// where the `platform_name` and the `tenant_name` will be converted to uppercase and
/// `-` will be replaced by `_`.
///
/// # Parameters
/// * `platform` - target platform
/// * `tenant_name` - client tenant name
///
/// # Returns
/// Client password environment variable.
///
/// # Example
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dsh_api::password_environment_variable;
/// use dsh_api::platform::DshPlatform;
///
/// let env_var =
///   password_environment_variable(&DshPlatform::try_from("np-aws-lz-dsh")?, "my-tenant");
/// assert_eq!(env_var, "DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT".to_string());
/// # Ok(())
/// # }
/// ```
pub fn password_environment_variable(platform: &DshPlatform, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    ENV_VAR_PREFIX_PASSWORD,
    platform.name().to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

/// # Create client password file environment variable
///
/// This function creates the environment variable used to define the client tenant's password file
/// from the platform name and the tenant name. The format of the environment variable is
/// `DSH_API_PASSWORD_FILE_[platform_name]_[tenant_name]`,
/// where the `platform_name` and the `tenant_name` will be converted to uppercase and
/// `-` will be replaced by `_`.
///
/// # Parameters
/// * `platform` - target platform
/// * `tenant_name` - client tenant name
///
/// # Returns
/// Client password file environment variable.
///
/// # Example
/// ```
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use dsh_api::password_file_environment_variable;
/// use dsh_api::platform::DshPlatform;
///
/// let env_var =
///   password_file_environment_variable(&DshPlatform::try_from("np-aws-lz-dsh")?, "my-tenant");
/// assert_eq!(env_var, "DSH_API_PASSWORD_FILE_NP_AWS_LZ_DSH_MY_TENANT".to_string());
/// # Ok(())
/// # }
/// ```
pub fn password_file_environment_variable(platform: &DshPlatform, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    ENV_VAR_PREFIX_PASSWORD_FILE,
    platform.name().to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

/// # Create client tenant guid environment variable
///
/// This function creates the environment variable used to define the client tenant's guid
/// from the tenant's name. The format of the environment variable is
/// `DSH_API_GUID_[tenant_name]`,
/// where the `tenant_name` will be converted to uppercase and
/// `-` will be replaced by `_`.
///
/// # Parameters
/// * `tenant_name` - client tenant name
///
/// # Returns
/// Client tenants guid environment variable.
///
/// # Example
/// ```
/// use dsh_api::guid_environment_variable;
///
/// let env_var = guid_environment_variable("my-tenant");
/// assert_eq!(env_var, "DSH_API_GUID_MY_TENANT".to_string());
/// ```
pub fn guid_environment_variable(tenant_name: &str) -> String {
  format!("{}_{}", ENV_VAR_PREFIX_GUID, tenant_name.to_ascii_uppercase().replace('-', "_"))
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
