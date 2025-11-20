//! # Additional methods to manage buckets
//!
//! Module that contains methods and functions to manage buckets.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`delete_bucket_access_configuration(id, name)`](DshApiClient::delete_bucket_access_configuration)
//! * [`delete_bucket_configuration(id)`](DshApiClient::delete_bucket_configuration)
//! * [`delete_bucket_watch_configuration(id)`](DshApiClient::delete_bucket_watch_configuration)
//! * [`get_bucket(id) -> BucketStatus`](DshApiClient::get_bucket)
//! * [`get_bucket_access(id, name) -> BucketAccessStatus`](DshApiClient::get_bucket_access)
//! * [`get_bucket_access_actual(id, name) -> BucketAccess`](DshApiClient::get_bucket_access_actual)
//! * [`get_bucket_access_configuration(id, name) -> BucketAccessConfiguration`](DshApiClient::get_bucket_access_configuration)
//! * [`get_bucket_access_ids(id) -> [id]`](DshApiClient::get_bucket_access_ids)
//! * [`get_bucket_access_status(id, name) -> AllocationStatus`](DshApiClient::get_bucket_access_status)
//! * [`get_bucket_actual(id) -> Bucket`](DshApiClient::get_bucket_actual)
//! * [`get_bucket_configuration(id) -> Bucket`](DshApiClient::get_bucket_configuration)
//! * [`get_bucket_fromthirdparty_ids() -> [id]`](DshApiClient::get_bucket_fromthirdparty_ids)
//! * [`get_bucket_ids() -> [id]`](DshApiClient::get_bucket_ids)
//! * [`get_bucket_status(id) -> AllocationStatus`](DshApiClient::get_bucket_status)
//! * [`get_bucket_watch(id) -> BucketWatchStatus`](DshApiClient::get_bucket_watch)
//! * [`get_bucket_watch_actual(id) -> BucketWatch`](DshApiClient::get_bucket_watch_actual)
//! * [`get_bucket_watch_configuration(id) -> BucketWatch`](DshApiClient::get_bucket_watch_configuration)
//! * [`get_bucket_watch_status(id) -> AllocationStatus`](DshApiClient::get_bucket_watch_status)
//! * [`get_bucketaccess_ids() -> [id]`](DshApiClient::get_bucketaccess_ids)
//! * [`get_bucketwatch_ids() -> [id]`](DshApiClient::get_bucketwatch_ids)
//! * [`get_thirdpartybucket(id) -> ThirdPartyBucketConcessionStatus`](DshApiClient::get_thirdpartybucket)
//! * [`get_thirdpartybucket_actual(id) -> ThirdPartyBucketConcession`](DshApiClient::get_thirdpartybucket_actual)
//! * [`get_thirdpartybucket_configuration(id) -> ThirdPartyBucketConcessionConfiguration`](DshApiClient::get_thirdpartybucket_configuration)
//! * [`get_thirdpartybucket_ids() -> [id]`](DshApiClient::get_thirdpartybucket_ids)
//! * [`get_thirdpartybucket_status(id) -> AllocationStatus`](DshApiClient::get_thirdpartybucket_status)
//! * [`post_thirdpartybucket(body)`](DshApiClient::post_thirdpartybucket)
//! * [`put_bucket_access_configuration(id, name, body)`](DshApiClient::put_bucket_access_configuration)
//! * [`put_bucket_configuration(id, body)`](DshApiClient::put_bucket_configuration)
//! * [`put_bucket_watch_configuration(id)`](DshApiClient::put_bucket_watch_configuration)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`bucket_ids_with_dependants() -> [(bucket id, [dependant])]`](DshApiClient::bucket_ids_with_dependants)
//! * [`bucket_map() -> map(bucket id -> bucket)`](DshApiClient::bucket_map)
//! * [`bucket_name(bucket id) -> bucket name`](DshApiClient::bucket_name)
//! * [`bucket_with_dependants(bucket id) -> (bucket id, [dependant])]`](DshApiClient::bucket_with_dependants)
//! * [`buckets() -> [(bucket id, bucket)]`](DshApiClient::buckets)
//! * [`buckets_with_dependant_applications() -> [(bucket_id, bucket, [dependant application])]`](DshApiClient::buckets_with_dependant_applications)
//! * [`buckets_with_dependant_apps() -> [(bucket id, bucket, [dependant app])]`](DshApiClient::buckets_with_dependant_apps)
//! * [`buckets_with_dependants() -> [(bucket_id, [dependant])]`](DshApiClient::bucket_with_dependants)

use crate::app::{app_resources, apps_that_use_resource};
use crate::application_types::{ApplicationValues, EnvVarInjection};
use crate::dsh_api_client::DshApiClient;
use crate::parse::parse_function1;
use crate::platform::CloudProvider;
use crate::types::{AppCatalogApp, AppCatalogAppResourcesValue, Application, Bucket, BucketStatus};
#[allow(unused_imports)]
use crate::DshApiError;
use crate::{Dependant, DependantApp, DependantApplication, DshApiResult};
use futures::future::try_join_all;
use futures::try_join;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// Secret name for object store access key id
pub const OBJECT_STORE_ACCESS_KEY_ID: &str = "system/objectstore/access_key_id";
/// Secret name for object store secret access key
pub const OBJECT_STORE_SECRET_ACCESS_KEY: &str = "system/objectstore/secret_access_key";

/// # Describes an injection of a resource in an application
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BucketInjection {
  /// Environment variable injection, where the value is the name of the environment variable.
  #[serde(rename = "env")]
  EnvVar(String),
  /// Variable function, where the value is the name of the environment variable.
  #[serde(rename = "variable")]
  Variable(String),
}

impl Display for BucketInjection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      BucketInjection::EnvVar(env_var) => write!(f, "{}", env_var),
      BucketInjection::Variable(variable) => write!(f, "{{ bucket_name('{}') }}", variable),
    }
  }
}

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
/// * [`bucket_ids_with_dependants() -> [(bucket id, [dependant])]`](DshApiClient::bucket_ids_with_dependants)
/// * [`bucket_map() -> map(bucket id -> bucket)`](DshApiClient::bucket_map)
/// * [`bucket_name(bucket id) -> bucket name`](DshApiClient::bucket_name)
/// * [`bucket_with_dependants(bucket id) -> (bucket id, [dependant])]`](DshApiClient::bucket_with_dependants)
/// * [`buckets() -> [(bucket id, bucket)]`](DshApiClient::buckets)
/// * [`buckets_with_dependant_applications() -> [(bucket_id, bucket, [dependant application])]`](DshApiClient::buckets_with_dependant_applications)
/// * [`buckets_with_dependant_apps() -> [(bucket id, bucket, [dependant app])]`](DshApiClient::buckets_with_dependant_apps)
/// * [`buckets_with_dependants() -> [(bucket_id, [dependant])]`](DshApiClient::bucket_with_dependants)
impl DshApiClient {
  /// # Returns all bucket identifiers with dependant applications and apps
  ///
  /// Returns a sorted list of all bucket ids together with the applications and apps that use them.
  pub async fn bucket_ids_with_dependants(&self) -> DshApiResult<Vec<(String, Vec<Dependant<BucketInjection>>)>> {
    let (bucket_ids, applications, apps, access_key_id) = try_join!(
      self.get_bucket_ids(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map(),
      self.object_store_access_key_id_if_required()
    )?;
    let mut buckets = Vec::<(String, Vec<Dependant<BucketInjection>>)>::new();
    for bucket_id in &bucket_ids {
      let mut dependants: Vec<Dependant<BucketInjection>> = vec![];
      let bucket_name = self.platform().bucket_name(self.tenant_name(), bucket_id, access_key_id.as_deref()).ok();
      for application_injections in bucket_injections_from_applications(bucket_id, bucket_name.as_deref(), &applications) {
        dependants.push(Dependant::application(
          application_injections.id.to_string(),
          application_injections.application.instances,
          application_injections.values,
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_resource(bucket_id.as_str(), &apps, &bucket_resources_from_app) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      buckets.push((bucket_id.to_string(), dependants));
    }
    Ok(buckets)
  }

  /// Return a map of all bucket ids and buckets
  ///
  /// # Returns
  /// * `HashMap<String, `[`BucketStatus`]`>` - `HashMap` that maps all bucket ids to buckets.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn bucket_map(&self) -> DshApiResult<HashMap<String, BucketStatus>> {
    Ok(self.buckets().await?.into_iter().collect())
  }

  /// Create a bucket name from a bucket id
  ///
  /// # Returns
  /// * `Ok<String>` - When the key was successfully craeted.
  /// * `Err<`[`DshApiError::NotFound`]`>` - When on Azure the bucket secret is not set.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed.
  pub async fn bucket_name(&self, bucket_id: &str) -> DshApiResult<String> {
    match self.platform().cloud_provider() {
      CloudProvider::Azure => match self.object_store_access_key_id_if_required().await {
        Ok(Some(access_key_id)) => Ok(self.platform().bucket_name(self.tenant_name(), bucket_id, Some(access_key_id))?),
        _ => Err(DshApiError::NotFound(Some(format!(
          "bucket name for azure requires the object store access key '{}'",
          OBJECT_STORE_ACCESS_KEY_ID
        )))),
      },
      CloudProvider::AWS => Ok(self.platform().bucket_name(self.tenant_name(), bucket_id, None::<String>)?.to_string()),
    }
  }

  /// # Returns all buckets with dependant applications and apps
  ///
  /// # Parameters
  /// * `bucket_id` - Identifier of the requested bucket.
  ///
  /// Returns a bucket with the applications and apps that use it.
  pub async fn bucket_with_dependants(&self, bucket_id: &str) -> DshApiResult<(BucketStatus, Vec<Dependant<BucketInjection>>)> {
    let (bucket_status, application_configuration_map, appcatalogapp_configuration_map) = try_join!(
      self.get_bucket(bucket_id),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map()
    )?;
    let mut dependants: Vec<Dependant<BucketInjection>> = vec![];
    let bucket_name = self.bucket_name(bucket_id).await.ok();
    for application in bucket_injections_from_applications(bucket_id, bucket_name.as_deref(), &application_configuration_map) {
      dependants.push(Dependant::application(
        application.id.to_string(),
        application.application.instances,
        application.values,
      ));
    }
    for (app_id, _, resource_ids) in apps_that_use_resource(bucket_id, &appcatalogapp_configuration_map, &bucket_resources_from_app) {
      dependants.push(Dependant::app(
        app_id.to_string(),
        resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
      ));
    }
    Ok((bucket_status, dependants))
  }

  /// Return a list of all buckets
  ///
  /// # Returns
  /// * `Vec<(String, `[`BucketStatus`]`)>` - list of tuples that describe the buckets,
  ///   ordered by bucket id. Each tuple consist of
  ///   * `String` - id of the bucket,
  ///   * `BucketStatus` - the bucket data.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn buckets(&self) -> DshApiResult<Vec<(String, BucketStatus)>> {
    let bucket_ids: Vec<String> = self.get_bucket_ids().await?;
    let bucket_statuses = try_join_all(bucket_ids.iter().map(|bucket_id| self.get_bucket(bucket_id.as_str()))).await?;
    Ok(bucket_ids.into_iter().zip(bucket_statuses).collect_vec())
  }

  /// # Returns all buckets with dependant applications
  ///
  /// Returns a sorted list of all buckets ids, bucket statuses and applications that use them.
  pub async fn buckets_with_dependant_applications(&self) -> DshApiResult<Vec<(String, BucketStatus, Vec<DependantApplication<BucketInjection>>)>> {
    let (buckets, application_configuration_map) = try_join!(self.buckets(), self.get_application_configuration_map())?;
    let mut buckets_with_dependant_applications = Vec::<(String, BucketStatus, Vec<DependantApplication<BucketInjection>>)>::new();
    for (ref bucket_id, bucket_status) in buckets {
      let mut dependant_applications: Vec<DependantApplication<BucketInjection>> = vec![];
      let bucket_name = self.bucket_name(bucket_id).await.ok();
      for application in bucket_injections_from_applications(bucket_id.as_str(), bucket_name.as_deref(), &application_configuration_map) {
        dependant_applications.push(DependantApplication::new(
          application.id.to_string(),
          application.application.instances,
          application.values,
        ));
      }
      buckets_with_dependant_applications.push((bucket_id.to_string(), bucket_status, dependant_applications));
    }
    Ok(buckets_with_dependant_applications)
  }

  /// # Returns all buckets with dependant apps
  ///
  /// Returns a sorted list of all buckets ids, buckets and apps that use them.
  pub async fn buckets_with_dependant_apps(&self) -> DshApiResult<Vec<(String, BucketStatus, Vec<DependantApp>)>> {
    let (buckets, appcatalogapp_configuration_map) = try_join!(self.buckets(), self.get_appcatalogapp_configuration_map())?;
    let mut buckets_with_dependant_apps = Vec::<(String, BucketStatus, Vec<DependantApp>)>::new();
    for (bucket_id, bucket_status) in buckets {
      let mut dependant_apps: Vec<DependantApp> = vec![];
      for (app_id, _, resource_ids) in apps_that_use_resource(bucket_id.as_str(), &appcatalogapp_configuration_map, &bucket_resources_from_app) {
        dependant_apps.push(DependantApp::new(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      buckets_with_dependant_apps.push((bucket_id, bucket_status, dependant_apps));
    }
    Ok(buckets_with_dependant_apps)
  }

  /// # Returns all buckets with dependant applications and apps
  ///
  /// Returns a sorted list of all buckets ids, buckets and applications and apps that use them.
  ///
  /// # Returns
  /// Tuples describing the buckets. Each tuple contains:
  /// * bucket identifier,
  /// * bucket status,
  /// * list of dependants.
  pub async fn buckets_with_dependants(&self) -> DshApiResult<Vec<(String, BucketStatus, Vec<Dependant<BucketInjection>>)>> {
    let (buckets, application_configuration_map, apps, access_key_id) = try_join!(
      self.buckets(),
      self.get_application_configuration_map(),
      self.get_appcatalogapp_configuration_map(),
      self.object_store_access_key_id_if_required()
    )?;
    let mut buckets_with_dependants = Vec::<(String, BucketStatus, Vec<Dependant<BucketInjection>>)>::new();
    for (ref bucket_id, bucket_status) in buckets {
      let mut dependants: Vec<Dependant<BucketInjection>> = vec![];
      let bucket_name = self.platform().bucket_name(self.tenant_name(), bucket_id, access_key_id.as_deref()).ok();
      for application in bucket_injections_from_applications(bucket_id.as_str(), bucket_name.as_deref(), &application_configuration_map) {
        dependants.push(Dependant::application(
          application.id.to_string(),
          application.application.instances,
          application.values,
        ));
      }
      for (app_id, _, resource_ids) in apps_that_use_resource(bucket_id.as_str(), &apps, &bucket_resources_from_app) {
        dependants.push(Dependant::app(
          app_id.to_string(),
          resource_ids.iter().map(|resource_id| resource_id.to_string()).collect_vec(),
        ));
      }
      buckets_with_dependants.push((bucket_id.to_string(), bucket_status, dependants));
    }
    Ok(buckets_with_dependants)
  }

  /// # Returns the object store secrets
  ///
  /// Returns the object store `access_key_id` and `secret_access_key`.
  pub async fn bucket_secrets(&self) -> DshApiResult<(String, String)> {
    Ok(try_join!(
      self.get_secret(OBJECT_STORE_ACCESS_KEY_ID),
      self.get_secret(OBJECT_STORE_SECRET_ACCESS_KEY)
    )?)
  }

  /// Get object store access key
  async fn object_store_access_key_id_if_required(&self) -> DshApiResult<Option<String>> {
    match self.platform().cloud_provider() {
      CloudProvider::AWS => Ok(None),
      CloudProvider::Azure => self.get_secret(OBJECT_STORE_ACCESS_KEY_ID).await.map(Some),
    }
  }
}

/// # Get applications environment variables referencing bucket
///
/// Get all environment variables referencing bucket `bucket_id` or (if available) `bucket_name`
/// from multiple `Application`s. Applications are only included if they reference bucket
/// `bucket_id` or `bucket_name` at least once.
///
/// # Parameters
/// * `bucket_id` - Identifies the bucket to look for.
/// * `bucket_name` - Optional full bucket name for the platform.
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// List of tuples containing:
/// * application id,
/// * application reference,
/// * sorted list of environment variable keys that reference bucket `bucket_id`.
///
/// The list is sorted by application id.
pub fn bucket_injections_from_applications<'a>(
  bucket_id: &str,
  bucket_name: Option<&str>,
  applications: &'a HashMap<String, Application>,
) -> Vec<ApplicationValues<'a, BucketInjection>> {
  let mut application_injections: Vec<ApplicationValues<BucketInjection>> = vec![];
  for (application_id, application) in applications {
    let environment_variable_keys: Vec<BucketInjection> = bucket_injections_from_application(bucket_id, bucket_name, application);
    if !environment_variable_keys.is_empty() {
      application_injections.push(ApplicationValues::new(application_id, application, environment_variable_keys));
    }
  }
  application_injections.sort();
  application_injections
}

/// # Get application environment variables referencing bucket
///
/// Get all environment variables referencing bucket `bucket_id` from an `Application`.
/// When the application does not reference the bucket, an empty list will be returned.
///
/// # Parameters
/// * `bucket_id` - id of the bucket to look for
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<EnvVarKey>` - list of all environment variables referencing bucket `bucket_id`
///
/// The list is sorted by environment variable key.
pub fn bucket_injections_from_application(bucket_id: &str, bucket_name: Option<&str>, application: &Application) -> Vec<BucketInjection> {
  let mut env_var_keys = application
    .env
    .iter()
    .filter_map(|(env_key, env_value)| match parse_function1(env_value, "bucket_name") {
      Ok(bucket_string) => {
        if bucket_id == bucket_string {
          Some(BucketInjection::Variable(env_key.to_string()))
        } else {
          None
        }
      }
      Err(_) => match bucket_name {
        Some(name) => {
          if env_value.contains(name) {
            Some(BucketInjection::EnvVar(env_key.to_string()))
          } else {
            None
          }
        }
        None => None,
      },
    })
    .collect_vec();
  env_var_keys.sort();
  env_var_keys
}

/// Get bucket resources from `AppCatalogApp`
///
/// # Parameters
/// * `app` - app to get the bucket resources from
///
/// # Returns
/// Either `None` when the `app` does not have any bucket resources,
/// or a `Some` that contains tuples describing the bucket resources:
/// * resource id
/// * reference to the `Bucket`
pub fn bucket_resources_from_app(app: &AppCatalogApp) -> Vec<(&str, &Bucket)> {
  app_resources(app, &|resource_value| match resource_value {
    AppCatalogAppResourcesValue::Bucket(bucket) => Some(bucket),
    _ => None,
  })
}

/// # Get application environment variables referencing buckets
///
/// Get all environment variables from an `Application` that reference a bucket.
/// When the application does not reference any buckets, an empty list will be returned.
///
/// # Parameters
/// * `application` - reference to the `Application`
///
/// # Returns
/// `Vec<EnvInjection>` - list of tuples containing:
/// * bucket id
/// * list of environment variables referencing the bucket
///
/// The list is sorted by bucket id.
pub fn buckets_from_application(application: &Application) -> Vec<EnvVarInjection> {
  let mut buckets = HashMap::<&str, Vec<&str>>::new();
  for (env_key, env_value) in &application.env {
    if let Ok(bucket_id) = parse_function1(env_value, "bucket_name") {
      buckets.entry(bucket_id).or_default().push(env_key);
    }
  }
  let mut sorted_buckets: Vec<EnvVarInjection> = buckets.into_iter().map(EnvVarInjection::from).collect_vec();
  sorted_buckets.sort();
  sorted_buckets
}

/// # Get applications environment variables referencing buckets
///
/// Get all environment variables referencing buckets from all `Application`s.
/// Applications are only included if they reference at least one bucket.
///
/// # Parameters
/// * `applications` - hashmap containing id/application pairs
///
/// # Returns
/// `Vec<ApplicationTuple<EnvInjection>>` - list of tuples containing:
/// * application id
/// * application
/// * list of pairs of bucket ids and environment variables referencing those buckets,
///   sorted by bucket id
///
/// The list is sorted by application id.
pub fn buckets_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<EnvVarInjection>> {
  let mut application_tuples: Vec<ApplicationValues<EnvVarInjection>> = vec![];
  for (application_id, application) in applications {
    let injections: Vec<EnvVarInjection> = buckets_from_application(application);
    if !injections.is_empty() {
      application_tuples.push(ApplicationValues::new(application_id, application, injections));
    }
  }
  application_tuples.sort();
  application_tuples
}

// /// # Parse bucket string
// ///
// /// # Example
// ///
// /// ```
// /// # use std::str::FromStr;
// /// use dsh_api::bucket::parse_bucket_string;
// /// assert_eq!(parse_bucket_string("{ bucket_name('my_bucket_name') }"), Ok("my_bucket_name"));
// /// ```
// ///
// /// # Parameters
// /// * `bucket_string` - the bucket string to be parsed
// ///
// /// # Returns
// /// When the provided string is valid, the method returns the bucket name
// pub fn parse_bucket_string(bucket_string: &str) -> Result<&str, String> {
//   parse_function1(bucket_string, "bucket_name")
// }
//
// #[test]
// fn test_parse_bucket_string() {
//   assert_eq!(parse_bucket_string("{ bucket_name('my_bucket_name') }"), Ok("my_bucket_name"));
// }
