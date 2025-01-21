//! # Defines DSH platforms and their properties

use std::env;
use std::fmt::{Display, Formatter};

use crate::{DshApiError, PLATFORM_ENVIRONMENT_VARIABLE};
use dsh_sdk::Platform as SdkPlatform;
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};

/// # Describes the DSH platforms and their properties
///
/// The `DshPlatform` enum has variants for all supported DSH platforms and can be used
/// to identify a platform when invoking a function,
/// to get platform related parameters, like domain names and endpoints or to construct
/// urls related to the platform.
///
/// # Examples
/// ```rust
/// # use dsh_api::platform::DshPlatform;
/// println!("start the 'my_app' app for tenant 'my-tenant' at platform 'nplz'");
/// println!(
///   "open the url {} in your browser",
///   DshPlatform::NpAwsLzDsh.app_domain_for_tenant_app("my-tenant", "my_app")
/// );
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DshPlatform {
  /// # Staging platform for KPN internal tenants
  ///
  /// * name: `np-aws-lz-dsh`
  /// * cloud provider: `aws`
  /// * realm: `dev-lz-dsh`
  #[serde(rename = "np-aws-lz-dsh", alias = "nplz")]
  NpAwsLzDsh,

  /// # Staging platform for non KPN tenants
  ///
  /// * name: `poc-aws-dsh`
  /// * cloud provider: `aws`
  /// * realm: `poc-dsh`
  #[serde(rename = "poc-aws-dsh", alias = "poc")]
  PocAwsDsh,

  /// # Production platform for non KPN tenants
  ///
  /// * name: `prod-aws-dsh`
  /// * cloud provider: `aws`
  /// * realm: `tt-dsh`
  #[serde(rename = "prod-aws-dsh", alias = "prod")]
  ProdAwsDsh,

  /// # Production platform for KPN internal tenants
  ///
  /// * name: `prod-aws-lz-dsh`
  /// * cloud provider: `aws`
  /// * realm: `prod-lz-dsh`
  #[serde(rename = "prod-aws-lz-dsh", alias = "prodlz")]
  ProdAwsLzDsh,

  /// # Production platform for logstash as a service
  ///
  /// * name: `prod-aws-lz-laas`
  /// * cloud provider: `aws`
  /// * realm: `prod-aws-lz-laas`
  #[serde(rename = "prod-aws-lz-laas", alias = "prodls")]
  ProdAwsLzLaas,

  /// # Production platform for non KPN tenants
  ///
  /// * name: `prod-azure-dsh`
  /// * cloud provider: `azure`
  /// * realm: `prod-azure-dsh`
  #[serde(rename = "prod-azure-dsh", alias = "prodaz")]
  ProdAzureDsh,
}

/// # Lists all recognized dsh platforms
///
/// Static slice summarizes all recognized DSH platforms.
pub static DSH_PLATFORMS: [DshPlatform; 6] =
  [DshPlatform::NpAwsLzDsh, DshPlatform::PocAwsDsh, DshPlatform::ProdAwsDsh, DshPlatform::ProdAzureDsh, DshPlatform::ProdAwsLzLaas, DshPlatform::ProdAwsLzDsh];

lazy_static! {
  /// # List of aliases of all recognized platforms
  pub static ref DSH_PLATFORM_ALIASES: Vec<&'static str> = DSH_PLATFORMS.iter().map(|platform| platform.alias()).collect::<Vec<_>>();

  /// String summarizing aliases of all recognized platforms
  pub static ref DSH_PLATFORM_ALIASES_STRING: String = DSH_PLATFORMS.iter().map(|platform| platform.alias()).collect::<Vec<_>>().join(", ");

  /// List of full names of all recognized platforms
  pub static ref DSH_PLATFORM_FULL_NAMES: Vec<&'static str> = DSH_PLATFORMS.iter().map(|platform| platform.full_name()).collect::<Vec<_>>();

  /// # String summarizing full names of all recognized platforms
  pub static ref DSH_PLATFORM_FULL_NAMES_STRING: String = DSH_PLATFORMS.iter().map(|platform| platform.full_name()).collect::<Vec<_>>().join(", ");
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

impl Display for CloudProvider {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      CloudProvider::AWS => write!(f, "aws"),
      CloudProvider::Azure => write!(f, "azure"),
    }
  }
}
impl Display for DshPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.full_name())
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
  /// assert_eq!(DshPlatform::try_from("np-aws-lz-dsh"), Ok(DshPlatform::NpAwsLzDsh));
  /// assert_eq!(DshPlatform::try_from("nplz"), Ok(DshPlatform::NpAwsLzDsh));
  /// ```

  fn try_from(platform_name: &str) -> Result<Self, Self::Error> {
    match platform_name {
      "nplz" | "np-aws-lz-dsh" => Ok(Self::NpAwsLzDsh),
      "poc" | "poc-aws-dsh" => Ok(Self::PocAwsDsh),
      "prod" | "prod-aws-dsh" => Ok(Self::ProdAwsDsh),
      "prodlz" | "prod-aws-lz-dsh" => Ok(Self::ProdAwsLzDsh),
      "prodls" | "prod-aws-lz-laas" => Ok(Self::ProdAwsLzLaas),
      "prodaz" | "prod-azure-dsh" => Ok(Self::ProdAzureDsh),
      _ => Err(format!(
        "invalid platform name '{}' (possible values: {})",
        platform_name, *DSH_PLATFORM_ALIASES_STRING
      )),
    }
  }
}

/// Converts to [`dsh_sdk::Platform`] in DSH rust SDK
impl TryFrom<&DshPlatform> for SdkPlatform {
  type Error = DshApiError;

  fn try_from(dsh_platform: &DshPlatform) -> Result<Self, Self::Error> {
    match dsh_platform {
      DshPlatform::NpAwsLzDsh => Ok(SdkPlatform::NpLz),
      DshPlatform::PocAwsDsh => Ok(SdkPlatform::Poc),
      DshPlatform::ProdAwsDsh => Ok(SdkPlatform::Prod),
      DshPlatform::ProdAwsLzDsh => Ok(SdkPlatform::ProdLz),
      DshPlatform::ProdAzureDsh => Ok(SdkPlatform::ProdAz),
      unrecognized_dsh_platform => Err(DshApiError::Configuration(format!(
        "platform {} is not recognized by the dsh sdk library",
        unrecognized_dsh_platform
      ))),
    }
  }
}

impl DshPlatform {
  /// Returns the endpoint for the DSH Rest API access token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.access_token_rest_endpoint(),
  ///   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/dev-lz-dsh/protocol/openid-connect/token"
  ///     .to_string()
  /// );
  /// ```
  pub fn access_token_rest_endpoint(&self) -> String {
    format!("{}/{}/protocol/openid-connect/token", self.key_cloak_url(), self.realm())
  }

  /// # Returns the short/alias platform name
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpAwsLzDsh.alias(), "nplz");
  /// ```
  pub fn alias(&self) -> &str {
    match self {
      Self::NpAwsLzDsh => "nplz",
      Self::PocAwsDsh => "poc",
      Self::ProdAwsDsh => "prod",
      Self::ProdAwsLzDsh => "prodlz",
      Self::ProdAwsLzLaas => "prodls",
      Self::ProdAzureDsh => "prodaz",
    }
  }

  /// # Returns the endpoint for the DSH Rest API
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.api_rest_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0".to_string()
  /// );
  /// ```
  pub fn api_rest_endpoint(&self) -> String {
    format!("https://{}/resources/v0", self.rest_api_domain())
  }

  /// # Returns the domain used to expose tenant apps
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.app_domain_for_tenant("my-tenant"),
  ///   "my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn app_domain_for_tenant<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}.{}", tenant.as_ref(), self.vhost_domain())
  }

  /// # Returns the domain used to expose tenant app
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.app_domain_for_tenant_app("my-tenant", "cmd"),
  ///   "cmd.my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn app_domain_for_tenant_app<T: AsRef<str>, A: AsRef<str>>(&self, tenant: T, app: A) -> String {
    format!("{}.{}", app.as_ref(), self.app_domain_for_tenant(tenant))
  }

  /// # Returns the cloud provider for the platform
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{CloudProvider, DshPlatform};
  /// assert_eq!(DshPlatform::NpAwsLzDsh.cloud_provider(), CloudProvider::AWS);
  /// ```
  pub fn cloud_provider(&self) -> CloudProvider {
    match self {
      Self::NpAwsLzDsh => CloudProvider::AWS,
      Self::PocAwsDsh => CloudProvider::AWS,
      Self::ProdAwsDsh => CloudProvider::AWS,
      Self::ProdAwsLzDsh => CloudProvider::AWS,
      Self::ProdAwsLzLaas => CloudProvider::AWS,
      Self::ProdAzureDsh => CloudProvider::Azure,
    }
  }

  /// # Returns the domain of the platform console
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.console_domain(),
  ///   "console.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn console_domain(&self) -> String {
    format!("console.{}", self.vhost_domain())
  }

  /// # Returns the url of the platform console
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.console_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn console_url(&self) -> String {
    format!("https://{}", self.console_domain())
  }

  /// # Returns the url of the platform console for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.console_url_for_tenant("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services".to_string()
  /// );
  /// ```
  pub fn console_url_for_tenant<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("https://{}/#/profiles/{}/services", self.console_domain(), tenant.as_ref())
  }

  /// # Returns the url of the platform console for a tenant and service
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.console_url_for_tenant_service("my-tenant", "cmd"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services/cmd/service"
  ///     .to_string()
  /// );
  /// ```
  pub fn console_url_for_tenant_service<T: AsRef<str>, S: AsRef<str>>(&self, tenant: T, service: S) -> String {
    format!("{}/{}/service", self.console_url_for_tenant(tenant.as_ref()), service.as_ref())
  }

  /// # Returns a description of the platform
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpAwsLzDsh.description(), "Staging platform for KPN internal tenants");
  /// ```
  pub fn description(&self) -> &str {
    match self {
      Self::NpAwsLzDsh => "Staging platform for KPN internal tenants",
      Self::PocAwsDsh => "Staging platform for non KPN tenants",
      Self::ProdAwsDsh => "Production platform for non KPN tenants",
      Self::ProdAwsLzDsh => "Production platform for KPN internal tenants",
      Self::ProdAwsLzLaas => "Production platform for logstash as a service",
      Self::ProdAzureDsh => "Production platform for non KPN tenants",
    }
  }

  /// # Returns the full platform name
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpAwsLzDsh.full_name(), "np-aws-lz-dsh");
  /// ```
  pub fn full_name(&self) -> &str {
    match &self {
      Self::NpAwsLzDsh => "np-aws-lz-dsh",
      Self::PocAwsDsh => "poc-aws-dsh",
      Self::ProdAwsDsh => "prod-aws-dsh",
      Self::ProdAwsLzDsh => "prod-aws-lz-dsh",
      Self::ProdAwsLzLaas => "prod-aws-lz-laas",
      Self::ProdAzureDsh => "prod-azure-dsh",
    }
  }

  /// # Returns the tenant internal domain name
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.internal_domain_for_tenant("my-tenant"),
  ///   "my-tenant.marathon.mesos".to_string()
  /// );
  /// ```
  pub fn internal_domain_for_tenant<T: AsRef<str>>(&self, service: T) -> String {
    format!("{}.marathon.mesos", service.as_ref())
  }

  /// # Returns the key cloak url for the platform
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.key_cloak_url(),
  ///   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth"
  /// );
  /// ```
  pub fn key_cloak_url(&self) -> &str {
    match self {
      Self::NpAwsLzDsh => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth",
      Self::PocAwsDsh => "https://auth.prod.cp.kpn-dsh.com/auth",
      Self::ProdAwsDsh => "https://auth.prod.cp.kpn-dsh.com/auth//auth",
      Self::ProdAwsLzDsh => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth",
      Self::ProdAwsLzLaas => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth",
      Self::ProdAzureDsh => "https://auth.prod.cp.kpn-dsh.com/auth//auth",
    }
  }

  /// # Returns the url of the platform monitoring page for a tenant
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.monitoring_domain_for_tenant("my-tenant"),
  ///   "monitoring-my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn monitoring_domain_for_tenant<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("monitoring-{}.{}", tenant.as_ref(), self.vhost_domain())
  }

  /// Returns the endpoint for fetching an MQTT token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.mqtt_token_rest_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token".to_string()
  /// );
  /// ```
  pub fn mqtt_token_rest_endpoint(&self) -> String {
    format!("https://{}/datastreams/v0/mqtt/token", self.rest_api_domain())
  }

  /// # Returns the domain of a public vhost
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.public_vhost_domain("my_vhost"),
  ///   "my_vhost.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn public_vhost_domain<V: AsRef<str>>(&self, vhost: V) -> String {
    format!("{}.{}", vhost.as_ref(), self.vhost_domain())
  }

  /// # Returns the realm of the platform
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpAwsLzDsh.realm(), "dev-lz-dsh");
  /// ```
  pub fn realm(&self) -> &str {
    match self {
      Self::NpAwsLzDsh => "dev-lz-dsh",
      Self::PocAwsDsh => "poc-dsh",
      Self::ProdAwsDsh => "tt-dsh",
      Self::ProdAwsLzDsh => "prod-lz-dsh",
      Self::ProdAwsLzLaas => "prod-aws-lz-laas",
      Self::ProdAzureDsh => "prod-azure-dsh",
    }
  }

  /// # Returns the domain for the DSH Rest API
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.rest_api_domain(),
  ///   "api.dsh-dev.dsh.np.aws.kpn.com".to_string()
  /// );
  /// ```
  pub fn rest_api_domain(&self) -> String {
    format!("api.{}", self.vhost_domain())
  }

  /// # Returns properly formatted client_id
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.rest_client_id(),
  ///   "robot$dev-lz-dsh".to_string()
  /// );
  pub fn rest_client_id(&self) -> String {
    format!("robot${}", self.realm())
  }

  /// # Returns properly formatted client_id for tenant
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.rest_client_id_for_tenant("my-tenant"),
  ///   "robot$dev-lz-dsh$my-tenant".to_string()
  /// );
  /// ```
  pub fn rest_client_id_for_tenant<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("{}${}", self.rest_client_id(), tenant.as_ref())
  }

  /// # Returns the url of the platform swagger page
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpAwsLzDsh.swagger_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/tenant-api/spec?url=/tenant-api/assets/openapi.json".to_string()
  /// );
  /// ```
  pub fn swagger_url(&self) -> String {
    format!("https://{}/tenant-api/spec?url=/tenant-api/assets/openapi.json", self.console_domain())
  }

  /// # Returns the domain used for public vhosts
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpAwsLzDsh.vhost_domain(), "dsh-dev.dsh.np.aws.kpn.com".to_string());
  /// ```
  pub fn vhost_domain(&self) -> &str {
    match self {
      Self::NpAwsLzDsh => "dsh-dev.dsh.np.aws.kpn.com",
      Self::PocAwsDsh => "poc.kpn-dsh.com",
      Self::ProdAwsDsh => "kpn-dsh.com",
      Self::ProdAwsLzDsh => "dsh-prod.dsh.prod.aws.kpn.com",
      Self::ProdAwsLzLaas => "laas-prod.dsh.prod.aws.kpn.com",
      Self::ProdAzureDsh => "az.kpn-dsh.com",
    }
  }

  /// # Returns the default platform
  ///
  /// This method will read the value of the environment variable `DSH_API_PLATFORM` and
  /// will select the platform from this value. It will return an `Error<String>`
  /// when the environment variable is not set or contains an undefined value.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// match DshPlatform::try_default() {
  ///   Ok(default_platform) => println!("default platform is {}", default_platform),
  ///   Err(error) => println!("no default platform: {}", error),
  /// }
  /// ```
  pub fn try_default() -> Result<Self, String> {
    match env::var(PLATFORM_ENVIRONMENT_VARIABLE) {
      Ok(platform_name) => match DshPlatform::try_from(platform_name.as_str()) {
        Ok(platform) => Ok(platform),
        Err(_) => Err(format!(
          "environment variable {} contains invalid platform name '{}' (possible values: {})",
          PLATFORM_ENVIRONMENT_VARIABLE, platform_name, *DSH_PLATFORM_FULL_NAMES_STRING
        )),
      },
      Err(_) => Err(format!("environment variable {} not set", PLATFORM_ENVIRONMENT_VARIABLE)),
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
        dsh_platform
      }
      Err(error) => panic!("{}", error),
    }
  }
}

lazy_static! {
  pub static ref DEFAULT_DSH_PLATFORM: DshPlatform = DshPlatform::default();
}
