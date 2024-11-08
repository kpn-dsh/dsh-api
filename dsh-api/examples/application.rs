use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api_generated::types::{AllocationStatus, Application, Task, TaskStatus};
use std::collections::HashMap;

// create_application
// delete_application
// find_application_ids_with_derived_tasks *
// get_application *
// get_application_actual_configuration *
// get_application_actual_configurations *
// get_application_allocation_status *
// get_application_task *
// get_application_task_allocation_status *
// get_application_task_state *
// get_applications *
// list_application_derived_task_ids *

// find_applications
// find_applications_that_use_secret
// list_application_allocation_statuses
// list_application_ids
// list_applications
// list_applications_with_secret_injections

const APPLICATION_ID: &str = "keyring-dev";
const TASK_ID: &str = "974cf8b68-8glkc-00000000";
const SECRET: &str = "greenbox_backend_password";

#[tokio::main]
async fn main() -> Result<(), String> {
  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  // create_application

  // delete_application

  // find_application_ids_with_derived_tasks
  println!("\n---------------------------------------");
  println!("find_application_ids_with_derived_tasks");
  println!("---------------------------------------");
  let mut applications_with_tasks: Vec<String> = client.find_application_ids_with_derived_tasks().await?;
  applications_with_tasks.sort();
  println!("{}", applications_with_tasks.len());
  for application_id in applications_with_tasks {
    println!("{}", application_id);
  }

  // get_application
  println!("\n---------------");
  println!("get_application");
  println!("---------------");
  let application: Application = client.get_application(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, application);

  // get_application_actual_configuration
  println!("\n------------------------------------");
  println!("get_application_actual_configuration");
  println!("------------------------------------");
  let application: Application = client.get_application_actual_configuration(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, application);

  // get_application_actual_configurations
  println!("\n-------------------------------------");
  println!("get_application_actual_configurations");
  println!("-------------------------------------");
  let applications_actual: HashMap<String, Application> = client.get_application_actual_configurations().await?;
  println!("{}", applications_actual.len());
  for (application_id, application) in applications_actual {
    println!("{} -> {}", application_id, application);
  }

  // get_application_allocation_status
  println!("\n---------------------------------");
  println!("get_application_allocation_status");
  println!("---------------------------------");
  let allocation_status: AllocationStatus = client.get_application_allocation_status(APPLICATION_ID).await?;
  println!("{} -> {}", APPLICATION_ID, allocation_status);

  // get_application_task
  println!("\n--------------------");
  println!("get_application_task");
  println!("--------------------");
  let task_status: TaskStatus = client.get_application_task(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, task_status);

  // get_application_task_allocation_status
  println!("\n--------------------------------------");
  println!("get_application_task_allocation_status");
  println!("--------------------------------------");
  let allocation_status: AllocationStatus = client.get_application_task_allocation_status(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, allocation_status);

  // get_application_task_state
  println!("\n--------------------------");
  println!("get_application_task_state");
  println!("--------------------------");
  let task: Task = client.get_application_task_state(APPLICATION_ID, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION_ID, TASK_ID, task);

  // get_applications
  println!("\n----------------");
  println!("get_applications");
  println!("----------------");
  let applications: HashMap<String, Application> = client.get_applications().await?;
  println!("{}", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  // list_application_derived_task_ids
  println!("\n---------------------------------");
  println!("list_application_derived_task_ids");
  println!("---------------------------------");
  let mut application_task_ids: Vec<String> = client.list_application_derived_task_ids(APPLICATION_ID).await?;
  application_task_ids.sort();
  println!("{} -> {}", APPLICATION_ID, application_task_ids.len());
  for application_task_id in application_task_ids {
    println!("{}", application_task_id);
  }

  // find_applications
  println!("\n-----------------");
  println!("find_applications");
  println!("-----------------");
  let applications: Vec<(String, Application)> = client.find_applications(&|application| application.needs_token).await?;
  println!("{} applications need token", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  // find_applications_with_secret_injection
  println!("\n---------------------------------------");
  println!("find_applications_with_secret_injection");
  println!("---------------------------------------");
  let applications: Vec<(String, Application, Vec<String>)> = client.find_applications_with_secret_injection(SECRET).await?;
  println!("{} applications have secret injection for secret '{}'", applications.len(), SECRET);
  for (application_id, application, envs) in applications {
    println!("{} -> {} -> {}", application_id, application.cpus, envs.join(", "))
  }

  // list_application_allocation_statuses
  println!("\n------------------------------------");
  println!("list_application_allocation_statuses");
  println!("------------------------------------");
  let applications: Vec<(String, AllocationStatus)> = client.list_application_allocation_statuses().await?;
  println!("{}", applications.len());
  for (application_id, allocation_status) in applications {
    println!("{} -> {}", application_id, allocation_status);
  }

  // list_application_ids
  println!("\n--------------------");
  println!("list_application_ids");
  println!("--------------------");
  let application_ids: Vec<String> = client.list_application_ids().await?;
  println!("{}", application_ids.len());
  for application_id in application_ids {
    println!("{}", application_id);
  }

  // list_applications
  println!("\n-----------------");
  println!("list_applications");
  println!("-----------------");
  let applications: Vec<(String, Application)> = client.list_applications().await?;
  println!("{}", applications.len());
  for (application_id, application) in applications {
    println!("{} -> {}", application_id, application);
  }

  // list_applications_with_secret_injections
  println!("\n----------------------------------------");
  println!("list_applications_with_secret_injections");
  println!("----------------------------------------");
  let applications: Vec<(String, Application, Vec<(String, Vec<String>)>)> = client.list_applications_with_secret_injections().await?;
  println!("{}", applications.len());
  for (application_id, application, secrets) in applications {
    println!("{} -> {}", application_id, application);
    for (secret_id, envs) in secrets {
      println!("  {} -> [{}]", secret_id, envs.join(", "));
    }
  }

  Ok(())
}
