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
//!     [`DshApiClientFactory::default()`](dsh_api_client_factory::DshApiClientFactory::default)
//!     function, which returns a client factory configured from
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
//! # [`types`]
//!
//! For their parameters and return values the methods and functions in the crate
//! make use of rust `struct`s that where generated from the DSH resource management API
//! Openapi specification (version 1.10.0).
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
//! * Some [selected types](default) also have an implementation of the [`Default`] trait.
//! * Some [selected types](display) also have an implementation of the [`Display`] trait.
//! * Some [selected types](new) also have a `new` function.
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
//! * `generic` - Enables the generic methods.
//! * `manage` -  Enables the manage methods.
//! * `robot` - Enables the robot operation.
/// # Types generated from openapi file
pub use crate::generated::types;
use std::cmp::Ordering;

#[allow(dead_code)]
pub(crate) mod generated {
  include!(concat!(env!("OUT_DIR"), "/progenitor_client.rs"));
}

include!(concat!(env!("OUT_DIR"), "/wrapped.rs"));

/// Openapi specification version 1.10.0
pub static OPENAPI_SPEC: &str = include_str!(concat!(env!("OUT_DIR"), "/openapi.json"));

/// Specification of default platforms
pub static DEFAULT_PLATFORMS: &str = include_str!("../default-platforms.json");

use crate::token_fetcher::ManagementApiTokenError;
use crate::types::error::ConversionError;
use chrono::{TimeZone, Utc};
use itertools::Itertools;
use log::{debug, error, trace};
use progenitor_client::Error as ProgenitorError;
use reqwest::StatusCode as ReqwestStatusCode;
use reqwest::{Error as ReqwestError, Response};
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeJsonError;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use std::str::Utf8Error;

pub mod app;
pub mod application;
pub mod application_types;
pub mod bucket;
pub mod certificate;
pub mod database;
pub mod default;
pub mod display;
pub mod dsh_api_client;
pub mod dsh_api_client_factory;
pub mod dsh_api_tenant;
pub mod dsh_jwt;
#[cfg(feature = "generic")]
pub mod generic;
pub mod manifest;
pub mod new;
pub mod parse;
pub mod platform;
pub mod query_processor;
pub mod secret;
#[cfg(feature = "manage")]
pub mod stream;
#[cfg(feature = "manage")]
pub mod tenant;
pub mod token_fetcher;
pub mod topic;
pub mod version;
pub mod vhost;
pub mod volume;

/// # Returns the version of the lib crate
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::crate_version(), "0.8.1");
/// ```
pub fn crate_version() -> &'static str {
  "0.8.1"
}

/// # Returns the version of the openapi spec
///
/// Version number of the openapi file that the crate has been generated from.
///
/// ## Example
///
/// ```
/// assert_eq!(dsh_api::openapi_version(), "1.10.0");
/// ```
pub fn openapi_version() -> &'static str {
  generated::Client::new("").api_version()
}

/// # Indicates access rights
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AccessRights {
  /// Indicates read access to a resource
  Read,
  /// Indicates read and write access to a resource
  ReadWrite,
  /// Indicates write access to a resource
  Write,
}

/// # Describes an app dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant app.
/// This struct represents one usage of the resource by an app.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantApp {
  /// App identifier
  pub app_id: String,
  /// Resources that the app depends on
  pub resources: Vec<String>,
}

impl DependantApp {
  pub fn new(app_id: String, resources: Vec<String>) -> Self {
    DependantApp { app_id, resources }
  }
}

impl Display for DependantApp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.app_id, self.resources.join(", "))
  }
}

/// # Describes an application dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant application.
/// This struct represents one usage of the resource by an application.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantApplication<T> {
  /// Identifies the dependant application
  pub application_id: String,
  /// Number of instances of the dependant application
  pub instances: u64,
  /// Injections that the dependencies originate from
  pub injections: Vec<T>,
}

impl<T> DependantApplication<T> {
  pub fn new(application_id: String, instances: u64, injections: Vec<T>) -> Self {
    DependantApplication { application_id, instances, injections }
  }
}

impl<T: Display> Display for DependantApplication<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.application_id)?;
    if !self.injections.is_empty() {
      write!(f, ": {}", self.injections.iter().map(|inj| inj.to_string()).collect_vec().join(", "))?
    }
    Ok(())
  }
}

/// # Describes a app or application dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant app or application.
/// This enum represents one usage of the resource by an app or application.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Dependant<T> {
  /// Identifies an app dependent on the resource
  App(DependantApp),
  /// Identifies an application dependent on the resource
  Application(DependantApplication<T>),
}

impl<T> Dependant<T> {
  pub fn app(app_id: String, resources: Vec<String>) -> Self {
    Dependant::App(DependantApp { app_id, resources })
  }

  pub fn application(application_id: String, instances: u64, injections: Vec<T>) -> Self {
    Dependant::Application(DependantApplication { application_id, instances, injections })
  }
}

impl<T: Display> Display for Dependant<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Dependant::App(app) => Display::fmt(app, f),
      Dependant::Application(application) => Display::fmt(application, f),
    }
  }
}

impl<T: PartialOrd> PartialOrd<Self> for Dependant<T> {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self {
      Dependant::App(app) => match other {
        Dependant::App(other_app) => Some(app.app_id.cmp(&other_app.app_id)),
        Dependant::Application(other_application) => Some(app.app_id.cmp(&other_application.application_id)),
      },
      Dependant::Application(application) => match other {
        Dependant::App(other_app) => Some(application.application_id.cmp(&other_app.app_id)),
        Dependant::Application(other_application) => Some(application.application_id.cmp(&other_application.application_id)),
      },
    }
  }
}

impl<T: Ord> Ord for Dependant<T> {
  fn cmp(&self, other: &Self) -> Ordering {
    match self {
      Dependant::App(self_app) => match other {
        Dependant::App(other_app) => self_app.app_id.cmp(&other_app.app_id),
        Dependant::Application(other_application) => self_app.app_id.cmp(&other_application.application_id),
      },
      Dependant::Application(self_application) => match other {
        Dependant::App(other_app) => self_application.application_id.cmp(&other_app.app_id),
        Dependant::Application(other_application) => self_application.application_id.cmp(&other_application.application_id),
      },
    }
  }
}

/// Describes an API error
#[derive(Debug)]
pub enum DshApiError {
  BadRequest(String),
  Configuration(String),
  NotAuthorized(Option<String>),
  NotFound(Option<String>),
  Parameter(String),
  Unexpected(String, Option<String>),
  Unprocessable(Option<String>),
}

/// Generic result type
pub type DshApiResult<T> = Result<T, DshApiError>;

impl AccessRights {
  /// Checks whether read access is granted
  pub fn has_read_access(&self) -> bool {
    self == &Self::Read || self == &Self::ReadWrite
  }

  /// Checks whether write access is granted
  pub fn has_write_access(&self) -> bool {
    self == &Self::Write || self == &Self::ReadWrite
  }

  pub fn from(read_access: bool, write_access: bool) -> Option<Self> {
    match (read_access, write_access) {
      (false, false) => None,
      (false, true) => Some(AccessRights::Write),
      (true, false) => Some(AccessRights::Read),
      (true, true) => Some(AccessRights::ReadWrite),
    }
  }
}

impl Display for AccessRights {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Read => write!(f, "read"),
      Self::ReadWrite => write!(f, "read/write"),
      Self::Write => write!(f, "write"),
    }
  }
}

impl StdError for DshApiError {}

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::BadRequest(message) => write!(f, "{}", message),
      DshApiError::Configuration(message) => write!(f, "{}", message),
      DshApiError::NotAuthorized(cause) => match cause {
        Some(cause_message) => write!(f, "not authorized ({})", cause_message),
        None => write!(f, "not authorized"),
      },
      DshApiError::NotFound(cause) => match cause {
        Some(cause_message) => write!(f, "not found ({})", cause_message),
        None => write!(f, "not found"),
      },
      DshApiError::Parameter(message) => write!(f, "{}", message),
      DshApiError::Unexpected(message, cause) => match cause {
        Some(cause) => write!(f, "unexpected error ({}, {})", message, cause),
        None => write!(f, "unexpected error ({})", message),
      },
      DshApiError::Unprocessable(message) => match message {
        Some(message) => write!(f, "unprocessable entity ({})", message),
        None => write!(f, "unprocessable entity"),
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
          DshApiError::NotAuthorized(None)
        } else {
          let message = format!("unexpected error fetching token (status code {})", status_code);
          error!("{}", message);
          debug!("{:?}", error_body);
          DshApiError::Unexpected(message, Some(error.to_string()))
        }
      }
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
      ProgenitorError::ErrorResponse(progenitor_response_value) => match progenitor_response_value.status() {
        ReqwestStatusCode::BAD_REQUEST => Self::BadRequest("".to_string()),
        ReqwestStatusCode::FORBIDDEN => Self::NotAuthorized(None),
        ReqwestStatusCode::METHOD_NOT_ALLOWED => Self::NotAuthorized(None),
        ReqwestStatusCode::NOT_FOUND => Self::NotFound(None),
        ReqwestStatusCode::UNAUTHORIZED => Self::NotAuthorized(None),
        ReqwestStatusCode::UNPROCESSABLE_ENTITY => Self::Unprocessable(None),
        other_status_code => Self::Unexpected(format!("unexpected response {}", other_status_code), None),
      },
      ProgenitorError::ResponseBodyError(ref reqwest_error) => Self::Unexpected(
        format!("response body error (reqwest error: {})", reqwest_error),
        Some(progenitor_error.to_string()),
      ),
      ProgenitorError::InvalidResponsePayload(ref _bytes, ref json_error) => {
        Self::Unexpected(format!("invalid response payload (json error: {})", json_error), Some(progenitor_error.to_string()))
      }
      ProgenitorError::UnexpectedResponse(reqwest_response) => {
        trace!("unexpected progenitor response\n{:#?}", &reqwest_response);
        match &reqwest_response.status().clone() {
          &ReqwestStatusCode::BAD_REQUEST => Self::BadRequest(Self::error_from_reqwest_response(reqwest_response).await.unwrap_or_default()),
          &ReqwestStatusCode::FORBIDDEN => Self::NotAuthorized(Self::error_from_reqwest_response(reqwest_response).await),
          &ReqwestStatusCode::METHOD_NOT_ALLOWED => Self::NotAuthorized(Self::error_from_reqwest_response(reqwest_response).await),
          &ReqwestStatusCode::NOT_FOUND => Self::NotFound(Self::error_from_reqwest_response(reqwest_response).await),
          &ReqwestStatusCode::UNAUTHORIZED => Self::NotAuthorized(Self::error_from_reqwest_response(reqwest_response).await),
          &ReqwestStatusCode::UNPROCESSABLE_ENTITY => Self::Unprocessable(Self::error_from_reqwest_response(reqwest_response).await),
          other_status_code => Self::Unexpected(
            format!("unexpected response {}", other_status_code),
            Self::error_from_reqwest_response(reqwest_response).await,
          ),
        }
      }
      ProgenitorError::PreHookError(string) => Self::Unexpected(format!("pre-hook error ({})", string), None),
    }
  }

  async fn error_from_reqwest_response(reqwest_response: Response) -> Option<String> {
    match reqwest_response.text().await {
      Ok(error_text) => Some(error_text),
      Err(response_error) => Some(response_error.to_string()),
    }
    .filter(|m| !m.is_empty())
  }
}

// Environment variable used to specify the name of a file with an alternative list of platforms
pub(crate) const ENV_VAR_PLATFORMS_FILE_NAME: &str = "DSH_API_PLATFORMS_FILE";

// Environment variable used to define the target platform
pub(crate) const ENV_VAR_PLATFORM: &str = "DSH_API_PLATFORM";

// Environment variable used to define the client tenant
pub(crate) const ENV_VAR_TENANT: &str = "DSH_API_TENANT";

// Converts epoch timestamp in seconds to utc representation
pub(crate) fn epoch_seconds_to_string(timestamp: impl Into<i64>) -> String {
  Utc.timestamp_opt(timestamp.into(), 0).single().map(|ts| ts.to_string()).unwrap_or_default()
}

// Converts epoch timestamp in milliseconds to utc representation
pub(crate) fn epoch_milliseconds_to_string(timestamp: impl Into<i64>) -> String {
  epoch_seconds_to_string(timestamp.into() / 1000)
}

#[test]
fn test_epoch_seconds_to_string() {
  const REPRESENTATION: &str = "2000-01-01 00:00:00 UTC";
  assert_eq!(epoch_seconds_to_string(946684800_i64), REPRESENTATION);
  assert_eq!(epoch_seconds_to_string(946684800_u64 as i64), REPRESENTATION);
  assert_eq!(epoch_seconds_to_string(946684800_u128 as i64), REPRESENTATION);
  assert_eq!(epoch_seconds_to_string(946684800.0_f64 as i64), REPRESENTATION);
}

#[test]
fn test_epoch_milliseconds_to_string() {
  const REPRESENTATION: &str = "2000-01-01 00:00:00 UTC";
  assert_eq!(epoch_milliseconds_to_string(946684800000_i64), REPRESENTATION);
  assert_eq!(epoch_milliseconds_to_string(946684800000_u64 as i64), REPRESENTATION);
  assert_eq!(epoch_milliseconds_to_string(946684800000_u128 as i64), REPRESENTATION);
  assert_eq!(epoch_milliseconds_to_string(946684800000.0_f64 as i64), REPRESENTATION);
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
