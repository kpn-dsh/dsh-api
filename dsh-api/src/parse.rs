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

/// Structure that describes a vhost string. Vhost strings are used in the `exposedPorts` section
/// of a service definition file and are deserialized into the `vhost` field of the
/// [`PortMapping`](crate::types::PortMapping) data structure.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct VhostString {
  /// Domain name of the vhost
  pub vhost_name: String,
  /// Indicates whether the vhost name contains the substring `.kafka`
  pub kafka: bool,
  /// Optional tenant name
  pub tenant_name: Option<String>,
  /// Optional zone (`public` or `private`)
  pub zone: Option<String>,
}

impl VhostString {
  /// # Create a `VhostString`
  ///
  /// # Parameters
  /// * `vhost_name` - mandatory identifier of the vhost
  /// * `kafka` - whether the vhost name contains the substring `.kafka`
  /// * `tenant_name` - optional tenant name
  /// * `zone` - optional zone, typically `private` or `public`
  pub fn new<T, U, V>(vhost_name: T, kafka: bool, tenant_name: Option<U>, zone: Option<V>) -> Self
  where
    T: Into<String>,
    U: Into<String>,
    V: Into<String>,
  {
    Self { vhost_name: vhost_name.into(), kafka, tenant_name: tenant_name.map(Into::<String>::into), zone: zone.map(Into::<String>::into) }
  }
}

impl FromStr for VhostString {
  type Err = String;

  /// # Parse vhost string
  ///
  /// Multiple vhosts using the `join` function are not supported.
  ///
  /// # Example
  ///
  /// ```
  /// # use std::str::FromStr;
  /// # use dsh_api::parse::VhostString;
  /// assert_eq!(
  ///   VhostString::from_str("{ vhost('my-vhost-name') }"),
  ///   Ok(VhostString::new("my-vhost-name".to_string(), false, None::<String>, None::<String>))
  /// );
  /// assert_eq!(
  ///   VhostString::from_str("{ vhost('my-vhost-name.kafka.my-tenant','public') }"),
  ///   Ok(VhostString::new(
  ///     "my-vhost-name".to_string(),
  ///     true,
  ///     Some("my-tenant".to_string()),
  ///     Some("public".to_string())
  ///   ))
  /// );
  /// ```
  ///
  /// # Parameters
  /// * `vhost_string` - the vhost string to be parsed
  ///
  /// # Returns
  /// When the provided string is valid, the method returns an instance of the `VhostString`
  /// struct, describing the auth string.
  fn from_str(vhost_string: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref VALUE_REGEX: Regex = Regex::new(r"([a-zA-Z0-9_-]+)(\.kafka)?(?:\.([a-zA-Z0-9_-]+))?").unwrap();
    }
    let (value_string, zone) = match parse_function(vhost_string, "vhost") {
      Ok(value_string) => (value_string, None),
      Err(_) => parse_function2(vhost_string, "vhost").map(|(value_string, zone_string)| (value_string, Some(zone_string.to_string())))?,
    };
    VALUE_REGEX
      .captures(value_string)
      .map(|captures| {
        VhostString::new(
          captures.get(1).map(|vhost_match| vhost_match.as_str()).unwrap_or_default(),
          captures.get(2).is_some(),
          captures.get(3).map(|tenant_match| tenant_match.as_str()),
          zone,
        )
      })
      .ok_or(format!("invalid value in vhost string (\"{}\")", vhost_string))
  }
}

impl Display for VhostString {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.vhost_name)?;
    if self.kafka {
      write!(f, ".kafka")?;
    }
    if let Some(tenant_name) = &self.tenant_name {
      write!(f, ".{}", tenant_name)?;
    }
    if let Some(zone) = &self.zone {
      write!(f, ".{}", zone)?;
    }
    Ok(())
  }
}

/// # Parse bucket string
///
/// # Example
///
/// ```
/// # use std::str::FromStr;
/// # use dsh_api::parse::parse_bucket_string;
/// assert_eq!(parse_bucket_string("{ bucket_name('my_bucket_name') }"), Ok("my_bucket_name"));
/// ```
///
/// # Parameters
/// * `bucket_string` - the bucket string to be parsed
///
/// # Returns
/// When the provided string is valid, the method returns the bucket name
pub fn parse_bucket_string(bucket_string: &str) -> Result<&str, String> {
  parse_function(bucket_string, "bucket_name")
}

/// # Parse function string
///
/// # Example
///
/// ```
/// # use dsh_api::parse::parse_function;
/// assert_eq!(parse_function("{ function('parameter') }", "function"), Ok("parameter"));
/// ```
///
/// # Parameters
/// * `string` - the function string to be parsed
/// * `f_name` - the name of the function to match
///
/// # Returns
/// When the provided string is valid, the method returns the function parameter value
pub fn parse_function<'a>(string: &'a str, f_name: &str) -> Result<&'a str, String> {
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
  parse_function(volume_string, "volume")
}

// Parse basic authentication string
fn parse_basic_authentication_string(basic_authentication_string: &str) -> Result<AuthString, String> {
  let parts = basic_authentication_string.split(":").collect_vec();
  if parts.len() == 2 {
    Ok(AuthString::basic(None::<String>, *parts.first().unwrap()))
  } else if parts.len() == 3 {
    Ok(AuthString::basic(Some(*parts.first().unwrap()), *parts.get(1).unwrap()))
  } else {
    Err(format!("invalid basic authentication string (\"{}\")", basic_authentication_string))
  }
}

#[test]
fn test_display_auth_string() {
  assert_eq!(
    AuthString::basic(Some("my-realm"), "my-username").to_string(),
    "basic@my-realm:my-username".to_string()
  );
  assert_eq!(AuthString::basic(None::<String>, "my-username").to_string(), "basic@my-username".to_string());
  assert_eq!(
    AuthString::fwd("https://my-authentication-service.com", Some("my-headers")).to_string(),
    "fwd@https://my-authentication-service.com@my-headers".to_string()
  );
  assert_eq!(
    AuthString::fwd("https://my-authentication-service.com", None::<String>).to_string(),
    "fwd@https://my-authentication-service.com".to_string()
  );
  assert_eq!(AuthString::system_fwd("view,manage").to_string(), "sys-fwd@view,manage".to_string());
}

#[test]
fn test_parse_auth_string() {
  assert_eq!(
    AuthString::from_str("basic-auth@my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    AuthString::from_str("basic-auth@my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
  assert_eq!(
    AuthString::from_str("my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    AuthString::from_str("my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
  assert_eq!(
    AuthString::from_str("fwd-auth@https://my-authentication-service.com@my-headers"),
    Ok(AuthString::fwd("https://my-authentication-service.com", Some("my-headers".to_string())))
  );
  assert_eq!(AuthString::from_str("system-fwd-auth@view,manage"), Ok(AuthString::system_fwd("view,manage")));
}

#[test]
fn test_parse_bucket_string() {
  assert_eq!(parse_bucket_string("{ bucket_name('my_bucket_name') }"), Ok("my_bucket_name"));
}

#[test]
fn test_image_string() {
  let registry_image = ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string());
  assert_eq!(registry_image.id(), "my-image".to_string());
  assert_eq!(registry_image.source(), "harbor");
  assert_eq!(registry_image.tenant(), "my-tenant".to_string());
  assert_eq!(registry_image.version(), "0.0.1".to_string());
  let app_image = ImageString::app(
    "draft".to_string(),
    "kpn".to_string(),
    "my-tenant".to_string(),
    "my-image".to_string(),
    "0.0.1".to_string(),
  );
  assert_eq!(app_image.id(), "my-image".to_string());
  assert_eq!(app_image.source(), "app-catalog");
  assert_eq!(app_image.tenant(), "my-tenant".to_string());
  assert_eq!(app_image.version(), "0.0.1".to_string());
}

#[test]
fn test_display_image_string() {
  let registry_image = ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string());
  assert_eq!(registry_image.to_string(), "registry:my-tenant:my-image:0.0.1".to_string());
  let app_image = ImageString::app(
    "draft".to_string(),
    "kpn".to_string(),
    "my-tenant".to_string(),
    "my-image".to_string(),
    "0.0.1".to_string(),
  );
  assert_eq!(app_image.to_string(), "app:draft:kpn:my-tenant:my-image:0.0.1".to_string());
}

#[test]
fn test_parse_image_string() {
  assert_eq!(
    ImageString::from("registry.cp.kpn-dsh.com/my-tenant/my-image:0.0.1"),
    ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string())
  );
  assert_eq!(
    ImageString::from("APPCATALOG_REGISTRY/dsh-appcatalog/tenant/my-tenant/1234/1234/draft/kpn/my-image:0.0.1"),
    ImageString::app(
      "draft".to_string(),
      "kpn".to_string(),
      "my-tenant".to_string(),
      "my-image".to_string(),
      "0.0.1".to_string()
    )
  );
  assert_eq!(
    ImageString::from("APPCATALOG_REGISTRY/dsh-appcatalog/tenant/my-tenant/1234/1234/release/klarrio/whoami:1.6.1"),
    ImageString::app(
      "release".to_string(),
      "klarrio".to_string(),
      "my-tenant".to_string(),
      "whoami".to_string(),
      "1.6.1".to_string()
    )
  );
  assert_eq!(
    ImageString::from("registry.cp.kpn-dsh.com/greenbox-dev/postgres:pooria.20241211.1"),
    ImageString::registry("greenbox-dev".to_string(), "postgres".to_string(), "pooria.20241211.1".to_string())
  );
  assert_eq!(
    ImageString::from("registry/greenbox-dev/postgres:pooria.20241211.1"),
    ImageString::Unrecognized("registry/greenbox-dev/postgres:pooria.20241211.1".to_string())
  );
}

#[test]
fn test_parse_vhost_string() {
  assert_eq!(
    VhostString::from_str("{ vhost('my-vhost-name') }"),
    Ok(VhostString::new("my-vhost-name", false, None::<String>, None::<String>))
  );
  assert_eq!(
    VhostString::from_str("{ vhost('my-vhost-name.kafka.my-tenant','public') }"),
    Ok(VhostString::new("my-vhost-name", true, Some("my-tenant"), Some("public")))
  );
}

#[test]
fn test_parse_function() {
  let valids_under_test = vec![
    ("{function('par')}", "function", "par"),
    ("{function('p.a_r-1')}", "function", "p.a_r-1"),
    ("{ function( 'par' ) }", "function", "par"),
    ("{function('')}", "function", ""),
  ];
  for (valid_string, function, parameter) in valids_under_test {
    assert_eq!(parse_function(valid_string, function), Ok(parameter));
  }
  let invalids_under_test = vec![
    ("{function('par')}", "other"),
    ("{('par')}", ""),
    ("{function(par)}", "function"),
    ("{function('p$ar')}", "function"),
    ("{function()}", "function"),
    ("{function('par1','par2')}", "function"),
  ];
  for (invalid_string, function) in invalids_under_test {
    assert!(parse_function(invalid_string, function).is_err_and(|error| error == format!("invalid {} string (\"{}\")", function, invalid_string)));
  }
}

#[test]
fn test_parse_function2() {
  let valids_under_test = vec![
    ("{function('par1','par2')}", "function", ("par1", "par2")),
    ("{function('p.a_r-1','p.a_r-2')}", "function", ("p.a_r-1", "p.a_r-2")),
    ("{ function( 'par1' , 'par2' ) }", "function", ("par1", "par2")),
    ("{function('','')}", "function", ("", "")),
  ];
  for (valid_string, function, parameter) in valids_under_test {
    assert_eq!(parse_function2(valid_string, function), Ok(parameter));
  }
  let invalids_under_test = vec![
    ("{function('par1','par2')}", "other"),
    ("{('par1','par2')}", ""),
    ("{function(par1,par2)}", "function"),
    ("{function('par1',par2)}", "function"),
    ("{function(par1,'par2')}", "function"),
    ("{function('p$ar1','p$ar2')}", "function"),
    ("{function('par1','p$ar2')}", "function"),
    ("{function('p$ar1','par2')}", "function"),
    ("{function()}", "function"),
    ("{function('par')}", "function"),
  ];
  for (invalid_string, function) in invalids_under_test {
    assert!(parse_function2(invalid_string, function).is_err_and(|error| error == format!("invalid {} string (\"{}\")", function, invalid_string)));
  }
}

#[test]
fn test_parse_basic_authentication_string() {
  assert_eq!(
    parse_basic_authentication_string("my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    parse_basic_authentication_string("my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
}
