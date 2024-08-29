use trifonius_engine::processor::JunctionId;

use crate::common::default_dshservice_instance;

#[path = "common.rs"]
mod common;

#[tokio::main]
async fn main() -> Result<(), String> {
  let dshservice_instance = default_dshservice_instance();

  let inbound_kafka_topic = JunctionId::new("inbound-kafka-topic");
  let inbound_compatible_resources = dshservice_instance.compatible_resources(&inbound_kafka_topic).await;
  println!(
    "{}",
    serde_json::to_string_pretty(&inbound_compatible_resources).map_err(|error| error.to_string())?
  );

  let outbound_kafka_topic = JunctionId::new("outbound-kafka-topic");
  let outbound_compatible_resources = dshservice_instance.compatible_resources(&outbound_kafka_topic).await;
  println!("{}", serde_json::to_string_pretty(&outbound_compatible_resources).unwrap());

  Ok(())
}
