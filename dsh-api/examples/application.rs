#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::query_processor::{parts_to_ansi_formatted_string, Part, RegexQueryProcessor};
use dsh_api::types::{AllocationStatus, Application, TaskStatus};
use dsh_api::Injection;
use std::collections::HashMap;

const APPLICATION_ID: &str = "my-application";
const TASK_ID: &str = "974cf8b68-smlmg-00000000";
const SECRET: &str = "backend_password";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("find_application_ids_with_derived_tasks");
  let mut applications_with_tasks: Vec<String> = client.get_task_ids().await?;
  applications_with_tasks.sort();
  println!("{}", applications_with_tasks.len());
  for application_id in applications_with_tasks {
    println!("{}", application_id);
  }

  print_header("get_application");
  let application: Application = client.get_application_configuration(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, application);

  print_header("get_application_actual_configuration");
  let application: Application = client.get_application_actual(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, application);

  print_header("get_applications_actual");
  let applications_actual: HashMap<String, Application> = client.get_application_actual_map().await?;
  println!("{}", applications_actual.len());
  for (application_id, application) in applications_actual {
    println!("{} -> {}", application_id, application);
  }

  print_header("get_application_allocation_status");
  let allocation_status: AllocationStatus = client.get_application_status(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, allocation_status);

  print_header("get_application_task");
  let task_status: TaskStatus = client.get_task(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, task_status);

  print_header("get_application_task_allocation_status");
  let allocation_status: AllocationStatus = client.get_task_status(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, allocation_status);

  use dsh_api::types::Task;
  print_header("get_application_task_state");
  let task: Task = client.get_task_actual(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, task);

  print_header("get_applications");
  let applications: HashMap<String, Application> = client.get_application_configuration_map().await?;
  println!("{}", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  print_header("list_application_derived_task_ids");
  let mut application_task_ids: Vec<String> = client.get_task_appid_ids(APPLICATION_ID).await?;
  application_task_ids.sort();
  println!("{} -> {}", APPLICATION_ID, application_task_ids.len());
  for application_task_id in application_task_ids {
    println!("{}", application_task_id);
  }

  print_header("find_applications");
  let predicate = |application: &Application| application.needs_token;
  let applications: Vec<(String, Application)> = client.find_applications(&predicate).await?;
  println!("{} applications need token", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  print_header("find_applications_with_secret_injection");
  let applications: Vec<(String, Application, Vec<Injection>)> = client.find_applications_with_secrets(SECRET).await?;
  println!("{} applications have secret injection for secret '{}'", applications.len(), SECRET);
  for (application_id, application, envs) in applications {
    println!(
      "{} -> {} -> {}",
      application_id,
      application.cpus,
      envs.iter().map(|inj| inj.to_string()).collect::<Vec<_>>().join(", ")
    )
  }

  print_header("list_application_allocation_statuses");
  let applications: Vec<(String, AllocationStatus)> = client.list_application_allocation_statuses().await?;
  println!("{}", applications.len());
  for (application_id, allocation_status) in applications {
    println!("{} -> {}", application_id, allocation_status);
  }

  print_header("list_application_ids");
  let application_ids: Vec<String> = client.list_application_ids().await?;
  println!("{}", application_ids.len());
  for application_id in application_ids {
    println!("{}", application_id);
  }

  print_header("list_applications");
  let applications: Vec<(String, Application)> = client.list_applications().await?;
  println!("{}", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  print_header("list_applications_with_secret_injections");
  let applications: Vec<(String, Application, Vec<(String, Vec<Injection>)>)> = client.list_applications_with_secrets().await?;
  println!("{}", applications.len());
  for (application_id, application, secrets) in applications {
    println!("{} -> {}", application_id, application);
    for (secret_id, envs) in secrets {
      println!("  {} -> [{}]", secret_id, envs.iter().map(|inj| inj.to_string()).collect::<Vec<_>>().join(", "));
    }
  }

  print_header("list_applications_that_match_a_query");
  let query_processor = RegexQueryProcessor::create("level").unwrap();
  let applications: Vec<(String, Application, Vec<(String, Vec<Part>)>)> = client.find_applications_that_use_env_value(&query_processor).await?;
  for (application_id, application, matches) in applications {
    println!("{} -> {}", application_id, application.cpus);
    for (key, parts) in matches {
      println!("  {} -> {}", key, parts_to_ansi_formatted_string(&parts));
    }
  }

  print_header("find_applications");
  let predicate = |application: &Application| application.needs_token;
  let applications = client.find_applications(&predicate).await?;
  for (application_id, _) in applications {
    println!("{}", application_id);
  }
  Ok(())
}
