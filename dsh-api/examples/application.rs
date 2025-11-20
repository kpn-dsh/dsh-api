#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::{AllocationStatus, Application};
use std::collections::HashMap;

const APPLICATION: &str = "eavesdropper";
const APPLICATION_NON_EXISTING: &str = "non-existing-service";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("client.guid()");
  match client.guid().await {
    Ok((gid, uid)) => println!("{}:{}", gid, uid),
    Err(error) => println!("{:?}", error),
  }

  print_header(&format!("client.delete_application_configuration('{}')", APPLICATION_NON_EXISTING));
  match client.delete_application_configuration(APPLICATION_NON_EXISTING).await {
    Ok(()) => {}
    Err(error) => println!("{:?}", error),
  }

  print_header(&format!("client.get_application_actual('{}')", APPLICATION));
  let application_actual: Application = client.get_application_actual(APPLICATION).await?;
  println!("{} -> {}", APPLICATION, application_actual);

  print_header("client.get_application_actual_map()");
  let application_actual_map: HashMap<String, Application> = client.get_application_actual_map().await?;
  println!("{}", application_actual_map.len());
  for (application_id, application) in application_actual_map {
    println!("{} -> {}", application_id, application);
  }

  print_header(&format!("client.get_application_configuration('{}')", APPLICATION));
  let application_configuration = client.get_application_configuration(APPLICATION).await?;
  println!("{} -> {}", APPLICATION, application_configuration);

  print_header("client.get_application_configuration_map()");
  let application_configuration_map = client.get_application_configuration_map().await?;
  println!("{}", application_configuration_map.len());
  for (application_id, application) in application_configuration_map {
    println!("{} -> {}", application_id, application);
  }

  print_header(&format!("client.get_application_status('{}')", APPLICATION));
  let application_status: AllocationStatus = client.get_application_status(APPLICATION).await?;
  println!("{} -> {}", APPLICATION, application_status);

  print_header(&format!("client.put_application_configuration('{}', empty)", APPLICATION_NON_EXISTING));
  let empty_application = Application::default();
  match client.put_application_configuration(APPLICATION_NON_EXISTING, &empty_application).await {
    Ok(()) => {}
    Err(error) => println!("{:?}", error),
  }

  Ok(())
}
