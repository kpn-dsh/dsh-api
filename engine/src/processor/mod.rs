use std::fmt::{Display, Formatter};
use std::ops::Deref;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{config_dir_name, identifier};

pub mod dsh_service;
pub mod processor;
pub mod processor_config;
pub mod processor_descriptor;
pub mod processor_registry;

#[derive(Clone, Debug, Deserialize, Hash, Eq, PartialEq, Serialize)]
pub enum ProcessorType {
  // #[serde(rename = "dsh-app")]
  // DshApp,
  #[serde(rename = "dsh-service")]
  DshService,
}

identifier!(
  "processor",
  JunctionId,
  "junction identifier",
  "^[a-z][a-z0-9-]{0,39}$",
  "valid-junction-id",
  "invalid_junction_id"
);
identifier!(
  "processor",
  ParameterId,
  "parameter identifier",
  "^[a-z][a-z0-9-]{0,39}$",
  "valid-parameter-id",
  "invalid_parameter_id"
);
identifier!(
  "processor",
  ProcessorId,
  "processor identifier",
  "^[a-z][a-z0-9-]{0,39}$",
  "valid-processor-id",
  "invalid_processor_id"
);
identifier!("processor", ProcessorName, "processor name", "^[a-z][a-z0-9]{0,17}$", "validname", "invalid-name");
identifier!(
  "processor",
  ProfileId,
  "profile identifier",
  "^[a-z][a-z0-9-]{0,39}$",
  "valid-profile-id",
  "invalid_profile_id"
);
identifier!(
  "processor",
  ServiceName,
  "service name",
  "^[a-z][a-z0-9]{0,17}(-[a-z][a-z0-9]{0,17})?$",
  "validname-validname",
  "validname_validname"
);
identifier!(
  "processor",
  TaskId,
  "task identifier",
  "^[a-z0-9-._]{1,32}$",
  "84db5b4b79-6bgtl-00000000",
  "invalid task id"
);

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ProcessorIdentifier {
  pub processor_type: ProcessorType,
  pub id: ProcessorId,
}

impl Display for ProcessorType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      ProcessorType::DshService => write!(f, "dsh-service"),
    }
  }
}

impl ProcessorType {
  fn description(&self) -> &str {
    match self {
      ProcessorType::DshService => "DSH service managed by the DSH platform",
    }
  }

  fn label(&self) -> &str {
    match self {
      ProcessorType::DshService => "DSH Service",
    }
  }
}

impl Display for ProcessorIdentifier {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", &self.id, &self.processor_type)
  }
}

pub(crate) fn processor_config_dir_name() -> String {
  format!("{}/processors", config_dir_name())
}
