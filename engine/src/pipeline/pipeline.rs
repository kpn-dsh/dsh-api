#![allow(clippy::module_inception)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::pipeline::PipelineId;
use crate::processor::{JunctionIdentifier, ParameterId, ProcessorId, ProcessorIdentifier};
use crate::resource::ResourceIdentifier;
use crate::ProfileId;

pub struct Pipeline {
  id: PipelineId,
  resources: Vec<PipelineResource>,
  processors: Vec<PipelineProcessor>,
  junctions: Vec<PipelineJunction>,
  dependencies: Vec<PipelineDependency>,
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
