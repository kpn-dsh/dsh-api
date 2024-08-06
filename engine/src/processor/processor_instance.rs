//! # Defines the behavior of a Trifonius `Processor`
//!

#![allow(clippy::module_inception)]

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use async_trait::async_trait;

use crate::pipeline::PipelineName;
use crate::processor::processor_realization::ProcessorRealization;
use crate::processor::{JunctionId, ParameterId, ProcessorName, ProfileId};
use crate::resource::ResourceIdentifier;

/// Defines the behavior of a Trifonius `ProcessorInstance`
#[async_trait]
pub trait ProcessorInstance {
  /// # Deploy this `ProcessorInstance`
  ///
  /// ## Parameters
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
    inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    deploy_parameters: &HashMap<ParameterId, String>,
    profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<(), String>;

  /// # Dry-run for deployment of this `ProcessorInstance`
  ///
  /// This method does everything that the regular `deploy()` method does,
  /// except for the actual deployment to the target platform.
  /// Instead, it returns the configuration that would be used if the deployment would be real.
  ///
  /// ## Parameters
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
    inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    deploy_parameters: &HashMap<ParameterId, String>,
    profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<String, String>;

  /// # Get the resources compatible with this `ProcessorInstance`
  ///
  /// ## Parameters
  /// * `junction_id` - identifies the junction for which the compatible resources need to be
  ///                   retrieved.
  ///
  /// ## Returns
  /// * `Ok<Vec<ResourceIdentifier>` - list of identifiers of compatible resources.
  /// * `Err(msg)`                   - when the list could not be composed.
  async fn compatible_resources(&self, junction_id: &JunctionId) -> Result<Vec<ResourceIdentifier>, String>;

  /// # Returns the pipeline name of this `ProcessorInstance`
  ///
  /// ## Returns
  /// * The optional `PipelineName` of this `ProcessorInstance`.
  fn pipeline_name(&self) -> Option<&PipelineName>;

  /// # Returns the processor name of this `ProcessorInstance`
  ///
  /// ## Returns
  /// * The `ProcessorName` of this `ProcessorInstance`.
  fn processor_name(&self) -> &ProcessorName;

  /// # Returns the `ProcessorRealizaton` for this `ProcessorInstance`
  ///
  /// ## Returns
  /// * The `ProcessorRealization` for this `ProcessorInstance`.
  fn processor_realization(&self) -> &dyn ProcessorRealization;

  /// # Start this `ProcessorInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the start request was successfully sent.
  /// * `Ok<false>` - when no processor instance with `service_id` exists.
  /// * `Err(msg)`  - when the start request could not be sent.
  async fn start(&self) -> Result<bool, String>;

  /// # Get this `ProcessorInstance`s status
  ///
  /// ## Returns
  /// * `Ok<ProcessorStatus>` - signals whether the processor instance  with the given
  ///                           `service_name` is active or not.
  /// * `Err(msg)`            - when the status request could not be sent.
  async fn status(&self) -> Result<ProcessorStatus, String>;

  /// # Stop this `ProcessorInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the stop request was successfully sent.
  /// * `Ok<false>` - when no processor instance with `service_id` exists.
  /// * `Err(msg)`  - when the stop request could not be sent.
  async fn stop(&self) -> Result<bool, String>;

  /// # Undeploy this `ProcessorInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the undeployment request was successfully sent.
  /// * `Ok<false>` - when no processor instance with `service_id` exists.
  /// * `Err(msg)`  - when the undeployment request could not be sent.
  async fn undeploy(&self) -> Result<bool, String>;
}

#[derive(Debug)]
pub struct ProcessorStatus {
  pub deployed: bool,
  pub up: Option<bool>,
}

impl Display for ProcessorStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.deployed {
      match self.up {
        Some(up) => {
          if up {
            write!(f, "deployed:up")
          } else {
            write!(f, "deployed:down")
          }
        }
        None => write!(f, "deployed:unknown"),
      }
    } else {
      write!(f, "not-deployed")
    }
  }
}

pub fn service_name(pipeline_name: Option<&PipelineName>, processor_name: &ProcessorName) -> String {
  match pipeline_name {
    Some(pipeline_name) => format!("{}-{}", pipeline_name, processor_name),
    None => processor_name.to_string(),
  }
}
