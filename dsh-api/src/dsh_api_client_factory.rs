//! # Factory for DSH API client
//!
//! This module provides factories for creating [`DshApiClient`] instances,
//! based on the platform and the tenant's name, group id and user id.
//! These parameters can either be provided as function arguments,
//! or via a set of environment variables.
//!
//! ## Environment variables
//!
//! ### `DSH_API_PLATFORM`
//! Target platform on which the tenant's environment lives.
//! * `nplz` - Non production landing zone
//! * `poc` - Proof of concept platform
//! * `prod` - Production landing zone
//! * `prodaz` -
//! * `prodlz` -
//!
//! ### `DSH_API_TENANT`
//! Tenant id for the target tenant. The target tenant is the tenant whose resources
//! will be managed via the api.
//!
//! ### `DSH_API_SECRET_[platform]_[tenant]`
//! Secret api token for the target tenant.
//! The placeholders `[platform]` and `[tenant]`
//! need to be substituted with the platform name and the tenant name in all capitals,
//! with hyphens (`-`) replaced by underscores (`_`).
//!
//! E.g. if the platform is `nplz` and the tenant name is
//! `greenbox-dev`, the environment variable must be
//! `DSH_API_SECRET_NPLZ_GREENBOX_DEV`.
//!
//! ### `DSH_API_GUID_[tenant]`
//! Group id and user id for the target tenant.
//! The placeholder `[tenant]` needs to be substituted
//! with the tenant name in all capitals, with hyphens (`-`)
//! replaced by underscores (`_`).
//!
//! E.g. if the tenant name is `greenbox-dev`, the environment variable must be
//! `DSH_API_GUID_GREENBOX_DEV`.
use std::env;

use crate::dsh_api_client::DshApiClient;
use crate::dsh_api_tenant::{DshApiTenant, DEFAULT_DSH_API_TENANT};
use crate::generated::Client as GeneratedClient;
use crate::platform::{DshPlatform, DEFAULT_DSH_PLATFORM};
use dsh_sdk::RestTokenFetcherBuilder;
use dsh_sdk::{Platform as SdkPlatform, RestTokenFetcher};
use lazy_static::lazy_static;
use log::info;

/// ## Factory for DSH API client
#[derive(Debug)]
pub struct DshApiClientFactory {
  token_fetcher: RestTokenFetcher,
  generated_client: GeneratedClient,
  tenant: DshApiTenant,
}

impl DshApiClientFactory {
  /// # Create default factory for DSH API client
  ///
  /// This function will create a new `DshApiClientFactory` from the default environment variables.
  ///
  /// ## Examples
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
  /// ## Panics
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
  /// ## Parameters
  /// * `tenant` - Tenant struct, containing the platform, tenant name and the
  ///   tenant's group and user id.
  /// * `secret` - The secret used to retrieve the DSH API tokens.
  ///
  /// ## Returns
  /// * `Ok<DshApiClientFactory>` - the created client factory
  /// * `Err<String>` - when the client factory could not be created
  ///
  /// ## Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// use dsh_api::dsh_api_tenant::DshApiTenant;
  /// use dsh_api::platform::DshPlatform;
  ///
  /// # async fn hide() -> Result<(), String> {
  /// let secret = "...".to_string();
  /// let tenant = DshApiTenant::from_tenant("greenbox".to_string())?;
  /// let client_factory = DshApiClientFactory::create(tenant, secret)?;
  /// let client = client_factory.client().await?;
  /// println!("rest api version is {}", client.api_version());
  /// # Ok(())
  /// # }
  /// ```
  pub fn create(tenant: DshApiTenant, secret: String) -> Result<Self, String> {
    match RestTokenFetcherBuilder::new(SdkPlatform::from(tenant.platform()))
      .tenant_name(tenant.name().clone())
      .client_secret(secret)
      .build()
    {
      Ok(token_fetcher) => {
        let generated_client = GeneratedClient::new(tenant.platform().endpoint_rest_api().as_str());
        Ok(DshApiClientFactory { token_fetcher, generated_client, tenant })
      }
      Err(e) => Err(format!("could not create token fetcher ({})", e)),
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
  pub fn guid(&self) -> &str {
    self.tenant.guid()
  }

  /// # Create an DSH API client
  ///
  /// This function will create a new `DshApiClient`.
  ///
  /// ## Returns
  /// * `Ok<DshApiClient>` - the created client
  /// * `Err<String>` - error message when the client could not be created
  ///
  /// ## Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  ///
  /// # async fn hide() -> Result<(), String> {
  /// let client_factory = DshApiClientFactory::new();
  /// match client_factory.client().await {
  ///   Ok(client) => println!("rest api version is {}", client.api_version()),
  ///   Err(error) => println!("could not create client ({})", error)
  /// }
  /// # Ok(())
  /// # }
  /// ```
  pub async fn client(&self) -> Result<DshApiClient, String> {
    match self.token_fetcher.get_token().await {
      Ok(token) => Ok(DshApiClient::new(token, &self.generated_client, &self.tenant)),
      Err(e) => Err(format!("could not create token ({})", e)),
    }
  }
}

impl Default for DshApiClientFactory {
  /// # Create default factory for DSH API client
  ///
  /// For the explanation, see the [`new()`](DshApiClientFactory::new) function,
  /// which delegates to the default implementation.
  fn default() -> Self {
    let platform: &DEFAULT_DSH_PLATFORM = &DEFAULT_DSH_PLATFORM;
    let tenant: &DEFAULT_DSH_API_TENANT = &DEFAULT_DSH_API_TENANT;
    let secret = match get_secret_from_platform_and_tenant(platform.to_string().as_str(), tenant.name()) {
      Ok(secret) => secret,
      Err(error) => panic!("{}", error),
    };
    match DshApiClientFactory::create((*tenant).clone(), secret) {
      Ok(factory) => {
        info!("default dsh api client factory for {}@{} created", tenant.name(), platform.to_string());
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
  /// ## Examples
  /// ```no_run
  /// use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
  ///
  /// # async fn hide() -> Result<(), String> {
  /// let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  /// let client = client_factory.client().await?;
  /// println!("rest api version is {}", client.api_version());
  /// # Ok(())
  /// # }
  /// ```
  /// ## Panics
  /// Lazily creating the instance will panic if a new `DshApiClientFactory` cannot be created
  /// from the default environment variables.
  pub static ref DEFAULT_DSH_API_CLIENT_FACTORY: DshApiClientFactory = DshApiClientFactory::default();
}

pub fn get_secret_from_platform_and_tenant(platform_name: &str, tenant_name: &str) -> Result<String, String> {
  let secret_env = format!(
    "DSH_API_SECRET_{}_{}",
    platform_name.to_ascii_uppercase().replace('-', "_"),
    tenant_name.to_ascii_uppercase().replace('-', "_")
  );
  env::var(&secret_env).map_err(|_| format!("environment variable {} not set", secret_env))
}
