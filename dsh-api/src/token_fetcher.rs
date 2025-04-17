//! # Management API token fetching for DSH
//!
//! _This token fetcher is an embedded (and slightly altered) copy of the token fetcher
//! from the [`dsh_sdk`](https://crates.io/crates/dsh_sdk) crate.
//! If all you need is the token fetcher and if you do not plan to use any of the other
//! capabilities of the `dsh_api`, you can also use the token fetcher from the `dsh_sdk`._
//!
//! This module provides the capabilities for fetching and
//! caching access tokens required to communicate with DSH’s management (REST) API.
//! Access tokens are automatically refreshed when expired, allowing seamless
//! integrations with the DSH platform.
//!
//! # Overview
//!
//! * [`AccessToken`] - Access token from the authentication server
//! * [`ManagementApiTokenFetcher`] - A token fetcher that caches tokens and
//!   refreshes them upon expiration
//! * [`ManagementApiTokenFetcherBuilder`] - A builder for customizing the fetcher’s
//!   client, credentials, and target platform
//!
//! # Typical Usage
//!
//! ```
//! # use dsh_api::DshApiError;
//! # async fn hide() -> Result<(), DshApiError> {
//! use dsh_api::platform::DshPlatform;
//! use dsh_api::token_fetcher::ManagementApiTokenFetcherBuilder;
//!
//! let platform = DshPlatform::try_from("nplz")?;
//! let token_fetcher = ManagementApiTokenFetcherBuilder::new(platform)
//!   .tenant_name("my-tenant")
//!   .client_secret("my-secret")
//!   .build()?;
//! let token = token_fetcher.get_token().await?;
//! # Ok(())
//! # }
//! ```
//!
//! The token fetcher can be reused in subsequent calls during the lifetime of your application.
//! The token will be refreshed when it is about to expire.
//!
//! For a more advanced explanation and examples, see the documentation of the token fetcher
//! in the [`dsh_sdk`](https://docs.rs/dsh_sdk/latest/dsh_sdk/management_api/index.html).

use crate::platform::DshPlatform;
use log::debug;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// # Representation of an access token
///
/// This struct is a representation of an access token, as used by DSH's authentication services.
/// The fields include information about the token’s validity window,
/// token type, and scope. Typically, you won't instantiate `AccessToken` directly but instead
/// use [`ManagementApiTokenFetcher::get_token`](ManagementApiTokenFetcher::get_token)
/// to automatically obtain or refresh a valid token.
///
/// * All fields are `pub`.
/// * [`AccessToken`] has derived implementations of the [`Clone`], [`Debug`], [`Default`],
///   [`Deserialize`], [`PartialEq`] and [`Serialize`] traits.
/// * [`Display`] is implemented for [`AccessToken`].
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccessToken {
  /// Raw access token string (without the token type).
  pub access_token: String,
  /// Number of seconds until this token expires.
  pub expires_in: u64,
  /// Number of seconds until the refresh token expires.
  pub refresh_expires_in: u32,
  /// Token type (e.g., `"Bearer"`).
  pub token_type: String,
  /// “not before” policy timestamp from the authentication server.
  #[serde(rename(deserialize = "not-before-policy"))]
  pub not_before_policy: u32,
  /// Scope string (e.g., `"email"`).
  pub scope: String,
}

impl AccessToken {
  /// Returns a complete token string, i.e. `"{token_type} {access_token}"`
  pub fn formatted_token(&self) -> String {
    format!("{} {}", self.token_type, self.access_token)
  }
}

/// # Fetcher for access tokens
///
/// A fetcher for obtaining and storing access tokens, enabling authenticated
/// requests to DSH’s management (REST) API.
/// This struct caches the token in memory and refreshes it automatically
/// once expired.
///
/// # Usage
///
/// * `new` - Construct a fetcher with provided credentials
/// * `new_with_client` - Provide a custom [`reqwest::Client`] if needed
/// * `get_token` - Returns the current token if still valid, or fetches a new one
///
/// # Example
///
/// ```no_run
/// use dsh_api::platform::DshPlatform;
/// use dsh_api::token_fetcher::ManagementApiTokenFetcher;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), dsh_api::DshApiError> {
/// let platform = DshPlatform::try_from("nplz")?;
/// let client_id = platform.tenant_client_id("my-tenant");
/// let client_secret = "my-secret".to_string();
/// let token_fetcher = ManagementApiTokenFetcher::new(
///   client_id,
///   client_secret,
///   platform.access_token_endpoint().to_string(),
/// );
/// let token = token_fetcher.get_token().await?;
/// println!("token: {}", token);
/// # Ok(())
/// # }
/// ```
pub struct ManagementApiTokenFetcher {
  access_token: Mutex<AccessToken>,
  fetched_at: Mutex<Instant>,
  client_id: String,
  client_secret: String,
  client: reqwest::Client,
  auth_url: String,
}

impl ManagementApiTokenFetcher {
  /// # Create a new token fetcher
  ///
  /// # Example
  ///
  /// ```no_run
  /// use dsh_api::platform::DshPlatform;
  /// use dsh_api::token_fetcher::ManagementApiTokenFetcher;
  ///
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), dsh_api::DshApiError> {
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let client_id = platform.tenant_client_id("my-tenant");
  /// let client_secret = "my-secret";
  /// let token_fetcher =
  ///   ManagementApiTokenFetcher::new(client_id, client_secret, platform.access_token_endpoint());
  /// let token = token_fetcher.get_token().await?;
  /// println!("token: {}", token);
  /// # Ok(())
  /// # }
  /// ```
  pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>, auth_url: impl Into<String>) -> Self {
    Self::new_with_client(client_id, client_secret, auth_url, reqwest::Client::default())
  }

  /// # Create a [`ManagementApiTokenFetcherBuilder`]
  ///
  /// Returns a [`ManagementApiTokenFetcherBuilder`] for more flexible creation
  /// of a token fetcher (e.g., specifying a custom client).
  pub fn builder(platform: DshPlatform) -> ManagementApiTokenFetcherBuilder {
    ManagementApiTokenFetcherBuilder::new(platform)
  }

  /// # Create a new fetcher
  ///
  /// Creates a new fetcher with a custom [`reqwest::Client`].
  ///
  /// # Example
  ///
  /// ```no_run
  /// use dsh_api::platform::DshPlatform;
  /// use dsh_api::token_fetcher::ManagementApiTokenFetcher;
  ///
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), dsh_api::DshApiError> {
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let client_id = platform.tenant_client_id("my-tenant");
  /// let client_secret = "my-secret";
  /// let custom_client = reqwest::Client::new();
  /// let token_fetcher = ManagementApiTokenFetcher::new_with_client(
  ///   client_id,
  ///   client_secret,
  ///   platform.access_token_endpoint().to_string(),
  ///   custom_client,
  /// );
  /// let token = token_fetcher.get_token().await?;
  /// println!("token: {}", token);
  /// # Ok(())
  /// # }
  /// ```
  pub fn new_with_client(client_id: impl Into<String>, client_secret: impl Into<String>, auth_url: impl Into<String>, client: reqwest::Client) -> Self {
    Self {
      access_token: Mutex::new(AccessToken::default()),
      fetched_at: Mutex::new(Instant::now()),
      client_id: client_id.into(),
      client_secret: client_secret.into(),
      client,
      auth_url: auth_url.into(),
    }
  }

  /// # Get a cached token
  ///
  /// Obtains the token from the cache if still valid, otherwise fetches a new one.
  /// The returned string is formatted as `"{token_type} {access_token}"`.
  ///
  /// # Example
  ///
  /// ```no_run
  /// use dsh_api::token_fetcher::ManagementApiTokenFetcher;
  /// # #[tokio::main]
  /// # async fn main() {
  /// let tf = ManagementApiTokenFetcher::new(
  ///   "client_id".to_string(),
  ///   "client_secret".to_string(),
  ///   "http://example.com/auth".to_string(),
  /// );
  /// match tf.get_token().await {
  ///   Ok(token) => println!("Got token: {}", token),
  ///   Err(e) => eprintln!("Error fetching token: {}", e),
  /// }
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// * [`ManagementApiTokenError::FailureTokenFetch`] -
  ///   If the network request fails or times out when fetching a new token
  /// * [`ManagementApiTokenError::StatusCode`] -
  ///   If the authentication server returns a non-success HTTP status code
  pub async fn get_token(&self) -> Result<String, ManagementApiTokenError> {
    if self.is_valid() {
      Ok(self.access_token.lock().unwrap().formatted_token())
    } else {
      debug!("token is expired, fetching new token");
      let access_token = self.fetch_access_token_from_server().await?;
      let mut token = self.access_token.lock().unwrap();
      let mut fetched_at = self.fetched_at.lock().unwrap();
      *token = access_token;
      *fetched_at = Instant::now();
      Ok(token.formatted_token())
    }
  }

  /// # Determine if the cached token is still valid
  ///
  /// Determines if the internally cached token is still valid.
  /// A token is considered valid if its remaining lifetime
  /// (minus a 5-second safety margin) is greater than zero.
  pub fn is_valid(&self) -> bool {
    let access_token = self.access_token.lock().unwrap_or_else(|mut e| {
      **e.get_mut() = AccessToken::default();
      self.access_token.clear_poison();
      e.into_inner()
    });
    let fetched_at = self.fetched_at.lock().unwrap_or_else(|e| {
      self.fetched_at.clear_poison();
      e.into_inner()
    });
    // Check if 'expires_in' has elapsed (+ 5-second safety margin)
    fetched_at.elapsed().add(Duration::from_secs(5)) < Duration::from_secs(access_token.expires_in)
  }

  /// # Fetch a fresh `AccessToken`
  ///
  /// Fetches a fresh `AccessToken` from the authentication server.
  ///
  /// # Errors
  ///
  /// * [`ManagementApiTokenError::FailureTokenFetch`] -
  ///   If the network request fails or times out
  /// * [`ManagementApiTokenError::StatusCode`] -
  ///   If the server returns a non-success status code
  pub async fn fetch_access_token_from_server(&self) -> Result<AccessToken, ManagementApiTokenError> {
    let auth_url = &self.auth_url;
    let client_id = self.client_id.as_ref();
    let client_secret = self.client_secret.as_ref();
    let response = self
      .client
      .post(auth_url)
      .form(&[("client_id", client_id), ("client_secret", client_secret), ("grant_type", "client_credentials")])
      .send()
      .await
      .map_err(ManagementApiTokenError::FailureTokenFetch)?;
    if !response.status().is_success() {
      Err(ManagementApiTokenError::StatusCode { status_code: response.status(), error_body: response.text().await.unwrap_or_default() })
    } else {
      response.json::<AccessToken>().await.map_err(ManagementApiTokenError::FailureTokenFetch)
    }
  }
}

impl Debug for ManagementApiTokenFetcher {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ManagementApiTokenFetcher")
      .field("access_token", &self.access_token)
      .field("fetched_at", &self.fetched_at)
      .field("client_id", &self.client_id)
      // For security, obfuscate the secret
      .field("client_secret", &"xxxxxx")
      .field("auth_url", &self.auth_url)
      .finish()
  }
}

/// # Builder for [`ManagementApiTokenFetcher`]
///
/// A builder for constructing a [`ManagementApiTokenFetcher`].
///
/// This builder allows customization of the token fetcher by specifying:
/// * `client_id` or `tenant_name` (tenant name is used to generate the client_id)
/// * `client_secret`
/// * custom [`reqwest::Client`] (optional)
/// * `platform`
///
/// # Example
///
/// ```
/// use dsh_api::platform::DshPlatform;
/// use dsh_api::token_fetcher::{ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder};
///
/// # fn main() -> Result<(), dsh_api::DshApiError> {
/// let platform = DshPlatform::try_from("nplz")?;
/// let client_id = "robot:dev-lz-dsh:my-tenant".to_string();
/// let client_secret = "secret".to_string();
/// let token_fetcher = ManagementApiTokenFetcherBuilder::new(platform)
///   .client_id(client_id)
///   .client_secret(client_secret)
///   .build()?;
/// # Ok(())
/// # }
/// ```
pub struct ManagementApiTokenFetcherBuilder {
  client: Option<reqwest::Client>,
  client_id: Option<String>,
  client_secret: Option<String>,
  platform: DshPlatform,
  tenant_name: Option<String>,
}

impl ManagementApiTokenFetcherBuilder {
  /// # Create a new builder
  ///
  /// Creates a new builder configured for the specified [`DshPlatform`].
  ///
  /// # Parameters
  ///
  /// * `platform` - The target platform to determine default endpoints for fetching tokens
  pub fn new(platform: DshPlatform) -> Self {
    Self { client: None, client_id: None, client_secret: None, platform, tenant_name: None }
  }

  /// # Set an explicit client id
  ///
  /// Set an explicit client id for authentication. If you also specify `tenant_name`,
  /// the client id specified explicitly takes precedence.
  pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
    self.client_id = Some(client_id.into());
    self
  }

  /// # Set the client secret
  ///
  /// Set the client secret required for token fetching.
  pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
    self.client_secret = Some(client_secret.into());
    self
  }

  /// # Set the tenant name
  ///
  /// Set the tenant name from which the client id will be derived, using
  /// `DshPlatform::tenant_client_id(tenant_name)`, unless the `client_id`
  /// is already explicitly set.
  pub fn tenant_name(mut self, tenant_name: impl Into<String>) -> Self {
    self.tenant_name = Some(tenant_name.into());
    self
  }

  /// # Supply a custom [`reqwest::Client`]
  ///
  /// Supplies a custom [`reqwest::Client`] if you need specialized settings
  /// (e.g., proxy configuration, timeouts, etc.).
  pub fn client(mut self, client: reqwest::Client) -> Self {
    self.client = Some(client);
    self
  }

  /// # Build the token fetcher
  ///
  /// Builds the [`ManagementApiTokenFetcher`] based on the provided configuration.
  ///
  /// # Example
  /// ```
  /// use dsh_api::platform::DshPlatform;
  /// use dsh_api::token_fetcher::{
  ///   ManagementApiTokenError, ManagementApiTokenFetcher, ManagementApiTokenFetcherBuilder,
  /// };
  ///
  /// # fn main() -> Result<(), dsh_api::DshApiError> {
  /// let platform = DshPlatform::try_from("nplz")?;
  /// let fetcher = ManagementApiTokenFetcherBuilder::new(platform)
  ///   .client_id("robot:dev-lz-dsh:my-tenant".to_string())
  ///   .client_secret("secret".to_string())
  ///   .build()?;
  /// # Ok(())
  /// # }
  /// ```
  ///
  /// # Errors
  ///
  /// * [`ManagementApiTokenError::UnknownClientSecret`] -
  ///   If the client secret is unset
  /// * [`ManagementApiTokenError::UnknownClientId`] -
  ///   If neither `client_id` nor `tenant_name` is provided
  pub fn build(self) -> Result<ManagementApiTokenFetcher, ManagementApiTokenError> {
    let client_secret = self.client_secret.ok_or(ManagementApiTokenError::UnknownClientSecret)?;
    let client_id = self
      .client_id
      .or_else(|| self.tenant_name.as_ref().map(|tenant_name| self.platform.tenant_client_id(tenant_name)))
      .ok_or(ManagementApiTokenError::UnknownClientId)?;
    let client = self.client.unwrap_or_default();
    let token_fetcher = ManagementApiTokenFetcher::new_with_client(client_id, client_secret, self.platform.access_token_endpoint().to_string(), client);
    Ok(token_fetcher)
  }
}

impl Debug for ManagementApiTokenFetcherBuilder {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let client_secret = self.client_secret.as_ref().map(|_| "Some(\"client_secret\")");
    f.debug_struct("ManagementApiTokenFetcherBuilder")
      .field("client_id", &self.client_id)
      .field("client_secret", &client_secret)
      .field("platform", &self.platform)
      .field("tenant_name", &self.tenant_name)
      .finish()
  }
}

#[derive(Debug)]
pub enum ManagementApiTokenError {
  UnknownClientId,
  UnknownClientSecret,
  FailureTokenFetch(reqwest::Error),
  StatusCode { status_code: reqwest::StatusCode, error_body: String },
}

impl Display for ManagementApiTokenError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ManagementApiTokenError::UnknownClientId => write!(f, "client id is unknown"),
      ManagementApiTokenError::UnknownClientSecret => write!(f, "client secret not set"),
      ManagementApiTokenError::FailureTokenFetch(reqwest_error) => write!(f, "unexpected failure while fetching token from server: {}", reqwest_error),
      ManagementApiTokenError::StatusCode { status_code, error_body } => write!(f, "unexpected status code: {}, error body: {}", status_code, error_body),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn create_mock_tf() -> ManagementApiTokenFetcher {
    ManagementApiTokenFetcher {
      access_token: Mutex::new(AccessToken::default()),
      fetched_at: Mutex::new(Instant::now()),
      client_id: "client_id".to_string(),
      client_secret: "client_secret".to_string(),
      client: reqwest::Client::new(),
      auth_url: "http://localhost".to_string(),
    }
  }

  /// Ensures `AccessToken` is properly deserialized and returns expected fields.
  #[test]
  fn test_access_token() {
    let token_str = r#"{
          "access_token": "secret_access_token",
          "expires_in": 600,
          "refresh_expires_in": 0,
          "token_type": "Bearer",
          "not-before-policy": 0,
          "scope": "email"
        }"#;
    let token: AccessToken = serde_json::from_str(token_str).unwrap();
    assert_eq!(token.access_token, "secret_access_token");
    assert_eq!(token.expires_in, 600);
    assert_eq!(token.refresh_expires_in, 0);
    assert_eq!(token.token_type, "Bearer");
    assert_eq!(token.not_before_policy, 0);
    assert_eq!(token.scope, "email");
    assert_eq!(token.formatted_token(), "Bearer secret_access_token");
  }

  /// Validates the default constructor yields an empty `AccessToken`.
  #[test]
  fn test_access_token_default() {
    let token = AccessToken::default();
    assert_eq!(token.access_token, "");
    assert_eq!(token.expires_in, 0);
    assert_eq!(token.refresh_expires_in, 0);
    assert_eq!(token.token_type, "");
    assert_eq!(token.not_before_policy, 0);
    assert_eq!(token.scope, "");
    assert_eq!(token.formatted_token(), " ");
  }

  /// Verifies that a default token is considered invalid since it expires immediately.
  #[test]
  fn test_rest_token_fetcher_is_valid_default_token() {
    let tf = create_mock_tf();
    assert!(!tf.is_valid(), "Default token should be invalid");
  }

  /// Demonstrates that `is_valid` returns true if a token is configured with future expiration.
  #[test]
  fn test_rest_token_fetcher_is_valid_valid_token() {
    let tf = create_mock_tf();
    tf.access_token.lock().unwrap().expires_in = 600;
    assert!(tf.is_valid(), "Token with 600s lifetime should be valid initially");
  }

  /// Confirms `is_valid` returns false after the token’s entire lifetime has elapsed.
  #[test]
  fn test_rest_token_fetcher_is_valid_expired_token() {
    let tf = create_mock_tf();
    tf.access_token.lock().unwrap().expires_in = 600;
    *tf.fetched_at.lock().unwrap() = Instant::now() - Duration::from_secs(600);
    assert!(!tf.is_valid(), "Token should expire after 600s have passed");
  }

  /// Tests behavior when a token is “poisoned” (i.e., panicked while locked).
  #[test]
  fn test_rest_token_fetcher_is_valid_poisoned_token() {
    let tf = create_mock_tf();
    tf.access_token.lock().unwrap().expires_in = 600;
    let tf_arc = std::sync::Arc::new(tf);
    let tf_clone = tf_arc.clone();
    assert!(tf_arc.is_valid(), "Token should be valid before poison");
    let handle = std::thread::spawn(move || {
      let _unused = tf_clone.access_token.lock().unwrap();
      panic!("Poison token");
    });
    let _ = handle.join();
    assert!(!tf_arc.is_valid(), "Token should be reset to default after poisoning");
  }

  /// Checks success scenario for fetching a new token from a mock server.
  #[tokio::test]
  async fn test_fetch_access_token_from_server() {
    let mut auth_server = mockito::Server::new_async().await;
    auth_server
      .mock("POST", "/")
      .with_status(200)
      .with_body(
        r#"{
          "access_token": "secret_access_token",
          "expires_in": 600,
          "refresh_expires_in": 0,
          "token_type": "Bearer",
          "not-before-policy": 0,
          "scope": "email"
        }"#,
      )
      .create();
    let mut tf = create_mock_tf();
    tf.auth_url = auth_server.url();
    let token = tf.fetch_access_token_from_server().await.unwrap();
    assert_eq!(token.access_token, "secret_access_token");
    assert_eq!(token.expires_in, 600);
  }

  /// Checks that an HTTP 400 response is handled as an error.
  #[tokio::test]
  async fn test_fetch_access_token_from_server_error() {
    let mut auth_server = mockito::Server::new_async().await;
    auth_server.mock("POST", "/").with_status(400).with_body("Bad request").create();
    let mut tf = create_mock_tf();
    tf.auth_url = auth_server.url();
    let err = tf.fetch_access_token_from_server().await.unwrap_err();
    match err {
      ManagementApiTokenError::StatusCode { status_code, error_body } => {
        assert_eq!(status_code, reqwest::StatusCode::BAD_REQUEST);
        assert_eq!(error_body, "Bad request");
      }
      _ => panic!("Unexpected error: {:?}", err),
    }
  }

  /// Ensures the builder sets `client_id` explicitly.
  #[test]
  fn test_token_fetcher_builder_client_id() {
    let platform = DshPlatform::try_from("nplz").unwrap();
    let client_id = "robot:dev-lz-dsh:my-tenant";
    let client_secret = "secret";
    let tf = ManagementApiTokenFetcherBuilder::new(platform.clone())
      .client_id(client_id.to_string())
      .client_secret(client_secret.to_string())
      .build()
      .unwrap();
    assert_eq!(tf.client_id, client_id);
    assert_eq!(tf.client_secret, client_secret);
    assert_eq!(tf.auth_url, platform.access_token_endpoint());
  }

  /// Ensures the builder can auto-generate `client_id` from the `tenant_name`.
  #[test]
  fn test_token_fetcher_builder_tenant_name() {
    let platform = DshPlatform::try_from("nplz").unwrap();
    let tenant_name = "my-tenant";
    let client_secret = "secret";
    let tf = ManagementApiTokenFetcherBuilder::new(platform.clone())
      .tenant_name(tenant_name.to_string())
      .client_secret(client_secret.to_string())
      .build()
      .unwrap();
    assert_eq!(tf.client_id, format!("robot:{}:{}", platform.realm(), tenant_name));
    assert_eq!(tf.client_secret, client_secret);
    assert_eq!(tf.auth_url, platform.access_token_endpoint());
  }

  /// Validates that a custom `reqwest::Client` can be injected into the builder.
  #[test]
  fn test_token_fetcher_builder_custom_client() {
    let platform = DshPlatform::try_from("nplz").unwrap();
    let client_id = "robot:dev-lz-dsh:my-tenant";
    let client_secret = "secret";
    let custom_client = reqwest::Client::builder().use_rustls_tls().build().unwrap();
    let tf = ManagementApiTokenFetcherBuilder::new(platform.clone())
      .client_id(client_id.to_string())
      .client_secret(client_secret.to_string())
      .client(custom_client.clone())
      .build()
      .unwrap();
    assert_eq!(tf.client_id, client_id);
    assert_eq!(tf.client_secret, client_secret);
    assert_eq!(tf.auth_url, platform.access_token_endpoint());
  }

  /// Tests precedence of `client_id` over a derived tenant-based client ID.
  #[test]
  fn test_token_fetcher_builder_client_id_precedence() {
    let platform = DshPlatform::try_from("nplz").unwrap();
    let tenant = "my-tenant";
    let client_id_override = "override";
    let client_secret = "secret";
    let tf = ManagementApiTokenFetcherBuilder::new(platform.clone())
      .tenant_name(tenant.to_string())
      .client_id(client_id_override.to_string())
      .client_secret(client_secret.to_string())
      .build()
      .unwrap();
    assert_eq!(tf.client_id, client_id_override);
    assert_eq!(tf.client_secret, client_secret);
    assert_eq!(tf.auth_url, platform.access_token_endpoint());
  }

  /// Ensures builder returns errors if `client_id` or `client_secret` are missing.
  #[test]
  fn test_token_fetcher_builder_build_error() {
    let platform = DshPlatform::try_from("nplz").unwrap();
    let err = ManagementApiTokenFetcherBuilder::new(platform.clone())
      .client_secret("client_secret".to_string())
      .build()
      .unwrap_err();
    assert!(matches!(err, ManagementApiTokenError::UnknownClientId));

    let err = ManagementApiTokenFetcherBuilder::new(platform)
      .tenant_name("tenant_name".to_string())
      .build()
      .unwrap_err();
    assert!(matches!(err, ManagementApiTokenError::UnknownClientSecret));
  }
}
