//! # Additional methods to manage buckets
//!
//! Module that contains methods to manage buckets.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_buckets() -> map<bucket_id, bucket>`](DshApiClient::get_buckets)
//! * [`list_buckets() -> [(bucket_id, bucket)]`](DshApiClient::list_buckets)
//! * [`list_bucket_ids() -> [id]`](DshApiClient::list_bucket_ids)

use crate::dsh_api_client::DshApiClient;
use crate::types::BucketStatus;
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;
use futures::future::try_join_all;
use std::collections::HashMap;

/// # Additional methods to manage buckets
///
/// Module that contains methods and functions to manage buckets.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_buckets() -> map<bucket_id, bucket>`](DshApiClient::get_buckets)
/// * [`list_buckets() -> [(bucket_id, bucket)]`](DshApiClient::list_buckets)
/// * [`list_bucket_ids() -> [id]`](DshApiClient::list_bucket_ids)
impl DshApiClient {
  /// # Return list of bucket ids
  ///
  /// API function: `GET /allocation/{tenant}/bucket`
  ///
  /// # Returns
  /// * `Ok<Vec<String>` - bucket ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_bucket_ids(&self) -> DshApiResult<Vec<String>> {
    let mut bucket_ids = self.get_bucket_ids().await?;
    bucket_ids.sort();
    Ok(bucket_ids)
  }

  /// Return a list of all buckets
  ///
  /// # Returns
  /// * `Vec<(String, `[`BucketStatus`]`)>` - list of tuples that describe the buckets,
  ///   ordered by bucket id. Each tuple consist of
  ///   * `String` - id of the bucket,
  ///   * `BucketStatus` - the bucket data.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_buckets(&self) -> DshApiResult<Vec<(String, BucketStatus)>> {
    let bucket_ids: Vec<String> = self.list_bucket_ids().await?;
    let bucket_statuses = try_join_all(bucket_ids.iter().map(|bucket_id| self.get_bucket(bucket_id.as_str()))).await?;
    Ok(bucket_ids.into_iter().zip(bucket_statuses).collect::<Vec<_>>())
  }

  /// Return a map of all bucket ids and buckets
  ///
  /// # Returns
  /// * `HashMap<String, `[`BucketStatus`]`>` - `HashMap` that maps all bucket ids to buckets.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_buckets(&self) -> DshApiResult<HashMap<String, BucketStatus>> {
    Ok(self.list_buckets().await?.into_iter().collect())
  }
}
