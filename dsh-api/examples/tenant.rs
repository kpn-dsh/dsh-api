#[path = "common.rs"]
mod common;

#[cfg(feature = "manage")]
#[tokio::main]
async fn main() -> Result<(), String> {
  use crate::common::{get_client, initialize_logger, print_header};
  use dsh_api::types::{
    LimitValueCertificateCountName, LimitValueConsumerRateName, LimitValueCpuName, LimitValueKafkaAclGroupCountName, LimitValueMemName, LimitValuePartitionCountName,
    LimitValueProducerRateName, LimitValueRequestRateName, LimitValueSecretCountName, LimitValueTopicCountName, ManagedTenant, ManagedTenantServices, ManagedTenantServicesName,
  };
  const MANAGED_TENANT_UNDER_TEST: &str = "ajuc-test";
  const MANAGED_TENANT_TO_CREATE: &str = "ajuc-create";
  const MANAGED_TENANT_TO_DELETE: &str = "ajuc-delete";
  const MANAGING_TENANT: &str = "ajuc";

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
  match client.put_tenant_configuration(MANAGED_TENANT_TO_CREATE, &tenant_configuration).await {
    Ok(_) => println!("tenant created"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  print_header("put_tenant_configuration (update)");
  let tenant_configuration = ManagedTenant {
    manager: MANAGING_TENANT.to_string(),
    name: MANAGED_TENANT_TO_CREATE.to_string(),
    services: vec![ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Vpn }],
  };
  match client.put_tenant_configuration(MANAGED_TENANT_TO_CREATE, &tenant_configuration).await {
    Ok(_) => println!("tenant updated"),
    Err(error) => println!("{}\n{:#?}", error, error),
  }

  print_header("get_tenant_actual");
  let tenant_actual_configuration = client.get_tenant_actual(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{}", tenant_actual_configuration);

  print_header("get_tenant_status");
  let tenant_status = client.get_tenant_status(MANAGED_TENANT_UNDER_TEST).await?;
  println!("{}", tenant_status);

  print_header("get_granted_internal_streams");
  let granted_internal_streams = client.managed_tenant_granted_internal_streams(MANAGED_TENANT_UNDER_TEST).await?;
  for (stream_id, internal_stream, access_rights) in granted_internal_streams {
    println!("{} -> {} -> {}", stream_id, internal_stream, access_rights);
  }

  print_header("get_granted_managed_streams");
  let granted_managed_streams = client.managed_tenant_granted_managed_streams(MANAGED_TENANT_UNDER_TEST).await?;
  for (stream_id, stream, access_rights) in granted_managed_streams {
    println!("{} -> {} -> {}", stream_id, stream, access_rights);
  }

  print_header("get_granted_public_streams");
  let granted_public_streams = client.managed_tenant_granted_public_streams(MANAGED_TENANT_UNDER_TEST).await?;
  for (stream_id, public_stream, access_rights) in granted_public_streams {
    println!("{} -> {} -> {}", stream_id, public_stream, access_rights);
  }

  print_header("get_internal_streams_access_rights");
  let internal_streams_access_rights = client.managed_tenant_internal_streams_access_rights(MANAGED_TENANT_UNDER_TEST).await?;
  for (stream_id, access_rights) in internal_streams_access_rights {
    println!("{} -> {}", stream_id, access_rights);
  }

  print_header("get_public_streams_access_rights");
  let public_streams_access_rights = client.managed_tenant_public_streams_access_rights(MANAGED_TENANT_UNDER_TEST).await?;
  for (stream_id, access_rights) in public_streams_access_rights {
    println!("{} -> {}", stream_id, access_rights);
  }

  print_header("get_tenant_limits");
  let managed_tenant_limits = client.get_tenant_limits(MANAGED_TENANT_TO_CREATE).await?;
  for limit_value in managed_tenant_limits {
    println!("{:?}", limit_value);
  }

  print_header("get_tenant_limit (str)");
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "certificatecount").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "consumerrate").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "cpu").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "kafkaaclgroupcount").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "mem").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "partitioncount").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "producerrate").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "requestrate").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "secretcount").await?);
  println!("{:?}", client.get_tenant_limit(MANAGED_TENANT_UNDER_TEST, "topiccount").await?);

  print_header("managed_tenant_limits");
  let managed_tenant_limits = client.managed_tenant_limits(MANAGED_TENANT_TO_CREATE).await?;
  println!("{}", managed_tenant_limits);
  println!("{:?}", managed_tenant_limits);

  print_header("managed_tenant_limit (str)");
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "certificatecount").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "consumerrate").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "cpu").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "kafkaaclgroupcount").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "mem").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "partitioncount").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "producerrate").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "requestrate").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "secretcount").await?);
  println!("{:?}", client.managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, "topiccount").await?);

  print_header("managed_tenant_limit (kind)");
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueCertificateCountName::CertificateCount.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueConsumerRateName::ConsumerRate.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueCpuName::Cpu.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueKafkaAclGroupCountName::KafkaAclGroupCount.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueMemName::Mem.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValuePartitionCountName::PartitionCount.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueProducerRateName::ProducerRate.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueRequestRateName::RequestRate.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueSecretCountName::SecretCount.to_string().as_str())
      .await?
  );
  println!(
    "{:?}",
    client
      .managed_tenant_limit(MANAGED_TENANT_UNDER_TEST, LimitValueTopicCountName::TopicCount.to_string().as_str())
      .await?
  );
  Ok(())
}

#[cfg(not(feature = "manage"))]
fn main() {}
