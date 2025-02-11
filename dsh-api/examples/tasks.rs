use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::AllocationStatus;

const SERVICE_ID: &str = "consentfilter-test002";
const TASK_ID: &str = "8f4b5747-lnmj4-00000000";

#[tokio::main]
async fn main() -> Result<(), String> {
  env_logger::init();

  let application_id = SERVICE_ID;
  let task_id = TASK_ID;

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  // Return applications that have derived tasks
  let applications: Vec<String> = client.get_task_ids().await?;
  println!("applications with tasks\n{}", serde_json::to_string_pretty(&applications).unwrap());

  // Return task ids
  let tasks: Vec<String> = client.get_task_appid_ids(&application_id).await?;
  println!("task ids {}\n{}", application_id, serde_json::to_string_pretty(&tasks).unwrap());

  // Return task allocation status
  let allocation_status: AllocationStatus = client.get_task_status(&application_id, &task_id).await?;
  println!(
    "task allocation status {}, {}\n{}",
    application_id,
    task_id,
    serde_json::to_string_pretty(&allocation_status).unwrap()
  );

  Ok(())
}
