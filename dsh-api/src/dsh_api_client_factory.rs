//! # Factory for DSH API client
//!
//! This module provides factories for creating [`DshApiClient`] instances,
//! based on the platform and the tenant's name, group id and user id.
//! These parameters can either be provided as function arguments,
//! or via a set of environment variables.
//!
//! # Environment variables
//!
//! ## `DSH_API_PLATFORM`
//! Platform on which the tenant's environment lives. The default platforms are:
//! * `np-aws-lz-dsh / nplz` - Staging platform for KPN internal tenants.
//! * `poc-aws-dsh / poc` - Staging platform for non KPN tenants.
//! * `prod-aws-dsh / prod` - Production platform for non KPN tenants.
//! * `prod-aws-lz-dsh / prodlz` - Production platform for KPN internal tenants.
//! * `prod-aws-lz-laas / prodls` - Production platform for logstash as a service.
//! * `prod-azure-dsh / prodaz` - Production platform for non KPN tenants.
//!
//! ## `DSH_API_TENANT`
//! Tenant id for the client tenant that is making the api requests.
//! In some cases this is not the same tenant as the tenant whose resources
//! will be managed via the api. The latter will be called the target client.
//!
//! ## `DSH_API_PASSWORD_[platform]_[tenant]`
//! Secret api token for the client tenant.
//! The placeholders `[platform]` and `[tenant]`
//! need to be substituted with the platform name and the tenant name in all capitals,
//! with hyphens (`-`) replaced by underscores (`_`).
//!
//! E.g. if the platform is `np-aws-lz-dsh` and the tenant name is
//! `my-tenant`, the environment variable must be
//! `DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT`.
//!
//! ## `DSH_API_GUID_[tenant]`
//! Group id and user id for the clienttenant.
//! The placeholder `[tenant]` needs to be substituted
//! with the tenant name in all capitals, with hyphens (`-`)
//! replaced by underscores (`_`).
//!
//! E.g. if the tenant name is `my-tenant`, the environment variable must be
//! `DSH_API_GUID_MY_TENANT`.
use crate::dsh_api_client::DshApiClient;
use crate::dsh_api_tenant::DshApiTenant;
use crate::generated::Client as GeneratedClient;
use crate::platform::DshPlatform;
use crate::{password_environment_variable, password_file_environment_variable, DshApiError};
use dsh_sdk::Platform as SdkPlatform;
use dsh_sdk::RestTokenFetcherBuilder;
use lazy_static::lazy_static;
use log::info;
use std::env;

/// # Factory for DSH API client
#[derive(Debug)]
pub struct DshApiClientFactory {
  generated_client: GeneratedClient,
  tenant: DshApiTenant,
  password: String,
}

impl DshApiClientFactory {
  /// # Create default factory for DSH API client
  ///
  /// This function will create a new `DshApiClientFactory` from the default environment variables.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  ///
  /// # async fn hide() {
  /// let client_factory = DshApiClientFactory::new();
  /// if let Ok(client) = client_factory.client().await {
  ///   println!("rest api version is {}", client.api_version());
  /// }
  /// # }
  /// ```
  /// # Panics
  /// This function will panic if it cannot create a new `DshApiClientFactory` from the default
  /// environment variables. If you want to capture such a failure, use the
  /// [`create()`](DshApiClientFactory::create) function.
  pub fn new() -> DshApiClientFactory {
    DshApiClientFactory::default()
  }

  /// # Create factory for DSH API client
  ///
  /// This function will create a new `DshApiClientFactory` from the provided parameters.
  ///
  /// # Parameters
  /// * `tenant` - Tenant struct, containing the platform, tenant name and the
  ///   tenant's group and user id.
  /// * `password` - The secret password used to retrieve the DSH API tokens.
  ///
  /// # Returns
  /// * `Ok<DshApiClientFactory>` - the created client factory
  /// * `Err<String>` - when the client factory could not be created
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// use dsh_api::dsh_api_tenant::DshApiTenant;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let password = "...".to_string();
  /// let tenant = DshApiTenant::from_tenant("my-tenant".to_string())?;
  /// let client_factory = DshApiClientFactory::create(tenant, password)?;
  /// let client = client_factory.client().await?;
  /// println!("rest api version is {}", client.api_version());
  /// # Ok(())
  /// # }
  /// ```
  pub fn create(tenant: DshApiTenant, password: String) -> Result<Self, DshApiError> {
    Ok(DshApiClientFactory { generated_client: GeneratedClient::new(tenant.platform().rest_api_endpoint().as_str()), tenant, password })
  }

  /// # Create default factory for DSH API client
  ///
  /// This function will create a new `DshApiClientFactory` from the default platform and tenant.
  ///
  /// # Returns
  /// * `Ok<DshApiClientFactory>` - the created client factory
  /// * `Err<String>` - when the client factory could not be created
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let client_factory = DshApiClientFactory::try_default()?;
  /// let client = client_factory.client().await?;
  /// println!("rest api version is {}", client.api_version());
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default() -> Result<Self, DshApiError> {
    let platform = DshPlatform::try_default()?;
    let tenant = DshApiTenant::try_default()?;
    let password = match get_password(&tenant) {
      Ok(password) => password,
      Err(error) => panic!("{}", error),
    };
    match DshApiClientFactory::create(tenant.clone(), password) {
      Ok(factory) => {
        info!("default dsh api client factory for {}@{} created", tenant.name(), platform.to_string());
        Ok(factory)
      }
      Err(error) => panic!("{}", error),
    }
  }

  /// # Returns the factories platform
  pub fn platform(&self) -> &DshPlatform {
    self.tenant.platform()
  }

  /// # Returns the factories tenant
  pub fn tenant(&self) -> &DshApiTenant {
    &self.tenant
  }

  /// # Returns the name of the factories tenant
  pub fn tenant_name(&self) -> &str {
    self.tenant.name()
  }

  /// # Returns the group and user id of the factories tenant
  pub fn guid(&self) -> u16 {
    self.tenant.guid()
  }

  /// # Create an DSH API client
  ///
  /// This function will create a new `DshApiClient`.
  ///
  /// # Returns
  /// * `Ok<DshApiClient>` - the created client
  /// * `Err<String>` - error message when the client could not be created
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let client_factory = DshApiClientFactory::new();
  /// match client_factory.client().await {
  ///   Ok(client) => println!("rest api version is {}", client.api_version()),
  ///   Err(error) => println!("could not create client ({})", error),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub async fn client(&self) -> Result<DshApiClient, DshApiError> {
    match RestTokenFetcherBuilder::new(SdkPlatform::try_from(self.tenant.platform())?)
      .tenant_name(self.tenant.name().clone())
      .client_secret(self.password.clone())
      .build()
    {
      Ok(token_fetcher) => Ok(DshApiClient::new(token_fetcher, &self.generated_client, &self.tenant)),
      Err(rest_token_error) => Err(DshApiError::Unexpected(
        format!("could not create token fetcher ({})", rest_token_error),
        Some(Box::new(rest_token_error)),
      )),
    }
  }
}

impl Default for DshApiClientFactory {
  /// # Create default factory for DSH API client
  ///
  /// For the explanation, see the [`new()`](DshApiClientFactory::new) function,
  /// which delegates to the default implementation.
  ///
  /// # Panics
  /// This function will panic if it cannot create a new `DshApiClientFactory` from the default
  /// environment variables. If you want to capture such a failure, use the
  /// [`create()`](DshApiClientFactory::create) function.
  fn default() -> Self {
    match Self::try_default() {
      Ok(factory) => {
        info!("default dsh api client factory for {} created", factory.tenant);
        factory
      }
      Err(error) => panic!("{}", error),
    }
  }
}

lazy_static! {
  /// # Default factory for DSH API client
  ///
  /// Static `DshApiClientFactory`, created lazily from the default environment variables.
  /// This value is targeted at testing and examples and should not be used in real applications.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  /// let client = client_factory.client().await?;
  /// println!("rest api version is {}", client.api_version());
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Panics
  /// Lazily creating the instance will panic if a new `DshApiClientFactory` cannot be created
  /// from the default environment variables.
  pub static ref DEFAULT_DSH_API_CLIENT_FACTORY: DshApiClientFactory = DshApiClientFactory::default();

  /// # Fallible default factory for DSH API client
  ///
  /// Static `Result<DshApiClientFactory, DshApiError>`, created lazily from the default
  /// environment variables. This value is targeted at testing and examples and should not be
  /// used in real applications.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::{
  ///   DshApiClientFactory,
  ///   TRY_DEFAULT_DSH_API_CLIENT_FACTORY
  /// };
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  ///
  /// // Explicit type declaration is important since type inference will not work here
  /// let try_factory: &Result<DshApiClientFactory, DshApiError> =
  ///   &TRY_DEFAULT_DSH_API_CLIENT_FACTORY;
  /// match try_factory {
  ///   Ok(factory) => {
  ///     let client = factory.client().await?;
  ///     println!("rest api version is {}", client.api_version());
  ///   },
  ///   Err(error) => println!("could not create client factory: {}", error)
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub static ref TRY_DEFAULT_DSH_API_CLIENT_FACTORY: Result<DshApiClientFactory, DshApiError> = DshApiClientFactory::try_default();
}

fn get_password(tenant: &DshApiTenant) -> Result<String, DshApiError> {
  let password_file_env_var = password_file_environment_variable(tenant.platform().name(), tenant.name());
  match env::var(&password_file_env_var) {
    Ok(password_file_from_env_var) => match std::fs::read_to_string(&password_file_from_env_var) {
      Ok(password_from_file) => {
        let trimmed_password = password_from_file.trim();
        if trimmed_password.is_empty() {
          Err(DshApiError::Configuration(format!("password file '{}' is empty", password_file_from_env_var)))
        } else {
          Ok(trimmed_password.to_string())
        }
      }
      Err(_) => Err(DshApiError::Configuration(format!(
        "password file '{}' could not be read",
        password_file_from_env_var
      ))),
    },
    Err(_) => {
      let password_env_var = password_environment_variable(tenant.platform().name(), tenant.name());
      match env::var(&password_env_var) {
        Ok(password_from_env_var) => Ok(password_from_env_var),
        Err(_) => Err(DshApiError::Configuration(format!("environment variable {} not set", password_env_var))),
      }
    }
  }
}
