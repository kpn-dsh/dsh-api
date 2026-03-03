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
//!
//! * First you need to get an instance of
//!   [`DshApiClientFactory`](dsh_api_client_factory::DshApiClientFactory) or [`DshApiPlatformClientFactory`](dsh_api_client_factory::DshApiPlatformClientFactory).
//! * Once you have the client factory instance, you can call its
//!   [`client()`](dsh_api_client_factory::DshApiClientFactory::client) method to create the
//!   [`DshApiClient`].
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
//! # use dsh_api::error::DshApiResult;
//! # async fn hide() -> DshApiResult<()> {
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
//! # use dsh_api::error::DshApiResult;
//! # async fn hide() -> DshApiResult<()> {
//! let tenant = DshApiTenant::new("my-tenant", DshPlatform::try_from("np-aws-lz-dsh")?);
//! let password = "...";
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
//! * `generic` - Enables the generic methods, which allows calling all api operations by name.
//! * `manage` -  Enables the manage modules [`stream`] and [`tenant`], which support creating
//!   managed streams and tenants. This feature is only useful when you have the proper
//!   authorizations for these capabilities.
//! * `robot` - Enables the
//!   [`post_robot_generate_secret()`](DshApiClient::post_robot_generate_secret) operation, which
//!   will generate a new robot password, invalidating the old password.

/// # Types generated from openapi file
#[allow(clippy::clone_on_copy)]
#[allow(clippy::derivable_impls)]
#[allow(clippy::large_enum_variant)]
pub mod types {
  include!(concat!(env!("OUT_DIR"), "/types.rs"));
}

include!(concat!(env!("OUT_DIR"), "/methods.rs"));

/// Openapi specification version 1.11.1
pub static OPENAPI_SPEC: &str = include_str!(concat!(env!("OUT_DIR"), "/openapi.json"));

/// Specification of default platforms
pub static DEFAULT_PLATFORMS: &str = include_str!("../default-platforms.json");

use crate::error::DshApiError;
use crate::version::Version;
use chrono::{TimeZone, Utc};
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::sync::LazyLock;

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
pub mod error;
#[cfg(feature = "generic")]
pub mod generic;
pub mod manifest;
pub mod new;
pub mod nodepool;
pub mod parse;
pub mod platform;
pub mod proxy;
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
/// # use dsh_api::version::Version;
/// assert_eq!(dsh_api::crate_version(), &Version::new(0, 9, 0, None));
/// ```
pub fn crate_version() -> &'static Version {
  static CRATE_VERSION: LazyLock<Version> = LazyLock::new(|| Version::new(0, 9, 0, None));
  &CRATE_VERSION
}

/// # Returns the version of the openapi spec
///
/// Version number of the openapi file that the crate has been generated from.
///
/// ## Example
///
/// ```
/// # use dsh_api::version::Version;
/// assert_eq!(dsh_api::openapi_version(), &Version::new(1, 11, 1, None));
/// ```
pub fn openapi_version() -> &'static Version {
  DshApiClient::api_version()
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
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantApp {
  /// App identifier
  pub app_id: String,
  /// Resources that the app depends on
  pub resources: Vec<String>,
}

impl DependantApp {
  pub fn new(app_id: String, resources: Vec<String>) -> Self {
    Self { app_id, resources }
  }
}

impl Display for DependantApp {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: {}", self.app_id, self.resources.join(", "))
  }
}

impl PartialOrd<Self> for DependantApp {
  /// Ordering uses `app_id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DependantApp {
  /// Ordering uses `app_id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.app_id.cmp(&other.app_id)
  }
}

/// # Describes an application dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant application.
/// This struct represents one usage of the resource by an application.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantApplication<T> {
  /// Identifies the dependant application.
  pub application_id: String,
  /// Number of instances of the dependant application.
  pub instances: u64,
  /// Injections that the dependencies originate from.
  pub injections: Vec<T>,
}

impl<T> DependantApplication<T> {
  pub fn new(application_id: String, instances: u64, injections: Vec<T>) -> Self {
    Self { application_id, instances, injections }
  }
}

impl<T> Display for DependantApplication<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.application_id)?;
    if !self.injections.is_empty() {
      write!(f, ": {}", self.injections.iter().map(|inj| inj.to_string()).collect_vec().join(", "))?
    }
    Ok(())
  }
}

impl<T: Ord> PartialOrd<Self> for DependantApplication<T> {
  /// Ordering uses `application_id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: Ord> Ord for DependantApplication<T> {
  /// Ordering uses `application_id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.application_id.cmp(&other.application_id)
  }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CertificateSecretKind {
  CertChainSecret,
  KeySecret,
  PassphraseSecret,
}

impl Display for CertificateSecretKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::CertChainSecret => write!(f, "cert-chain-secret"),
      Self::KeySecret => write!(f, "key-secret"),
      Self::PassphraseSecret => write!(f, "passphrase-secret"),
    }
  }
}

/// # Describes a certificate dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret) is
/// used by a certificate. This struct represents one usage of the resource by a certificate.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantCertificate {
  /// Identifies the dependant certificate
  pub certificate_id: String,
  /// Kind of secret for the Certificate dependency.
  pub secret_kind: CertificateSecretKind,
}

impl DependantCertificate {
  pub fn new(certificate_id: String, secret_kind: CertificateSecretKind) -> Self {
    Self { certificate_id, secret_kind }
  }
}

impl Display for DependantCertificate {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.certificate_id, self.secret_kind)
  }
}

impl PartialOrd<Self> for DependantCertificate {
  /// Ordering uses `certificate_id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DependantCertificate {
  /// Ordering uses `certificate_id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.certificate_id.cmp(&other.certificate_id)
  }
}

/// # Describes a proxy dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// or a certificate) is used by a proxy.
/// This struct represents one usage of the resource by a proxy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantProxy {
  /// Identifies the dependant proxy
  pub proxy_id: String,
  /// Number of instances of the dependant application
  pub instances: u64,
}

impl DependantProxy {
  pub fn new(proxy_id: String, instances: u64) -> Self {
    Self { proxy_id, instances }
  }
}

impl Display for DependantProxy {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.proxy_id)
  }
}

impl PartialOrd<Self> for DependantProxy {
  /// Ordering uses `proxy_id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DependantProxy {
  /// Ordering uses `proxy_id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.proxy_id.cmp(&other.proxy_id)
  }
}

/// # Describes a Trifonius dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant application.
/// This struct represents one usage of the resource by an application.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DependantTrifonius<T> {
  /// Identifies the dependant application.
  pub trifonius_id: String,
  /// Number of instances of the dependant application.
  pub instances: u64,
  /// Injections that the dependencies originate from.
  pub injections: Vec<T>,
}

static TRIFONIUS_ID_REGEX_NEW: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([a-z])-([a-z0-9]{8})-([a-z])-([a-z0-9]{8})-([a-z])-(.+)$").unwrap());
static TRIFONIUS_ID_REGEX_OLD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([a-z])-([a-z0-9]{6})-([a-z])-([a-z0-9]{6})-([a-z])-([a-z0-9]{6})-(.+)$").unwrap());

/// Returns a string slice with the Trifonius prefix removed
///
/// If the string starts with a Trifonius prefix this function returns the substring after
/// the prefix, wrapped in `Some`. If the string does not start with a Trifonius prefix
/// `None` will be returned.
///
/// There are two types of Trifonius prefixes:
/// * `f-df87c48f-n-5104548e-p-sleep` (new syntax),
/// * `p-6aythn-v-a6hrkl-n-7bvetd-metadata-manager` (old syntax).
///
/// Although the old syntax will not be used anymore, it is still supported for older builds.
///
/// # Examples
///
/// ```
/// # use dsh_api::strip_trifonius_prefix;
/// assert_eq!(strip_trifonius_prefix("f-df87c48f-n-5104548e-p-sleep"), Some("sleep"));
/// ```
pub fn strip_trifonius_prefix(id: &str) -> Option<&str> {
  match TRIFONIUS_ID_REGEX_NEW.captures(id) {
    Some(captures_new) => captures_new.get(6).map(|matching| matching.as_str()),
    None => match TRIFONIUS_ID_REGEX_OLD.captures(id) {
      Some(captures_old) => captures_old.get(7).map(|matching| matching.as_str()),
      None => None,
    },
  }
}

impl<T> DependantTrifonius<T> {
  pub fn new(application_id: String, instances: u64, injections: Vec<T>) -> Self {
    Self { trifonius_id: application_id, instances, injections }
  }

  pub fn try_new(application_id: &str, instances: u64, injections: Vec<T>) -> DshApiResult<Self> {
    if let Some(trifonius_prefix) = strip_trifonius_prefix(application_id) {
      Ok(Self { trifonius_id: trifonius_prefix.to_string(), instances, injections })
    } else {
      Err(DshApiError::NotFound { message: None })
    }
  }
}

impl<T> Display for DependantTrifonius<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "tr:{}", self.trifonius_id)?;
    if !self.injections.is_empty() {
      write!(f, ": {}", self.injections.iter().map(|inj| inj.to_string()).collect_vec().join(", "))?
    }
    Ok(())
  }
}

impl<T: Ord> PartialOrd<Self> for DependantTrifonius<T> {
  /// Ordering uses `application_id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl<T: Ord> Ord for DependantTrifonius<T> {
  /// Ordering uses `application_id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.trifonius_id.cmp(&other.trifonius_id)
  }
}

/// # Describes a app or application dependency
///
/// There are a number of methods that return whether a certain resource (e.g. a secret,
/// a volume or an environment variable) is used by a dependant app or application.
/// This enum represents one usage of the resource by an app or application.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Dependant<T> {
  /// Identifies an app dependent on the resource
  App { app: DependantApp },
  /// Identifies an application dependent on the resource
  Application { application: DependantApplication<T> },
  /// Identifies a certificate dependent on the resource
  Certificate { certificate: DependantCertificate },
  /// Identifies a proxy dependent on the resource
  Proxy { proxy: DependantProxy },
  /// Identifies a Trifonius dependent on the resource
  Trifonius { trifonius: DependantTrifonius<T> },
}

impl<T> Dependant<T> {
  pub fn app(app_id: String, resources: Vec<String>) -> Self {
    Dependant::App { app: DependantApp::new(app_id, resources) }
  }

  pub fn application(application_id: String, instances: u64, injections: Vec<T>) -> Self {
    Dependant::Application { application: DependantApplication::new(application_id, instances, injections) }
  }

  pub fn certificate(certificate_id: String, secret_kind: CertificateSecretKind) -> Self {
    Dependant::Certificate { certificate: DependantCertificate::new(certificate_id, secret_kind) }
  }

  pub fn proxy(proxy_id: String, instances: u64) -> Self {
    Dependant::Proxy { proxy: DependantProxy::new(proxy_id, instances) }
  }

  pub fn service(service_id: &str, instances: u64, injections: Vec<T>) -> Self {
    if let Some(trifonius_id) = strip_trifonius_prefix(service_id) {
      Self::trifonius(trifonius_id.to_string(), instances, injections)
    } else {
      Self::application(service_id.to_string(), instances, injections)
    }
  }

  pub fn trifonius(trifonius_id: String, instances: u64, injections: Vec<T>) -> Self {
    Dependant::Trifonius { trifonius: DependantTrifonius::new(trifonius_id, instances, injections) }
  }

  pub fn id(&self) -> &str {
    match self {
      Self::App { app } => &app.app_id,
      Self::Application { application } => &application.application_id,
      Self::Certificate { certificate } => &certificate.certificate_id,
      Self::Proxy { proxy } => &proxy.proxy_id,
      Self::Trifonius { trifonius } => &trifonius.trifonius_id,
    }
  }
}

impl<T: Display> Display for Dependant<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::App { app } => Display::fmt(app, f),
      Self::Application { application } => Display::fmt(application, f),
      Self::Certificate { certificate } => Display::fmt(certificate, f),
      Self::Proxy { proxy } => Display::fmt(proxy, f),
      Self::Trifonius { trifonius } => Display::fmt(trifonius, f),
    }
  }
}

impl<T: PartialOrd> PartialOrd<Self> for Dependant<T> {
  /// Ordering uses `id` only
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.id().cmp(other.id()))
  }
}

impl<T: Ord> Ord for Dependant<T> {
  /// Ordering uses `id` only
  fn cmp(&self, other: &Self) -> Ordering {
    self.id().cmp(other.id())
  }
}

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
