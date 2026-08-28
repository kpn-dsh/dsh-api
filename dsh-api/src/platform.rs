//! # Defines DSH platforms and their properties

use crate::error::{DshApiError, DshApiResult};
use crate::types::PortMapping;
use crate::vhost::VhostString;
use crate::{DEFAULT_PLATFORMS, ENV_VAR_PLATFORM, ENV_VAR_PLATFORMS_FILE_NAME};
use itertools::Itertools;
use log::{debug, info};
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;
use std::{env, fs};

/// Describes the DSH platforms and their properties.
///
/// The `DshPlatform` enum has variants for all supported DSH platforms and can be used
/// to identify a platform when invoking a function,
/// to get platform related parameters, like domain names and endpoints or to construct
/// urls related to the platform.
///
/// # Example
///
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
#[serde(deny_unknown_fields)]
pub struct DshPlatform {
  name: String,
  description: String,
  alias: String,
  #[serde(rename = "is-production")]
  is_production: bool,
  #[serde(rename = "cloud-provider")]
  cloud_provider: CloudProvider,
  region: Option<String>,
  #[serde(rename = "issuer-endpoint")]
  issuer_endpoint: String,
  realm: String,
  #[serde(rename = "public-domain")]
  public_domain: String,
  #[serde(rename = "private-domain")]
  private_domain: Option<String>,
}

/// Cloud service provider that hosts a platform.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum CloudProvider {
  /// Amazon Web Services
  #[serde(rename = "aws")]
  AWS,
  /// Microsoft Azure
  #[serde(rename = "azure")]
  Azure,
}

/// Selects the vhost zone
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum VhostZone {
  #[serde(rename = "private")]
  Private,
  #[serde(rename = "public")]
  Public,
}

#[rustfmt::skip]
/// Custom serializer serializes to platform name.
///
/// Custom serializer for `DshPlatform`. If you include a `DshPlatform` in an enum, struct or
/// tuple that implements `Serialize`, the serialized output will include the complete
/// serialized `DshPlatform` struct with all its fields. If all you want or need is the `name` of
/// the platform, you can set the `serialize_with`[^ser] and `deserialize_with`[^des]
/// attributes to this function and its companion function [`deserialize_platform`].
///
/// If you prefer to use the `alias` instead of the `name`, see [`serialize_platform_alias`].
///
/// # Example
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// use dsh_api::platform::{deserialize_platform, serialize_platform, DshPlatform};
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// pub struct StructUnderTest {
///   #[serde(deserialize_with = "deserialize_platform",
///           serialize_with = "serialize_platform")]
///   pub platform: DshPlatform,
/// }
///
/// let sut = StructUnderTest { platform: DshPlatform::new("nplz") };
/// let serialized_sut = serde_json::to_string(&sut)?;
/// assert_eq!(serialized_sut, r#"{"platform":"np-aws-lz-dsh"}"#);
/// let deserialized_sut = serde_json::from_str(&serialized_sut)?;
/// assert_eq!(sut, deserialized_sut);
/// # Ok(())
/// # }
/// ```
///
/// [^des]: <https://serde.rs/field-attrs.html#deserialize_with>
/// [^ser]: <https://serde.rs/field-attrs.html#serialize_with>
pub fn serialize_platform<S>(platform: &DshPlatform, serialize: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serialize.serialize_str(platform.name())
}

#[rustfmt::skip]
/// Custom serializer serializes to platform alias.
///
/// Custom serializer for `DshPlatform`. If you include a `DshPlatform` in an enum, struct or
/// tuple that implements `Serialize`, the serialized output will include the complete
/// serialized `DshPlatform` struct with all its fields. If all you want or need is the `alias` of
/// the platform, you can set the `serialize_with`[^ser] and `deserialize_with`[^des]
/// attributes to this function and its companion function [`deserialize_platform`].
///
/// If you prefer to use the `name` instead of the `alias`, see [`serialize_platform`].
///
/// # Example
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// use dsh_api::platform::{
///   deserialize_platform,
///   serialize_platform_alias,
///   DshPlatform
/// };
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// pub struct StructUnderTest {
///   #[serde(deserialize_with = "deserialize_platform",
///           serialize_with = "serialize_platform_alias")]
///   pub platform: DshPlatform,
/// }
///
/// let sut = StructUnderTest { platform: DshPlatform::new("np-aws-lz-dsh") };
/// let serialized_sut = serde_json::to_string(&sut)?;
/// assert_eq!(serialized_sut, r#"{"platform":"nplz"}"#);
/// let deserialized_sut = serde_json::from_str(&serialized_sut)?;
/// assert_eq!(sut, deserialized_sut);
/// # Ok(())
/// # }
/// ```
///
/// [^des]: <https://serde.rs/field-attrs.html#deserialize_with>
/// [^ser]: <https://serde.rs/field-attrs.html#serialize_with>
pub fn serialize_platform_alias<S>(platform: &DshPlatform, serialize: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
{
  serialize.serialize_str(platform.alias())
}

#[rustfmt::skip]
/// Custom deserializer deserializes from platform name or alias.
///
/// Custom deserializer for `DshPlatform`. If you include a `DshPlatform` in an enum, struct or
/// tuple that implements `Deserialize`, this custom deserializer will expect just the `name`
/// or `alias` for the value of the field, instead of the entire `DshPlatform` struct. To
/// accomplish this, set the `serialize_with`[^ser] and `deserialize_with`[^des] attributes
/// to this function and its companion function [`serialize_platform`].
///
/// # Examples
///
///
/// ```rust
/// # use serde::{Deserialize, Serialize};
/// use dsh_api::platform::{deserialize_platform, serialize_platform, DshPlatform};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///
/// #[derive(Debug, Deserialize, PartialEq, Serialize)]
/// pub struct StructUnderTest {
///   #[serde(deserialize_with = "deserialize_platform",
///           serialize_with = "serialize_platform")]
///   pub platform: DshPlatform,
/// }
///
/// let sut_json = r#"{"platform":"np-aws-lz-dsh"}"#;
/// let sut = serde_json::from_str::<StructUnderTest>(sut_json)?;
/// assert_eq!(sut, StructUnderTest { platform: DshPlatform::new("nplz") });
/// let serialized_sut = serde_json::to_string(&sut)?;
/// assert_eq!(serialized_sut, sut_json);
/// # Ok(())
/// # }
/// ```
///
/// [^des]: <https://serde.rs/field-attrs.html#deserialize_with>
/// [^ser]: <https://serde.rs/field-attrs.html#serialize_with>
pub fn deserialize_platform<'de, D>(deserializer: D) -> Result<DshPlatform, D::Error>
where
  D: Deserializer<'de>,
{
  DshPlatform::from_str(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}

const CLIENT_ID_SEPARATOR: &str = ":";

impl DshPlatform {
  /// Returns the endpoint for the DSH Rest API access token.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").access_token_endpoint(),
  ///   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh/protocol/openid-connect/token"
  /// );
  /// ```
  pub fn access_token_endpoint(&self) -> String {
    format!("{}/protocol/openid-connect/token", self.issuer_endpoint())
  }

  /// Returns the optional short/alias platform name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("np-aws-lz-dsh").alias(), "nplz");
  /// ```
  pub fn alias(&self) -> &str {
    self.alias.as_str()
  }

  /// Returns list of all platforms.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// for platform in DshPlatform::all()? {
  ///   println!("{} / {} -> {}", platform.name(), platform.alias(), platform.description());
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub fn all() -> DshApiResult<&'static Vec<DshPlatform>> {
    match &*DSH_PLATFORMS {
      Ok(platforms) => Ok(platforms),
      Err(error) => Err(error.clone()),
    }
  }

  /// Returns properly formatted bucket name.
  ///
  /// Creates a bucket name from the bucket identifier for this platform and the provided tenant.
  /// For Azure this method requires the object store access key (stored as
  /// `system/objectstore/access_key_id` in the secret store).
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `bucket_id` - Bucket identifier.
  /// * `access_key` - Bucket access key. This value is mandatory for the Azure platform. For
  ///   AWS this parameter is not used (you can provide `None`).
  ///
  /// # Example
  /// ```
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").bucket_name("my-tenant", "my-bucket", None::<String>)?,
  ///   "dev-lz-dsh-my-tenant-my-bucket"
  /// );
  /// assert_eq!(
  ///   DshPlatform::new("prodaz").bucket_name("my-tenant", "my-bucket", Some("my-access-key"))?,
  ///   "prod-azure-dsh-my-tenant-my-bucket@my-access-key"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn bucket_name(&self, tenant_name: impl Display, bucket_id: impl Display, access_key: Option<impl Display>) -> DshApiResult<String> {
    match self.cloud_provider {
      CloudProvider::AWS => Ok(format!("{}-{}-{}", self.realm, tenant_name, bucket_id)),
      CloudProvider::Azure => match access_key {
        Some(access_key) => Ok(format!("{}-{}-{}@{}", self.realm, tenant_name, bucket_id, access_key)),
        None => Err(DshApiError::Parameter { message: "bucket name for azure requires the bucket access secret system/objectstore/access_key_id".to_string() }),
      },
    }
  }

  /// Returns the cloud provider for the platform.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{CloudProvider, DshPlatform};
  /// assert_eq!(DshPlatform::new("nplz").cloud_provider(), &CloudProvider::AWS);
  /// ```
  pub fn cloud_provider(&self) -> &CloudProvider {
    &self.cloud_provider
  }

  /// Returns the domain of the platform console.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").console_domain(), "console.dsh-dev.dsh.np.aws.kpn.com");
  /// ```
  pub fn console_domain(&self) -> String {
    format!("console.{}", self.public_domain())
  }

  #[rustfmt::skip]
  /// Returns the url of the platform console.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").console_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn console_url(&self) -> String {
    format!("https://{}", self.console_domain())
  }

  /// Returns the consumer group for a service.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `service_name` - Name/id of the service.
  /// * `index` - Proxy consumer group index.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").consumer_group("my-tenant", "my-service", 2),
  ///   "my-tenant_my-service_2"
  /// );
  /// ```
  pub fn consumer_group(&self, tenant_name: impl Display, service_name: impl Display, index: usize) -> String {
    format!("{}_{}_{}", tenant_name, service_name, index)
  }

  #[rustfmt::skip]
  /// Returns a description of the platform.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").description(),
  ///   "Staging platform for KPN internal tenants"
  /// );
  /// ```
  pub fn description(&self) -> &str {
    &self.description
  }

  #[rustfmt::skip]
  /// Returns the private or public domain.
  ///
  /// The private domain for a platform is optional.
  ///
  /// # Parameters
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// assert_eq!(
  ///   DshPlatform::new("nplz").domain(VhostZone::Private),
  ///   Ok("dsh-dev.dsh.np.aws.kpn.org")
  /// );
  /// ```
  pub fn domain(&self, vhost_zone: VhostZone) -> DshApiResult<&str> {
    match vhost_zone {
      VhostZone::Private => match self.private_domain() {
        Some(private_domain) => Ok(private_domain),
        None => Err(DshApiError::parameter(format!("platform '{}' does not support private vhosts", self))),
      },
      VhostZone::Public => Ok(self.public_domain()),
    }
  }

  /// Generate domain from vhost string.
  ///
  /// Generates the domain from the `DshPlatform` and the provided `VhostString` and `tenant`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// # use dsh_api::vhost::VhostString;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let platform = DshPlatform::new("nplz");
  /// let vhost_string = VhostString::from_resource_str("my-vhost.my-tenant@private")?;
  /// assert_eq!(
  ///   platform.domain_from_vhost_string(&vhost_string, Some("my-tenant")),
  ///   Ok("my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org".to_string())
  /// );
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Parameters
  /// * `vhost_string` - Vhost string.
  /// * `tenant` - Optional tenant name. Note tenant name is mandatory for private zone and for
  ///   proxy vhosts.
  pub fn domain_from_vhost_string(&self, vhost_string: &VhostString, tenant: Option<&str>) -> DshApiResult<String> {
    match vhost_string.zone {
      Some(VhostZone::Private) => match tenant {
        Some(tenant) => {
          if vhost_string.kafka {
            self.proxy_vhost(tenant, vhost_string.vhost_name.as_str(), VhostZone::Private)
          } else {
            self.tenant_private_vhost_domain(tenant, vhost_string.vhost_name.as_str())
          }
        }
        None => Err(DshApiError::Conversion { message: "tenant is mandatory for private zone".to_string() }),
      },
      Some(VhostZone::Public) => {
        if vhost_string.kafka {
          match tenant {
            Some(tenant) => self.proxy_vhost(tenant, vhost_string.vhost_name.as_str(), VhostZone::Public),
            None => Err(DshApiError::Conversion { message: "tenant is mandatory for proxy url".to_string() }),
          }
        } else {
          Ok(self.public_vhost_domain(vhost_string.vhost_name.as_str()))
        }
      }
      None => Err(DshApiError::Conversion { message: "zone is missing".to_string() }),
    }
  }

  #[rustfmt::skip]
  /// Finds a platform from a domain name.
  ///
  /// Tries to find a platform that matches the provided private or public domain name.
  ///
  /// # Parameters
  /// * `domain_name` - Domain to match against.
  ///
  /// # Example
  /// ```rust
  /// # use std::str::FromStr;
  /// use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// use dsh_api::platform::VhostZone;
  /// let (platform, vhost_zone) =
  ///   DshPlatform::from_domain("dsh.np.aws.kpn.com")?.unwrap();
  /// assert_eq!(platform, DshPlatform::new("nplz"));
  /// assert_eq!(vhost_zone, VhostZone::Public);
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Returns
  /// * `Ok(Some((DshPlatform, VhostZone::Public)))` - When a single platform with a matching
  ///   public vhost domain was found.
  /// * `Ok(Some((DshPlatform, VhostZone::Private)))` - When a single platform with a matching
  ///   private vhost domain was found.
  /// * `Ok(None)` - When no match was found.
  /// * `Err()` - When multiple matches were found.
  pub fn from_domain(domain_name: &str) -> DshApiResult<Option<(Self, VhostZone)>> {
    match &*DSH_PLATFORMS {
      Ok(platforms) => {
        let matching_platforms: Vec<(DshPlatform, VhostZone)> = platforms
          .iter()
          .filter_map(|platform| {
            match (
              platform.public_domain.ends_with(domain_name),
              platform.private_domain.as_ref().is_some_and(|private_domain| private_domain.ends_with(domain_name)),
            ) {
              (false, false) => None,
              (false, true) => Some((platform.clone(), VhostZone::Private)),
              (true, false) => Some((platform.clone(), VhostZone::Public)),
              (true, true) => None,
            }
          })
          .collect_vec();
        match matching_platforms.len() {
          0 => Ok(None),
          1 => Ok(matching_platforms.first().cloned()),
          _ => Err(DshApiError::parameter(format!("domain '{}' matches to multiple domains", domain_name))),
        }
      }
      Err(error) => Err(error.clone()),
    }
  }

  #[rustfmt::skip]
  /// Find a platform from an environment variable.
  ///
  /// Tries to find a platform from the value of an environment variable.
  ///
  /// # Parameters
  /// * `platform_env_var` - Name of the environment variable.
  ///
  /// # Example
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// const PLATFORM_ENV_VAR: &str = "DSH_PLATFORM";
  /// match DshPlatform::from_env_var(PLATFORM_ENV_VAR) {
  ///   Ok(Some(platform)) => println!("platform is {}", platform),
  ///   Ok(None) => println!("environment variable {} not set", PLATFORM_ENV_VAR),
  ///   Err(error) => println!("{}", error) // Illegal platform name
  /// }
  /// ```
  ///
  /// # Returns
  /// * `Ok(Some(DshPlatform))` - When the environment variable is set and contains a valid
  ///   platform name or alias.
  /// * `Ok(None)` - When the environment variable is not set.
  /// * `Err(DshApiError::Configuration)` - When the environment variable is set but does not
  ///   contain a valid platform name or alias.
  pub fn from_env_var(platform_env_var: &str) -> DshApiResult<Option<Self>> {
    match env::var(platform_env_var) {
      Ok(platform_name) => match DshPlatform::from_str(&platform_name) {
        Ok(platform) => Ok(Some(platform)),
        Err(_) => Err(DshApiError::configuration(format!(
          "environment variable '{}' contains unrecognized platform name '{}'",
          platform_env_var, platform_name
        ))),
      },
      Err(_) => Ok(None),
    }
  }

  /// Find a platform from an environment variable containing the realm.
  ///
  /// Tries to find a platform from the realm value of an environment variable.
  /// This function can be used if your application is running in a container as a DSH service
  /// and needs to know the platform it is running on. For this you need to inject the
  /// `DSH_ENVIRONMENT` variable (which provides the realm) in your service definition file:
  ///
  /// ```json
  ///   "env": {
  ///     "REALM": "{ variables('DSH_ENVIRONMENT') }",
  ///     "TENANT": "{ variables('DSH_TENANT') }",
  ///     ...
  ///   },
  /// ```
  /// Note that the `DSH_TENANT` variable can be used in a similar way to inject the tenant name.
  /// See the [service definition](https://docs.kpn-dsh.com/reference/custom-service/service-definition/#environment-variables)
  /// for more information.
  ///
  /// # Parameters
  /// * `realm_env_var` - Name of the realm environment variable.
  ///
  /// # Example
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// match DshPlatform::from_env_var_realm("REALM") {
  ///   Ok(Some(platform)) => println!("I'm running on platform {}", platform),
  ///   _ => println!("Please tell me where I am"),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Returns
  /// * `Ok(Some(DshPlatform))` - When the environment variable is set and contains a valid
  ///   platform realm.
  /// * `Ok(None)` - When the environment variable is not set.
  /// * `Err(DshApiError::Configuration)` - When the environment variable is set but does not
  ///   contain a valid realm.
  pub fn from_env_var_realm(realm_env_var: &str) -> DshApiResult<Option<Self>> {
    match env::var(realm_env_var) {
      Ok(realm) => match DshPlatform::from_str(&realm) {
        Ok(platform) => Ok(Some(platform)),
        Err(_) => Err(DshApiError::configuration(format!(
          "environment variable '{}' contains unrecognized realm '{}'",
          realm_env_var, realm
        ))),
      },
      Err(_) => Ok(None),
    }
  }

  #[rustfmt::skip]
  /// Find a platform from the realm.
  ///
  /// Tries to find a platform from the provided realm value.
  ///
  /// # Parameters
  /// * `realm` - Realm value.
  ///
  /// # Example
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # use std::str::FromStr;
  /// assert_eq!(
  ///   DshPlatform::from_realm("dev-lz-dsh"),
  ///   DshPlatform::from_str("np-aws-lz-dsh")
  /// );
  /// ```
  ///
  /// # Returns
  /// * `Ok(DshPlatform)` - When the realm matches a platform.
  /// * `Err(DshApiError::Parameter)` - When the realm does not match any platform.
  pub fn from_realm(realm: &str) -> DshApiResult<Self> {
    match &*DSH_PLATFORMS {
      Ok(platforms) => match platforms.iter().find(|dsh_platform| dsh_platform.realm() == realm) {
        Some(platform) => Ok(platform.clone()),
        None => Err(DshApiError::Parameter { message: format!("invalid realm '{}'", realm) }),
      },
      Err(error) => Err(error.clone()),
    }
  }

  /// Returns the endpoint for the http messaging api (multi)
  ///
  /// # Parameters
  /// * `mqtt_topic` - Mqtt topic name.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").http_messaging_api_url_multi("my-topic"),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/data/v0/multi/my-topic"
  /// );
  /// ```
  pub fn http_messaging_api_url_multi(&self, mqtt_topic: impl Display) -> String {
    format!("https://{}/data/v0/multi/{}", self.rest_api_domain(), mqtt_topic)
  }

  /// Returns the endpoint for the http messaging api (single)
  ///
  /// # Parameters
  /// * `mqtt_topic` - Mqtt topic name.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").http_messaging_api_url_single("my-topic"),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/data/v0/single/my-topic"
  /// );
  /// ```
  pub fn http_messaging_api_url_single(&self, mqtt_topic: impl Display) -> String {
    format!("https://{}/data/v0/single/{}", self.rest_api_domain(), mqtt_topic)
  }

  #[rustfmt::skip]
  /// Returns the internal domain name for a tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").internal_domain("my-tenant"),
  ///   "my-tenant.marathon.mesos"
  /// );
  /// ```
  pub fn internal_domain(&self, tenant_name: impl Display) -> String {
    format!("{}.marathon.mesos", tenant_name)
  }

  /// Returns the internal domain name for a service.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `service_name` - Name/id of the service.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").internal_service_domain("my-tenant", "my-service"),
  ///   "my-service.my-tenant.marathon.mesos"
  /// );
  /// ```
  pub fn internal_service_domain(&self, tenant_name: impl Display, service_name: impl Display) -> String {
    format!("{}.{}", service_name, self.internal_domain(tenant_name))
  }

  /// Returns the base issuer endpoint for openid connect
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").issuer_endpoint(),
  ///   "https://auth.prod.cp-prod.dsh.prod.aws.kpn.com/auth/realms/dev-lz-dsh"
  /// );
  /// ```
  pub fn issuer_endpoint(&self) -> &str {
    self.issuer_endpoint.as_str()
  }

  /// Returns whether the platform is production.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("np-aws-lz-dsh").is_production(), false);
  /// ```
  pub fn is_production(&self) -> bool {
    self.is_production
  }

  /// Returns the endpoint for the mqtt messaging api
  ///
  /// It is preferred to use the endpoint in the mqtt token.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").mqtt_messaging_api_endpoint(),
  ///   "mqtt.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn mqtt_messaging_api_endpoint(&self) -> String {
    format!("mqtt.{}", self.public_domain())
  }

  /// Returns the port for the mqtt messaging api
  ///
  /// It is preferred to use the endpoint in the mqtt token.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").mqtt_messaging_api_port(), 8883);
  /// ```
  pub fn mqtt_messaging_api_port(&self) -> usize {
    8883
  }

  /// Returns the endpoint for fetching an MQTT token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").mqtt_token_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/datastreams/v0/mqtt/token"
  /// );
  /// ```
  pub fn mqtt_token_endpoint(&self) -> String {
    format!("https://{}/datastreams/v0/mqtt/token", self.rest_api_domain())
  }

  /// Returns the full platform name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").name(), "np-aws-lz-dsh");
  /// ```
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Create platform from platform name or alias.
  ///
  /// # Parameters
  /// * `platform_name` - Platform name or alias.
  ///
  /// # Panics
  /// This method will panic if the provided platform name is not valid.
  /// Use [`DshPlatform::try_from`] if you need to catch this situation.
  ///
  /// # Example
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("np-aws-lz-dsh").alias(), "nplz");
  /// assert_eq!(DshPlatform::new("nplz").name(), "np-aws-lz-dsh");
  /// ```
  ///
  /// The following example will panic.
  ///
  /// ```should_panic
  /// # use dsh_api::platform::DshPlatform;
  /// DshPlatform::new("illegal-platform-name");
  /// ```
  pub fn new(platform_name: &str) -> Self {
    match DshPlatform::try_from(platform_name) {
      Ok(dsh_platform) => dsh_platform,
      Err(error) => panic!("{}", error),
    }
  }

  /// Returns the private domain.
  ///
  /// The private domain for a platform is optional.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").private_domain(), Some("dsh-dev.dsh.np.aws.kpn.org"));
  /// ```
  pub fn private_domain(&self) -> Option<&str> {
    self.private_domain.as_deref()
  }

  #[rustfmt::skip]
  /// Returns the proxy common name.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .proxy_common_name("my-proxy", "my-tenant", VhostZone::Public)?,
  ///   "my-proxy.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// #   Ok(())
  /// # }
  /// ```
  pub fn proxy_common_name(&self, proxy_name: impl Display, tenant_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    Ok(format!("{}.{}", proxy_name, self.proxy_vhost_domain(tenant_name, vhost_zone)?))
  }

  /// Returns the proxy consumer group.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `index` - Proxy consumer group index.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").proxy_consumer_group("my-tenant", "my-proxy", 2),
  ///   "my-tenant_my-proxy_2"
  /// );
  /// ```
  pub fn proxy_consumer_group(&self, tenant_name: impl Display, proxy_name: impl Display, index: usize) -> String {
    format!("{}_{}_{}", tenant_name, proxy_name, index)
  }

  #[rustfmt::skip]
  /// Returns the proxy consumer group for acl groups.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `acl_group_id` - Acl group id.
  /// * `index` - Proxy consumer group index.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .proxy_consumer_group_acl("my-tenant", "my-proxy", "my-acl-group", 2),
  ///   "my-tenant.my-acl-group_my-proxy_2"
  /// );
  /// ```
  pub fn proxy_consumer_group_acl(&self, tenant_name: impl Display, proxy_name: impl Display, acl_group_id: impl Display, index: usize) -> String {
    format!("{}.{}_{}_{}", tenant_name, acl_group_id, proxy_name, index)
  }

  /// Returns the proxy schema store vhost.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz").proxy_schema_store_vhost(
  ///     "my-tenant",
  ///     "my-proxy",
  ///     VhostZone::Public
  ///   )?,
  ///   "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// #   Ok(())
  /// # }
  /// ```
  pub fn proxy_schema_store_vhost(&self, tenant_name: impl Display, proxy_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    Ok(format!("{}-schema-store.{}", proxy_name, self.proxy_vhost_domain(tenant_name, vhost_zone)?))
  }

  #[rustfmt::skip]
  /// Returns the proxy vhost.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .proxy_vhost("my-tenant", "my-proxy", VhostZone::Public)?,
  ///   "my-proxy.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// #   Ok(())
  /// # }
  /// ```
  pub fn proxy_vhost(&self, tenant_name: impl Display, proxy_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    Ok(format!("{}.{}", proxy_name, self.proxy_vhost_domain(tenant_name, vhost_zone)?))
  }

  #[rustfmt::skip]
  /// Returns the indexed proxy vhost.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  /// * `index` - Proxy vhost index.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .proxy_vhost_index("my-tenant", "my-proxy", VhostZone::Public, 2)?,
  ///   "my-proxy-2.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// #   Ok(())
  /// # }
  /// ```
  pub fn proxy_vhost_index(&self, tenant_name: impl Display, proxy_name: impl Display, vhost_zone: VhostZone, index: usize) -> DshApiResult<String> {
    Ok(format!("{}-{}.{}", proxy_name, index, self.proxy_vhost_domain(tenant_name, vhost_zone)?))
  }

  /// Returns the proxy vhost domain.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz").proxy_vhost_domain("my-tenant", VhostZone::Public)?,
  ///   "kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// #   Ok(())
  /// # }
  /// ```
  pub fn proxy_vhost_domain(&self, tenant_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    Ok(format!("kafka.{}.{}", tenant_name, self.domain(vhost_zone)?))
  }

  /// Returns the domain used for public vhosts.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").public_domain(), "dsh-dev.dsh.np.aws.kpn.com");
  /// ```
  pub fn public_domain(&self) -> &str {
    &self.public_domain
  }

  /// Returns the public domain for a vhost.
  ///
  /// # Parameters
  /// * `vhost_name` - Public vhost name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").public_vhost_domain("my-vhost"),
  ///   "my-vhost.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn public_vhost_domain(&self, vhost_name: impl Display) -> String {
    format!("{}.{}", vhost_name, self.public_domain())
  }

  /// Returns the realm for the platform.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").realm(), "dev-lz-dsh");
  /// ```
  pub fn realm(&self) -> &str {
    &self.realm
  }

  /// Returns the cloud provider region for the platform.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").region().unwrap(), "eu-west-1");
  /// ```
  pub fn region(&self) -> Option<&str> {
    self.region.as_deref()
  }

  /// Returns the domain for the DSH Rest API.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").rest_api_domain(), "api.dsh-dev.dsh.np.aws.kpn.com");
  /// ```
  pub fn rest_api_domain(&self) -> String {
    format!("api.{}", self.public_domain())
  }

  /// Returns the endpoint for the DSH Rest API.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").rest_api_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/resources/v0"
  /// );
  /// ```
  pub fn rest_api_endpoint(&self) -> String {
    format!("https://{}/resources/v0", self.rest_api_domain())
  }

  /// Returns the endpoint for fetching a rest token
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").rest_token_endpoint(),
  ///   "https://api.dsh-dev.dsh.np.aws.kpn.com/auth/v0/token"
  /// );
  /// ```
  pub fn rest_token_endpoint(&self) -> String {
    format!("https://{}/auth/v0/token", self.rest_api_domain())
  }

  /// Returns properly formatted robot client_id.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::new("nplz").robot_client_id(), "robot:dev-lz-dsh");
  /// ```
  pub fn robot_client_id(&self) -> String {
    format!("robot{}{}", CLIENT_ID_SEPARATOR, self.realm())
  }

  #[rustfmt::skip]
  /// Returns properly formatted robot client_id for tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Example
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").robot_tenant_client_id("my-tenant"),
  ///   "robot:dev-lz-dsh:my-tenant"
  /// );
  /// ```
  pub fn robot_tenant_client_id(&self, tenant_name: impl Display) -> String {
    format!("{}{}{}", self.robot_client_id(), CLIENT_ID_SEPARATOR, tenant_name)
  }

  /// Returns the url of the platform swagger page.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").swagger_url(),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/tenant-api/spec?url=/tenant-api/assets/openapi.json"
  /// );
  /// ```
  pub fn swagger_url(&self) -> String {
    format!("https://{}/tenant-api/spec?url=/tenant-api/assets/openapi.json", self.console_domain())
  }

  #[rustfmt::skip]
  /// Returns the url of the app in the app catalog for a tenant.
  ///
  /// Note that this method also requires the `vendor` to be specified.
  /// This will most likely be `kpn`.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `vendor_name` - Vendor name.
  /// * `app_name` - App name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .tenant_app_catalog_app_url("my-tenant", "kpn", "my-app"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog/app/kpn%2Fmy-app"
  /// );
  /// ```
  pub fn tenant_app_catalog_app_url(&self, tenant_name: impl Display, vendor_name: impl Display, app_name: impl Display) -> String {
    format!(
      "https://{}/#/profiles/{}/app-catalog/app/{}%2F{}",
      self.console_domain(),
      tenant_name,
      vendor_name,
      app_name
    )
  }

  /// Returns the url of the app catalog for a tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_app_catalog_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog"
  /// );
  /// ```
  pub fn tenant_app_catalog_url(&self, tenant_name: impl Display) -> String {
    format!("https://{}/#/profiles/{}/app-catalog", self.console_domain(), tenant_name)
  }

  /// Returns the url of the platform console for a tenant app.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `app_name` - App name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_app_console_url("my-tenant", "my-app"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services/my-app/app"
  /// );
  /// ```
  pub fn tenant_app_console_url(&self, tenant_name: impl Display, app_name: impl Display) -> String {
    format!("{}/services/{}/app", self.tenant_console_url(tenant_name), app_name)
  }

  /// Returns the url of the platform console for a tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_console_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant"
  /// );
  /// ```
  pub fn tenant_console_url(&self, tenant_name: impl Display) -> String {
    format!("{}/#/profiles/{}", self.console_url(), tenant_name)
  }

  /// Returns the url of the data catalog for a tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_app_catalog_url("my-tenant"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/app-catalog"
  /// );
  /// ```
  pub fn tenant_data_catalog_url(&self, tenant_name: impl Display) -> String {
    format!("https://{}/#/profiles/{}/data-catalog", self.console_domain(), tenant_name)
  }

  /// Returns the domain for a tenant.
  ///
  /// Returns the private or public domain for a tenant. The private domain for a tenant can
  /// only be constructed if the optional private domain for the platform is defined. If it
  /// is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_domain("my-tenant", VhostZone::Private)?,
  ///   "my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_domain(&self, tenant_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    Ok(format!("{}.{}", tenant_name, self.domain(vhost_zone)?))
  }

  /// Returns the url of the platform monitoring page for a tenant.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_monitoring_url("my-tenant"),
  ///   "https://monitoring-my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn tenant_monitoring_url(&self, tenant_name: impl Display) -> String {
    format!("https://monitoring-{}.{}", tenant_name, self.public_domain)
  }

  /// Returns the private domain for an app.
  ///
  /// The private domain for an app can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `app_name` - Name of the app.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_private_app_domain("my-tenant", "my-app")?,
  ///   "my-app.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_private_app_domain(&self, tenant_name: impl Display, app_name: impl Display) -> DshApiResult<String> {
    self
      .tenant_domain(tenant_name, VhostZone::Private)
      .map(|tenant_private_domain| format!("{}.{}", app_name, tenant_private_domain))
  }

  /// Returns the private domain for a vhost.
  ///
  /// The private domain for a vhost can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `vhost_name` - Name of the vhost.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_private_vhost_domain("my-tenant", "my-vhost")?,
  ///   "my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_private_vhost_domain(&self, tenant_name: impl Display, vhost_name: impl Display) -> DshApiResult<String> {
    self
      .tenant_domain(tenant_name, VhostZone::Private)
      .map(|tenant_private_domain| format!("{}.{}", vhost_name, tenant_private_domain))
  }

  #[rustfmt::skip]
  /// Returns the bootstrap server for a configured proxy.
  ///
  /// The private bootstrap server can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  /// * `port` - Port number.
  /// * `index` - Number of proxy servers.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .tenant_proxy_bootstrap_server(
  ///       "my-tenant",
  ///       "my-proxy",
  ///       VhostZone::Private,
  ///       Some(19091),
  ///       2
  ///     )?,
  ///   "my-proxy-2.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org:19091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_bootstrap_server(
    &self,
    tenant_name: impl Display,
    proxy_name: impl Display,
    vhost_zone: VhostZone,
    port: Option<usize>,
    index: usize,
  ) -> DshApiResult<String> {
    match port {
      Some(port) => Ok(format!("{}-{}.kafka.{}:{}", proxy_name, index, self.tenant_domain(tenant_name, vhost_zone)?, port)),
      None => Ok(format!("{}-{}.kafka.{}:9091", proxy_name, index, self.tenant_domain(tenant_name, vhost_zone)?)),
    }
  }

  #[rustfmt::skip]
  /// Returns the bootstrap servers for a configured proxy.
  ///
  /// The private bootstrap server can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  /// * `number_of_servers` - Number of proxy servers.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// use dsh_api::platform::VhostZone;
  /// assert_eq!(
  ///   DshPlatform::new("nplz")
  ///     .tenant_proxy_bootstrap_servers(
  ///       "my-tenant",
  ///       "my-proxy",
  ///       VhostZone::Private,
  ///       3
  ///     )?
  ///     .first()
  ///     .unwrap(),
  ///   "my-proxy-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org:9091"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_bootstrap_servers(
    &self,
    tenant_name: impl Display,
    proxy_name: impl Display,
    vhost_zone: VhostZone,
    number_of_servers: usize,
  ) -> DshApiResult<Vec<String>> {
    (0..number_of_servers)
      .map(|index| self.tenant_proxy_bootstrap_server(&tenant_name, &proxy_name, vhost_zone.clone(), None, index))
      .collect::<Result<Vec<_>, _>>()
  }

  /// Returns the private schema store host for a configured proxy.
  ///
  /// The private schema store host can only be constructed if the optional private domain
  /// for the platform is defined. If it is not, an `Err` will be returned.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `proxy_name` - Proxy name.
  /// * `vhost_zone` - Vhost zone.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// use dsh_api::platform::VhostZone;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_proxy_schema_store_host(
  ///     "my-tenant",
  ///     "my-proxy",
  ///     VhostZone::Private
  ///   )?,
  ///   "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn tenant_proxy_schema_store_host(&self, tenant_name: impl Display, proxy_name: impl Display, vhost_zone: VhostZone) -> DshApiResult<String> {
    self
      .tenant_domain(tenant_name, vhost_zone)
      .map(|tenant_private_domain| format!("{}-schema-store.kafka.{}", proxy_name, tenant_private_domain))
  }

  /// Returns the public domain for an app.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `app_name` - App name.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_public_app_domain("my-tenant", "my-app-vhost"),
  ///   "my-app-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn tenant_public_app_domain(&self, tenant_name: impl Display, app_name: impl Display) -> String {
    format!("{}.{}.{}", app_name, tenant_name, self.public_domain)
  }

  /// Returns the url of the platform console for a tenant and service.
  ///
  /// # Parameters
  /// * `tenant_name` - Tenant name.
  /// * `service_name` - Name/id of the service.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tenant_service_console_url("my-tenant", "cmd"),
  ///   "https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/my-tenant/services/cmd/service"
  /// );
  /// ```
  pub fn tenant_service_console_url(&self, tenant_name: impl Display, service_name: impl Display) -> String {
    format!("{}/services/{}/service", self.tenant_console_url(tenant_name), service_name)
  }

  #[rustfmt::skip]
  /// Returns the url of the tracing application.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::DshPlatform;
  /// assert_eq!(
  ///   DshPlatform::new("nplz").tracing_url(),
  ///   "https://tracing.dsh-dev.dsh.np.aws.kpn.com"
  /// );
  /// ```
  pub fn tracing_url(&self) -> String {
    format!("https://tracing.{}", self.public_domain())
  }

  /// Returns the default platform.
  ///
  /// This method will read the value of the environment variable `DSH_API_PLATFORM` and
  /// will select the platform from this value. It will return an `Err<String>`
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
  pub fn try_default() -> DshApiResult<Self> {
    match &*DSH_PLATFORMS {
      Ok(dsh_platforms) => match env::var(ENV_VAR_PLATFORM) {
        Ok(platform_name_from_env_var) => match DshPlatform::try_from(platform_name_from_env_var.as_str()) {
          Ok(platform) => {
            debug!("platform '{}' (environment variable '{}')", platform, ENV_VAR_PLATFORM);
            Ok(platform)
          }
          Err(_) => Err(DshApiError::Configuration {
            message: format!(
              "environment variable {} contains invalid platform name '{}' (possible values: {})",
              ENV_VAR_PLATFORM,
              platform_name_from_env_var,
              dsh_platforms
                .iter()
                .map(|dsh_platform| format!("{}/{}", dsh_platform.name(), dsh_platform.alias()))
                .collect_vec()
                .join(", ")
            ),
          }),
        },
        Err(_) => Err(DshApiError::Configuration { message: format!("environment variable '{}' not set", ENV_VAR_PLATFORM) }),
      },
      Err(error) => Err(error.clone()),
    }
  }

  /// Generate url from vhost string.
  ///
  /// Generates the url from the `DshPlatform` and the provided `VhostString` and `tenant`.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::platform::DshPlatform;
  /// # use dsh_api::vhost::VhostString;
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let platform = DshPlatform::new("nplz");
  /// let vhost_string = VhostString::from_resource_str("my-vhost.my-tenant@private")?;
  /// assert_eq!(
  ///   platform.url_from_vhost_string(&vhost_string, Some("my-tenant")),
  ///   Ok("https://my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org".to_string())
  /// );
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Parameters
  /// * `vhost_string` - Vhost string.
  /// * `tenant` - Optional tenant name. Note tenant name is mandatory for private zone and for
  ///   proxy vhosts.
  pub fn url_from_vhost_string(&self, vhost_string: &VhostString, tenant: Option<&str>) -> DshApiResult<String> {
    self.domain_from_vhost_string(vhost_string, tenant).map(|domain| format!("https://{}", domain))
  }

  /// Validate vhost domain.
  ///
  /// Validates whether a vhost domain is valid for this `DshPlatform`. If it is valid, some
  /// parameters are returned. If it is not valid, an error is returned.
  ///
  /// # Parameters
  /// * `vhost_domain` - Vhost domain to validate.
  ///
  /// # Examples
  /// ```rust
  /// # use dsh_api::platform::{DshPlatform, VhostZone};
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// let (vhost, tenant, kafka, zone) = DshPlatform::new("nplz")
  ///   .validate_vhost_domain("my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org")?;
  /// assert_eq!(vhost, "my-vhost");
  /// assert_eq!(tenant, Some("my-tenant".to_string()));
  /// assert_eq!(kafka, false);
  /// assert_eq!(zone, VhostZone::Private);
  /// #   Ok(())
  /// # }
  /// ```
  ///
  /// # Returns
  /// * `Ok((subdomain, Option(vhost), kafka, zone))`
  ///   * `subdomain` - Vhost subdomain string.
  ///   * `tenant` - Optional tenant name.
  ///   * `kafka` - `true` if vhost domain is for a Kafka proxy, `false` otherwise.
  ///   * `zone` - Vhost zone, `Public` or `Private`.
  /// * `Err()` - When `vhost_domain` is not valid for this platform.
  pub fn validate_vhost_domain(&self, vhost_domain: &str) -> DshApiResult<(String, Option<String>, bool, VhostZone)> {
    // Subfunction parses the domain prefix, returns None when prefix is not valid
    fn validate(public_subdomain: &str, zone: VhostZone) -> Option<(String, Option<String>, bool, VhostZone)> {
      static PART_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-z0-9][a-z0-9\-]*[a-z0-9]$").unwrap());

      let parts = public_subdomain.split(".").collect_vec();
      if parts.iter().all(|part| part.len() <= 63 && PART_REGEX.is_match(part)) {
        if parts.len() == 1 {
          Some((parts.first().unwrap().to_string(), None, false, zone))
        } else if parts.len() == 2 {
          Some((parts.first().unwrap().to_string(), Some(parts.get(1).unwrap().to_string()), false, zone))
        } else {
          if *parts.get(parts.len() - 2).unwrap() == "kafka" {
            let subdomain = parts.iter().take(parts.len() - 2).join(".");
            Some((subdomain, Some(parts.last().unwrap().to_string()), true, zone))
          } else {
            let subdomain = parts.iter().take(parts.len() - 1).join(".");
            Some((subdomain, Some(parts.last().unwrap().to_string()), false, zone))
          }
        }
      } else {
        None
      }
    }

    vhost_domain
      .strip_suffix(&format!(".{}", self.public_domain()))
      .and_then(|public_domain_prefix| validate(public_domain_prefix, VhostZone::Public))
      .or_else(|| {
        self.private_domain().and_then(|private_domain| {
          vhost_domain
            .strip_suffix(&format!(".{}", private_domain))
            .and_then(|private_domain_prefix| validate(private_domain_prefix, VhostZone::Private))
        })
      })
      .ok_or_else(|| DshApiError::conversion(format!("vhost domain '{}' not valid for platform {}", vhost_domain, self.name())))
  }
}

impl Default for DshPlatform {
  /// Returns the default platform
  ///
  /// This method will read the value of the environment variable `DSH_API_PLATFORM` and
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
    match DshPlatform::try_default() {
      Ok(dsh_platform) => {
        info!("default dsh platform {} created", dsh_platform);
        dsh_platform
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

impl Display for VhostZone {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Private => f.write_str("private"),
      Self::Public => f.write_str("public"),
    }
  }
}

impl TryFrom<&str> for DshPlatform {
  type Error = DshApiError;

  /// Converts a platform name to a `DshPlatform`.
  ///
  /// Both a full name and an alias are accepted.
  ///
  /// Use [from_str()](Self::from_str) instead.
  ///
  /// # Example
  /// ```rust
  /// use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::try_from("np-aws-lz-dsh").unwrap().alias(), "nplz");
  /// assert_eq!(DshPlatform::try_from("nplz").unwrap().name(), "np-aws-lz-dsh");
  /// assert!(DshPlatform::try_from("illegal-platform-name").is_err());
  /// ```
  fn try_from(platform_name: &str) -> DshApiResult<Self> {
    Self::from_str(platform_name)
  }
}

impl FromStr for DshPlatform {
  type Err = DshApiError;

  /// Converts a platform name to a `DshPlatform`.
  ///
  /// Both a full name and an alias are accepted.
  ///
  /// # Example
  /// ```rust
  /// # use std::str::FromStr;
  /// use dsh_api::platform::DshPlatform;
  /// assert_eq!(DshPlatform::from_str("np-aws-lz-dsh").unwrap().alias(), "nplz");
  /// assert_eq!(DshPlatform::from_str("nplz").unwrap().name(), "np-aws-lz-dsh");
  /// assert!(DshPlatform::from_str("illegal-platform-name").is_err());
  /// ```
  fn from_str(platform_name: &str) -> DshApiResult<Self> {
    match &*DSH_PLATFORMS {
      Ok(dsh_platforms) => match dsh_platforms
        .iter()
        .find(|dsh_platform| dsh_platform.name() == platform_name || dsh_platform.alias() == platform_name)
      {
        Some(platform) => Ok(platform.clone()),
        None => Err(DshApiError::Parameter {
          message: format!(
            "invalid platform name '{}' (possible values: {})",
            platform_name,
            dsh_platforms
              .iter()
              .map(|dsh_platform| format!("{}/{}", dsh_platform.name(), dsh_platform.alias()))
              .collect_vec()
              .join(", ")
          ),
        }),
      },
      Err(error) => Err(error.clone()),
    }
  }
}

impl VhostZone {
  pub(crate) fn try_from(port_mapping: &PortMapping) -> DshApiResult<Option<Self>> {
    Ok(
      port_mapping
        .vhost
        .as_ref()
        .map(|vhost| VhostString::from_str(vhost))
        .transpose()?
        .and_then(|vhost_string| vhost_string.zone),
    )
  }
}

impl FromStr for VhostZone {
  type Err = DshApiError;

  fn from_str(representation: &str) -> DshApiResult<Self> {
    match representation {
      "private" => Ok(Self::Private),
      "public" => Ok(Self::Public),
      _ => Err(DshApiError::parameter(format!("invalid vhost zone '{}'", representation))),
    }
  }
}

impl FromStr for CloudProvider {
  type Err = DshApiError;

  fn from_str(representation: &str) -> DshApiResult<Self> {
    match representation {
      "aws" => Ok(Self::AWS),
      "azure" => Ok(Self::Azure),
      _ => Err(DshApiError::parameter(format!("invalid cloud provider '{}'", representation))),
    }
  }
}

// Static list of all recognized DSH platforms, lazily initialized
static DSH_PLATFORMS: LazyLock<DshApiResult<Vec<DshPlatform>>> = LazyLock::new(configured_platforms);

/// Get the configured platforms.
///
/// If the environment variable `DSH_API_PLATFORMS_FILE` is set it must refer to a file containing
/// the platforms configuration. The function will read and parse the file and use this
/// configuration instead of the default configuration, or it will return an error when something
/// fails.
///
/// # Returns
/// * `Ok(Vec<DshPlatform>)` - When everything is configured properly, the list of platforms is
///   returned.
/// * `Err(DshApiError::Configuration)` - When the list of platforms could not be determined
///   because of a mis-configuration.
fn configured_platforms() -> DshApiResult<Vec<DshPlatform>> {
  match env::var(ENV_VAR_PLATFORMS_FILE_NAME) {
    Ok(platform_file_name_from_env_var) => match fs::read_to_string(&platform_file_name_from_env_var) {
      Ok(platform_list_from_file) => match serde_json::from_str(platform_list_from_file.as_str()) {
        Ok(mut dsh_platforms_from_file) => {
          check_for_duplicate_names_or_aliases(&dsh_platforms_from_file)?;
          dsh_platforms_from_file.sort_by(|platform_a, platform_b| platform_a.name.cmp(&platform_b.name));
          info!("dsh platform list read from '{}'", platform_file_name_from_env_var);
          Ok(dsh_platforms_from_file)
        }
        Err(parse_error) => Err(DshApiError::Configuration { message: format!("invalid platforms file '{}' ({})", platform_file_name_from_env_var, parse_error) }),
      },
      Err(file_error) => Err(DshApiError::Configuration { message: format!("unable to read platforms file '{}' ({})", platform_file_name_from_env_var, file_error.kind()) }),
    },
    Err(_) => match serde_json::from_str::<Vec<DshPlatform>>(DEFAULT_PLATFORMS) {
      Ok(mut default_dsh_platforms) => {
        default_dsh_platforms.sort_by(|platform_a, platform_b| platform_a.name.cmp(&platform_b.name));
        debug!("default platform list");
        Ok(default_dsh_platforms)
      }
      Err(parse_error) => Err(DshApiError::Configuration { message: format!("illegal default platforms file ({})", parse_error) }),
    },
  }
}

// Check whether duplicate names or aliases exist
#[allow(suspicious_double_ref_op)]
fn check_for_duplicate_names_or_aliases(platforms: &Vec<DshPlatform>) -> DshApiResult<()> {
  let mut names_and_aliases: Vec<&str> = vec![];
  for platform in platforms {
    names_and_aliases.push(platform.name.as_str());
    names_and_aliases.push(platform.alias.as_str());
  }
  names_and_aliases.sort();
  let mut duplicates = Vec::new();
  for (name_or_alias, chunk) in &names_and_aliases.into_iter().chunk_by(|b| b.clone()) {
    if chunk.collect_vec().len() > 1 {
      duplicates.push(name_or_alias);
    }
  }
  if !duplicates.is_empty() {
    Err(DshApiError::Configuration { message: format!("platforms file contains duplicate names and/or aliases ({})", duplicates.into_iter().join(", ")) })
  } else {
    Ok(())
  }
}
