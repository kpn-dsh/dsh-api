//! # Defines DSH platforms and their properties

use crate::{DEFAULT_PLATFORMS, ENV_VAR_PLATFORM, ENV_VAR_PLATFORMS_FILE_NAME};
use itertools::Itertools;
use lazy_static::lazy_static;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::{env, fs};

/// # Describes the DSH platforms and their properties
///
/// The `DshPlatform` enum has variants for all supported DSH platforms and can be used
/// to identify a platform when invoking a function,
/// to get platform related parameters, like domain names and endpoints or to construct
/// urls related to the platform.
///
/// # Examples
/// ```rust
/// # use std::convert::Infallible;
/// use dsh_api::platform::DshPlatform;
/// println!("start the 'my_app' app for tenant 'my-tenant' at platform 'my-platform'");
/// match DshPlatform::try_from("my-platform") {
///   Ok(platform) => {
///     println!(
///       "open the url {} in your browser",
///       platform.tenant_public_app_domain("my-tenant", "my_app")
///     )
///   }
///   Err(_) => println!("platform 'my-platform' is not recognized"),
/// }
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DshPlatform {
  name: String,
  description: String,
  alias: String,
  #[serde(rename = "is-production")]
  is_production: bool,
  #[serde(rename = "cloud-provider")]
  cloud_provider: CloudProvider,
  #[serde(rename = "access-token-endpoint")]
  access_token_endpoint: String,
  realm: String,
  #[serde(rename = "public-domain")]
  public_domain: String,
  #[serde(rename = "private-domain")]
  private_domain: Option<String>,
}

/// # Cloud service provider that hosts a platform
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CloudProvider {
  /// # Amazon Web Services
  #[serde(rename = "aws")]
  AWS,
  /// # Microsoft Azure
  #[serde(rename = "azure")]
  Azure,
}

lazy_static! {
  // Static list of all recognized DSH platforms lazily initialized
  static ref DSH_PLATFORMS: Vec<DshPlatform> = {
    match env::var(ENV_VAR_PLATFORMS_FILE_NAME) {
      Ok(platform_file_name_from_env_var) => match fs::read_to_string(&platform_file_name_from_env_var) {
        Ok(platform_list_from_file) => match serde_json::from_str(platform_list_from_file.as_str()) {
          Ok(mut dsh_platforms_from_file) => {
            if let Err(validation_error) = check_for_duplicate_names_or_aliases(&dsh_platforms_from_file) {
              error!("{}", validation_error);
              panic!("{}", validation_error)
            }
            dsh_platforms_from_file.sort_by(|platform_a, platform_b| platform_a.name.cmp(&platform_b.name));
            info!("dsh platform list read from '{}'", platform_file_name_from_env_var);
            dsh_platforms_from_file
          },
          Err(parse_error) => {
            let message = format!("invalid platforms file '{}' ({})", platform_file_name_from_env_var, parse_error);
            error!("{}", message);
            panic!("{}", message)
          }
        },
        Err(file_error) => {
          let message = format!("unable to read platforms file '{}' ({})", platform_file_name_from_env_var, file_error);
          error!("{}", message);
          panic!("{}", message)
        }
      },
      Err(_) => match serde_json::from_str::<Vec<DshPlatform>>(DEFAULT_PLATFORMS) {
        Ok(mut default_dsh_platforms) => {
          default_dsh_platforms.sort_by(|platform_a, platform_b| platform_a.name.cmp(&platform_b.name));
          debug!("default dsh platform list used");
          default_dsh_platforms
        },
        Err(_) => panic!()
      }
    }
  };
}

// Check whether duplicate names or aliases exist
#[allow(suspicious_double_ref_op)]
fn check_for_duplicate_names_or_aliases(platforms: &Vec<DshPlatform>) -> Result<(), String> {
  let mut names_and_aliases: Vec<&str> = vec![];
  for platform in platforms {
    names_and_aliases.push(platform.name.as_str());
    names_and_aliases.push(platform.alias.as_str());
  }
  names_and_aliases.sort();
  let mut duplicates = Vec::new();
  for (name_or_alias, chunk) in &names_and_aliases.into_iter().chunk_by(|b| b.clone()) {
    if chunk.collect::<Vec<_>>().len() > 1 {
      duplicates.push(name_or_alias);
    }
  }
  if !duplicates.is_empty() {
    Err(format!(
      "platforms file contains duplicate names and/or aliases ({})",
      duplicates.into_iter().join(", ")
    ))
  } else {
    Ok(())
  }
}

const CLIENT_ID_SEPARATOR: &str = ":";

impl DshPlatform {
  /// Returns the endpoint for the DSH Rest API access token
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.access_token_endpoint(),
  ///   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token"
  ///     .to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn access_token_endpoint(&self) -> &str {
    self.access_token_endpoint.as_str()
  }

  /// # Returns the optional short/alias platform name
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("np-aws-lz-dsh")?.alias(), "nplz");
  /// # Ok(())
  /// # }
  /// ```
  pub fn alias(&self) -> &str {
    self.alias.as_str()
  }

  /// # Returns all platforms
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// for platform in DshPlatform::all() {
  ///   println!("{} / {} -> {}", platform.name(), platform.alias(), platform.description());
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub fn all() -> &'static [DshPlatform] {
    &DSH_PLATFORMS
  }

  /// # Returns properly formatted client_id
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("nplz")?.client_id(), "robot:dev-lz-dsh".to_string());
  /// # Ok(())
  /// # }
  /// ```
  pub fn client_id(&self) -> String {
    format!("robot{}{}", CLIENT_ID_SEPARATOR, self.realm())
  }

  /// # Returns the cloud provider for the platform
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::{CloudProvider, DshPlatform};
  /// assert_eq!(*DshPlatform::try_from("nplz")?.cloud_provider(), CloudProvider::AWS);
  /// # Ok(())
  /// # }
  /// ```
  pub fn cloud_provider(&self) -> &CloudProvider {
    &self.cloud_provider
  }

  /// # Returns the domain of the platform console
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.console_domain(),
  ///   "console.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn console_domain(&self) -> String {
    format!("console.{}", self.public_domain())
  }

  /// # Returns the url of the platform console
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.console_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn console_url(&self) -> String {
    format!("https://{}", self.console_domain())
  }

  /// # Returns a description of the platform
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.description(),
  ///   "Staging platform for KPN internal tenants"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn description(&self) -> &str {
    &self.description
  }

  /// # Returns the internal domain name for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.internal_domain("my-tenant"),
  ///   "my-tenant.marathon.mesos".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn internal_domain<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}.marathon.mesos", tenant.as_ref())
  }

  /// # Returns the internal domain name for a service
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.internal_service_domain("my-tenant", "my-service"),
  ///   "my-service.my-tenant.marathon.mesos".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn internal_service_domain<T: AsRef<str>, S: AsRef<str>>(&self, tenant: T, service: S) -> String {
    format!("{}.{}", service.as_ref(), self.internal_domain(tenant))
  }

  /// # Returns whether the platform is production
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("np-aws-lz-dsh")?.is_production(), false);
  /// # Ok(())
  /// # }
  /// ```
  pub fn is_production(&self) -> bool {
    self.is_production
  }

  /// Returns the endpoint for fetching an MQTT token
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.mqtt_token_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn mqtt_token_endpoint(&self) -> String {
    format!("https://{}/datastreams/v0/mqtt/token", self.rest_api_domain())
  }

  /// # Returns the full platform name
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("nplz")?.name(), "np-aws-lz-dsh");
  /// # Ok(())
  /// # }
  /// ```
  pub fn name(&self) -> &str {
    &self.name
  }

  /// # Returns the private domain
  ///
  /// The private domain for a platform is optional.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.private_domain().ok_or("".to_string())?,
  ///   "dsh-dev.dsh.np.aws.kpn.org"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn private_domain(&self) -> Option<&str> {
    self.private_domain.as_deref()
  }

  /// # Returns the domain used for public vhosts
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.public_domain(),
  ///   "dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn public_domain(&self) -> &str {
    &self.public_domain
  }

  /// # Returns the public domain for a vhost
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.public_vhost_domain("my-vhost"),
  ///   "my-vhost.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn public_vhost_domain<V: AsRef<str>>(&self, vhost: V) -> String {
    format!("{}.{}", vhost.as_ref(), self.public_domain())
  }

  /// # Returns the realm for the platform
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("nplz")?.realm(), "dev-lz-dsh");
  /// # Ok(())
  /// # }
  /// ```
  pub fn realm(&self) -> &str {
    &self.realm
  }

  /// # Returns the domain for the DSH Rest API
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.rest_api_domain(),
  ///   "api.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn rest_api_domain(&self) -> String {
    format!("api.{}", self.public_domain())
  }

  /// # Returns the endpoint for the DSH Rest API
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.rest_api_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn rest_api_endpoint(&self) -> String {
    format!("https://{}/resources/v0", self.rest_api_domain())
  }

  /// # Returns the url of the platform swagger page
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.swagger_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/tenant-api/spec?url=/tenant-api/assets/openapi.json".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn swagger_url(&self) -> String {
    format!("https://{}/tenant-api/spec?url=/tenant-api/assets/openapi.json", self.console_domain())
  }

  /// # Returns the url of the app in the app catalog for a tenant
  ///
  /// Note that this method also requires the `vendor` to be specified.
  /// This will most likely be `kpn`.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_app_catalog_app_url("my-tenant", "kpn", "my-app"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog/app/kpn%2Fmy-app"
  ///     .to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_app_catalog_app_url<T: AsRef<str>, V: AsRef<str>, A: AsRef<str>>(&self, tenant: T, vendor: V, app: A) -> String {
    format!(
      "https://{}/#/profiles/{}/app-catalog/app/{}%2F{}",
      self.console_domain(),
      tenant.as_ref(),
      vendor.as_ref(),
      app.as_ref()
    )
  }

  /// # Returns the url of the app catalog for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_app_catalog_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_app_catalog_url<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("https://{}/#/profiles/{}/app-catalog", self.console_domain(), tenant.as_ref())
  }

  /// # Returns the url of the platform console for a tenant app
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_app_console_url("my-tenant", "my-app"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services/my-app/app"
  ///     .to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_app_console_url<T: AsRef<str>, A: AsRef<str>>(&self, tenant: T, app: A) -> String {
    format!("{}/services/{}/app", self.tenant_console_url(tenant.as_ref()), app.as_ref())
  }

  /// # Returns properly formatted client_id for tenant
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_client_id("my-tenant"),
  ///   "robot:dev-lz-dsh:my-tenant".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_client_id<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}{}{}", self.client_id(), CLIENT_ID_SEPARATOR, tenant.as_ref())
  }

  /// # Returns the url of the platform console for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_console_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_console_url<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}/#/profiles/{}", self.console_url(), tenant.as_ref())
  }

  /// # Returns the url of the data catalog for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_app_catalog_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_data_catalog_url<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("https://{}/#/profiles/{}/data-catalog", self.console_domain(), tenant.as_ref())
  }

  /// # Returns the url of the platform monitoring page for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_monitoring_url("my-tenant"),
  ///   "https://monitoring-my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_monitoring_url<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("https://monitoring-{}", self.tenant_public_domain(tenant))
  }

  /// # Returns the private domain for a tenant
  ///
  /// The private domain for a tenant can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_private_domain("my-tenant")?,
  ///   "my-tenant.dsh-dev.dsh.np.aws.kpn.org".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_private_domain<T: AsRef<str>>(&self, tenant: T) -> Result<String, String> {
    match self.private_domain() {
      Some(private_domain) => Ok(format!("{}.{}", tenant.as_ref(), private_domain)),
      None => Err(format!("private domain is not set for platform {}", self.name())),
    }
  }

  /// # Returns the private domain for a vhost
  ///
  /// The private domain for a vhost can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_private_vhost_domain("my-tenant", "my-vhost")?,
  ///   "my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_private_vhost_domain<T: AsRef<str>, V: AsRef<str>>(&self, tenant: T, vhost: V) -> Result<String, String> {
    self
      .tenant_private_domain(tenant)
      .map(|tenant_private_domain| format!("{}.{}", vhost.as_ref(), tenant_private_domain))
  }

  /// # Returns the private bootstrap servers for a configured proxy
  ///
  /// The private bootstrap server can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?
  ///     .tenant_proxy_private_bootstrap_servers("my-tenant", "my-proxy")?
  ///     .first()
  ///     .unwrap(),
  ///   "my-proxy-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org:9091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_private_bootstrap_servers<T: AsRef<str>, P: AsRef<str>>(&self, tenant: T, proxy: P) -> Result<Vec<String>, String> {
    self.tenant_private_domain(tenant).map(|tenant_private_domain| {
      [0, 1, 2]
        .iter()
        .map(|n| format!("{}-{}.kafka.{}:9091", proxy.as_ref(), n, tenant_private_domain))
        .collect::<Vec<_>>()
    })
  }

  /// # Returns the private schema store host for a configured proxy
  ///
  /// The private schema store host can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?
  ///     .tenant_proxy_private_schema_store_host("my-tenant", "my-proxy")?,
  ///   "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org:9091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_private_schema_store_host<T: AsRef<str>, P: AsRef<str>>(&self, tenant: T, proxy: P) -> Result<String, String> {
    self
      .tenant_private_domain(tenant)
      .map(|tenant_private_domain| format!("{}-schema-store.kafka.{}:9091", proxy.as_ref(), tenant_private_domain))
  }

  /// # Returns the public bootstrap servers for a configured proxy
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?
  ///     .tenant_proxy_public_bootstrap_servers("my-tenant", "my-proxy")
  ///     .first()
  ///     .unwrap(),
  ///   "my-proxy-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com:9091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_public_bootstrap_servers<T: AsRef<str>, P: AsRef<str>>(&self, tenant: T, proxy: P) -> Vec<String> {
    [0, 1, 2]
      .iter()
      .map(|n| format!("{}-{}.kafka.{}:9091", proxy.as_ref(), n, self.tenant_public_domain(tenant.as_ref())))
      .collect::<Vec<_>>()
  }

  /// # Returns the public schema store host for a configured proxy
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_proxy_public_schema_store_host("my-tenant", "my-proxy"),
  ///   "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com:9091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_public_schema_store_host<T: AsRef<str>, P: AsRef<str>>(&self, tenant: T, proxy: P) -> String {
    format!("{}-schema-store.kafka.{}:9091", proxy.as_ref(), self.tenant_public_domain(tenant.as_ref()))
  }

  /// # Returns the public domain for an app
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_public_app_domain("my-tenant", "my-app-vhost"),
  ///   "my-app-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_public_app_domain<T: AsRef<str>, A: AsRef<str>>(&self, tenant: T, app: A) -> String {
    format!("{}.{}", app.as_ref(), self.tenant_public_domain(tenant))
  }

  /// # Returns the public domain for apps
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// #[allow(deprecated)]
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_public_apps_domain("my-tenant"),
  ///   "my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  /// This method is deprecated. Use [`tenant_public_domain`](self.tenant_public_domain) instead.
  #[deprecated]
  pub fn tenant_public_apps_domain<T: AsRef<str>>(&self, tenant: T) -> String {
    self.tenant_public_domain(tenant)
  }

  /// # Returns the public domain for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_public_domain("my-tenant"),
  ///   "my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_public_domain<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}.{}", tenant.as_ref(), self.public_domain)
  }

  /// # Returns the url of the platform console for a tenant and service
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tenant_service_console_url("my-tenant", "cmd"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services/cmd/service"
  ///     .to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_service_console_url<T: AsRef<str>, S: AsRef<str>>(&self, tenant: T, service: S) -> String {
    format!("{}/services/{}/service", self.tenant_console_url(tenant.as_ref()), service.as_ref())
  }

  /// # Returns the url of the tracing application
  ///
  /// # Examples
  /// ```rust
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::try_from("nplz")?.tracing_url(),
  ///   "https://tracing.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tracing_url(&self) -> String {
    format!("https://tracing.{}", self.public_domain())
  }

  /// # Returns the default platform
  ///
  /// This method will read the value of the environment variable `DSH_API_PLATFORM` and
  /// will select the platform from this value. It will return an `Err<String>`
  /// when the environment variable is not set or contains an undefined value.
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// match DshPlatform::try_default() {
  ///   Ok(default_platform) => println!("default platform is {}", default_platform),
  ///   Err(error) => println!("no default platform: {}", error),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default() -> Result<Self, String> {
    match env::var(ENV_VAR_PLATFORM) {
      Ok(platform_name_from_env_var) => match &DshPlatform::try_from(platform_name_from_env_var.as_str()) {
        Ok(platform) => {
          debug!("default platform '{}' read from environment variable '{}'", platform, ENV_VAR_PLATFORM);
          Ok(platform.clone())
        }
        Err(_) => Err(format!(
          "environment variable {} contains invalid platform name '{}' (possible values: {})",
          ENV_VAR_PLATFORM,
          platform_name_from_env_var,
          DSH_PLATFORMS
            .iter()
            .map(|dsh_platform| format!("{}/{}", dsh_platform.name(), dsh_platform.alias()))
            .collect::<Vec<_>>()
            .join(", ")
        )),
      },
      Err(_) => Err(format!("environment variable '{}' not set", ENV_VAR_PLATFORM)),
    }
  }
}

impl Default for DshPlatform {
  /// Returns the default platform
  ///
  /// This method will read the value of the environment variable
  /// `DSH_API_PLATFORM` and
  /// will select the platform from this value.
  ///
  /// # Panics
  /// This method will panic if the environment variable is not set or
  /// if it contains an invalid platform name.
  ///
  /// # Example
  /// ```ignore
  /// # use dsh_api::platform::DshPlatform;
  /// println!("default platform is {}", DshPlatform::default());
  /// ```
  fn default() -> Self {
    match Self::try_default() {
      Ok(dsh_platform) => {
        info!("default dsh platform {} created", dsh_platform);
        dsh_platform.clone()
      }
      Err(error) => panic!("{}", error),
    }
  }
}

impl Display for DshPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Display for CloudProvider {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      CloudProvider::AWS => write!(f, "aws"),
      CloudProvider::Azure => write!(f, "azure"),
    }
  }
}

impl TryFrom<&str> for DshPlatform {
  type Error = String;

  /// # Converts a platform name to a `DshPlatform`
  ///
  /// Both a full name and an alias are accepted.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(DshPlatform::try_from("np-aws-lz-dsh")?.alias(), "nplz");
  /// assert_eq!(DshPlatform::try_from("nplz")?.name(), "np-aws-lz-dsh");
  /// # Ok(())
  /// # }
  /// ```
  fn try_from(platform_name: &str) -> Result<Self, Self::Error> {
    match DSH_PLATFORMS
      .iter()
      .find(|dsh_platform| dsh_platform.name() == platform_name || dsh_platform.alias() == platform_name)
    {
      Some(platform) => Ok(platform.clone()),
      None => Err(format!(
        "invalid platform name '{}' (possible values: {})",
        platform_name,
        DSH_PLATFORMS
          .iter()
          .map(|dsh_platform| format!("{}/{}", dsh_platform.name(), dsh_platform.alias()))
          .collect::<Vec<_>>()
          .join(", ")
      )),
    }
  }
}
