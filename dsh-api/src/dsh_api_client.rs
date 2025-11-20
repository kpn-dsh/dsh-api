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
//!     [`DshApiClientFactory::create()`](crate::dsh_api_client_factory::DshApiClientFactory::create)
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
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! let client = DshApiClientFactory::default().client().await?;
//! for (application_id, application) in client.list_applications()? {
//!   println!("{} -> {}", application_id, application);
//! }
//! # Ok(())
//! # }
//! ```

use crate::dsh_api_tenant::DshApiTenant;
use crate::dsh_jwt::DshJwt;
use crate::generated::Client as GeneratedClient;
use crate::platform::DshPlatform;
use crate::token_fetcher::ManagementApiTokenFetcher;
use crate::{DshApiError, OPENAPI_SPEC};
use bytes::Bytes;
use futures::TryStreamExt;
use log::{debug, log_enabled, trace, Level};
use progenitor_client::{ByteStream, Error as ProgenitorError, ResponseValue as ProgenitorResponseValue};
use reqwest::StatusCode as ReqwestStatusCode;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct DshApiClient {
  static_token: Option<String>,
  token_fetcher: Option<ManagementApiTokenFetcher>,
  pub(crate) generated_client: GeneratedClient,
  tenant: DshApiTenant,
}

pub(crate) enum DshApiResponseStatus {
  Accepted,
  Created,
  NoContent,
  Ok,
  Unknown,
}

pub(crate) type DshApiProcessResult<T> = Result<(DshApiResponseStatus, T), DshApiError>;

impl DshApiClient {
  /// Create a `DshApiClient` from a static token
  ///
  /// # Parameters
  /// `static_token` - Static token.
  /// `generated_client` - Api functions generated from the openapi specification.
  /// `tenant` - Dsh api client, containing tenant name and platform.
  ///
  /// # Returns
  /// * [DshApiClient] - The created dsh api client.
  pub(crate) fn from_static_token(static_token: String, generated_client: GeneratedClient, tenant: DshApiTenant) -> Self {
    Self { static_token: Some(static_token), token_fetcher: None, generated_client, tenant }
  }

  /// Create a `DshApiClient` from a token fetcher
  ///
  /// # Parameters
  /// `token_fetcher` - Token fetcher that creates a token when required.
  /// `generated_client` - Api functions generated from the openapi specification.
  /// `tenant` - Dsh api client, containing tenant name and platform.
  ///
  /// # Returns
  /// * [DshApiClient] - The created dsh api client.
  pub(crate) fn from_token_fetcher(token_fetcher: ManagementApiTokenFetcher, generated_client: GeneratedClient, tenant: DshApiTenant) -> Self {
    Self { static_token: None, token_fetcher: Some(token_fetcher), generated_client, tenant }
  }

  /// # Returns the openapi spec used to generate the client code
  ///
  /// Note that this is not the original openapi specification exposed by the
  /// DSH resource management web service.
  /// The version exposed by this function differs from the original specification as follows:
  /// * Added authorization header specification to each operation.
  /// * Added operationId parameter to each operation.
  /// * Depending on whether the `manage` and/or `robot` features are
  ///   enabled or not, not all operations might be present.
  pub fn openapi_spec() -> &'static str {
    OPENAPI_SPEC
  }

  pub(crate) async fn process<T>(&self, progenitor_response: Result<ProgenitorResponseValue<T>, ProgenitorError>) -> DshApiProcessResult<T>
  where
    T: Debug,
  {
    match progenitor_response {
      Ok(response_value) => {
        let response_status = response_value.status();
        let status = DshApiResponseStatus::from(response_status);
        let response = response_value.into_inner();
        if log_enabled!(Level::Trace) {
          trace!("response / {} / {}\n{:#?}", response_status, status, response);
        } else {
          debug!("response / {} / {}", response_status, status);
        }
        Ok((status, response))
      }
      Err(progenitor_error) => {
        debug!("progenitor error / {}", progenitor_error);
        Err(DshApiError::async_from_progenitor_error(progenitor_error).await)
      }
    }
  }

  pub(crate) async fn process_string(&self, progenitor_response: Result<ProgenitorResponseValue<ByteStream>, ProgenitorError>) -> DshApiProcessResult<String> {
    match progenitor_response {
      Ok(response_value) => {
        let response_status = response_value.status();
        let status = DshApiResponseStatus::from(response_status);
        let mut inner = response_value.into_inner();
        let mut string = String::new();
        while let Some::<Bytes>(ref bytes) = inner.try_next().await? {
          string.push_str(std::str::from_utf8(bytes)?)
        }
        if log_enabled!(Level::Trace) {
          trace!("response / {} / {}\n{}", response_status, status, string);
        } else {
          debug!("response / {} / {}", response_status, status);
        }
        Ok((status, string))
      }
      Err(progenitor_error) => {
        debug!("progenitor error / {}", progenitor_error);
        Err(DshApiError::async_from_progenitor_error(progenitor_error).await)
      }
    }
  }

  /// Returns the static token
  pub fn static_token(&self) -> &Option<String> {
    &self.static_token
  }

  /// Returns the token fetcher
  pub fn token_fetcher(&self) -> &Option<ManagementApiTokenFetcher> {
    &self.token_fetcher
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
  /// to the DSH resource management web service. These header value are of the form
  /// `Bearer ey...`.
  ///
  /// Since this token has a relatively short lifespan,
  /// it is advised to request a new token from this method before each API call.
  /// An internal caching mechanism will make sure that no unnecessary calls will be made.
  pub async fn token(&self) -> Result<String, DshApiError> {
    if let Some(static_token) = &self.static_token {
      // The static token does not have the 'Bearer' prefix
      Ok(format!("Bearer {}", static_token))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      // The token_fetcher.get_token() method already includes the 'Bearer' prefix
      token_fetcher.get_token().await.map_err(DshApiError::from)
    } else {
      Err(DshApiError::Unexpected("".to_string(), None))
    }
  }

  /// Returns a token for the rest API
  ///
  /// This method returns a token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service.
  /// Since this token has a relatively short lifespan,
  /// it is advised to request a new token from this method before each API call.
  /// An internal caching mechanism will make sure that no unnecessary calls will be made.
  pub async fn fresh_jwt(&self) -> Result<DshJwt, DshApiError> {
    if let Some(static_token) = &self.static_token {
      DshJwt::from_token(static_token.clone()).map_err(|_| DshApiError::Unexpected("could not parse jwt".to_string(), None))
    } else if let Some(token_fetcher) = &self.token_fetcher {
      token_fetcher
        .fresh_token()
        .await
        .map_err(DshApiError::from)
        .and_then(|token| DshJwt::from_token(token).map_err(|_| DshApiError::Unexpected("could not parse jwt".to_string(), None)))
    } else {
      Err(DshApiError::Unexpected("".to_string(), None))
    }
  }
}

impl Display for DshApiResponseStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiResponseStatus::Accepted => write!(f, "accepted"),
      DshApiResponseStatus::Created => write!(f, "created"),
      DshApiResponseStatus::NoContent => write!(f, "no content"),
      DshApiResponseStatus::Ok => write!(f, "ok"),
      DshApiResponseStatus::Unknown => write!(f, "unknown"),
    }
  }
}

impl From<ReqwestStatusCode> for DshApiResponseStatus {
  fn from(status: ReqwestStatusCode) -> Self {
    match status {
      ReqwestStatusCode::ACCEPTED => Self::Accepted,
      ReqwestStatusCode::CREATED => Self::Created,
      ReqwestStatusCode::NO_CONTENT => Self::NoContent,
      ReqwestStatusCode::OK => Self::Ok,
      _ => Self::Unknown,
    }
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
