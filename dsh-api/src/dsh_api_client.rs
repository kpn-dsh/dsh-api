//! # DSH API client
//!
//! The struct [`DshApiClient`] is the base of this library and has many associated methods
//! that you can use to call the operations of the DSH resource management API.
//!
//! In order to use these methods, you first need to acquire an instance of this struct.
//! This is a two-step process.
//! * First you need to get a
//!   [`DshApiClientFactory`](crate::dsh_api_client_factory::DshApiClientFactory):
//!   * Either use the
//!     [`DshApiClientFactory::default()`](crate::dsh_api_client_factory::DshApiClientFactory::default),
//!     method, which is configured from
//!     [environment variables](dsh_api_client_factory/index.html#environment-variables),
//!   * or you can create a factory explicitly by providing the `platform`,
//!     `tenant` and API `password` yourself and feeding them to the
//!     [`DshApiClientFactory::create_with_token_fetcher()`](crate::dsh_api_client_factory::DshApiClientFactory::create_with_token_fetcher)
//!     function.
//! * Once you have the
//!   [`DshApiClientFactory`](crate::dsh_api_client_factory::DshApiClientFactory),
//!   you can call its
//!   [`client()`](crate::dsh_api_client_factory::DshApiClientFactory::client) method.
//!
//! You can now call the client's methods to interact with the DSH resource management API.
//!
//! # Example
//!
//! This example will print a list of all the applications that are deployed
//! in a tenant environment. This example requires that the tenant's name,
//! platform and API password are configured via
//! [environment variables](crate::dsh_api_client_factory).
//!
//! ```ignore
//! use dsh_api::dsh_api_client_factory::DshApiClientFactory;
//!
//! # use dsh_api::error::DshApiResult;
//! # async fn hide() -> DshApiResult<()> {
//! let client = DshApiClientFactory::default().client().await?;
//! for (application_id, application) in client.list_applications()? {
//!   println!("{} -> {}", application_id, application);
//! }
//! # Ok(())
//! # }
//! ```

use crate::dsh_api_tenant::DshApiTenant;
use crate::dsh_jwt::DshJwt;
use crate::error::DshApiResult;
use crate::platform::DshPlatform;
use crate::token_fetcher::TokenFetcher;
use crate::{DshApiError, OPENAPI_SPEC};
use log::{debug, trace};
use reqwest::{Client, Error as ReqwestError, Response, StatusCode};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::str::{from_utf8, FromStr};

// #[derive(Debug)]
pub struct DshApiClient {
  static_token: Option<String>,
  token_fetcher: Option<TokenFetcher>,
  pub(crate) client: Client,
  tenant: DshApiTenant,
}

impl DshApiClient {
  /// # Returns the openapi spec used to generate the client code
  ///
  /// Note that this is not the original openapi specification exposed by the
  /// DSH resource management web service.
  /// The version exposed by this function differs from the original specification as follows:
  /// * Depending on whether the `manage` and/or `robot` features are
  ///   enabled or not, not all operations might be present.
  pub fn openapi_spec() -> &'static str {
    OPENAPI_SPEC
  }

  /// Returns whether the client has a static token
  pub fn has_static_token(&self) -> bool {
    self.static_token.is_some()
  }

  /// Returns the static token
  pub fn static_token(&self) -> &Option<String> {
    &self.static_token
  }

  /// Returns the token fetcher
  pub fn token_fetcher(&self) -> &Option<TokenFetcher> {
    &self.token_fetcher
  }

  /// Returns whether the client has a static token
  pub fn has_token_fetcher(&self) -> bool {
    self.token_fetcher.is_some()
  }

  /// Returns the tenant
  pub fn tenant(&self) -> &DshApiTenant {
    &self.tenant
  }

  /// Returns the name of the tenant
  pub fn tenant_name(&self) -> &str {
    self.tenant.name()
  }

  /// Returns the platform
  pub fn platform(&self) -> &DshPlatform {
    self.tenant.platform()
  }

  /// Returns the Authorization header for the rest API
  ///
  /// This method returns a token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service. This header value is of the form
  /// `Bearer ey...`.
  ///
  /// Since this token has a relatively short lifespan, it is advised to request a new token
  /// from this method before each API call. An internal caching mechanism will make sure that
  /// no unnecessary calls will be made.
  pub async fn bearer_token(&self) -> DshApiResult<String> {
    if let Some(static_token) = &self.static_token {
      // The static token does not have the 'Bearer' prefix
      Ok(format!("Bearer {}", static_token))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      // The token_fetcher.get_bearer_token() method already includes the 'Bearer' prefix
      token_fetcher.get_bearer_token().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Returns the Authorization header for the rest API
  ///
  /// This method returns a token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service. This header value is of the form
  /// `Bearer ey...`.
  ///
  /// This method will not use the cached token and will always request a new access token (which
  /// will overwrite the cached token if available). In normal cases it is preferred to use the
  /// [`bearer_token`] method instead of this one.
  /// For static tokens this method has the same effect as the `bearer_token` method.
  pub async fn fresh_bearer_token(&self) -> DshApiResult<String> {
    if let Some(static_token) = &self.static_token {
      // The static token does not have the 'Bearer' prefix
      Ok(format!("Bearer {}", static_token))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      // The token_fetcher.get_bearer_token() method already includes the 'Bearer' prefix
      token_fetcher.get_fresh_bearer_token().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Returns the raw access token
  ///
  /// This method returns a raw access token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service.
  ///
  /// Since this token has a relatively short lifespan,
  /// it is advised to request a new token from this method before each API call.
  /// An internal caching mechanism will make sure that no unnecessary calls will be made.
  pub async fn raw_token(&self) -> DshApiResult<String> {
    if let Some(static_token) = &self.static_token {
      Ok(static_token.clone())
    } else if let Some(token_fetcher) = &self.token_fetcher {
      token_fetcher.get_raw_token().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Returns the raw access token
  ///
  /// This method returns a raw access token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service.
  ///
  /// This method will not use the cached token and will always request a new access token (which
  /// will overwrite the cached token if available). In normal cases it is preferred to use the
  /// [`raw_token`] method instead of this one.
  /// For static tokens this method has the same effect as the `raw_token` method.
  pub async fn fresh_raw_token(&self) -> DshApiResult<String> {
    if let Some(static_token) = &self.static_token {
      Ok(static_token.clone())
    } else if let Some(token_fetcher) = &self.token_fetcher {
      token_fetcher.get_fresh_raw_token().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Returns a json web token
  ///
  /// This method returns a struct that contains information from the access token.
  ///
  /// Since this token has a relatively short lifespan,
  /// it is advised to request a new token from this method before each API call.
  /// An internal caching mechanism will make sure that no unnecessary calls will be made.
  pub async fn jwt(&self) -> DshApiResult<DshJwt> {
    if let Some(static_token) = &self.static_token {
      DshJwt::from_str(static_token).map_err(|_| DshApiError::Unexpected("could not parse static jwt token".to_string(), None))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      token_fetcher.get_jwt().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Returns a json web token
  ///
  /// This method returns a struct that contains information from the access token.
  ///
  /// This method will not use the cached token and will always request a new access token (which
  /// will overwrite the cached token if available). In normal cases it is preferred to use the
  /// [`jwt`] method instead of this one.
  /// For static tokens this method has the same effect as the `jwt` method.
  pub async fn fresh_jwt(&self) -> DshApiResult<DshJwt> {
    if let Some(static_token) = &self.static_token {
      DshJwt::from_str(static_token).map_err(|_| DshApiError::Unexpected("could not parse static jwt token".to_string(), None))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      token_fetcher.get_fresh_jwt().await
    } else {
      Err(DshApiError::Configuration("either a static token or a token fetcher must be provided".to_string()))
    }
  }

  /// Create a `DshApiClient` from a static token
  ///
  /// # Parameters
  /// * `static_token` - Static token.
  /// * `client` - Optional Reqwest client. When empty, a default client will be created.
  /// * `tenant` - Containing tenant name and platform.
  ///
  /// # Returns
  /// * [DshApiClient] - The created dsh api client.
  pub(crate) fn with_static_token(static_token: String, client: Option<Client>, tenant: DshApiTenant) -> Self {
    match client {
      Some(client) => {
        debug!("create dsh api client from static token");
        Self { static_token: Some(static_token), token_fetcher: None, client, tenant }
      }
      None => {
        debug!("create dsh api client from static token with default https client");
        Self { static_token: Some(static_token), token_fetcher: None, client: Client::new(), tenant }
      }
    }
  }

  /// Create a `DshApiClient` from a token fetcher
  ///
  /// # Parameters
  /// * `token_fetcher` - Token fetcher that creates a token when required.
  /// * `client` - Optional Reqwest client. When empty, a default client will be created.
  /// * `tenant` - Containing tenant name and platform.
  ///
  /// # Returns
  /// * [DshApiClient] - The created dsh api client.
  pub(crate) fn with_token_fetcher(token_fetcher: TokenFetcher, client: Option<Client>, tenant: DshApiTenant) -> Self {
    match client {
      Some(client) => {
        debug!("create dsh api client from token fetcher");
        Self { static_token: None, token_fetcher: Some(token_fetcher), client, tenant }
      }
      None => {
        debug!("create dsh api client from token fetcher with default https client");
        Self { static_token: None, token_fetcher: Some(token_fetcher), client: Client::new(), tenant }
      }
    }
  }

  #[allow(dead_code)]
  pub(crate) async fn process_delete(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    self.process_no_content(reqwest_response).await
  }

  #[allow(dead_code)]
  pub(crate) async fn process_get_deserializable<T>(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<T>
  where
    T: Debug + DeserializeOwned,
  {
    match reqwest_response {
      Ok(response) => match response.status() {
        StatusCode::OK => response.json::<T>().await.map_err(DshApiError::from),
        status_code => Self::process_errors(status_code, response).await,
      },
      Err(reqwest_error) => Err(DshApiError::from(reqwest_error)),
    }
  }

  pub(crate) async fn process_get_string(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<String> {
    match reqwest_response {
      Ok(response) => match response.status() {
        StatusCode::OK => Ok(from_utf8(response.bytes().await?.as_ref())?.to_string()),
        status_code => Self::process_errors(status_code, response).await,
      },
      Err(response_error) => Err(DshApiError::from(response_error)),
    }
  }

  #[allow(dead_code)]
  pub(crate) async fn process_head(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    self.process_no_content(reqwest_response).await
  }

  #[allow(dead_code)]
  pub(crate) async fn process_patch(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    self.process_no_content(reqwest_response).await
  }

  #[allow(dead_code)]
  pub(crate) async fn process_post(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    self.process_no_content(reqwest_response).await
  }

  #[allow(dead_code)]
  pub(crate) async fn process_post_deserializable<T>(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<T>
  where
    T: DeserializeOwned,
  {
    match reqwest_response {
      Ok(response) => match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED => response.json::<T>().await.map_err(DshApiError::from),
        status_code => Self::process_errors(status_code, response).await,
      },
      Err(reqwest_error) => Err(DshApiError::from(reqwest_error)),
    }
  }

  #[allow(dead_code)]
  pub(crate) async fn process_put(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    self.process_no_content(reqwest_response).await
  }

  async fn process_no_content(&self, reqwest_response: Result<Response, ReqwestError>) -> DshApiResult<()> {
    match reqwest_response {
      Ok(response) => match response.status() {
        StatusCode::OK | StatusCode::CREATED | StatusCode::ACCEPTED | StatusCode::NO_CONTENT => Self::trace_response_string(response.status(), response).await,
        status_code => Self::process_errors(status_code, response).await,
      },
      Err(reqwest_error) => Err(DshApiError::from(reqwest_error)),
    }
  }

  async fn trace_response_string(status_code: StatusCode, response: Response) -> DshApiResult<()> {
    if let Some(response_string) = Self::get_response_string(response).await {
      trace!("{} -> response string: {}", status_code, response_string);
    }
    Ok(())
  }

  async fn process_errors<T>(status_code: StatusCode, response: Response) -> DshApiResult<T> {
    match status_code {
      StatusCode::BAD_REQUEST => Err(DshApiError::BadRequest(Self::get_response_string(response).await)),
      StatusCode::UNAUTHORIZED => Err(DshApiError::NotAuthorized(Self::get_response_string(response).await)),
      StatusCode::NOT_FOUND => Err(DshApiError::NotFound(Self::get_response_string(response).await)),
      unexpected_status_code => Err(DshApiError::Unexpected(
        format!("unexpected status code: {}", unexpected_status_code),
        Self::get_response_string(response).await,
      )),
    }
  }

  async fn get_response_string(response: Response) -> Option<String> {
    response
      .bytes()
      .await
      .ok()
      .and_then(|response_bytes| from_utf8(response_bytes.as_ref()).map(|response_str| response_str.to_string()).ok())
      .and_then(|response_string| if response_string.is_empty() { None } else { Some(response_string) })
  }
}

#[test]
fn test_dsh_api_client_is_send() {
  fn assert_send<T: Send>() {}
  assert_send::<DshApiClient>();
}

#[test]
fn test_dsh_api_client_is_sync() {
  fn assert_sync<T: Sync>() {}
  assert_sync::<DshApiClient>();
}
