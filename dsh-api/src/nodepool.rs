//! # Additional methods to manage nodepools
//!
//! Module that contains methods and functions to manage nodepools.
//!
//! # Generated methods
//!
//! [`DshApiClient`] methods that are generated from the `openapi` specification.
//!
//! * [`get_nodepool_actual(id) -> NodepoolActual`](DshApiClient::get_nodepool_actual)
//! * [`get_nodepool_ids() -> [id]`](DshApiClient::get_nodepool_ids)
//! * [`get_nodepool_status(id) -> AllocationStatus`](DshApiClient::get_nodepool_status)
//!
//! # Derived methods
//!
//! [`DshApiClient`] methods that add extra capabilities but do not directly call the
//! DSH resource management API. These derived methods depend on the API methods for this.
//!
//! * [`nodepools() -> [(id, nodepool)]`](DshApiClient::nodepools)
//! * [`nodepool_with_dependant_applications(id) -> (nodepool, [dependant])`](DshApiClient::nodepool_with_dependant_applications)
//! * [`nodepools_with_dependant_applications() -> [(id, nodepool, [dependant])]`](DshApiClient::nodepools_with_dependant_applications)

use crate::application_types::ApplicationValues;
use crate::dsh_api_client::DshApiClient;
use crate::error::DshApiResult;
use crate::types::{Application, NodeFeatures, NodepoolActual};
use crate::DependantApplication;
#[allow(unused_imports)]
use crate::DshApiError;
use futures::future::try_join_all;
use futures::try_join;
use itertools::Itertools;
use std::collections::HashMap;

/// # Additional methods to manage nodepools
///
/// Module that contains methods to manage nodepools.
/// * Derived methods - DshApiClient methods that add extra capabilities
///   but depend on the API methods.
///
/// # Derived methods
///
/// [`DshApiClient`] methods that add extra capabilities but do not directly call the
/// DSH resource management API. These derived methods depend on the API methods for this.
///
/// * [`nodepools() -> [(id, nodepool)]`](DshApiClient::nodepools)
/// * [`nodepool_with_dependant_applications(id) -> (nodepool, [dependant])`](DshApiClient::nodepool_with_dependant_applications)
/// * [`nodepools_with_dependant_applications() -> [(id, nodepool, [dependant])]`](DshApiClient::nodepools_with_dependant_applications)
impl DshApiClient {
  /// # Get all nodepools
  ///
  /// Returns all nodepool ids and configurations.
  ///
  /// # Returns
  /// * `Ok<(String, NodepoolActual)>` - Nodepool ids and configurations, sorted alphabetically.
  /// * `Err<`[`DshApiError`]`>` - When the request could not be processed by the DSH.
  pub async fn nodepools(&self) -> DshApiResult<Vec<(String, NodepoolActual)>> {
    let nodepool_ids = self.get_nodepool_ids().await?;
    let nodepool_actuals = try_join_all(nodepool_ids.iter().map(|nodepool_id| self.get_nodepool_actual(nodepool_id))).await?;
    Ok(nodepool_ids.into_iter().zip(nodepool_actuals).collect_vec())
  }

  /// # Get nodepool with usage dependant applications
  ///
  /// Returns the configuration of a given nodepool with the ids and instance numbers of the
  /// applications that depend on the nodepool.
  ///
  /// # Parameters
  /// * `nodepool_id` - Identifies the requested nodepool.
  ///
  /// # Returns
  /// * `Ok<(NodepoolActual, Vec<DependantApplication<()>>)>` - Tuple containing the nodepool
  ///   configuration and the dependant applications.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn nodepool_with_dependant_applications(&self, nodepool_id: &str) -> DshApiResult<(NodepoolActual, Vec<DependantApplication<NodeFeatures>>)> {
    let (nodepool_actual, applications) = try_join!(self.get_nodepool_actual(nodepool_id), self.get_application_configuration_map(),)?;
    let mut dependants: Vec<DependantApplication<NodeFeatures>> = vec![];
    for ApplicationValues { id, application, values } in nodepool_from_applications(nodepool_id, &applications) {
      dependants.push(DependantApplication::new(
        id.to_string(),
        application.instances,
        values.into_iter().cloned().collect_vec(),
      ));
    }
    Ok((nodepool_actual, dependants))
  }

  /// # Returns all nodepools with their dependant applications
  ///
  /// Returns a sorted list of all nodepool ids, nodepool configurations and lists of applications
  /// that use them.
  ///
  /// # Returns
  /// * `Ok<(NodepoolActual, Vec<DependantApplication<()>>)>` - Tuple containing the nodepool ids,
  ///   configurations and dependant applications.
  /// * `Err<`[`DshApiError`]`>` - when the request could not be processed by the DSH
  pub async fn nodepools_with_dependant_applications(&self) -> DshApiResult<Vec<(String, NodepoolActual, Vec<DependantApplication<NodeFeatures>>)>> {
    let (nodepools, applications): (Vec<(String, NodepoolActual)>, HashMap<String, Application>) = try_join!(self.nodepools(), self.get_application_configuration_map())?;
    let mut nodepools_tuples = Vec::<(String, NodepoolActual, Vec<DependantApplication<NodeFeatures>>)>::new();
    for (nodepool_id, nodepool_actual) in nodepools {
      let mut dependant_applications: Vec<DependantApplication<NodeFeatures>> = vec![];
      for ApplicationValues { id, application, values } in nodepool_from_applications(&nodepool_id, &applications) {
        dependant_applications.push(DependantApplication::new(
          id.to_string(),
          application.instances,
          values.into_iter().cloned().collect_vec(),
        ));
      }
      nodepools_tuples.push((nodepool_id, nodepool_actual, dependant_applications));
    }
    Ok(nodepools_tuples)
  }
}

/// # Get nodepool from applications
///
/// Find nodepool configurations in all `Applications`.
///
/// # Parameters
/// * `nodepool_id` - Nodepool to look for.
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// `Vec<ApplicationValues<&NodeFeatures>>` - List of tuples containing:
/// * Application id,
/// * Application reference,
/// * Single item list containing the node features.
///
/// The list is sorted by application id.
pub fn nodepool_from_applications<'a>(nodepool_id: &str, applications: &'a HashMap<String, Application>) -> Vec<ApplicationValues<'a, &'a NodeFeatures>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| match &application.node_features {
      Some(node_features) => {
        if node_features.nodepool == nodepool_id {
          Some(ApplicationValues::new(application_id, application, vec![node_features]))
        } else {
          None
        }
      }
      None => None,
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}

/// # Get all nodepools from applications
///
/// Get all nodepools from all `Applications`.
///
/// # Parameters
/// * `applications` - Hashmap containing id/application pairs.
///
/// # Returns
/// `Vec<ApplicationValues<(&str, Path)>>` - list of tuples containing:
/// * Application id,
/// * Application reference,
/// * Single item list containing the node features.
///
/// The list is sorted by application id.
pub fn nodepools_from_applications(applications: &HashMap<String, Application>) -> Vec<ApplicationValues<&NodeFeatures>> {
  let mut application_tuples = applications
    .iter()
    .filter_map(|(application_id, application)| match &application.node_features {
      Some(node_features) => Some(ApplicationValues::new(application_id, application, vec![node_features])),
      None => None,
    })
    .collect_vec();
  application_tuples.sort();
  application_tuples
}
