//! # Defines the behavior of a Trifonius `Processor`
//!

#![allow(clippy::module_inception)]

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;

use crate::pipeline::PipelineName;
use crate::processor::{JunctionId, ParameterId, ProcessorName, ProfileId, ServiceName};
use crate::resource::ResourceIdentifier;

/// Defines the behavior of a Trifonius `Processor`
#[async_trait]
pub trait Processor {
  /// # Deploy this `Processor`
  ///
  /// ## Parameters
  /// * `service_name`       - Service name of the deployed processor.
  /// * `inbound_junctions`  - Map containing the inbound resources.
  /// * `outbound_junctions` - Map containing the outbound resources.
  /// * `deploy_parameters`  - Map containing the deployment parameters.
  /// * `profile_id`         - Profile id.
  ///
  /// ## Returns
  /// * `Ok<()>`   - when the deployment request was successfully sent.
  /// * `Err(msg)` - when the deployment request could not be sent.
  async fn deploy(
    &self,
    service_name: &ServiceName,
    inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    deploy_parameters: &HashMap<ParameterId, String>,
    profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<(), String>;

  /// # Dry-run for deployment of this `Processor`
  ///
  /// This method does everything that the regular `deploy()` method does,
  /// except for the actual deployment to the target platform.
  /// Instead, it returns the configuration that would be used if the deployment would be real.
  ///
  /// ## Parameters
  /// * `service_name`       - Service name of the deployed processor.
  /// * `inbound_junctions`  - Map containing the inbound resources.
  /// * `outbound_junctions` - Map containing the outbound resources.
  /// * `deploy_parameters`  - Map containing the deployment parameters.
  /// * `profile_id`         - Profile id.
  ///
  /// ## Returns
  /// * `Ok<String>` - when the deployment request was successfully sent.
  /// * `Err(msg)`   - when the deployment request could not be sent.
  async fn deploy_dry_run(
    &self,
    service_name: &ServiceName,
    inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    deploy_parameters: &HashMap<ParameterId, String>,
    profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<String, String>;

  /// # Get the resources compatible with this `Processor`
  ///
  /// ## Parameters
  /// * `junction_id` - identifies the junction for which the compatible resources need to be
  ///                   retrieved.
  ///
  /// ## Returns
  /// * `Ok<Vec<ResourceIdentifier>` - list of identifiers of compatible resources.
  /// * `Err(msg)`                   - when the list could not be composed.
  async fn compatible_resources(&self, junction_id: &JunctionId) -> Result<Vec<ResourceIdentifier>, String>;

  /// # Start this `Processor`
  ///
  /// ## Parameters
  /// * `service_name` - Service name of the deployed processor.
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the start request was successfully sent.
  /// * `Ok<false>` - when no processor with `service_id` exists.
  /// * `Err(msg)`  - when the start request could not be sent.
  async fn start(&self, service_name: &ServiceName) -> Result<bool, String>;

  /// # Get this `Processor`s status
  ///
  /// ## Parameters
  /// * `service_name` - Service name of the deployed processor.
  ///
  /// ## Returns
  /// * `Ok<ProcessorStatus>` - signals whether the processor with the given `service_id` is active
  ///                           or not.
  /// * `Err(msg)`            - when the status request could not be sent.
  async fn status(&self, service_name: &ServiceName) -> Result<ProcessorStatus, String>;

  /// # Stop this `Processor`
  ///
  /// ## Parameters
  /// * `service_name` - Service name of the deployed processor.
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the stop request was successfully sent.
  /// * `Ok<false>` - when no processor with `service_id` exists.
  /// * `Err(msg)`  - when the stop request could not be sent.
  async fn stop(&self, service_name: &ServiceName) -> Result<bool, String>;

  /// # Undeploy this `Processor`
  ///
  /// ## Parameters
  /// * `service_name` - Service name of the deployed processor.
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the undeployment request was successfully sent.
  /// * `Ok<false>` - when no processor with `service_id` exists.
  /// * `Err(msg)`  - when the undeployment request could not be sent.
  async fn undeploy(&self, service_name: &ServiceName) -> Result<bool, String>;
}

#[derive(Debug)]
pub struct ProcessorStatus {
  pub up: bool,
}

impl Display for ProcessorStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.up {
      write!(f, "up")
    } else {
      write!(f, "down")
    }
  }
}

pub fn service_name(pipeline_name: &PipelineName, processor_name: &ProcessorName) -> String {
  format!("{}-{}", pipeline_name, processor_name)
}
