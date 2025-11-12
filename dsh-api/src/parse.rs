//! # Parsers for formatted strings
//!
//! Module that contains parse functions for selected formatted strings as used in the DSH and
//! the DSH resource management API.
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Enum that describes an auth string. Auth strings are used in the `exposedPorts` section
/// of a service definition file and are deserialized into the  `auth` field of the
/// [`PortMapping`](crate::types::PortMapping) data structure.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum AuthString {
  /// Represents a basic authentication string, like
  /// `"basic-auth@<realm>:<username>:<password-hash>"` or one of the older formats
  /// that are still supported for backwards compatibility.
  /// The enum fields contain the optional realm string and the username.
  Basic(Option<String>, String),
  /// Represents a forward authentication string, like
  /// `"fwd-auth@<auth service endpoint>@<auth response headers>"`.
  /// The enum field contains the authentication service endpoint and the optional headers string.
  Fwd(String, Option<String>),
  /// Represents a tenant authentication string, like
  /// `"system-fwd-auth@<accepted roles>"`.
  /// The enum field contains the accepted roles.
  SystemFwd(String),
}

impl AuthString {
  /// # Create an `AuthString::Basic`
  ///
  /// # Parameters
  /// * `realm` - optional realm
  /// * `username` - mandatory username
  pub fn basic<T, U>(realm: Option<T>, username: U) -> Self
  where
    T: Into<String>,
    U: Into<String>,
  {
    Self::Basic(realm.map(Into::<String>::into), username.into())
  }

  /// # Create an `AuthString::Fwd`
  ///
  /// # Parameters
  /// * `endpoint` - the authentication service endpoint
  pub fn fwd<T, U>(endpoint: T, headers: Option<U>) -> Self
  where
    T: Into<String>,
    U: Into<String>,
  {
    Self::Fwd(endpoint.into(), headers.map(Into::into))
  }

  /// # Create an `AuthString::SystemFwd`
  ///
  /// # Parameters
  /// * `roles` - comma separated list of accepted roles
  pub fn system_fwd<T>(roles: T) -> Self
  where
    T: Into<String>,
  {
    Self::SystemFwd(roles.into())
  }
}

impl FromStr for AuthString {
  type Err = String;

  /// # Parse auth string
  ///
  /// # Example
  ///
  /// ```
  /// # use std::str::FromStr;
  /// # use dsh_api::parse::AuthString;
  /// assert_eq!(
  ///   AuthString::from_str("basic-auth@my-realm:my-username:$password-hash/"),
  ///   Ok(AuthString::basic(Some("my-realm"), "my-username"))
  /// );
  /// assert_eq!(
  ///   AuthString::from_str("fwd-auth@https://my-authentication-service.com@my-headers"),
  ///   Ok(AuthString::fwd("https://my-authentication-service.com", Some("my-headers")))
  /// );
  /// assert_eq!(
  ///   AuthString::from_str("system-fwd-auth@view,manage"),
  ///   Ok(AuthString::system_fwd("view,manage"))
  /// );
  /// ```
  ///
  /// # Parameters
  /// * `auth_string` - the auth string to be parsed
  ///
  /// # Returns
  /// When the provided string is valid, the method returns an instance of the `AuthString`
  /// struct, describing the auth string.
  fn from_str(auth_string: &str) -> Result<Self, Self::Err> {
    if let Some(basic_authentication_string) = auth_string.strip_prefix("basic-auth@") {
      parse_basic_authentication_string(basic_authentication_string)
    } else if let Some(fwd_auth_string) = auth_string.strip_prefix("fwd-auth@") {
      let split_string = fwd_auth_string.split("@").collect_vec();
      match split_string.first() {
        Some(authentication_service) => Ok(Self::fwd(*authentication_service, split_string.get(1).map(|headers| headers.to_string()))),
        None => Err(format!("invalid forward authentication string (\"{}\")", auth_string)),
      }
    } else if let Some(roles) = auth_string.strip_prefix("system-fwd-auth@") {
      Ok(Self::system_fwd(roles))
    } else {
      parse_basic_authentication_string(auth_string)
    }
  }
}

impl Display for AuthString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Basic(realm, username) => match realm {
        Some(realm) => write!(f, "basic@{}:{}", realm, username),
        None => write!(f, "basic@{}", username),
      },
      Self::Fwd(endpoint, headers) => match headers {
        Some(headers) => write!(f, "fwd@{}@{}", endpoint, headers),
        None => write!(f, "fwd@{}", endpoint),
      },
      Self::SystemFwd(roles) => write!(f, "sys-fwd@{}", roles),
    }
  }
}

/// Struct that describes an image string for a registry image.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct RegistryImage {
  /// Tenant
  pub tenant: String,
  /// Image identifier
  pub id: String,
  /// Image version
  pub version: String,
}

/// Struct that describes an image string for an app catalog image.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub struct AppImage {
  /// Stage of development (`draft` or `release`)
  pub stage: String,
  /// Supplier of the image (`klarrio` or `kpn`)
  pub supplier: String,
  /// Tenant
  pub tenant: String,
  /// Image identifier
  pub id: String,
  /// Image version
  pub version: String,
}

/// Enum that describes an image string. Image strings are used in the
/// service definition file and are deserialized into the `image` field of the
/// [`Application`](crate::types::Application) data structure.
#[derive(Clone, Debug, Deserialize, Hash, PartialEq, Serialize)]
pub enum ImageString {
  Registry(RegistryImage),
  App(AppImage),
  Unrecognized(String),
}

impl ImageString {
  /// # Create an `ImageString::Registry`
  ///
  /// # Parameters
  /// * `registry` - registry that contains the image (`app` or `registry`)
  /// * `tenant` - tenant
  /// * `id` - image identifier
  /// * `version` - image version
  pub fn registry(tenant: String, id: String, version: String) -> Self {
    Self::Registry(RegistryImage { tenant, id, version })
  }

  /// # Create an `ImageString::App`
  ///
  /// # Parameters
  /// * `stage` - stage of development (`draft` or `release`)
  /// * `supplier` - supplier of the image (`klarrio` or `kpn`)
  /// * `tenant` - tenant
  /// * `id` - image identifier
  /// * `version` - image version
  pub fn app(stage: String, supplier: String, tenant: String, id: String, version: String) -> Self {
    Self::App(AppImage { stage, supplier, tenant, id, version })
  }

  /// Get the image id
  pub fn id(&self) -> String {
    match self {
      ImageString::Registry(registry) => registry.id.clone(),
      ImageString::App(app) => app.id.clone(),
      ImageString::Unrecognized(image) => image.to_string(),
    }
  }

  /// Get the image id
  pub fn source(&self) -> &str {
    match self {
      ImageString::Registry(_) => "harbor",
      ImageString::App(_) => "app-catalog",
      ImageString::Unrecognized(_) => "",
    }
  }

  /// Get the image tenant
  pub fn tenant(&self) -> String {
    match self {
      ImageString::Registry(registry) => registry.tenant.clone(),
      ImageString::App(app) => app.tenant.clone(),
      ImageString::Unrecognized(_) => "".to_string(),
    }
  }

  /// Get the image version
  pub fn version(&self) -> String {
    match self {
      ImageString::Registry(registry) => registry.version.clone(),
      ImageString::App(app) => app.version.clone(),
      ImageString::Unrecognized(_) => "".to_string(),
    }
  }
}

impl From<&str> for ImageString {
  /// # Parse image string
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::parse::ImageString;
  /// assert_eq!(
  ///   ImageString::from(
  ///     "APPCATALOG_REGISTRY/dsh-appcatalog/tenant/my-tenant/\
  ///      1234/1234/draft/kpn/my-image:0.0.1"
  ///   ),
  ///   ImageString::app(
  ///     "draft".to_string(),
  ///     "kpn".to_string(),
  ///     "my-tenant".to_string(),
  ///     "my-image".to_string(),
  ///     "0.0.1".to_string()
  ///   )
  /// );
  /// assert_eq!(
  ///   ImageString::from("registry.cp.kpn-dsh.com/my-tenant/my-image:0.0.1"),
  ///   ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string())
  /// );
  /// ```
  ///
  /// # Parameters
  /// * `image_string` - the image string to be parsed
  ///
  /// # Returns
  /// When the provided string is valid, the method returns an `ImageString::App` or
  /// `ImageString::Registry` instance. When the string is invalid,
  /// a `ImageString::Unrecognized` will be returned.
  fn from(image_string: &str) -> Self {
    lazy_static! {
      static ref APP_CATALOG_IMAGE_REGEX: Regex =
        Regex::new(r"APPCATALOG_REGISTRY/dsh-appcatalog/tenant/([a-z0-9-_]+)/([0-9]+)/([0-9]+)/(release|draft)/(klarrio|kpn)/([a-zA-Z][a-zA-Z0-9-_]*):([a-zA-Z0-9-_.]*)").unwrap();
      static ref REGISTRY_IMAGE_REGEX: Regex = Regex::new(r"registry.cp.kpn-dsh.com/([a-z0-9-_]+)/([a-zA-Z][a-zA-Z0-9-_]*):([a-zA-Z0-9-_.]*)").unwrap();
    }
    match APP_CATALOG_IMAGE_REGEX.captures(image_string) {
      Some(captures) => Self::app(
        captures.get(4).map(|stage| stage.as_str().to_string()).unwrap_or_default(),
        captures.get(5).map(|supplier| supplier.as_str().to_string()).unwrap_or_default(),
        captures.get(1).map(|tenant| tenant.as_str().to_string()).unwrap_or_default(),
        captures.get(6).map(|id| id.as_str().to_string()).unwrap_or_default(),
        captures.get(7).map(|version| version.as_str().to_string()).unwrap_or_default(),
      ),
      None => match REGISTRY_IMAGE_REGEX.captures(image_string) {
        Some(registry_captures) => Self::registry(
          registry_captures.get(1).map(|tenant| tenant.as_str().to_string()).unwrap_or_default(),
          registry_captures.get(2).map(|id| id.as_str().to_string()).unwrap_or_default(),
          registry_captures.get(3).map(|version| version.as_str().to_string()).unwrap_or_default(),
        ),
        None => ImageString::Unrecognized(image_string.to_string()),
      },
    }
  }
}

impl Display for ImageString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ImageString::Registry(registry_image) => write!(f, "registry:{}:{}:{}", registry_image.tenant, registry_image.id, registry_image.version),
      ImageString::App(app_image) => write!(
        f,
        "app:{}:{}:{}:{}:{}",
        app_image.stage, app_image.supplier, app_image.tenant, app_image.id, app_image.version
      ),
      ImageString::Unrecognized(unrecognized_image) => write!(f, "{}", unrecognized_image),
    }
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum TopicString<'a> {
  Internal(&'a str, &'a str),
  Scratch(&'a str, &'a str),
  Stream(&'a str, &'a str),
}

impl<'a> TopicString<'a> {
  pub fn internal(name: &'a str, tenant: &'a str) -> Self {
    Self::Internal(name, tenant)
  }

  pub fn scratch(name: &'a str, tenant: &'a str) -> Self {
    Self::Scratch(name, tenant)
  }

  pub fn stream(name: &'a str, tenant: &'a str) -> Self {
    Self::Stream(name, tenant)
  }

  pub fn name(&self) -> &str {
    match self {
      TopicString::Internal(name, _) => name,
      TopicString::Scratch(name, _) => name,
      TopicString::Stream(name, _) => name,
    }
  }

  pub fn tenant(&self) -> &str {
    match self {
      TopicString::Internal(_, tenant) => tenant,
      TopicString::Scratch(_, tenant) => tenant,
      TopicString::Stream(_, tenant) => tenant,
    }
  }
}

impl<'a> TryFrom<&'a str> for TopicString<'a> {
  type Error = String;

  fn try_from(topic_string: &'a str) -> Result<Self, Self::Error> {
    parse_topic_string(topic_string)
  }
}

impl Display for TopicString<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      TopicString::Internal(name, tenant) => write!(f, "internal.{}.{}", name, tenant),
      TopicString::Scratch(name, tenant) => write!(f, "scratch.{}.{}", name, tenant),
      TopicString::Stream(name, tenant) => write!(f, "stream.{}.{}", name, tenant),
    }
  }
}

// Parse basic authentication string
pub fn parse_basic_authentication_string(basic_authentication_string: &str) -> Result<AuthString, String> {
  let parts = basic_authentication_string.split(":").collect_vec();
  if parts.len() == 2 {
    Ok(AuthString::basic(None::<String>, *parts.first().unwrap()))
  } else if parts.len() == 3 {
    Ok(AuthString::basic(Some(*parts.first().unwrap()), *parts.get(1).unwrap()))
  } else {
    Err(format!("invalid basic authentication string (\"{}\")", basic_authentication_string))
  }
}

/// # Parse function string
///
/// # Example
///
/// ```
/// # use dsh_api::parse::parse_function1;
/// assert_eq!(parse_function1("{ function('parameter') }", "function"), Ok("parameter"));
/// ```
///
/// # Parameters
/// * `string` - the function string to be parsed
/// * `f_name` - the name of the function to match
///
/// # Returns
/// When the provided string is valid, the method returns the function parameter value
pub fn parse_function1<'a>(string: &'a str, f_name: &str) -> Result<&'a str, String> {
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\{\s*([a-z][a-z0-9_]*)\(\s*'([a-zA-Z0-9_\.-]*)'\s*\)\s*\}").unwrap();
  }
  match REGEX.captures(string).map(|captures| {
    (
      captures.get(1).map(|first_match| first_match.as_str()),
      captures.get(2).map(|second_match| second_match.as_str()),
    )
  }) {
    Some((Some(function), Some(par))) if function == f_name => Ok(par),
    _ => Err(format!("invalid {} string (\"{}\")", f_name, string)),
  }
}

/// # Parse function string with two parameters
///
/// # Example
///
/// ```
/// # use dsh_api::parse::parse_function2;
/// assert_eq!(
///   parse_function2("{ function2('parameter1', 'parameter2') }", "function2"),
///   Ok(("parameter1", "parameter2"))
/// );
/// ```
///
/// # Parameters
/// * `string` - the function string to be parsed
/// * `f_name` - the name of the function to match
///
/// # Returns
/// When the provided string is valid, the method returns the two function parameter values
pub fn parse_function2<'a>(string: &'a str, f_name: &str) -> Result<(&'a str, &'a str), String> {
  lazy_static! {
    static ref REGEX: Regex = Regex::new(r"\{\s*([a-z][a-z0-9_]*)\(\s*'([a-zA-Z0-9_\.-]*)'\s*,\s*'([a-zA-Z0-9_\.-]*)'\s*\)\s*\}").unwrap();
  }
  match REGEX.captures(string).map(|captures| {
    (
      captures.get(1).map(|first_match| first_match.as_str()),
      captures.get(2).map(|second_match| second_match.as_str()),
      captures.get(3).map(|second_match| second_match.as_str()),
    )
  }) {
    Some((Some(function), Some(par1), Some(par2))) if function == f_name => Ok((par1, par2)),
    _ => Err(format!("invalid {} string (\"{}\")", f_name, string)),
  }
}

/// # Parse function string with one or two parameters
///
/// # Example
///
/// ```
/// # use dsh_api::parse::parse_function;
/// assert_eq!(parse_function("{ function1('parameter') }", "function1"), Ok(("parameter", None)));
/// assert_eq!(
///   parse_function("{ function2('parameter1', 'parameter2') }", "function2"),
///   Ok(("parameter1", Some("parameter2")))
/// );
/// ```
///
/// # Parameters
/// * `string` - the function string to be parsed
/// * `f_name` - the name of the function to match
///
/// # Returns
/// When the provided string is valid, the method returns the two function parameter values,
/// the second of which can be `None`
pub fn parse_function<'a>(string: &'a str, f_name: &str) -> Result<(&'a str, Option<&'a str>), String> {
  match parse_function2(string, f_name) {
    Ok((first_parameter, second_parameter)) => Ok((first_parameter, Some(second_parameter))),
    Err(_) => match parse_function1(string, f_name) {
      Ok(parameter) => Ok((parameter, None)),
      Err(_) => Err(format!("invalid {} string (\"{}\")", f_name, string)),
    },
  }
}

/// # Parse volume string
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use dsh_api::parse::parse_volume_string;
/// assert_eq!(parse_volume_string("{ volume('my_volume') }"), Ok("my_volume"));
/// ```
///
/// # Parameters
/// * `volume_string` - the volume string to be parsed
///
/// # Returns
/// When the provided string is valid, the method returns the volume name
pub fn parse_volume_string(volume_string: &str) -> Result<&str, String> {
  parse_function1(volume_string, "volume")
}

/// # Parse topic string
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use dsh_api::parse::{parse_topic_string, TopicString};
/// assert_eq!(
///   parse_topic_string("scratch.topic-name.my-tenant"),
///   Ok(TopicString::scratch("topic-name", "my-tenant"))
/// );
/// ```
///
/// # Parameters
/// * `volume_string` - the volume string to be parsed
///
/// # Returns
/// When the provided string is valid, the method returns the volume name
pub fn parse_topic_string<'a>(topic_string: &'a str) -> Result<TopicString<'a>, String> {
  lazy_static! {
    static ref TOPIC_REGEX: Regex = Regex::new(r"(internal|stream|scratch)\.([a-z][a-z0-9-]*)\.([a-z][a-z0-9-]*)").unwrap();
  }
  match TOPIC_REGEX.captures(topic_string) {
    Some(registry_captures) => {
      let name = registry_captures.get(2).map(|name| name.as_str()).unwrap();
      let tenant = registry_captures.get(3).map(|tenant| tenant.as_str()).unwrap();
      match registry_captures.get(1).map(|kind| kind.as_str()) {
        Some("internal") => Ok(TopicString::internal(name, tenant)),
        Some("stream") => Ok(TopicString::stream(name, tenant)),
        Some("scratch") => Ok(TopicString::scratch(name, tenant)),
        _ => unreachable!(),
      }
    }
    None => Err(format!("illegal topic name {}", topic_string)),
  }
}
