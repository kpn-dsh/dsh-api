//! # Generic API function calls
//!
//! Module that contains methods to call the api methods in a generic way.
//! What this means is that the API functions can be called indirect,
//! where the path of the method must be provided as an argument.
//!
//! This has a number of consequences,
//! which are caused by the limitations of the `rust` language with respect to abstraction:
//! * The number and types of the required parameters for each method
//!   are not known at compile time, which means that (emulated) dynamic typing is used
//!   and parameters errors will occur at run-time instead of compile time.
//!   * Path parameters must be provided as `&str`.
//!   * Body parameters must be provided as a json `&str` that can be deserialized at runtime
//!     into the expected type.
//! * The response type for each method is not known at compile time.
//!   * For `GET` methods the responses will be returned as dynamic trait objects
//!     that implement [`erased_serde::Serialize`], defined in the
//!     [`erased_serde`](https://crates.io/crates/erased-serde) crate.
//!     These objects can be serialized into json, yaml or toml without any type information.
//!   * If `DELETE`, `POST` and `PUT` methods return data this will be ignored
//!     and only errors will be returned.
//!
//! # Examples
//!
//! Get the configuration of the application `keyring-dev` and print it as json.
//!
//! ```ignore
//! # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
//! let application = client.get(
//!   "get_application_configuration_by_tenant_by_appid",
//!   &["keyring-dev"]
//! ).await?;
//! println!("{}", serde_json::to_string_pretty(&application)?);
//! # Ok(())
//! # }
//! ```
//!
//! Update the secret `abcdef` to the value `ABCDEF`.
//!
//! ```ignore
//! # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
//!  let secret = serde_json::to_string("ABCDEF")?;
//!  client.put("put_secret_by_tenant_by_id", &["abcdef"], &secret).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # API functions
//!
//! [`DshApiClient`] methods that call the DSH resource management API.
//!
//! * [`delete(path, [parameters]) -> Ok`](DshApiClient::delete)
//! * [`get(path, [parameters]) -> serialize`](DshApiClient::get)
//! * [`head(path, [parameters]) -> serialize`](DshApiClient::head)
//! * [`patch(path, [parameters]) -> serialize`](DshApiClient::patch)
//! * [`post(path, [parameters], body) -> Ok`](DshApiClient::post)
//! * [`put(path, [parameters], body) -> Ok`](DshApiClient::put)
#[cfg(feature = "generic")]
include!(concat!(env!("OUT_DIR"), "/generic.rs"));
