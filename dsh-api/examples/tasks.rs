#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::{AllocationStatus, TaskStatus};

const APPLICATION: &str = "flink-cluster-jobmanager";
const TASK_ID: &str = "65b969ffbf-9d89w-00000000";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header(&format!("client.get_task('{}', '{}',)", APPLICATION, TASK_ID));
  let task: TaskStatus = client.get_task(APPLICATION, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION, TASK_ID, task);

  use dsh_api::types::Task;
  print_header(&format!("client.get_task_actual('{}', '{}')", APPLICATION, TASK_ID));
  let task_actual: Task = client.get_task_actual(APPLICATION, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION, TASK_ID, task_actual);

  print_header(&format!("client.get_task_appid_ids('{}')", APPLICATION));
  let task_appid_ids: Vec<String> = client.get_task_appid_ids(APPLICATION).await?;
  println!("{} -> {}", APPLICATION, task_appid_ids.len());
  for task_appid_id in task_appid_ids {
    println!("{}", task_appid_id);
  }

  print_header("client.get_task_ids()");
  let task_ids: Vec<String> = client.get_task_ids().await?;
  println!("{}", task_ids.len());
  for task_id in task_ids {
    println!("{}", task_id);
  }

  print_header(&format!("client.get_task_status('{}', '{}')", APPLICATION, TASK_ID));
  let task_status: AllocationStatus = client.get_task_status(APPLICATION, TASK_ID).await?;
  println!("({}, {}) -> {}", APPLICATION, TASK_ID, task_status);

  Ok(())
}
