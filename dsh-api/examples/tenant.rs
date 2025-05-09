use crate::common::{get_client, initialize_logger, print_header};
use dsh_api::types::{ManagedTenant, ManagedTenantServices, ManagedTenantServicesName};

#[path = "common.rs"]
mod common;

const MANAGED_TENANT_UNDER_TEST: &str = "ajuc-test";
const MANAGED_TENANT_TO_CREATE: &str = "ajuc-create";
const MANAGED_TENANT_TO_DELETE: &str = "ajuc-delete";
const MANAGED_TENANT_TO_UPDATE: &str = "ajuc-update";
const MANAGING_TENANT: &str = "ajuc";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
  let client = get_client().await?;

  print_header("get_tenant_ids");
  let tenant_ids = client.get_tenant_ids().await?;
  println!("{:#?}", tenant_ids);

  print_header("delete_tenant_configuration");
  match client.delete_tenant_configuration(MANAGED_TENANT_TO_DELETE).await {
    Ok(_) => println!("tenant deleted"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  print_header("get_tenant_configuration");
  let tenant_configuration = client.get_tenant_configuration(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{:#?}", tenant_configuration);

  print_header("put_tenant_configuration (create)");
  let tenant_configuration = ManagedTenant {
    manager: MANAGING_TENANT.to_string(),
    name: MANAGED_TENANT_TO_CREATE.to_string(),
    services: vec![
      ManagedTenantServices { enabled: true, name: ManagedTenantServicesName::Monitoring },
      ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Tracing },
      ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Vpn },
    ],
  };
  match client.put_tenant_configuration(MANAGED_TENANT_UNDER_TEST, &tenant_configuration).await {
    Ok(_) => println!("tenant created"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  print_header("put_tenant_configuration (update)");
  let tenant_configuration = ManagedTenant {
    manager: MANAGING_TENANT.to_string(),
    name: MANAGED_TENANT_TO_UPDATE.to_string(),
    services: vec![ManagedTenantServices { enabled: true, name: ManagedTenantServicesName::Vpn }],
  };
  match client.put_tenant_configuration(MANAGED_TENANT_TO_DELETE, &tenant_configuration).await {
    Ok(_) => println!("tenant created"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  print_header("get_tenant_actual");
  let tenant_actual_configuration = client.get_tenant_actual(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{:#?}", tenant_actual_configuration);

  print_header("get_tenant_status");
  let tenant_status = client.get_tenant_status(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{:#?}", tenant_status);

  Ok(())
}
