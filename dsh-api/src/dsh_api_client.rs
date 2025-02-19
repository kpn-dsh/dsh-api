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
use crate::generated::Client as GeneratedClient;
use crate::platform::DshPlatform;
use crate::{DshApiError, OPENAPI_SPEC};
use bytes::Bytes;
use dsh_sdk::ManagementApiTokenFetcher;
use futures::TryStreamExt;
use log::{debug, trace};
use progenitor_client::{ByteStream, Error as ProgenitorError, ResponseValue as ProgenitorResponseValue};
use reqwest::StatusCode as ReqwestStatusCode;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct DshApiClient {
  token_fetcher: ManagementApiTokenFetcher,
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
  pub(crate) fn new(token_fetcher: ManagementApiTokenFetcher, generated_client: GeneratedClient, tenant: DshApiTenant) -> Self {
    Self { token_fetcher, generated_client, tenant }
  }

  /// # Returns the version of the openapi spec
  #[deprecated(since = "0.4.0", note = "please use `dsh_api::api_version()` method instead")]
  pub fn api_version(&self) -> &'static str {
    self.generated_client.api_version()
  }

  /// # Returns the openapi spec used to generate the client code
  ///
  /// Note that this is not the original openapi specification exposed by the
  /// DSH resource management web service.
  /// The version exposed by this function differs from the original specification as follows:
  /// * Added authorization header specification to each operation.
  /// * Added operationId parameter to each operation.
  /// * Depending on whether the `appcatalog`, `manage` and/or `robot` features are
  ///   enabled or not, not all operations might be present.
  pub fn openapi_spec() -> &'static str {
    OPENAPI_SPEC
  }

  pub(crate) async fn process<T>(&self, progenitor_response: Result<ProgenitorResponseValue<T>, ProgenitorError>) -> DshApiProcessResult<T>
  where
    T: Debug,
  {
    match progenitor_response {
      Ok::<ProgenitorResponseValue<T>, ProgenitorError>(response) => {
        let status = DshApiResponseStatus::from(response.status());
        let response = response.into_inner();
        debug!("response / {}", status);
        trace!("{:#?}", response);
        Ok((status, response))
      }
      Err(progenitor_error) => Err(DshApiError::async_from_progenitor_error(progenitor_error).await),
    }
  }

  pub(crate) async fn process_string(&self, progenitor_response: Result<ProgenitorResponseValue<ByteStream>, ProgenitorError>) -> DshApiProcessResult<String> {
    match progenitor_response {
      Ok(response) => {
        let status = DshApiResponseStatus::from(response.status());
        let mut inner = response.into_inner();
        let mut string = String::new();
        debug!("string response / {} / {}", status, string);
        while let Some::<Bytes>(ref bytes) = inner.try_next().await? {
          string.push_str(std::str::from_utf8(bytes)?)
        }
        Ok((status, string))
      }
      Err(progenitor_error) => Err(DshApiError::async_from_progenitor_error(progenitor_error).await),
    }
  }

  /// Returns the token fetcher
  pub fn token_fetcher(&self) -> &ManagementApiTokenFetcher {
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

  /// Returns a token for the rest API
  ///
  /// This method returns a token that can be used to authenticate and authorize a call
  /// to the DSH resource management web service.
  /// Since this token has a relatively short lifespan,
  /// it is advised to request a new token from this method before each API call.
  /// An internal caching mechanism will make sure that no unnecessary calls will be made.
  pub async fn token(&self) -> Result<String, DshApiError> {
    match self.token_fetcher.get_token().await {
      Ok(token) => {
        debug!("token fetched");
        Ok(token)
      }
      Err(error) => Err(DshApiError::from(error)),
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
