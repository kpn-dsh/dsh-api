use std::collections::HashMap;

use async_trait::async_trait;
use clap::ArgMatches;
use lazy_static::lazy_static;

use dsh_api::dsh_api_client::DshApiClient;
use dsh_api::types::Application;

use crate::capability::{Capability, CapabilityType, CommandExecutor, DeclarativeCapability};
use crate::flags::FlagType;
use crate::formatters::formatter::StringTableBuilder;
use crate::subject::Subject;
use crate::{DcliContext, DcliResult};

const TRIFONIUS_PIPELINE_ID: &str = "TRIFONIUS_PIPELINE_ID";
const TRIFONIUS_PROCESSOR_REALIZATION_ID: &str = "TRIFONIUS_PROCESSOR_REALIZATION_ID";
const TRIFONIUS_PROCESSOR_ID: &str = "TRIFONIUS_PROCESSOR_ID";
const TRIFONIUS_PROCESSOR_TECHNOLOGY: &str = "TRIFONIUS_PROCESSOR_TECHNOLOGY";
const TRIFONIUS_SERVICE_NAME: &str = "TRIFONIUS_SERVICE_NAME";

pub(crate) struct ProcessorSubject {}

const PROCESSOR_SUBJECT_TARGET: &str = "processor";

lazy_static! {
  pub static ref PROCESSOR_SUBJECT: Box<dyn Subject + Send + Sync> = Box::new(ProcessorSubject {});
}

#[async_trait]
impl Subject for ProcessorSubject {
  fn subject(&self) -> &'static str {
    PROCESSOR_SUBJECT_TARGET
  }

  fn subject_first_upper(&self) -> &'static str {
    "Processor"
  }

  fn subject_command_about(&self) -> String {
    "Show, manage and list Trifonius processors.".to_string()
  }

  fn subject_command_long_about(&self) -> String {
    "Show, manage and list Trifonius processors.".to_string()
  }

  fn subject_command_name(&self) -> &str {
    self.subject()
  }

  fn subject_command_alias(&self) -> Option<&str> {
    Some("p")
  }

  fn capabilities(&self) -> HashMap<CapabilityType, &(dyn Capability + Send + Sync)> {
    let mut capabilities: HashMap<CapabilityType, &(dyn Capability + Send + Sync)> = HashMap::new();
    capabilities.insert(CapabilityType::List, PROCESSORS_LIST_CAPABILITY.as_ref());
    capabilities
  }
}

lazy_static! {
  pub static ref PROCESSORS_LIST_CAPABILITY: Box<(dyn Capability + Send + Sync)> = Box::new(DeclarativeCapability {
    capability_type: CapabilityType::List,
    command_about: "List Trifonius processors".to_string(),
    command_long_about: Some("Lists all available Trifonius processors.".to_string()),
    command_after_help: None,
    command_after_long_help: None,
    command_executors: vec![(FlagType::All, &ListAll {}, None),],
    default_command_executor: Some(&ListAll {}),
    run_all_executors: true,
    extra_arguments: vec![],
    extra_flags: vec![],
  });
}

struct ListAll {}

#[async_trait]
impl CommandExecutor for ListAll {
  async fn execute(&self, _: Option<String>, _: Option<String>, _: &ArgMatches, context: &DcliContext, dsh_api_client: &DshApiClient<'_>) -> DcliResult {
    if context.show_capability_explanation() {
      println!("list all Trifonius processors");
    }
    let applications = dsh_api_client.get_application_configurations().await?;
    let mut builder = StringTableBuilder::new(
      &["application", "pipeline", "processor", "type", "processor id", "service name", "ports", "cpus", "mem", "#", "user", "metrics"],
      context,
    );
    for (application_id, application) in applications {
      if let Some(trifonius_parameters) = find_trifonius_parameters(&application) {
        let parameters = vec![
          application_id,
          trifonius_parameters.get(TRIFONIUS_PIPELINE_ID).cloned().unwrap_or("-".to_string()),
          trifonius_parameters.get(TRIFONIUS_PROCESSOR_ID).cloned().unwrap_or("-".to_string()),
          trifonius_parameters.get(TRIFONIUS_PROCESSOR_TECHNOLOGY).cloned().unwrap_or("-".to_string()),
          trifonius_parameters.get(TRIFONIUS_PROCESSOR_REALIZATION_ID).cloned().unwrap_or("-".to_string()),
          trifonius_parameters.get(TRIFONIUS_SERVICE_NAME).cloned().unwrap_or("-".to_string()),
          application.exposed_ports.keys().map(|k| k.to_string()).collect::<Vec<String>>().join(","),
          application.cpus.to_string(),
          application.mem.to_string(),
          application.instances.to_string(),
          application.user,
          application.metrics.clone().map(|m| format!("{}:{}", m.path, m.port)).unwrap_or_default(),
        ];
        builder.vec(&parameters);
      }
    }
    builder.print_list();
    Ok(false)
  }
}

fn find_trifonius_parameters(application: &Application) -> Option<HashMap<&'static str, String>> {
  let mut parameters: HashMap<&'static str, String> = HashMap::new();
  if let Some(pipeline_id) = application.env.get(TRIFONIUS_PIPELINE_ID) {
    parameters.insert(TRIFONIUS_PIPELINE_ID, pipeline_id.to_string());
  }
  if let Some(processor_realization) = application.env.get(TRIFONIUS_PROCESSOR_REALIZATION_ID) {
    parameters.insert(TRIFONIUS_PROCESSOR_REALIZATION_ID, processor_realization.to_string());
  }
  if let Some(processor_id) = application.env.get(TRIFONIUS_PROCESSOR_ID) {
    parameters.insert(TRIFONIUS_PROCESSOR_ID, processor_id.to_string());
  }
  if let Some(processor_technology) = application.env.get(TRIFONIUS_PROCESSOR_TECHNOLOGY) {
    parameters.insert(TRIFONIUS_PROCESSOR_TECHNOLOGY, processor_technology.to_string());
  }
  if let Some(service_name) = application.env.get(TRIFONIUS_SERVICE_NAME) {
    parameters.insert(TRIFONIUS_SERVICE_NAME, service_name.to_string());
  }
  if parameters.is_empty() {
    None
  } else {
    Some(parameters)
  }
}