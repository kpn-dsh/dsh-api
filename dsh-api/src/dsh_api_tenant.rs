//! # Client tenant
use crate::platform::DshPlatform;
use crate::{guid_environment_variable, DshApiError, ENV_VAR_TENANT};
use lazy_static::lazy_static;
use log::info;
use std::env;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DshApiTenant {
  name: String,
  guid: u16,
  platform: DshPlatform,
}

impl DshApiTenant {
  /// # Create new dsh api tenant
  ///
  /// # Parameters
  /// * `name` - client tenant's name
  /// * `guid` - client tenant's group and user id (must be the same on DSH)
  /// * `platform` - target platform for the api
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::platform::DshPlatform;
  /// let name = String::from("my-tenant");
  /// let guid = 1234;
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let dsh_api_tenant = DshApiTenant::new(name, guid, platform);
  /// assert_eq!(
  ///   dsh_api_tenant.platform().internal_domain_service("my-service"),
  ///   "my-service.marathon.mesos".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn new(name: String, guid: u16, platform: DshPlatform) -> Self {
    Self { name, guid, platform }
  }

  /// # Create new dsh api tenant from tenant's name
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// tenant's name.
  /// The tenant's group and user ids will be read from the
  /// environment variable `DSH_API_GUID_[TENANT]`.
  /// The platform will be read from the
  /// environment variable `DSH_API_PLATFORM`.
  /// The function will return an `Error<String>` if the environment variables are not set
  /// or contains illegal values.
  ///
  /// # Parameters
  /// * `tenant_name` - tenant's name
  ///
  /// # Returns
  /// * `Ok(tenant)` - tenant object
  /// * `Err(error)` - error containing a configuration error
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::platform::DshPlatform;
  /// # use dsh_api::DshApiError;
  /// # fn main() -> Result<(), DshApiError> {
  /// let tenant_name = String::from("my-tenant");
  /// let dsh_api_tenant = DshApiTenant::from_tenant(tenant_name)?;
  /// println!("target platform: {}", dsh_api_tenant.platform());
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_tenant(tenant_name: String) -> Result<Self, DshApiError> {
    let guid = guid_from_tenant_name(tenant_name.as_str())?;
    let platform = DshPlatform::default();
    Ok(DshApiTenant::new(tenant_name, guid, platform))
  }

  /// # Create new dsh api tenant from tenant's name and platform
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// tenant's name. The group and user ids will be read from the
  /// environment variable `DSH_API_GUID_[TENANT]`.
  /// The function will return an `Error<String>` if the environment variable is not set.
  ///
  /// # Parameters
  /// * `tenant_name` - tenant's name
  /// * `platform` - target platform for the api
  ///
  /// # Returns
  /// * `Ok(tenant)` - tenant object
  /// * `Err(error)` - error containing a configuration error
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::platform::DshPlatform;
  /// # use dsh_api::DshApiError;
  /// # fn main() -> Result<(), DshApiError> {
  /// let tenant_name = String::from("my-tenant");
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let dsh_api_tenant = DshApiTenant::from_tenant_and_platform(tenant_name, platform)?;
  /// println!("{}@{}", dsh_api_tenant.name(), dsh_api_tenant.platform());
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_tenant_and_platform(tenant_name: String, platform: DshPlatform) -> Result<Self, DshApiError> {
    let guid = guid_from_tenant_name(tenant_name.as_str())?;
    Ok(DshApiTenant::new(tenant_name, guid, platform))
  }

  /// # Create new dsh api tenant from platform
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// `platform`. The tenant's name and group and user ids will be read from the
  /// environment variables `DSH_API_TENANT` and
  /// `DSH_API_GUID_[TENANT]`.
  /// The function will return an `Error<String>` if the environment variables are not set.
  ///
  /// # Parameters
  /// * `platform` - target platform for the api
  ///
  /// # Returns
  /// * `Ok(tenant)` - tenant object
  /// * `Err(error)` - error containing a configuration error
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::platform::DshPlatform;
  /// # use dsh_api::DshApiError;
  /// # fn main() -> Result<(), DshApiError> {
  /// let tenant_name = String::from("my-tenant");
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let dsh_api_tenant = DshApiTenant::from_tenant_and_platform(tenant_name, platform)?;
  /// println!("{}@{}", dsh_api_tenant.name(), dsh_api_tenant.platform());
  /// # Ok(())
  /// # }
  /// ```
  pub fn from_platform(platform: DshPlatform) -> Result<Self, DshApiError> {
    let tenant_name = match env::var(ENV_VAR_TENANT) {
      Ok(name) => name,
      Err(_) => return Err(DshApiError::Configuration(format!("environment variable {} not set", ENV_VAR_TENANT))),
    };
    let guid = guid_from_tenant_name(tenant_name.as_str())?;
    Ok(DshApiTenant::new(tenant_name, guid, platform))
  }

  /// Returns the default tenant
  ///
  /// This method will read the tenant name, guid and platform form the respective
  /// environment variables and will create a`DshApiTenant` if possible. It will return an
  /// `Error<String>` when one or more of these the environment variables is not set or
  /// contains an undefined value.
  ///
  /// # Returns
  /// * `Ok<DshPlatform>` - when the environment variables are provided and contain valid values
  /// * `Error<String>` - when one or more of the environment variables is not set or
  ///   contains an undefined value
  pub fn try_default() -> Result<Self, String> {
    let tenant_name = get_default_tenant_name()?;
    let guid = guid_from_tenant_name(tenant_name.as_str())?;
    let platform = DshPlatform::try_default()?;
    Ok(DshApiTenant::new(tenant_name, guid, platform))
  }

  pub fn platform(&self) -> &DshPlatform {
    &self.platform
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn guid(&self) -> u16 {
    self.guid
  }
}

impl Default for DshApiTenant {
  /// Returns the default tenant
  ///
  /// This method will read the tenant name, guid and platform form the respective
  /// environment variables and will create a`DshApiTenant` if possible.
  ///
  /// # Panics
  /// This method will panic if the environment variable is not set or
  /// if it contains an invalid platform name.
  fn default() -> Self {
    match Self::try_default() {
      Ok(dsh_api_tenant) => {
        info!("default dsh api tenant {} created", dsh_api_tenant);
        dsh_api_tenant
      }
      Err(error) => panic!("{}", error),
    }
  }
}

impl Display for DshApiTenant {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}@{}", self.name, self.guid, self.platform)
  }
}

lazy_static! {
  pub static ref DEFAULT_DSH_API_TENANT: DshApiTenant = DshApiTenant::default();
}

/// # Parse and validate guid string
///
/// # Parameters
/// * `guid` - Guid string
///
/// # Returns
/// `OK(guid)` - when the guid is valid
/// `Err(message)` - when the guid is invalid
///
/// # Examples
/// ```rust
/// use dsh_api::DshApiError;
/// # fn main() -> Result<(), DshApiError> {
/// # use dsh_api::dsh_api_tenant::parse_and_validate_guid;
/// let guid = parse_and_validate_guid("1234".to_string())?;
/// assert_eq!(1234, guid);
/// # Ok(())
/// # }
pub fn parse_and_validate_guid(guid: String) -> Result<u16, DshApiError> {
  match guid.parse::<u16>() {
    Ok(guid) => {
      if guid > 0 && guid < 60000 {
        Ok(guid)
      } else {
        Err(DshApiError::Configuration(format!("guid {} not in range (1 <= guid < 60000)", guid)))
      }
    }
    Err(_) => Err(DshApiError::Configuration(format!("could not parse guid '{}'", guid))),
  }
}

fn get_default_tenant_name() -> Result<String, DshApiError> {
  env::var(ENV_VAR_TENANT).map_err(|_| DshApiError::Configuration(format!("environment variable {} not set", ENV_VAR_TENANT)))
}

fn guid_from_tenant_name(tenant_name: &str) -> Result<u16, DshApiError> {
  let guid_env = guid_environment_variable(tenant_name);
  match env::var(&guid_env) {
    Ok(guid) => match parse_and_validate_guid(guid) {
      Ok(guid) => Ok(guid),
      Err(error) => Err(DshApiError::Configuration(format!("{} in environment variable {}", error, guid_env))),
    },
    Err(_) => Err(DshApiError::Configuration(format!("environment variable {} not set", guid_env))),
  }
}
