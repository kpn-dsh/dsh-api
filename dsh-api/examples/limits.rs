use std::num::NonZero;

#[path = "common.rs"]
mod common;

#[cfg(feature = "manage")]
#[tokio::main]
async fn main() -> Result<(), String> {
  use crate::common::{get_client, initialize_logger, print_header};
  use dsh_api::types::{LimitValue, LimitValueSecretCount, LimitValueSecretCountName};

  const MANAGED_TENANT_UNDER_TEST: &str = "ajuc-test";

  initialize_logger();
  let client = get_client().await?;

  print_header("get_tenant_limits");
  let tenant_limits = client.get_tenant_limits(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{:#?}", tenant_limits);
  println!("{}", serde_json::to_string_pretty(&tenant_limits).unwrap());

  print_header("get_tenant_limit");
  for kind in [
    LimitValueSecretCountName::CertificateCount,
    LimitValueSecretCountName::ConsumerRate,
    LimitValueSecretCountName::Cpu,
    LimitValueSecretCountName::KafkaAclGroupCount,
    LimitValueSecretCountName::Mem,
    LimitValueSecretCountName::PartitionCount,
    LimitValueSecretCountName::ProducerRate,
    LimitValueSecretCountName::RequestRate,
    LimitValueSecretCountName::SecretCount,
    LimitValueSecretCountName::TopicCount,
  ] {
    println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, kind.to_string()).await?);
  }

  print_header("patch_tenant_limits");
  let tenant_limits = vec![LimitValue::SecretCount(LimitValueSecretCount { name: LimitValueSecretCountName::SecretCount, value: NonZero::new(10).unwrap() })];
  println!("{}", serde_json::to_string_pretty(&tenant_limits).unwrap());
  match client.patch_tenant_limit(MANAGED_TENANT_UNDER_TEST, &tenant_limits).await {
    Ok(_) => println!("tenant limits patched"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  Ok(())
}

#[cfg(not(feature = "manage"))]
fn main() {}
