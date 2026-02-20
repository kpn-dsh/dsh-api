use reqwest::header::InvalidHeaderValue;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::PoisonError;

/// Generic result type
pub type DshApiResult<T> = Result<T, DshApiError>;

/// Describes an API error
#[derive(Clone, Debug, Serialize)]
pub enum DshApiError {
  /// DSH Api server indicated that the request was not correct (BAD_REQUEST).
  BadRequest(Option<String>),
  /// Misconfiguration error, e.g. a missing environment variable.
  Configuration(String),
  /// Conversion error, e.g. an unknown attribute.
  Conversion(String),
  /// Not authorized for the requested operation or resource.
  NotAuthorized(Option<String>),
  /// Requested resource does not exist.
  NotFound(Option<String>),
  /// Wrong parameters provided.
  Parameter(String),
  /// Unexpected error occurred.
  Unexpected(String, Option<String>),
  /// DSH Api server indicated that the request could not be processed (UNPROCESSABLE_ENTITY).
  Unprocessable(Option<String>),
}

/// Creates a closure that error logs a formatted string and returns the closure's argument.
///
/// The argument for the `error_map!` macro must be a format string and some parameters
/// (according to the [format_args] macro). The macro will then create a closure that just
/// returns its argument, but as a side effect will error log the string which was generated
/// from the macro arguments, appended with `error.to_string()`.
///
/// The intended use for `log_error!` is as a closure argument for the [Result::map_err] method,
/// error logging a message as a side effect.
///
/// # Examples
/// ```
/// # use std::path::PathBuf;
/// # use std::fs::write;
/// use dsh_api::log_error;
///
/// fn save(path: PathBuf, data: &[u8]) -> Result<(), std::io::Error> {
///   write(&path, data)
///     .map_err(log_error!("writing {} failed, caused by ", path.display()))?;
///   Ok(())
/// }
/// ```
#[macro_export]
macro_rules! log_error {
  () => {{
    |error| {
      log::error!("{}", error);
      log::debug!("{:?}", error);
      error
    }
  }};
  ($($t:tt)*) => {{
    |error| {
      log::error!("{}{}", format!($($t)*), error);
      log::debug!("{:?}", error);
      error
    }
  }};
}

impl Error for DshApiError {}

impl Display for DshApiError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      DshApiError::BadRequest(message) => match message {
        Some(message) => write!(f, "bad request: {}", message),
        None => write!(f, "bad request"),
      },
      DshApiError::Configuration(message) => write!(f, "configuration error: {}", message),
      DshApiError::Conversion(message) => write!(f, "conversion error: {}", message),
      DshApiError::NotAuthorized(cause) => match cause {
        Some(cause_message) => write!(f, "not authorized: {}", cause_message),
        None => write!(f, "not authorized"),
      },
      DshApiError::NotFound(cause) => match cause {
        Some(cause_message) => write!(f, "not found: {}", cause_message),
        None => write!(f, "not found"),
      },
      DshApiError::Parameter(message) => write!(f, "parameter error: {}", message),
      DshApiError::Unexpected(message, cause) => match cause {
        Some(cause) => write!(f, "unexpected error: {} ({})", message, cause),
        None => write!(f, "unexpected error: {}", message),
      },
      DshApiError::Unprocessable(message) => match message {
        Some(message) => write!(f, "unprocessable entity: {}", message),
        None => write!(f, "unprocessable entity"),
      },
    }
  }
}

impl From<crate::types::error::ConversionError> for DshApiError {
  fn from(value: crate::types::error::ConversionError) -> Self {
    DshApiError::Unexpected(value.to_string(), None)
  }
}

impl From<serde_json::Error> for DshApiError {
  fn from(error: serde_json::Error) -> Self {
    DshApiError::Conversion(error.to_string())
  }
}

impl From<reqwest::Error> for DshApiError {
  fn from(error: reqwest::Error) -> Self {
    DshApiError::Unexpected(error.to_string(), None)
  }
}

impl From<InvalidHeaderValue> for DshApiError {
  fn from(error: InvalidHeaderValue) -> Self {
    DshApiError::Unexpected(error.to_string(), None)
  }
}

impl<T> From<PoisonError<T>> for DshApiError {
  fn from(value: PoisonError<T>) -> Self {
    DshApiError::Unexpected("mutex error".to_string(), Some(value.to_string()))
  }
}

impl From<std::str::Utf8Error> for DshApiError {
  fn from(error: std::str::Utf8Error) -> Self {
    DshApiError::Unexpected(error.to_string(), None)
  }
}

impl From<String> for DshApiError {
  fn from(value: String) -> Self {
    DshApiError::Unexpected(value, None)
  }
}

impl From<&str> for DshApiError {
  fn from(value: &str) -> Self {
    DshApiError::Unexpected(value.to_string(), None)
  }
}

impl From<std::time::SystemTimeError> for DshApiError {
  fn from(value: std::time::SystemTimeError) -> Self {
    DshApiError::Unexpected("system time error".to_string(), Some(value.to_string()))
  }
}

impl From<DshApiError> for String {
  fn from(value: DshApiError) -> Self {
    value.to_string()
  }
}

#[test]
fn test_dsh_api_error_is_send() {
  fn assert_send<T: Send>() {}
  assert_send::<DshApiError>();
}

#[test]
fn test_dsh_api_error_is_sync() {
  fn assert_sync<T: Sync>() {}
  assert_sync::<DshApiError>();
}
