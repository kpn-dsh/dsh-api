use crate::common::{get_client, initialize_logger, print_header};
use dsh_api::types::{GetTenantLimitByManagerByTenantByKindKind, LimitValue, LimitValueSecretCount, LimitValueSecretCountName};

#[path = "common.rs"]
mod common;

const MANAGED_TENANT_UNDER_TEST: &str = "ajuc-test";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
  let client = get_client().await?;

  print_header("get_tenant_limits");
  let tenant_limits = client.get_tenant_limits(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{:#?}", tenant_limits);
  println!("{}", serde_json::to_string_pretty(&tenant_limits).unwrap());

  print_header("get_tenant_limit");
  for kind in [
    GetTenantLimitByManagerByTenantByKindKind::Certificatecount,
    GetTenantLimitByManagerByTenantByKindKind::Consumerrate,
    GetTenantLimitByManagerByTenantByKindKind::Cpu,
    GetTenantLimitByManagerByTenantByKindKind::Kafkaaclgroupcount,
    GetTenantLimitByManagerByTenantByKindKind::Mem,
    GetTenantLimitByManagerByTenantByKindKind::Partitioncount,
    GetTenantLimitByManagerByTenantByKindKind::Producerrate,
    GetTenantLimitByManagerByTenantByKindKind::Requestrate,
    GetTenantLimitByManagerByTenantByKindKind::Secretcount,
    GetTenantLimitByManagerByTenantByKindKind::Topiccount,
  ] {
    println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, kind).await?);
  }

  print_header("patch_tenant_limits");
  let tenant_limits = vec![LimitValue::SecretCount(LimitValueSecretCount { name: LimitValueSecretCountName::SecretCount, value: 10 })];
  println!("{}", serde_json::to_string_pretty(&tenant_limits).unwrap());
  match client.patch_tenant_limit(MANAGED_TENANT_UNDER_TEST, &tenant_limits).await {
    Ok(_) => println!("tenant limits patched"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  Ok(())
}
