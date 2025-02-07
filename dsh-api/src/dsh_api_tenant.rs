//! # Client tenant
use crate::platform::DshPlatform;
use crate::{DshApiError, ENV_VAR_TENANT};
use lazy_static::lazy_static;
use log::info;
use std::env;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct DshApiTenant {
  name: String,
  platform: DshPlatform,
}

impl DshApiTenant {
  /// # Create new DSH API tenant
  ///
  /// # Parameters
  /// * `name` - client tenant's name
  /// * `platform` - target platform for the API
  ///
  /// # Examples
  ///
  /// ```no_run
  /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::platform::DshPlatform;
  /// let name = String::from("my-tenant");
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let dsh_api_tenant = DshApiTenant::new(name, platform);
  /// assert_eq!(
  ///   dsh_api_tenant.platform().internal_service_domain("my-service"),
  ///   "my-service.marathon.mesos".to_string()
  /// );
  /// # Ok(())
  /// # }
  /// ```
  pub fn new(name: String, platform: DshPlatform) -> Self {
    Self { name, platform }
  }

  /// # Create new DSH API tenant from tenant's name
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// tenant's name.
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
    let platform = DshPlatform::default();
    Ok(DshApiTenant::new(tenant_name, platform))
  }

  /// # Create new DSH API tenant from tenant's name and platform
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// tenant's name.
  /// The function will return an `Error<String>` if the environment variable is not set.
  ///
  /// # Parameters
  /// * `tenant_name` - tenant's name
  /// * `platform` - target platform for the API
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
    Ok(DshApiTenant::new(tenant_name, platform))
  }

  /// # Create new DSH API tenant from platform
  ///
  /// This factory function will attempt to create a `DshapiTenant` instance from the provided
  /// `platform`. The tenant's name will be read from the environment variable.
  /// The function will return an `Error<String>` if the environment variables are not set.
  ///
  /// # Parameters
  /// * `platform` - target platform for the API
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
    Ok(DshApiTenant::new(tenant_name, platform))
  }

  /// Returns the default tenant
  ///
  /// This method will read the tenant name and platform form the respective
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
    let platform = DshPlatform::try_default()?;
    Ok(DshApiTenant::new(tenant_name, platform))
  }

  /// Returns the client's platform
  pub fn platform(&self) -> &DshPlatform {
    &self.platform
  }

  /// Returns the client's tenant name
  pub fn name(&self) -> &String {
    &self.name
  }
}

impl Default for DshApiTenant {
  /// Returns the default tenant
  ///
  /// This method will read the tenant name and platform form the respective
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
    write!(f, "{}@{}", self.name, self.platform)
  }
}

lazy_static! {
  pub static ref DEFAULT_DSH_API_TENANT: DshApiTenant = DshApiTenant::default();
}

fn get_default_tenant_name() -> Result<String, DshApiError> {
  env::var(ENV_VAR_TENANT).map_err(|_| DshApiError::Configuration(format!("environment variable {} not set", ENV_VAR_TENANT)))
}
