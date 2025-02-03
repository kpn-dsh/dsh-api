//! Client for accessing the DSH api

use crate::dsh_api_tenant::DshApiTenant;
use crate::generated::Client as GeneratedClient;
use crate::platform::DshPlatform;
use crate::types::error::ConversionError;
use crate::{DshApiError, OPENAPI_SPEC};
use bytes::Bytes;
use dsh_sdk::RestTokenFetcher;
use futures::TryStreamExt;
use progenitor_client::{ByteStream, Error as ProgenitorError, ResponseValue as ProgenitorResponseValue};
use reqwest::StatusCode as ReqwestStatusCode;
use serde::Serialize;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct DshApiClient<'a> {
  token_fetcher: RestTokenFetcher,
  pub(crate) generated_client: &'a GeneratedClient,
  tenant: &'a DshApiTenant,
}

pub(crate) enum DshApiResponseStatus {
  Accepted,
  Created,
  NoContent,
  Ok,
  Unknown,
}

pub(crate) type DshApiProcessResult<T> = Result<(DshApiResponseStatus, T), DshApiError>;

impl<'a> DshApiClient<'a> {
  pub fn new(token_fetcher: RestTokenFetcher, generated_client: &'a GeneratedClient, tenant: &'a DshApiTenant) -> Self {
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
  /// DSH resource management API. The version exposed by this function has two additions:
  /// * Added authorization header specification to each operation.
  /// * Added operationId parameter to each operation.
  pub fn openapi_spec() -> &'static str {
    OPENAPI_SPEC
  }

  pub(crate) async fn process<T>(&self, progenitor_response: Result<ProgenitorResponseValue<T>, ProgenitorError>) -> DshApiProcessResult<T>
  where
    T: Debug + Serialize,
  {
    match progenitor_response {
      Ok::<ProgenitorResponseValue<T>, ProgenitorError>(response) => {
        let status = DshApiResponseStatus::from(response.status());
        let response = response.into_inner();
        log::debug!("response / {} / {:?}", status, response);
        Ok((status, response))
      }
      Err(progenitor_error) => {
        log::debug!("progenitor error: {}", progenitor_error);
        Err(from_progenitor_error(progenitor_error).await)
      }
    }
  }

  pub(crate) async fn process_raw<T>(&self, progenitor_response: Result<ProgenitorResponseValue<T>, ProgenitorError>) -> DshApiProcessResult<T>
  where
    T: Debug,
  {
    match progenitor_response {
      Ok::<ProgenitorResponseValue<T>, ProgenitorError>(response) => {
        let status = DshApiResponseStatus::from(response.status());
        let response = response.into_inner();
        log::debug!("raw response / {} / {:?}", status, response);
        Ok((status, response))
      }
      Err(progenitor_error) => {
        log::debug!("progenitor error: {}", progenitor_error);
        Err(from_progenitor_error(progenitor_error).await)
      }
    }
  }

  pub(crate) async fn process_string(&self, progenitor_response: Result<ProgenitorResponseValue<ByteStream>, ProgenitorError>) -> DshApiProcessResult<String> {
    match progenitor_response {
      Ok(response) => {
        let status = DshApiResponseStatus::from(response.status());
        let mut inner = response.into_inner();
        let mut string = String::new();
        log::debug!("string response / {} / {}", status, string);
        while let Some::<Bytes>(ref bytes) = inner.try_next().await? {
          string.push_str(std::str::from_utf8(bytes)?)
        }
        Ok((status, string))
      }
      Err(progenitor_error) => {
        log::debug!("progenitor error: {}", progenitor_error);
        Err(from_progenitor_error(progenitor_error).await)
      }
    }
  }

  pub fn tenant(&self) -> &DshApiTenant {
    self.tenant
  }

  pub fn tenant_name(&self) -> &str {
    self.tenant.name()
  }

  pub fn platform(&self) -> &DshPlatform {
    self.tenant.platform()
  }

  pub fn guid(&self) -> u16 {
    self.tenant.guid()
  }

  pub async fn token(&self) -> Result<String, DshApiError> {
    self.token_fetcher.get_token().await.map_err(DshApiError::from)
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

impl From<ConversionError> for DshApiError {
  fn from(value: ConversionError) -> Self {
    DshApiError::Unexpected(value.to_string(), None)
  }
}

// async version of impl From<ProgenitorError> for DshApiError
async fn from_progenitor_error(progenitor_error: ProgenitorError) -> DshApiError {
  match progenitor_error {
    ProgenitorError::InvalidRequest(ref string) => DshApiError::Unexpected(format!("invalid request ({})", string), Some(progenitor_error.to_string())),
    ProgenitorError::CommunicationError(ref reqwest_error) => DshApiError::Unexpected(
      format!("communication error (reqwest error: {})", reqwest_error),
      Some(progenitor_error.to_string()),
    ),
    ProgenitorError::InvalidUpgrade(ref reqwest_error) => {
      DshApiError::Unexpected(format!("invalid upgrade (reqwest error: {})", reqwest_error), Some(progenitor_error.to_string()))
    }
    ProgenitorError::ErrorResponse(ref progenitor_response_value) => DshApiError::Unexpected(
      format!("error response (progenitor response value: {:?})", progenitor_response_value),
      Some(progenitor_error.to_string()),
    ),
    ProgenitorError::ResponseBodyError(ref reqwest_error) => DshApiError::Unexpected(
      format!("response body error (reqwest error: {})", reqwest_error),
      Some(progenitor_error.to_string()),
    ),
    ProgenitorError::InvalidResponsePayload(ref _bytes, ref json_error) => {
      DshApiError::Unexpected(format!("invalid response payload (json error: {})", json_error), Some(progenitor_error.to_string()))
    }
    ProgenitorError::UnexpectedResponse(reqwest_response) => match &reqwest_response.status().clone() {
      &ReqwestStatusCode::BAD_REQUEST => match reqwest_response.text().await {
        Ok(error_text) => DshApiError::BadRequest(error_text),
        Err(response_error) => DshApiError::BadRequest(response_error.to_string()),
      },
      &ReqwestStatusCode::NOT_FOUND => DshApiError::NotFound,
      &ReqwestStatusCode::UNAUTHORIZED | &ReqwestStatusCode::FORBIDDEN | &ReqwestStatusCode::METHOD_NOT_ALLOWED => DshApiError::NotAuthorized,
      other_status_code => DshApiError::Unexpected(format!("unexpected response (status: {}, reqwest response: )", other_status_code), None),
    },
    ProgenitorError::PreHookError(string) => DshApiError::Unexpected(format!("pre-hook error ({})", string), None),
  }
}

// 200 - OK
// 201 - CREATED
// 202 - ACCEPTED
// 204 - NO_CONTENT
// 400 - BAD_REQUEST
// 401 - UNAUTHORIZED
// 403 - FORBIDDEN
// 404 - NOT_FOUND
// 405 - NOT_ALLOWED
// 500 - INTERNAL_SERVER_ERROR

// DELETE  200,204  resource successfully deleted
//         202      request accepted, result unknown
// GET     200      resource successfully retrieved
// POST    200      resource created successfully
//         201      created new resource
//         202      request accepted, result unknown
// PUT     200,204  resource updated successfully
//         201      created new resource
//         202      request accepted, result unknown
