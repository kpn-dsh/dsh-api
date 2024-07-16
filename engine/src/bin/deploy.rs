use std::collections::HashMap;

use trifonius_engine::processor::application::application_registry::ApplicationRegistry;
use trifonius_engine::processor::application::DEFAULT_TARGET_CLIENT_FACTOR;
use trifonius_engine::processor::processor::ProcessorDeployParameters;

const SERVICE_ID: &str = "test-0-0-2";

#[tokio::main]
async fn main() {
  let registry = ApplicationRegistry::create(&DEFAULT_TARGET_CLIENT_FACTOR).unwrap();
  let application = registry.application_by_id("greenbox-consent-filter").unwrap();
  let deploy_parameters = ProcessorDeployParameters {
    inbound_junctions: &HashMap::from([("inbound-kafka-topic".to_string(), "stream.reference-implementation-3p.greenbox-dev".to_string())]),
    outbound_junctions: &HashMap::from([(
      "outbound-kafka-topic".to_string(),
      "scratch.reference-implementation-compliant.greenbox-dev".to_string(),
    )]),
    parameters: &HashMap::from([
      ("identifier-picker-regex".to_string(), "(?:cancelled|created|updated):([0-9]+)".to_string()),
      ("identifier-picker-source-system".to_string(), "boss".to_string()),
      ("enable-dsh-envelope".to_string(), "true".to_string()),
      ("compliancy-agent".to_string(), "classification".to_string()),
      ("mitigation-strategy".to_string(), "block".to_string()),
    ]),
    profile_id: Some("minimal"),
  };
  let _ = application.deploy(SERVICE_ID, &deploy_parameters).await;
}
