use reqwest::header::InvalidHeaderValue;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::PoisonError;

/// Generic result type.
pub type DshApiResult<T> = Result<T, DshApiError>;

/// Describes an API error.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum DshApiError {
  /// DSH Api server indicated that the request was not correct (BAD_REQUEST).
  BadRequest { message: Option<String> },
  /// Misconfiguration error, e.g. a missing environment variable.
  Configuration { message: String },
  /// Conversion error, e.g. an unknown attribute.
  Conversion { message: String },
  /// Not authorized for the requested operation or resource.
  NotAuthorized { message: Option<String> },
  /// Requested resource does not exist.
  NotFound { message: Option<String> },
  /// Wrong parameters provided.
  Parameter { message: String },
  /// Unexpected error occurred.
  Unexpected { message: String, cause: Option<String> },
  /// DSH Api server indicated that the request could not be processed (UNPROCESSABLE_ENTITY).
  Unprocessable { message: Option<String> },
}

impl DshApiError {
  pub(crate) fn configuration<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Configuration { message: message.into() }
  }

  pub(crate) fn conversion<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Conversion { message: message.into() }
  }

  pub(crate) fn not_found() -> Self {
    Self::NotFound { message: None }
  }

  // Allow dead_code since this method is used in the generated code
  #[allow(dead_code)]
  pub(crate) fn parameter<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Conversion { message: message.into() }
  }

  pub(crate) fn unexpected<T>(message: T) -> Self
  where
    T: Into<String>,
  {
    Self::Unexpected { message: message.into(), cause: None }
  }
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
///   write(&path, data).map_err(log_error!("writing {} failed, caused by ", path.display()))?;
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
      DshApiError::BadRequest { message } => match message {
        Some(message) => write!(f, "bad request: {}", message),
        None => write!(f, "bad request"),
      },
      DshApiError::Configuration { message } => write!(f, "configuration error: {}", message),
      DshApiError::Conversion { message } => write!(f, "conversion error: {}", message),
      DshApiError::NotAuthorized { message } => match message {
        Some(message) => write!(f, "not authorized: {}", message),
        None => write!(f, "not authorized"),
      },
      DshApiError::NotFound { message } => match message {
        Some(message) => write!(f, "not found: {}", message),
        None => write!(f, "not found"),
      },
      DshApiError::Parameter { message } => write!(f, "parameter error: {}", message),
      DshApiError::Unexpected { message, cause } => match cause {
        Some(cause) => write!(f, "unexpected error: {} ({})", message, cause),
        None => write!(f, "unexpected error: {}", message),
      },
      DshApiError::Unprocessable { message } => match message {
        Some(message) => write!(f, "unprocessable entity: {}", message),
        None => write!(f, "unprocessable entity"),
      },
    }
  }
}

impl From<crate::types::error::ConversionError> for DshApiError {
  fn from(conversion_error: crate::types::error::ConversionError) -> Self {
    DshApiError::unexpected(conversion_error.to_string())
  }
}

impl From<serde_json::Error> for DshApiError {
  fn from(json_error: serde_json::Error) -> Self {
    DshApiError::conversion(json_error.to_string())
  }
}

impl From<reqwest::Error> for DshApiError {
  fn from(reqwest_error: reqwest::Error) -> Self {
    DshApiError::unexpected(reqwest_error.to_string())
  }
}

impl From<InvalidHeaderValue> for DshApiError {
  fn from(invalid_header_error: InvalidHeaderValue) -> Self {
    DshApiError::unexpected(invalid_header_error.to_string())
  }
}

impl<T> From<PoisonError<T>> for DshApiError {
  fn from(poison_value: PoisonError<T>) -> Self {
    DshApiError::Unexpected { message: "mutex error".to_string(), cause: Some(poison_value.to_string()) }
  }
}

impl From<std::str::Utf8Error> for DshApiError {
  fn from(utf8_error: std::str::Utf8Error) -> Self {
    DshApiError::unexpected(utf8_error.to_string())
  }
}

impl From<String> for DshApiError {
  fn from(message: String) -> Self {
    DshApiError::unexpected(message)
  }
}

impl From<&str> for DshApiError {
  fn from(message: &str) -> Self {
    DshApiError::unexpected(message)
  }
}

impl From<std::time::SystemTimeError> for DshApiError {
  fn from(system_time_error: std::time::SystemTimeError) -> Self {
    DshApiError::Unexpected { message: "system time error".to_string(), cause: Some(system_time_error.to_string()) }
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
