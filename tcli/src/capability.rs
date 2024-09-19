use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command};

use trifonius_dsh_api::dsh_api_client::DshApiClient;

use crate::arguments::{query_argument, target_argument, QUERY_ARGUMENT, TARGET_ARGUMENT};
use crate::capability::CapabilityType::*;
use crate::flags::{create_flag, FlagType};
use crate::subject::Subject;
use crate::{TcliContext, TcliResult};

pub(crate) const CREATE: &str = "create";
pub(crate) const DELETE: &str = "delete";
pub(crate) const FIND: &str = "find";
pub(crate) const DIFF: &str = "diff";
pub(crate) const LIST: &str = "list";
pub(crate) const SHOW: &str = "show";
pub(crate) const START: &str = "start";
pub(crate) const STOP: &str = "stop";
pub(crate) const UPDATE: &str = "update";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) enum CapabilityType {
  Create,
  Delete,
  Diff,
  Find,
  List,
  Show,
  Start,
  Stop,
  Update,
}

pub(crate) static ALL_CAPABILITY_TYPES: [CapabilityType; 9] = [Create, Delete, Diff, Find, List, Show, Start, Stop, Update];

impl TryFrom<&str> for CapabilityType {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      CREATE => Ok(Create),
      DELETE => Ok(Delete),
      DIFF => Ok(Diff),
      FIND => Ok(Find),
      LIST => Ok(List),
      SHOW => Ok(Show),
      START => Ok(Start),
      STOP => Ok(Stop),
      UPDATE => Ok(Update),
      _ => Err(format!("unrecognized capability type {}", value)),
    }
  }
}

impl CapabilityType {
  fn command_name(&self) -> &'static str {
    match self {
      Create => CREATE,
      Delete => DELETE,
      Diff => DIFF,
      Find => FIND,
      List => LIST,
      Show => SHOW,
      Start => START,
      Stop => STOP,
      Update => UPDATE,
    }
  }

  fn command_id(&self) -> &'static str {
    self.command_name()
  }

  fn command_alias(&self) -> Option<&'static str> {
    match self {
      Create => None,
      Delete => None,
      Diff => Some("d"),
      Find => Some("f"),
      List => Some("l"),
      Show => Some("s"),
      Start => None,
      Stop => None,
      Update => None,
    }
  }

  pub(crate) fn command_target_arguments(&self, subject: &dyn Subject) -> Vec<Arg> {
    match self {
      Create => vec![target_argument(subject, None)],
      Delete => vec![target_argument(subject, None)],
      Diff => vec![target_argument(subject, None)],
      Find => vec![query_argument(None)],
      List => vec![],
      Show => vec![target_argument(subject, None)],
      Start => vec![target_argument(subject, None)],
      Stop => vec![target_argument(subject, None)],
      Update => vec![target_argument(subject, None)],
    }
  }

  pub(crate) fn command_target_argument_ids(&self) -> &[&str] {
    match self {
      Create => &[TARGET_ARGUMENT],
      Delete => &[TARGET_ARGUMENT],
      Diff => &[TARGET_ARGUMENT],
      Find => &[QUERY_ARGUMENT],
      List => &[],
      Show => &[TARGET_ARGUMENT],
      Start => &[TARGET_ARGUMENT],
      Stop => &[TARGET_ARGUMENT],
      Update => &[TARGET_ARGUMENT],
    }
  }
}

#[async_trait]
pub trait Capability {
  fn capability_type(&self) -> CapabilityType;

  fn clap_capability_command(&self, subject: &dyn Subject) -> Command;

  fn clap_flags(&self, subject: &dyn Subject) -> Vec<Arg>;

  fn long_about(&self) -> Option<String>;

  async fn execute_capability(
    &self,
    argument: Option<String>,
    sub_argument: Option<String>,
    matches: &ArgMatches,
    context: &TcliContext,
    dsh_api_client: &DshApiClient<'_>,
  ) -> TcliResult;
}

#[async_trait]
pub(crate) trait CommandExecutor {
  async fn execute(&self, argument: Option<String>, sub_argument: Option<String>, matches: &ArgMatches, context: &TcliContext, dsh_api_client: &DshApiClient<'_>) -> TcliResult;
}

pub(crate) struct DeclarativeCapability<'a> {
  pub(crate) capability_type: CapabilityType,
  pub(crate) command_about: String,
  pub(crate) command_long_about: Option<String>,
  pub(crate) command_after_help: Option<String>,
  pub(crate) command_after_long_help: Option<String>,
  pub(crate) command_executors: Vec<(FlagType, &'a (dyn CommandExecutor + Send + Sync), Option<&'a str>)>,
  pub(crate) default_command_executor: Option<&'a (dyn CommandExecutor + Send + Sync)>,
  pub(crate) run_all_executors: bool,
  pub(crate) extra_arguments: Vec<Arg>,
  pub(crate) extra_flags: Vec<Arg>,
}

#[async_trait]
impl Capability for DeclarativeCapability<'_> {
  fn capability_type(&self) -> CapabilityType {
    self.capability_type.clone()
  }

  fn clap_capability_command(&self, subject: &dyn Subject) -> Command {
    let mut capability_command = Command::new(self.capability_type().command_id())
      .name(self.capability_type.command_name())
      .about(&self.command_about)
      .args(self.capability_type.command_target_arguments(subject))
      .args(self.clap_flags(subject))
      .args(&self.extra_arguments)
      .args(&self.extra_flags);
    if let Some(alias) = self.capability_type.command_alias() {
      capability_command = capability_command.alias(alias)
    }
    if let Some(ref long_about) = self.command_long_about {
      capability_command = capability_command.long_about(long_about)
    }
    if let Some(ref after_help) = self.command_after_help {
      capability_command = capability_command.after_help(after_help)
    }
    if let Some(ref after_long_help) = self.command_after_long_help {
      capability_command = capability_command.after_long_help(after_long_help)
    }
    capability_command
  }

  fn clap_flags(&self, subject: &dyn Subject) -> Vec<Arg> {
    self
      .command_executors
      .iter()
      .map(|(flag_type, _, long_help)| create_flag(flag_type, subject, long_help))
      .collect::<Vec<Arg>>()
  }

  fn long_about(&self) -> Option<String> {
    self.command_long_about.clone()
  }

  async fn execute_capability(
    &self,
    argument: Option<String>,
    sub_argument: Option<String>,
    matches: &ArgMatches,
    context: &TcliContext,
    dsh_api_client: &DshApiClient<'_>,
  ) -> TcliResult {
    let mut last_tcli_result: Option<TcliResult> = None;
    if self.run_all_executors {
      for (flag_type, executor, _) in &self.command_executors {
        if matches.get_flag(flag_type.id()) {
          last_tcli_result = Some(executor.execute(argument.clone(), sub_argument.clone(), matches, context, dsh_api_client).await);
        }
      }
    } else {
      for (flag_type, executor, _) in &self.command_executors {
        if matches.get_flag(flag_type.id()) {
          last_tcli_result = Some(executor.execute(argument.clone(), sub_argument.clone(), matches, context, dsh_api_client).await);
          break;
        }
      }
    }
    match last_tcli_result {
      Some(tcli_result) => tcli_result,
      None => {
        if let Some(default_executor) = self.default_command_executor {
          default_executor
            .execute(argument.clone(), sub_argument.clone(), matches, context, dsh_api_client)
            .await
        } else {
          Ok(true)
        }
      }
    }
  }
}