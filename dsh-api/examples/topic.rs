#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::topic::TopicInjection;
use dsh_api::Dependant;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let topic_id = "greenbox-training";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("topic_dependants");
  let topic_dependants: Vec<Dependant<TopicInjection>> = client.topic_dependants(topic_id).await?;
  for topic_dependant in topic_dependants {
    println!("{}", topic_dependant);
  }

  Ok(())
}
