use crate::common::{get_client, initialize_logger, print_header};
use dsh_api::types::ManagedStreamId;
use std::str::FromStr;
// Tenant needs manage rights

#[path = "common.rs"]
mod common;

const INTERNAL_STREAM: &str = "ajuc---internal";
const PUBLIC_STREAM: &str = "ajuc---public";
const TENANT: &str = "ajuc-test";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
  let client = get_client().await?;

  let internal_managed_stream_id = ManagedStreamId::from_str(INTERNAL_STREAM).unwrap();
  let public_managed_stream_id = ManagedStreamId::from_str(PUBLIC_STREAM).unwrap();

  print_header("get-internal-stream-configurations");
  match client.get_internal_stream_configurations().await {
    Ok(stream_configurations) => {
      for (stream_id, internal_stream_configuration) in stream_configurations {
        println!("{} -> {}", stream_id, internal_stream_configuration)
      }
    }
    Err(error) => println!("{:?}", error),
  }

  print_header("get-public-stream-configurations");
  match client.get_public_stream_configurations().await {
    Ok(stream_configurations) => {
      for (stream_id, public_stream_configuration) in stream_configurations {
        println!("{} -> {}", stream_id, public_stream_configuration)
      }
    }
    Err(error) => println!("{:?}", error),
  }

  print_header("get-stream-configurations");
  match client.get_stream_configurations().await {
    Ok(stream_configurations) => {
      for (streaam_id, stream_configuration) in stream_configurations {
        println!("{} -> {}", streaam_id, stream_configuration)
      }
    }
    Err(error) => println!("{:?}", error),
  }

  print_header("get-granted-managed-streams");
  match client.get_granted_managed_streams(TENANT).await {
    Ok(granted_streams) => println!("{:#?}", granted_streams),
    Err(error) => println!("{:?}", error),
  }

  print_header("generic-head");
  println!(
    "{} {} {:?}",
    INTERNAL_STREAM,
    TENANT,
    client.head("stream-internal-access-read", &[INTERNAL_STREAM, TENANT]).await
  );
  println!(
    "{} {} {:?}",
    INTERNAL_STREAM,
    TENANT,
    client.head("stream-internal-access-write", &[INTERNAL_STREAM, TENANT]).await
  );
  println!(
    "{} {} {:?}",
    PUBLIC_STREAM,
    TENANT,
    client.head("stream-public-access-read", &[PUBLIC_STREAM, TENANT]).await
  );
  println!(
    "{} {} {:?}",
    PUBLIC_STREAM,
    TENANT,
    client.head("stream-public-access-write", &[PUBLIC_STREAM, TENANT]).await
  );

  print_header("head-stream-access");
  println!(
    "{} {} {:?}",
    internal_managed_stream_id,
    TENANT,
    client.head_stream_internal_access_read(&internal_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    internal_managed_stream_id,
    TENANT,
    client.head_stream_internal_access_write(&internal_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    public_managed_stream_id,
    TENANT,
    client.head_stream_public_access_read(&public_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    public_managed_stream_id,
    TENANT,
    client.head_stream_public_access_write(&public_managed_stream_id, TENANT).await
  );

  print_header("has-stream-access");
  println!(
    "{} {} {:?}",
    internal_managed_stream_id,
    TENANT,
    client.has_internal_read_access(&internal_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    internal_managed_stream_id,
    TENANT,
    client.has_internal_write_access(&internal_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    public_managed_stream_id,
    TENANT,
    client.has_public_read_access(&public_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    public_managed_stream_id,
    TENANT,
    client.has_public_write_access(&public_managed_stream_id, TENANT).await
  );

  print_header("managed-stream-access-rights");
  println!(
    "{} {} {:?}",
    internal_managed_stream_id,
    TENANT,
    client.managed_stream_access_rights(&internal_managed_stream_id, TENANT).await
  );
  println!(
    "{} {} {:?}",
    public_managed_stream_id,
    TENANT,
    client.managed_stream_access_rights(&public_managed_stream_id, TENANT).await
  );

  print_header("get_tenants_with_access_rights");
  match client.get_tenants_with_access_rights(&internal_managed_stream_id).await {
    Ok(tenants) => println!("{} {:?}", internal_managed_stream_id, tenants),
    Err(error) => println!("{:#?}", error),
  }
  match client.get_tenants_with_access_rights(&public_managed_stream_id).await {
    Ok(tenants) => println!("{} {:?}", public_managed_stream_id, tenants),
    Err(error) => println!("{:#?}", error),
  }

  print_header("get_stream_internal_configuration");
  match client.get_stream_internal_configuration(&internal_managed_stream_id).await {
    Ok(internal_managed_stream) => println!("{}", internal_managed_stream),
    Err(error) => println!("{:#?}", error),
  }

  print_header("get_stream_public_configuration");
  match client.get_stream_public_configuration(&public_managed_stream_id).await {
    Ok(public_managed_stream) => println!("{}", public_managed_stream),
    Err(error) => println!("{:#?}", error),
  }

  print_header("get_stream_configuration");
  match client.get_stream_configuration(&internal_managed_stream_id).await {
    Ok(Some(managed_stream)) => println!("{}", managed_stream),
    Ok(None) => println!("managed stream not found"),
    Err(error) => println!("{:#?}", error),
  }

  Ok(())
}
