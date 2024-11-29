//! # Defines DSH platforms and their properties

use std::env;
use std::fmt::{Display, Formatter};

use crate::PLATFORM_ENVIRONMENT_VARIABLE;
use dsh_sdk::Platform as SdkPlatform;
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DshPlatform {
  /// Non-Production (Dev) Landing Zone on AWS (dsh-dev.dsh.np.aws.kpn.com)
  #[serde(rename = "nplz")]
  NpLz,
  /// Proof of Concept platform (poc.kpn-dsh.com)
  #[serde(rename = "poc")]
  Poc,
  /// Production platform (kpn-dsh.com)
  #[serde(rename = "prod")]
  Prod,
  /// Production platform on Azure (az.kpn-dsh.com)
  #[serde(rename = "prodaz")]
  ProdAz,
  // TODO ProdCp,
  /// Production Landing Zone on AWS (dsh-prod.dsh.prod.aws.kpn.com)
  #[serde(rename = "prodlz")]
  ProdLz,
}

/// Slice summarizing all recognized dsh platforms
pub static DSH_PLATFORMS: [DshPlatform; 5] = [DshPlatform::NpLz, DshPlatform::Poc, DshPlatform::Prod, DshPlatform::ProdAz, DshPlatform::ProdLz];

lazy_static! {
  /// String summarizing all recognized dsh platforms
  pub static ref DSH_PLATFORM_NAMES: String = DSH_PLATFORMS.iter().map(|platform| platform.to_string()).collect::<Vec<_>>().join(", ");
}

impl Display for DshPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      DshPlatform::NpLz => write!(f, "nplz"),
      DshPlatform::Poc => write!(f, "poc"),
      DshPlatform::Prod => write!(f, "prod"),
      DshPlatform::ProdAz => write!(f, "prodaz"),
      DshPlatform::ProdLz => write!(f, "prodlz"),
    }
  }
}

impl TryFrom<&str> for DshPlatform {
  type Error = String;

  fn try_from(platform_name: &str) -> Result<Self, Self::Error> {
    match platform_name {
      "nplz" => Ok(DshPlatform::NpLz),
      "poc" => Ok(DshPlatform::Poc),
      "prod" => Ok(DshPlatform::Prod),
      "prodaz" => Ok(DshPlatform::ProdAz),
      "prodlz" => Ok(DshPlatform::ProdLz),
      _ => Err(format!("invalid platform name '{}' (possible values: {})", platform_name, *DSH_PLATFORM_NAMES)),
    }
  }
}

/// Converts to [`dsh_sdk::Platform`] in DSH rust SDK
impl From<&DshPlatform> for SdkPlatform {
  fn from(dsh_platform: &DshPlatform) -> Self {
    match dsh_platform {
      DshPlatform::NpLz => SdkPlatform::NpLz,
      DshPlatform::Poc => SdkPlatform::Poc,
      DshPlatform::Prod => SdkPlatform::Prod,
      DshPlatform::ProdAz => SdkPlatform::ProdAz,
      DshPlatform::ProdLz => SdkPlatform::ProdLz,
    }
  }
}

impl DshPlatform {
  /// # Returns the default platform
  ///
  /// This method will read the value of the environment variable `DSH_API_PLATFORM` and
  /// will select the platform from this value.
  /// It will return an `Error<String>` when the environment variable is not set or
  /// contains an undefined value.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// match DshPlatform::try_default() {
  ///   Ok(default_platform) => println!("default platform is {}", default_platform),
  ///   Err(error) => panic!("no default platform: {}", error),
  /// }
  /// ```
  pub fn try_default() -> Result<Self, String> {
    match env::var(PLATFORM_ENVIRONMENT_VARIABLE) {
      Ok(platform_name) => match DshPlatform::try_from(platform_name.as_str()) {
        Ok(platform) => Ok(platform),
        Err(_) => Err(format!(
          "environment variable {} contains invalid platform name '{}' (possible values: {})",
          PLATFORM_ENVIRONMENT_VARIABLE, platform_name, *DSH_PLATFORM_NAMES
        )),
      },
      Err(_) => Err(format!("environment variable {} not set", PLATFORM_ENVIRONMENT_VARIABLE)),
    }
  }

  /// # Returns the realm of the platform
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::NpLz.realm(), "dev-lz-dsh");
  /// ```
  pub fn realm(&self) -> &str {
    match self {
      Self::Prod => "tt-dsh",
      Self::NpLz => "dev-lz-dsh",
      Self::ProdLz => "prod-lz-dsh",
      Self::ProdAz => "prod-azure-dsh",
      Self::Poc => "poc-dsh",
    }
  }

  /// # Returns the url of the platform console
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// let url = DshPlatform::NpLz.console_url();
  /// assert_eq!(url, Some("https://console.dsh-dev.dsh.np.aws.kpn.com"));
  /// ```
  pub fn console_url(&self) -> Option<&str> {
    match self {
      Self::NpLz => Some("https://console.dsh-dev.dsh.np.aws.kpn.com"),
      Self::Poc => Some("https://console.poc.kpn-dsh.com"),
      Self::Prod => None,
      Self::ProdAz => None,
      Self::ProdLz => Some("https://console.dsh-prod.dsh.prod.aws.kpn.com"),
    }
  }

  /// # Returns the url of the platform/tenant Grafana page
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.monitoring_url("greenbox-dev"),
  ///   Some("https://monitoring-greenbox-dev.dsh-dev.dsh.np.aws.kpn.com".to_string())
  /// );
  /// ```
  pub fn monitoring_url<T: AsRef<str>>(&self, tenant: T) -> Option<String> {
    match self {
      Self::NpLz => Some(format!("https://monitoring-{}.dsh-dev.dsh.np.aws.kpn.com", tenant.as_ref())),
      Self::Poc => Some(format!("https://monitoring-{}.poc.kpn-dsh.com", tenant.as_ref())),
      Self::Prod => None,
      Self::ProdAz => None,
      Self::ProdLz => Some(format!("https://monitoring-{}.dsh-prod.dsh.prod.aws.kpn.com", tenant.as_ref())),
    }
  }

  /// # Returns the domain used to expose public vhosts
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// let domain = DshPlatform::NpLz.public_vhosts_domain();
  /// assert_eq!(domain, Some("dsh-dev.dsh.np.aws.kpn.com"));
  /// ```
  pub fn public_vhosts_domain(&self) -> Option<&str> {
    match self {
      Self::NpLz => Some("dsh-dev.dsh.np.aws.kpn.com"),
      Self::Poc => Some("poc.kpn-dsh.com"),
      Self::Prod => None,
      Self::ProdAz => None,
      Self::ProdLz => Some("dsh-prod.dsh.prod.aws.kpn.com"),
    }
  }

  /// # Returns the domain used to expose private vhosts
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.dsh_internal_domain("greenbox-dev"),
  ///   Some("greenbox-dev.marathon.mesos".to_string())
  /// );
  /// ```
  pub fn dsh_internal_domain<T: AsRef<str>>(&self, tenant: T) -> Option<String> {
    match self {
      Self::NpLz => Some(format!("{}.marathon.mesos", tenant.as_ref())),
      Self::Poc => Some(format!("{}.marathon.mesos", tenant.as_ref())),
      Self::Prod => None,
      Self::ProdAz => None,
      Self::ProdLz => Some(format!("{}.marathon.mesos", tenant.as_ref())),
    }
  }

  /// # Returns the domain used to expose tenant's apps
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.app_domain("greenbox-dev"),
  ///   Some("greenbox-dev.dsh-dev.dsh.np.aws.kpn.com".to_string())
  /// );
  /// ```
  pub fn app_domain<T: AsRef<str>>(&self, tenant: T) -> Option<String> {
    match self {
      Self::NpLz => Some(format!("{}.dsh-dev.dsh.np.aws.kpn.com", tenant.as_ref())),
      Self::Poc => Some(format!("{}.poc.kpn-dsh.com", tenant.as_ref())),
      Self::Prod => None,
      Self::ProdAz => None,
      Self::ProdLz => Some(format!("{}.dsh-prod.dsh.prod.aws.kpn.com", tenant.as_ref())),
    }
  }

  /// # Returns properly formatted client_id for the Rest API
  ///
  /// # Example
  /// ```
  /// # use dsh_sdk::Platform;
  /// assert_eq!(Platform::NpLz.rest_client_id("greenbox-dev"), "robot:dev-lz-dsh:greenbox-dev");
  /// ```
  pub fn rest_client_id<T: AsRef<str>>(&self, tenant: T) -> String {
    format!("robot:{}:{}", self.realm(), tenant.as_ref())
  }

  /// # Returns the endpoint for the DSH Rest API
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.endpoint_rest_api(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0"
  /// );
  /// ```
  pub fn endpoint_rest_api(&self) -> &str {
    match self {
      Self::Prod => "https://api.kpn-dsh.com/resources/v0",
      Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0",
      Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/resources/v0",
      Self::ProdAz => "https://api.az.kpn-dsh.com/resources/v0",
      Self::Poc => "https://api.poc.kpn-dsh.com/resources/v0",
    }
  }

  /// Returns the endpoint for the DSH Rest API access token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// println!("connect your client to {}", DshPlatform::NpLz.endpoint_rest_access_token());
  /// ```
  pub fn endpoint_rest_access_token(&self) -> &str {
    match self {
      Self::Prod => "https://auth.prod.cp.kpn-dsh.com/auth/realms/tt-dsh/protocol/openid-connect/token",
      Self::NpLz => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token",
      Self::ProdLz => "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/prod-lz-dsh/protocol/openid-connect/token",
      Self::ProdAz => "https://auth.prod.cp.kpn-dsh.com/auth/realms/prod-azure-dsh/protocol/openid-connect/token",
      Self::Poc => "https://auth.prod.cp.kpn-dsh.com/auth/realms/poc-dsh/protocol/openid-connect/token",
    }
  }

  /// Returns the endpoint for fetching DSH Rest Authentication Token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.endpoint_rest_token(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token"
  /// );
  /// ```
  pub fn endpoint_rest_token(&self) -> &str {
    match self {
      Self::Prod => "https://api.kpn-dsh.com/auth/v0/token",
      Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token",
      Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/auth/v0/token",
      Self::ProdAz => "https://api.az.kpn-dsh.com/auth/v0/token",
      Self::Poc => "https://api.poc.kpn-dsh.com/auth/v0/token",
    }
  }

  /// Returns the endpoint for fetching DSH MQTT token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::NpLz.endpoint_mqtt_token(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token"
  /// );
  /// ```
  pub fn endpoint_mqtt_token(&self) -> &str {
    match self {
      Self::Prod => "https://api.kpn-dsh.com/datastreams/v0/mqtt/token",
      Self::NpLz => "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token",
      Self::ProdLz => "https://api.dsh-prod.dsh.prod.aws.kpn.com/datastreams/v0/mqtt/token",
      Self::ProdAz => "https://api.az.kpn-dsh.com/datastreams/v0/mqtt/token",
      Self::Poc => "https://api.poc.kpn-dsh.com/datastreams/v0/mqtt/token",
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
  /// ```
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
