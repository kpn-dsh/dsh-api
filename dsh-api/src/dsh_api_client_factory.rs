//! # DSH API client factory
//!
//! This module provides factories for creating [`DshApiClient`] instances,
//! based on the platform and the tenant's name.
//! These parameters can either be provided via environment variables
//! or via explicit function arguments.
//!
//! There are two ways to acquire a `DshApiClientFactory`:
//! * Use the method [`DshApiClientFactory::default()`],
//!   which is configured from the environment variables listed below.
//! * Create a factory explicitly by providing the `platform`,
//!   `tenant` and API `password` parameters yourself and feeding them to the
//!   [`create()`](DshApiClientFactory::create) function.
//!
//! Once you have the `DshApiClientFactory` you can call its
//! [`client()`](DshApiClientFactory::client)
//! method to get a [`DshApiClient`].
//!
//! ## Example
//!
//! In this example explicit tenant parameters are used to create a `DshApiClientFactory`.
//!
//! ```ignore
//! # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//! # use dsh_api::dsh_api_tenant::DshApiTenant;
//! # use dsh_api::platform::DshPlatform;
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let tenant = DshApiTenant::new(
//!   "my-tenant".to_string(),
//!   DshPlatform::try_from("np-aws-lz-dsh")?
//! );
//! let robot_password = "...".to_string();
//! let client_factory = DshApiClientFactory::create_with_token_factory(tenant, robot_password)?;
//! let client = client_factory.client().await?;
//! ...
//! # Ok(())
//! # }
//! ```
//!
//! # Environment variables
//!
//! ## `DSH_API_PLATFORM`
//! Platform on which the tenant's environment lives. The default platforms are:
//! * `k8s-dev-aws-lz-dsh / devlz` - Development platform for Klarrio.
//! * `np-aws-lz-dsh / nplz` - Staging platform for KPN internal tenants.
//! * `poc-aws-dsh / poc` - Staging platform for non KPN tenants.
//! * `prod-aws-dsh / prod` - Production platform for non KPN tenants.
//! * `prod-aws-lz-dsh / prodlz` - Production platform for KPN internal tenants.
//! * `prod-aws-lz-laas / prodls` - Production platform for logstash as a service.
//! * `prod-azure-dsh / prodaz` - Production platform for non KPN tenants.
//!
//! ## `DSH_API_TENANT`
//! Tenant id for the client tenant that is making the API requests.
//! In some cases this is not the same tenant as the tenant whose resources
//! will be managed via the API. The latter will be called the target client.
//!
//! ## `DSH_API_PASSWORD_[platform]_[tenant]`
//! Secret API token for the client tenant.
//! The placeholders `[platform]` and `[tenant]`
//! need to be substituted with the platform name and the tenant name in all capitals,
//! with hyphens (`-`) replaced by underscores (`_`).
//!
//! E.g. if the platform is `np-aws-lz-dsh` and the tenant name is
//! `my-tenant`, the environment variable must be
//! `DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT`.
use crate::dsh_api_client::DshApiClient;
use crate::dsh_api_tenant::DshApiTenant;
use crate::generated::Client as GeneratedClient;
use crate::platform::DshPlatform;
use crate::token_fetcher::ManagementApiTokenFetcherBuilder;
use crate::DshApiError;
use log::debug;
use std::env;

/// # Factory for DSH API client
#[derive(Debug)]
pub struct DshApiClientFactory {
  generated_client: GeneratedClient,
  tenant: DshApiTenant,
  access_token: Option<String>,
  robot_password: Option<String>,
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
  ///   println!("tenant is {}", client.tenant());
  /// }
  /// # }
  /// ```
  /// # Panics
  /// This function will panic if it cannot create a new `DshApiClientFactory` from the default
  /// environment variables. If you want to capture such a failure, use:
  /// * [try_default()](Self::try_default),
  /// * [create_from_access_token()](Self::create_from_access_token) or
  /// * [create_with_token_fetcher()](Self::create_with_token_fetcher).
  pub fn new() -> DshApiClientFactory {
    DshApiClientFactory::default()
  }

  /// # Create factory for DSH API client with token fetcher
  ///
  /// This function will create a new `DshApiClientFactory` from the provided parameters.
  ///
  /// # Parameters
  /// * `tenant` - Tenant struct, containing the platform and tenant name.
  /// * `robot_password` - The secret robot password used to retrieve the DSH API tokens
  ///   by the token fetcher.
  ///
  /// # Returns
  /// * [DshApiClientFactory] - Created client factory.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// use dsh_api::dsh_api_tenant::DshApiTenant;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let robot_password = "...".to_string();
  /// let tenant = DshApiTenant::from_tenant("my-tenant".to_string())?;
  /// let client_factory =
  ///   DshApiClientFactory::create_with_token_fetcher(tenant, robot_password);
  /// let client = client_factory.client().await?;
  /// println!("tenant is {}", client.tenant());
  /// # Ok(())
  /// # }
  /// ```
  pub fn create_with_token_fetcher(tenant: DshApiTenant, robot_password: String) -> Self {
    let endpoint = tenant.platform().rest_api_endpoint();
    debug!("create dsh api client factory with token fetcher for '{}' at endpoint '{}'", tenant, endpoint);
    DshApiClientFactory { generated_client: GeneratedClient::new(endpoint.as_str()), tenant, access_token: None, robot_password: Some(robot_password) }
  }

  /// # Create factory for DSH API client with static access token
  ///
  /// This function will create a new `DshApiClientFactory` from the provided parameters.
  ///
  /// # Parameters
  /// * `tenant` - Tenant struct, containing the platform and tenant name.
  /// * `access_token` - The static access token used to access the API.
  ///
  /// # Returns
  /// * [DshApiClientFactory] - Created client factory.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// use dsh_api::dsh_api_tenant::DshApiTenant;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let access_token = "...".to_string();
  /// let tenant = DshApiTenant::from_tenant("my-tenant".to_string())?;
  /// let client_factory =
  ///   DshApiClientFactory::create_from_access_token(tenant, access_token);
  /// let client = client_factory.client().await?;
  /// println!("tenant is {}", client.tenant());
  /// # Ok(())
  /// # }
  /// ```
  pub fn create_from_access_token(tenant: DshApiTenant, access_token: String) -> Self {
    let endpoint = tenant.platform().rest_api_endpoint();
    debug!("create dsh api client factory with static access token for '{}' at endpoint '{}'", tenant, endpoint);
    DshApiClientFactory { generated_client: GeneratedClient::new(endpoint.as_str()), tenant, access_token: Some(access_token), robot_password: None }
  }

  /// # Create factory for DSH API client
  ///
  /// Deprecated, use [create_with_token_fetcher()](Self::create_with_token_fetcher).
  #[deprecated]
  pub fn create(tenant: DshApiTenant, password: String) -> Result<Self, DshApiError> {
    Ok(Self::create_with_token_fetcher(tenant, password))
  }

  /// # Create default factory for DSH API client with token fetcher
  ///
  /// This function will create a new `DshApiClientFactory` with token fetcher from the
  /// default platform and tenant.
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
  /// let client_factory = DshApiClientFactory::try_default_with_token_factory()?;
  /// let client = client_factory.client().await?;
  /// println!("tenant is {}", client.tenant());
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default_with_token_factory() -> Result<Self, DshApiError> {
    let tenant = DshApiTenant::try_default()?;
    let robot_password = get_robot_password(&tenant)?;
    debug!("create default dsh api client factory for '{}'", tenant);
    Ok(DshApiClientFactory::create_with_token_fetcher(tenant, robot_password))
  }

  /// # Create default factory for DSH API client with access token
  ///
  /// This function will create a new `DshApiClientFactory` with token fetcher from the default platform and tenant.
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
  /// let client_factory = DshApiClientFactory::try_default_from_access_token()?;
  /// let client = client_factory.client().await?;
  /// println!("tenant is {}", client.tenant());
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default_from_access_token() -> Result<Self, DshApiError> {
    let tenant = DshApiTenant::try_default()?;
    let access_token = get_access_token(&tenant)?;
    debug!("create default dsh api client factory for '{}'", tenant);
    Ok(DshApiClientFactory::create_from_access_token(tenant, access_token))
  }

  /// # Create default factory for DSH API client
  ///
  /// This function will create a new `DshApiClientFactory` with either a token fetcher or an
  /// access token from the default platform and tenant.
  /// This function will fail if both a robot password and an access token are configured.
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
  /// println!("tenant is {}", client.tenant());
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default() -> Result<Self, DshApiError> {
    let tenant = DshApiTenant::try_default()?;
    match (get_access_token(&tenant), get_robot_password(&tenant)) {
      (Err(_), Err(_)) => Err(DshApiError::Configuration("missing robot password or access token configuration".to_string())),
      (Err(_), Ok(robot_password)) => {
        debug!("create default dsh api client factory with token fetcher for '{}'", tenant);
        Ok(DshApiClientFactory::create_with_token_fetcher(tenant, robot_password))
      }
      (Ok(access_token), Err(_)) => {
        debug!("create default dsh api client factory with static access token for '{}'", tenant);
        Ok(DshApiClientFactory::create_from_access_token(tenant, access_token))
      }
      (Ok(_), Ok(_)) => Err(DshApiError::Configuration("both robot password and access token are configured".to_string())),
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

  /// # Create an DSH API client
  ///
  /// This function will create a new `DshApiClient`.
  ///
  /// # Returns
  /// * `Ok<DshApiClient>` - Created client.
  /// * `Err<String>` - Error message when the client could not be created.
  ///
  /// # Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  ///
  /// # use dsh_api::DshApiError;
  /// # async fn hide() -> Result<(), DshApiError> {
  /// let client_factory = DshApiClientFactory::new();
  /// match client_factory.client().await {
  ///   Ok(client) => println!("tenant is {}", client.tenant()),
  ///   Err(error) => println!("could not create client ({})", error),
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub async fn client(self) -> Result<DshApiClient, DshApiError> {
    if let Some(robot_password) = self.robot_password {
      match ManagementApiTokenFetcherBuilder::new(self.tenant.platform().clone())
        .tenant_name(self.tenant.name().clone())
        .client_secret(robot_password)
        .build()
      {
        Ok(token_fetcher) => Ok(DshApiClient::from_token_fetcher(token_fetcher, self.generated_client, self.tenant.clone())),
        Err(rest_token_error) => Err(DshApiError::Unexpected(
          format!("could not create token fetcher ({})", rest_token_error),
          Some(rest_token_error.to_string()),
        )),
      }
    } else if let Some(access_token) = self.access_token {
      Ok(DshApiClient::from_static_token(access_token, self.generated_client, self.tenant.clone()))
    } else {
      unreachable!()
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
  /// [`try_default()`](DshApiClientFactory::try_default) or the
  /// [`create()`](DshApiClientFactory::create) function.
  fn default() -> Self {
    match Self::try_default() {
      Ok(factory) => factory,
      Err(error) => panic!("{}", error),
    }
  }
}

const ENV_VAR_ACCESS_TOKEN_PREFIX: &str = "DSH_API_ACCESS_TOKEN";
const ENV_VAR_ACCESS_TOKEN__FILE_PREFIX: &str = "DSH_API_ACCESS_TOKEN_FILE";

fn get_access_token(tenant: &DshApiTenant) -> Result<String, DshApiError> {
  get_password(tenant, ENV_VAR_ACCESS_TOKEN_PREFIX, ENV_VAR_ACCESS_TOKEN__FILE_PREFIX)
}

const ENV_VAR_PASSWORD_PREFIX: &str = "DSH_API_PASSWORD";
const ENV_VAR_PASSWORD_FILE_PREFIX: &str = "DSH_API_PASSWORD_FILE";

fn get_robot_password(tenant: &DshApiTenant) -> Result<String, DshApiError> {
  get_password(tenant, ENV_VAR_PASSWORD_PREFIX, ENV_VAR_PASSWORD_FILE_PREFIX)
}

fn get_password(tenant: &DshApiTenant, password_env_var_prefix: &str, password_file_env_var_prefix: &str) -> Result<String, DshApiError> {
  let password_file_env_var = environment_variable(password_file_env_var_prefix, tenant.platform(), tenant.name());
  match env::var(&password_file_env_var) {
    Ok(password_file_from_env_var) => match std::fs::read_to_string(&password_file_from_env_var) {
      Ok(password_from_file) => {
        let trimmed_password = password_from_file.trim();
        if trimmed_password.is_empty() {
          Err(DshApiError::Configuration(format!("password file '{}' is empty", password_file_from_env_var)))
        } else {
          debug!(
            "password read from file '{}' in environment variable '{}'",
            password_file_from_env_var, password_file_env_var
          );
          Ok(trimmed_password.to_string())
        }
      }
      Err(_) => Err(DshApiError::Configuration(format!(
        "password file '{}' could not be read",
        password_file_from_env_var
      ))),
    },
    Err(_) => {
      let password_env_var = environment_variable(password_env_var_prefix, tenant.platform(), tenant.name());
      match env::var(&password_env_var) {
        Ok(password_from_env_var) => {
          debug!("password read from environment variable '{}'", password_env_var);
          Ok(password_from_env_var)
        }
        Err(_) => Err(DshApiError::Configuration(format!("environment variable '{}' not set", password_env_var))),
      }
    }
  }
}

fn environment_variable(env_var_prefix: &str, platform: &DshPlatform, tenant_name: &str) -> String {
  format!(
    "{}_{}_{}",
    env_var_prefix,
    platform.name().to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  )
}
