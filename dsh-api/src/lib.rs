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
//! [`dsh`](https://github.com/kpn-dsh/dsh) command line tool,
//! but has now been promoted to a separate library.
//!
//! # [`DshApiClient`]
//!
//! The crate consists basically of the struct [`DshApiClient`],
//! which has many associated methods.
//! In order to use these methods, you first need to acquire an instance of the struct.
//! This is a two-step process.
//! * First you need to get a
//!   [`DshApiClientFactory`](dsh_api_client_factory::DshApiClientFactory):
//!   * Either use the
//!     [`DshApiClientFactory::default()`](dsh_api_client_factory::DshApiClientFactory::default),
//!     method, which is configured from
//!     [environment variables](dsh_api_client_factory/index.html#environment-variables),
//!   * or you can create a factory explicitly by providing the `platform`,
//!     `tenant` and API `password` yourself and feeding them to the
//!     [`DshApiClientFactory::create()`](dsh_api_client_factory::DshApiClientFactory::create)
//!     function.
//! * Once you have the [`DshApiClientFactory`](dsh_api_client_factory::DshApiClientFactory),
//!   you can call its [`client()`](dsh_api_client_factory::DshApiClientFactory::client) method.
//!
//! You can now call the client's methods to interact with the DSH resource management API.
//!
//! The client will contain an embedded
//! [`ManagementApiTokenFetcher`](dsh_sdk::ManagementApiTokenFetcher)
//! (provided by the [`DSH SDK`](https://crates.io/crates/dsh_sdk)),
//! which will make sure that you always use a valid token when calling the API functions.
//!
//! # [`types`]
//!
//! For their parameters and return values the methods and functions in the crate
//! make use of rust `struct`s that where generated from the DSH resource management API
//! Openapi specification (version 1.9.0).
//!
//! The generated types are defined as follows:
//!
//! ```rust
//! # use serde::{Deserialize, Serialize};
//! #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
//! pub struct Secret {
//!   pub name: String,
//!   pub value: String,
//! }
//! ```
//!
//! * All types are `pub`.
//! * All fields are `pub`.
//! * All types have derived implementations of the [`Clone`], [`Debug`], [`Deserialize`],
//!   [`PartialEq`] and [`Serialize`] traits.
//! * Some [selected types](crate::display) also have an implementation of the [`Display`] trait.
//!
//! # Examples
//!
//! ## Minimal example
//!
//! The first minimal example will print a list of all the applications that are deployed
//! in a tenant environment. This example requires that the tenant's name,
//! platform and API password are configured via [environment variables](dsh_api_client_factory).
//!
//! ```ignore
//! use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//!
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let client = DshApiClientFactory::default().client().await?;
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
//! ```ignore
//! # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//! # use dsh_api::dsh_api_tenant::DshApiTenant;
//! # use dsh_api::platform::DshPlatform;
//! # use dsh_api::types::Application;
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let tenant = DshApiTenant::new(
//!   "my-tenant".to_string(),
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
//! # Features
//!
//! By enabling/disabling the features described below you have control over what's included
//! in your library and what's not.
//! All features are disabled by default.
//!
//! The following features are defined:
//!
//! * `appcatalog` - Enables the app catalog methods.
//! * `generic` - Enables the generic methods.
//! * `manage` -  Enables the manage methods.
//! * `robot` - Enables the robot operation.

/// # Types generated from openapi file
pub use crate::generated::types;

#[allow(dead_code)]
pub(crate) mod generated {
  include!(concat!(env!("OUT_DIR"), "/progenitor_client.rs"));
}

include!(concat!(env!("OUT_DIR"), "/wrapped.rs"));

/// Openapi specification version 1.9.0
pub static OPENAPI_SPEC: &str = include_str!(concat!(env!("OUT_DIR"), "/openapi.json"));

/// Specification of default platforms
pub static DEFAULT_PLATFORMS: &str = include_str!("../default-platforms.json");

use crate::platform::DshPlatform;
use crate::types::error::ConversionError;
use dsh_sdk::management_api::ManagementApiTokenError;
use log::{debug, error};
use progenitor_client::Error as ProgenitorError;
use reqwest::Error as ReqwestError;
use reqwest::StatusCode as ReqwestStatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

pub mod app;
#[cfg(feature = "appcatalog")]
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
pub mod query_processor;
pub mod secret;
pub mod topic;
pub mod vhost;
pub mod volume;

/// # Returns the version of the lib crate
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::crate_version(), "0.5.2");
/// ```
pub fn crate_version() -> &'static str {
  "0.5.2"
}

/// # Returns the version of the openapi spec
///
/// Version number of the openapi file that the crate has been generated from.
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::openapi_version(), "1.9.0");
/// ```
pub fn openapi_version() -> &'static str {
  generated::Client::new("").api_version()
}

/// # Describes an injection of a resource
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

/// # Describes where a resource has been used
///
/// There are a number of methods that return where a certain resource (e.g. a secret,
/// a volume or an environment variable) has been used.
/// This enum represents one usage of the resource.
#[derive(Debug, Deserialize, Serialize)]
pub enum UsedBy {
  /// Resource is used in an [`AppCatalogApp`].
  /// * Id of the `AppCatalogApp`.
  /// * Ids of the resources.
  App(String, Vec<String>),
  /// Resource is used in an [`Application`].
  /// * Application id.
  /// * Number of instances.
  /// * Injections.
  Application(String, u64, Vec<Injection>),
}

/// Describes an API error
#[derive(Debug)]
pub enum DshApiError {
  BadRequest(String),
  Configuration(String),
  NotAuthorized,
  NotFound,
  Parameter(String),
  Unexpected(String, Option<String>),
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

impl StdError for DshApiError {}

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::BadRequest(message) => write!(f, "{}", message),
      DshApiError::Configuration(message) => write!(f, "{}", message),
      DshApiError::NotAuthorized => write!(f, "not authorized"),
      DshApiError::NotFound => write!(f, "not found"),
      DshApiError::Parameter(message) => write!(f, "{}", message),
      DshApiError::Unexpected(message, cause) => match cause {
        Some(cause) => write!(f, "unexpected error ({}, {})", message, cause),
        None => write!(f, "unexpected error ({})", message),
      },
    }
  }
}

impl From<SerdeJsonError> for DshApiError {
  fn from(error: SerdeJsonError) -> Self {
    DshApiError::Unexpected("json error".to_string(), Some(error.to_string()))
  }
}

impl From<ManagementApiTokenError> for DshApiError {
  fn from(error: ManagementApiTokenError) -> Self {
    match error {
      ManagementApiTokenError::UnknownClientId => DshApiError::Unexpected("unknown client id".to_string(), Some(error.to_string())),
      ManagementApiTokenError::UnknownClientSecret => DshApiError::Unexpected("unknown client secret".to_string(), Some(error.to_string())),
      ManagementApiTokenError::FailureTokenFetch(_) => DshApiError::Unexpected("could not fetch token".to_string(), Some(error.to_string())),
      ManagementApiTokenError::StatusCode { status_code, ref error_body } => {
        if status_code == 401 {
          DshApiError::NotAuthorized
        } else {
          let message = format!("unexpected error fetching token (status code {})", status_code);
          error!("{}", message);
          debug!("{:?}", error_body);
          DshApiError::Unexpected(message, Some(error.to_string()))
        }
      }
      _ => DshApiError::Unexpected(format!("unrecognized error ({})", error), None),
    }
  }
}

impl From<ReqwestError> for DshApiError {
  fn from(error: ReqwestError) -> Self {
    DshApiError::Unexpected(error.to_string(), None)
  }
}

impl From<Utf8Error> for DshApiError {
  fn from(error: Utf8Error) -> Self {
    DshApiError::Unexpected(error.to_string(), None)
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

impl From<ConversionError> for DshApiError {
  fn from(value: ConversionError) -> Self {
    DshApiError::Unexpected(value.to_string(), None)
  }
}

impl DshApiError {
  // async version of impl From<ProgenitorError> for DshApiError
  pub(crate) async fn async_from_progenitor_error(progenitor_error: ProgenitorError) -> Self {
    match progenitor_error {
      ProgenitorError::InvalidRequest(ref string) => Self::Unexpected(format!("invalid request ({})", string), Some(progenitor_error.to_string())),
      ProgenitorError::CommunicationError(ref reqwest_error) => Self::Unexpected(
        format!("communication error (reqwest error: {})", reqwest_error),
        Some(progenitor_error.to_string()),
      ),
      ProgenitorError::InvalidUpgrade(ref reqwest_error) => Self::Unexpected(format!("invalid upgrade (reqwest error: {})", reqwest_error), Some(progenitor_error.to_string())),
      ProgenitorError::ErrorResponse(ref progenitor_response_value) => Self::Unexpected(
        format!("error response (progenitor response value: {:?})", progenitor_response_value),
        Some(progenitor_error.to_string()),
      ),
      ProgenitorError::ResponseBodyError(ref reqwest_error) => Self::Unexpected(
        format!("response body error (reqwest error: {})", reqwest_error),
        Some(progenitor_error.to_string()),
      ),
      ProgenitorError::InvalidResponsePayload(ref _bytes, ref json_error) => {
        Self::Unexpected(format!("invalid response payload (json error: {})", json_error), Some(progenitor_error.to_string()))
      }
      ProgenitorError::UnexpectedResponse(reqwest_response) => match &reqwest_response.status().clone() {
        &ReqwestStatusCode::BAD_REQUEST => match reqwest_response.text().await {
          Ok(error_text) => Self::BadRequest(error_text),
          Err(response_error) => Self::BadRequest(response_error.to_string()),
        },
        &ReqwestStatusCode::NOT_FOUND => Self::NotFound,
        &ReqwestStatusCode::UNAUTHORIZED | &ReqwestStatusCode::FORBIDDEN | &ReqwestStatusCode::METHOD_NOT_ALLOWED => Self::NotAuthorized,
        other_status_code => Self::Unexpected(format!("unexpected response (status: {}, reqwest response: )", other_status_code), None),
      },
      ProgenitorError::PreHookError(string) => Self::Unexpected(format!("pre-hook error ({})", string), None),
    }
  }
}

// Environment variable used to specify the name of a file with an alternative list of platforms
pub(crate) const ENV_VAR_PLATFORMS_FILE_NAME: &str = "DSH_API_PLATFORMS_FILE";

// Environment variable used to define the target platform
pub(crate) const ENV_VAR_PLATFORM: &str = "DSH_API_PLATFORM";

// Environment variable used to define the client tenant
pub(crate) const ENV_VAR_TENANT: &str = "DSH_API_TENANT";

pub(crate) const ENV_VAR_PREFIX_PASSWORD: &str = "DSH_API_PASSWORD";
pub(crate) const ENV_VAR_PREFIX_PASSWORD_FILE: &str = "DSH_API_PASSWORD_FILE";

// # Create client password environment variable
//
// This function creates the environment variable used to define the client tenant's password
// from the platform name and the tenant name. The format of the environment variable is
// `DSH_API_PASSWORD_[platform_name]_[tenant_name]`,
// where the `platform_name` and the `tenant_name` will be converted to uppercase and
// `-` will be replaced by `_`.
//
// # Parameters
// * `platform` - target platform
// * `tenant_name` - client tenant name
//
// # Returns
// Client password environment variable.
pub(crate) fn password_environment_variable(platform: &DshPlatform, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    ENV_VAR_PREFIX_PASSWORD,
    platform.name().to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

// # Create client password file environment variable
//
// This function creates the environment variable used to define the client tenant's password file
// from the platform name and the tenant name. The format of the environment variable is
// `DSH_API_PASSWORD_FILE_[platform_name]_[tenant_name]`,
// where the `platform_name` and the `tenant_name` will be converted to uppercase and
// `-` will be replaced by `_`.
//
// # Parameters
// * `platform` - target platform
// * `tenant_name` - client tenant name
//
// # Returns
// Client password file environment variable.
pub(crate) fn password_file_environment_variable(platform: &DshPlatform, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    ENV_VAR_PREFIX_PASSWORD_FILE,
    platform.name().to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}

#[test]
fn test_dsh_api_error_is_send() {
  fn assert_send<T: Send>() {}
  assert_send::<DshApiError>();
}

#[test]
fn test_dsh_api_error_is_sync() {
  fn assert_sync<T: Sync>() {}
  assert_sync::<DshApiError>();
}
