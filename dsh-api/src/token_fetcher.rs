//! # Token fetcher for DSH
//!
//! This module provides capabilities for fetching and caching access tokens required to
//! communicate with the DSH resource management API. Access tokens can be re-used and are
//! automatically refreshed when they expire. A token fetcher can be reused in subsequent
//! calls during the lifetime of your application.
//!
//! # Example
//!
//! ```
//! # use dsh_api::error::DshApiResult;
//! use dsh_api::token_fetcher::TokenFetcher;
//!
//! # async fn hide() -> DshApiResult<()> {
//! let token_fetcher = TokenFetcher::default();
//! let token = token_fetcher.get_bearer_token().await?;
//! # Ok(())
//! # }
//! ```
use crate::dsh_api_client_factory::get_robot_password;
use crate::dsh_api_tenant::DshApiTenant;
use crate::dsh_jwt::DshJwt;
use crate::error::{DshApiError, DshApiResult};
use log::{debug, error, trace};
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::fmt::Debug;
use std::ops::Add;
use std::str::FromStr;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// # Fetcher for access tokens
///
/// A token fetcher for obtaining and storing access tokens, enabling authenticated requests
/// to the DSH resource management API. This struct caches the token in memory and refreshes
/// it automatically once expired.
pub struct TokenFetcher {
  access_token_container: Mutex<Option<(AccessTokenContainer, Instant)>>,
  client: Client,
  client_id: String,
  client_secret: String,
  dsh_api_tenant: DshApiTenant,
}

type Unpacker<T> = dyn Fn(&AccessTokenContainer) -> DshApiResult<T> + Sync;

impl TokenFetcher {
  /// # Create a new token fetcher
  ///
  /// After creation the token fetcher can be reused in subsequent calls during the lifetime of
  /// your application. The token will automatically be refreshed when it is about to expire.
  ///
  /// # Parameters
  ///
  /// * `dsh_api_tenant` - [DshApiTenant] struct that contains the platform and the tenant name.
  /// * `client_secret` - Robot password for the platform and tenant.
  /// * `client_id` - Optional client id. When not provided a default client id will be created.
  /// * `client` - Optional `Reqwest` client. When not provided a default client will be created.
  ///
  /// # Example
  ///
  /// ```
  /// # use dsh_api::dsh_api_tenant::DshApiTenant;
  /// # use dsh_api::error::DshApiResult;
  /// # use dsh_api::token_fetcher::TokenFetcher;
  /// # async fn hide() -> DshApiResult<()> {
  /// # use dsh_api::platform::DshPlatform;
  /// let platform = DshPlatform::new("nplz");
  /// let dsh_api_tenant = DshApiTenant::new("my-tenant", platform);
  /// let token_fetcher = TokenFetcher::new(dsh_api_tenant, "my-secret".to_string(), None, None);
  /// let token = token_fetcher.get_bearer_token().await?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn new(dsh_api_tenant: DshApiTenant, client_secret: String, client_id: Option<String>, client: Option<Client>) -> Self {
    let client_id = client_id.unwrap_or(dsh_api_tenant.platform().tenant_client_id(dsh_api_tenant.name()));
    debug!(
      "new token fetcher with client, client id: '{}', url: '{}'",
      client_id,
      dsh_api_tenant.platform().access_token_endpoint()
    );
    Self { access_token_container: Mutex::new(None), client: client.unwrap_or_default(), client_id, client_secret, dsh_api_tenant }
  }

  /// # Create token fetcher from default settings
  ///
  /// This function will create a new `TokenFetcher` from default values, obtained from
  /// environment variables.
  ///
  /// # Returns
  /// * `Ok<DshApiClientFactory>` - Created client factory.
  /// * `Err<DshApiError>` - When the client factory could not be created.
  ///
  /// # Examples
  ///
  /// ```bash
  /// > export DSH_API_PLATFORM=np-aws-lz-dsh
  /// > export DSH_API_TENANT=my-tenant
  /// > export DSH_API_PASSWORD_NP_AWS_LZ_DSH_MY_TENANT=...
  /// ````
  ///
  /// ```no_run
  /// # use dsh_api::error::DshApiResult;  /// #
  /// use dsh_api::token_fetcher::TokenFetcher;
  ///
  /// # async fn hide() -> DshApiResult<()> {
  /// let token_fetcher = TokenFetcher::try_default()?;
  /// let token = token_fetcher.get_bearer_token().await?;
  /// # Ok(())
  /// # }
  /// ```
  pub fn try_default() -> DshApiResult<Self> {
    let tenant = DshApiTenant::try_default()?;
    match get_robot_password(&tenant)? {
      Some(client_secret) => {
        debug!("default token fetcher");
        Ok(Self::new(tenant, client_secret, None, None))
      }
      None => Err(DshApiError::configuration("missing robot password configuration")),
    }
  }

  /// # Get a bearer token
  ///
  /// Obtains a bearer token, using the cached access token if it is still valid, otherwise
  /// fetches a new one. The returned string is formatted as `"{token_type} {access_token}"`.
  pub async fn get_bearer_token(&self) -> DshApiResult<String> {
    self.fetch_container_and_unpack(&Self::unpack_bearer_token).await
  }

  /// # Get a fresh bearer token
  ///
  /// This function will request a fresh access token, write it in the cache and return the
  /// bearer token.
  pub async fn get_fresh_bearer_token(&self) -> DshApiResult<String> {
    self.refresh_container_and_unpack(&Self::unpack_bearer_token).await
  }

  /// # Get a raw token
  ///
  /// Obtains a raw token, using the cached access token if it is still valid, otherwise
  /// fetches a new one.
  pub async fn get_raw_token(&self) -> DshApiResult<String> {
    self.fetch_container_and_unpack(&Self::unpack_raw_token).await
  }

  /// # Get a fresh raw token
  ///
  /// This function will request a fresh access token, write it in the cache and return the
  /// raw token.
  pub async fn get_fresh_raw_token(&self) -> DshApiResult<String> {
    self.refresh_container_and_unpack(&Self::unpack_raw_token).await
  }

  /// # Get a json web token
  ///
  /// Obtains a json web token, using the cached access token if it is still valid, otherwise
  /// fetches a new one.
  pub async fn get_jwt(&self) -> DshApiResult<DshJwt> {
    self.fetch_container_and_unpack(&Self::unpack_jwt).await
  }

  /// # Get a fresh json web token
  ///
  /// This function will request a fresh access token, write it in the cache and return the
  /// json web token.
  pub async fn get_fresh_jwt(&self) -> DshApiResult<DshJwt> {
    self.refresh_container_and_unpack(&Self::unpack_jwt).await
  }

  fn unpack_bearer_token(container: &AccessTokenContainer) -> DshApiResult<String> {
    Ok(format!("{} {}", container.token_type, container.access_token))
  }

  fn unpack_raw_token(container: &AccessTokenContainer) -> DshApiResult<String> {
    Ok(container.access_token.clone())
  }

  fn unpack_jwt(container: &AccessTokenContainer) -> DshApiResult<DshJwt> {
    DshJwt::from_str(container.access_token.as_str()).map_err(|_| DshApiError::unexpected("could not parse fetched token"))
  }

  // Obtains an access token from the cache if it is available and still valid, otherwise
  // fetches a new one. The unpack function will be applied to the access token and its result
  // will be returned.
  async fn fetch_container_and_unpack<T>(&self, unpack: &Unpacker<T>) -> DshApiResult<T> {
    match self.status() {
      TokenStatus::Invalid => {
        debug!("fetch token (expired)");
        let container = self.fetch_access_token_container_from_server().await?;
        let mut guarded_container = self.access_token_container.lock()?;
        let unpacked_value = unpack(&container);
        *guarded_container = Some((container, Instant::now()));
        unpacked_value
      }
      TokenStatus::Uninitialized => {
        debug!("fetch token (initial)");
        let container = self.fetch_access_token_container_from_server().await?;
        let mut guarded_container = self.access_token_container.lock()?;
        let unpacked_value = unpack(&container);
        *guarded_container = Some((container, Instant::now()));
        unpacked_value
      }
      TokenStatus::Valid => {
        debug!("fetch token (from cache)");
        match self.access_token_container.lock()?.clone() {
          Some((container, _)) => unpack(&container),
          None => unreachable!(),
        }
      }
    }
  }

  // This function will fetch a fresh access token and write it in the cache. The unpack
  // function will be applied on the access token and its result will be returned.
  async fn refresh_container_and_unpack<T>(&self, unpack: &Unpacker<T>) -> DshApiResult<T> {
    debug!("fetch fresh token");
    let container = self.fetch_access_token_container_from_server().await?;
    let mut guarded_container = self.access_token_container.lock()?;
    let unpacked_value = unpack(&container);
    *guarded_container = Some((container, Instant::now()));
    unpacked_value
  }

  // Determines if the internally cached token is still valid. A token is considered valid if
  // its remaining lifetime is greater than zero, with a safety margin of 5 seconds.
  fn status(&self) -> TokenStatus {
    match self.access_token_container.lock() {
      Ok(guarded_fetcher_token) => match guarded_fetcher_token.clone() {
        Some((fetcher_token, fetched_at)) => {
          if fetched_at.elapsed().add(Duration::from_secs(5)) < Duration::from_secs(fetcher_token.expires_in) {
            TokenStatus::Valid
          } else {
            TokenStatus::Invalid
          }
        }
        None => TokenStatus::Uninitialized,
      },
      Err(mut poison_error) => {
        **poison_error.get_mut() = None;
        self.access_token_container.clear_poison();
        let _unused = poison_error.into_inner();
        TokenStatus::Uninitialized
      }
    }
  }

  // Fetches a fresh access token from the authentication server.
  async fn fetch_access_token_container_from_server(&self) -> DshApiResult<AccessTokenContainer> {
    let form = [("client_id", self.client_id.as_ref()), ("client_secret", self.client_secret.as_ref()), ("grant_type", "client_credentials")];
    debug!("post {}", self.dsh_api_tenant.platform().access_token_endpoint());
    let mut request_builder = self.client.post(self.dsh_api_tenant.platform().access_token_endpoint());
    request_builder = request_builder.form(&form);
    let request = request_builder.build().map_err(DshApiError::from)?;
    trace!("fetch access token from server -> {:#?}", request);
    let response = self.client.execute(request).await;
    trace!("fetch access token from server -> {:#?}", response);
    match response {
      Ok(response) => {
        if !response.status().is_success() {
          Err(DshApiError::Unexpected { message: format!("statuscode {}", response.status()), cause: response.text().await.ok() })
        } else {
          let json = response.text().await?;
          trace!("fetch access token from server -> {}", json);
          let container = serde_json::from_str::<AccessTokenContainer>(&json)?;
          Ok(container)
        }
      }
      Err(error) => {
        error!("could not fetch access token from server ({})", error);
        if let Some(source) = error.source() {
          debug!("error source: {:?}", source);
        }
        Err(DshApiError::from(error))
      }
    }
  }
}

impl Default for TokenFetcher {
  /// # Create default token fetcher
  ///
  /// # Panics
  /// This function will panic if it cannot create a new `TokenFetcher` from the default
  /// environment variables. If you want to capture such a failure, use the
  /// [`try_default()`](TokenFetcher::try_default) function.
  fn default() -> Self {
    Self::try_default().unwrap_or_else(|error| panic!("{}", error))
  }
}

#[derive(Clone, Deserialize)]
struct AccessTokenContainer {
  // Raw access token string (without the token type)
  access_token: String,
  // Number of seconds until this token expires
  expires_in: u64,
  // Number of seconds until the refresh token expires
  #[allow(unused)]
  refresh_expires_in: u32,
  // Token type (usually `"Bearer"`)
  token_type: String,
  // “not before” policy timestamp from the authentication server.
  #[serde(rename(deserialize = "not-before-policy"))]
  #[allow(unused)]
  not_before_policy: u32,
  // Scope string (e.g., `"email"`).
  #[allow(unused)]
  scope: String,
}

enum TokenStatus {
  Invalid,
  Uninitialized,
  Valid,
}

impl Debug for AccessTokenContainer {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("AccessTokenContainer")
      .field("access_token", &"[redacted]")
      .field("expires_in", &self.expires_in)
      .field("refresh_expires_in", &self.refresh_expires_in)
      .field("token_type", &self.token_type)
      .field("not_before_policy", &self.not_before_policy)
      .field("scope", &self.scope)
      .finish()
  }
}
