//! # Manage buckets
//!
//! Module that contains methods and functions to manage buckets.
//! * API methods - DshApiClient methods that directly call the API.
//! * Derived methods - DshApiClient methods that add extra capabilities
//!   but depend on the API methods.
//!
//! # API methods
//!
//! [`DshApiClient`] methods that directly call the DSH resource management API.
//!
//! * [`delete_bucket(id)`](DshApiClient::delete_bucket)
//! * [`get_bucket(id) -> bucket_status`](DshApiClient::get_bucket)
//! * [`get_bucket_allocation_status(id) -> allocation_status`](DshApiClient::get_bucket_allocation_status)
//! * [`get_bucket_configuration(id) -> bucket`](DshApiClient::get_bucket_configuration)
//! * [`get_buckets() -> map<id, bucket>`](DshApiClient::get_buckets)
//! * [`list_bucket_ids() -> [id]`](DshApiClient::list_bucket_ids)
//! * [`list_buckets() -> [(id, bucket)]`](DshApiClient::list_bucket_ids)
//! * [`update_bucket(id, bucket)`](DshApiClient::update_bucket)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`get_buckets() -> map<bucket_id, bucket>`](DshApiClient::get_buckets)
//! * [`list_buckets() -> [(bucket_id, bucket)]`](DshApiClient::list_buckets)
#![cfg_attr(feature = "actual", doc = "")]
#![cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#![cfg_attr(feature = "actual", doc = "* [`get_bucket_actual_configuration(bucket_id) -> Bucket`](DshApiClient::get_bucket_actual_configuration)")]

use crate::dsh_api_client::DshApiClient;
use crate::types::{AllocationStatus, Bucket, BucketStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::DshApiResult;
use futures::future::try_join_all;
use std::collections::HashMap;

/// # Manage buckets
///
/// Module that contains methods and functions to manage buckets.
/// * API methods - DshApiClient methods that directly call the API.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # API methods
///
/// [`DshApiClient`] methods that directly call the DSH resource management API.
///
/// * [`delete_bucket(id)`](DshApiClient::delete_bucket)
/// * [`get_bucket(id) -> bucket_status`](DshApiClient::get_bucket)
/// * [`get_bucket_allocation_status(id) -> allocation_status`](DshApiClient::get_bucket_allocation_status)
/// * [`get_bucket_configuration(id) -> bucket`](DshApiClient::get_bucket_configuration)
/// * [`get_buckets() -> map<id, bucket>`](DshApiClient::get_buckets)
/// * [`list_bucket_ids() -> [id]`](DshApiClient::list_bucket_ids)
/// * [`list_buckets() -> [(id, bucket)]`](DshApiClient::list_bucket_ids)
/// * [`update_bucket(id, bucket)`](DshApiClient::update_bucket)
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`get_buckets() -> map<bucket_id, bucket>`](DshApiClient::get_buckets)
/// * [`list_buckets() -> [(bucket_id, bucket)]`](DshApiClient::list_buckets)
#[cfg_attr(feature = "actual", doc = "")]
#[cfg_attr(feature = "actual", doc = "# Actual configuration methods")]
#[cfg_attr(feature = "actual", doc = "* [`get_bucket_actual_configuration(bucket_id) -> Bucket`](DshApiClient::get_bucket_actual_configuration)")]
impl DshApiClient<'_> {
  /// # Delete bucket
  ///
  /// API function: `DELETE /allocation/{tenant}/bucket/{id}/configuration`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the bucket to delete
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the bucket has been successfully deleted)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn delete_bucket(&self, bucket_id: &str) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .delete_bucket_configuration_by_tenant_by_id(self.tenant_name(), bucket_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Get bucket
  ///
  /// API function: `GET /allocation/{tenant}/bucket/{id}`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the requested bucket
  ///
  /// # Returns
  /// * `Ok<`[`BucketStatus`]`>` - bucket
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_bucket(&self, bucket_id: &str) -> DshApiResult<BucketStatus> {
    self
      .process_raw(self.generated_client.get_bucket_by_tenant_by_id(self.tenant_name(), bucket_id, self.token()).await)
      .map(|(_, result)| result)
  }

  /// # Return actual state of bucket
  ///
  /// API function: `GET /allocation/{tenant}/bucket/{id}/actual`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the requested bucket
  ///
  /// # Returns
  /// * `Ok<`[`Bucket`]`>` - indicates that bucket is ok
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  #[cfg(feature = "actual")]
  pub async fn get_bucket_actual_configuration(&self, bucket_id: &str) -> DshApiResult<Bucket> {
    self
      .process(
        self
          .generated_client
          .get_bucket_actual_by_tenant_by_id(self.tenant_name(), bucket_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return bucket allocation status
  ///
  /// API function: `GET /allocation/{tenant}/bucket/{id}/status`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the requested bucket
  ///
  /// # Returns
  /// * `Ok<`[`AllocationStatus`]`>` - allocation status of the bucket
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_bucket_allocation_status(&self, bucket_id: &str) -> DshApiResult<AllocationStatus> {
    self
      .process(
        self
          .generated_client
          .get_bucket_status_by_tenant_by_id(self.tenant_name(), bucket_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return bucket configuration
  ///
  /// API function: `GET /allocation/{tenant}/bucket/{id}/configuration`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the requested bucket
  ///
  /// # Returns
  /// * `Ok<`[`Bucket`]`>` - indicates that bucket is ok
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn get_bucket_configuration(&self, bucket_id: &str) -> DshApiResult<Bucket> {
    self
      .process(
        self
          .generated_client
          .get_bucket_configuration_by_tenant_by_id(self.tenant_name(), bucket_id, self.token())
          .await,
      )
      .map(|(_, result)| result)
  }

  /// # Return list of bucket ids
  ///
  /// API function: `GET /allocation/{tenant}/bucket`
  ///
  /// # Returns
  /// * `Ok<Vec<String>` - bucket ids
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn list_bucket_ids(&self) -> DshApiResult<Vec<String>> {
    let mut bucket_ids: Vec<String> = self
      .process(self.generated_client.get_bucket_by_tenant(self.tenant_name(), self.token()).await)
      .map(|(_, result)| result)
      .map(|bucket_ids| bucket_ids.iter().map(|bucket_id| bucket_id.to_string()).collect())?;
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

  /// # Update bucket
  ///
  /// API function: `PUT /allocation/{tenant}/bucket/{id}/configuration`
  ///
  /// # Parameters
  /// * `bucket_id` - id of the bucket to update
  /// * `bucket` - new value of the bucket
  ///
  /// # Returns
  /// * `Ok(())` - when DSH has properly received the request
  ///              (note that this does not mean that the bucket has been successfully updated)
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn update_bucket(&self, bucket_id: &str, bucket: Bucket) -> DshApiResult<()> {
    self
      .process(
        self
          .generated_client
          .put_bucket_configuration_by_tenant_by_id(self.tenant_name(), bucket_id, self.token(), &bucket)
          .await,
      )
      .map(|(_, result)| result)
  }
}
