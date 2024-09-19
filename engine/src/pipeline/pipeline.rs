#![allow(clippy::module_inception)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::pipeline::pipeline_instance::PipelineStatus;
use crate::pipeline::PipelineId;
use crate::processor::{JunctionId, JunctionIdentifier, ParameterId, ProcessorId, ProcessorIdentifier};
use crate::resource::ResourceIdentifier;
use crate::ProfileId;

pub struct Pipeline {
  id: PipelineId,
  resources: Vec<PipelineResource>,
  processors: Vec<PipelineProcessor>,
  junctions: Vec<PipelineJunction>,
  dependencies: Vec<PipelineDependency>,
}

impl Pipeline {
  /// # Deploy this `PipelineInstance`
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
    _inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    _outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    _deploy_parameters: &HashMap<ParameterId, String>,
    _profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<(), String> {
    todo!()
  }

  /// # Dry-run for deployment of this `PipelineInstance`
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
    _inbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    _outbound_junctions: &HashMap<JunctionId, Vec<ResourceIdentifier>>,
    _deploy_parameters: &HashMap<ParameterId, String>,
    _profile_id: Option<&ProfileId>, // TODO Move this to start() method
  ) -> Result<String, String> {
    todo!()
  }

  /// # Get the resources compatible with this `PipelineInstance`
  ///
  /// ## Parameters
  /// * `junction_id` - identifies the junction for which the compatible resources need to be
  ///                   retrieved.
  ///
  /// ## Returns
  /// * `Ok<Vec<ResourceIdentifier>` - list of identifiers of compatible resources.
  /// * `Err(msg)`                   - when the list could not be composed.
  async fn compatible_resources(&self, _junction_id: &JunctionId) -> Result<Vec<ResourceIdentifier>, String> {
    todo!()
  }

  /// # Returns the pipeline id of this `PipelineInstance`
  ///
  /// ## Returns
  /// * The `PipelineId` of this `PipelineInstance`.
  fn pipeline_id(&self) -> &PipelineId {
    todo!()
  }

  /// # Start this `PipelineInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the start request was successfully sent.
  /// * `Ok<false>` - when no pipeline instance with `pipeline_id` exists.
  /// * `Err(msg)`  - when the start request could not be sent.
  async fn start(&self) -> Result<bool, String> {
    todo!()
  }

  /// # Get this `PipelineInstance`s status
  ///
  /// ## Returns
  /// * `Ok<PipelineStatus>` - signals whether the pipeline instance  with the given
  ///                           `pipeline` is active or not.
  /// * `Err(msg)`            - when the status request could not be sent.
  async fn status(&self) -> Result<PipelineStatus, String> {
    todo!()
  }

  /// # Stop this `PipelineInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the stop request was successfully sent.
  /// * `Ok<false>` - when no pipeline instance with `pipeline_id` exists.
  /// * `Err(msg)`  - when the stop request could not be sent.
  async fn stop(&self) -> Result<bool, String> {
    todo!()
  }

  /// # Undeploy this `PipelineInstance`
  ///
  /// ## Returns
  /// * `Ok<true>`  - when the undeployment request was successfully sent.
  /// * `Ok<false>` - when no pipeline instance with `pipeline_id` exists.
  /// * `Err(msg)`  - when the undeployment request could not be sent.
  async fn undeploy(&self) -> Result<bool, String> {
    todo!()
  }
}

pub struct PipelineResource {
  resource: ResourceIdentifier,
  parameters: HashMap<ParameterId, String>,
}

pub struct PipelineProcessor {
  processor: ProcessorIdentifier,
  name: ProcessorId,
  parameters: HashMap<ParameterId, String>,
  profile_id: Option<ProfileId>,
}

pub struct PipelineJunction {
  junction: JunctionType,
  parameters: HashMap<ParameterId, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum JunctionType {
  ResourceToProcessor { source: Vec<ResourceIdentifier>, target: JunctionIdentifier },
  ProcessorToResource { source: JunctionIdentifier, target: Vec<ResourceIdentifier> },
  ProcessorToProcessor { source: JunctionIdentifier, target: JunctionIdentifier },
}

struct PipelineDependency {
  parameters: HashMap<ParameterId, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum DependencyType {
  ProcessorOnProcessor { depended: ProcessorIdentifier, depends_on: ProcessorIdentifier },
  ProcessorOnResource { depended: JunctionIdentifier, depends_on: Vec<ResourceIdentifier> },
  ResourceOnProcessor { depended: ProcessorIdentifier, depends_on: ResourceIdentifier },
}
