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

  match client.get_granted_managed_streams(TENANT).await {
    Ok(granted_streams) => println!("{:#?}", granted_streams),
    Err(error) => println!("{:?}", error),
  }

  match client.head("stream-internal-access-read", &[INTERNAL_STREAM, TENANT]).await {
    Ok(_) => println!("stream-internal-access-read -> Ok"),
    Err(error) => println!("stream-internal-access-read -> {:?}", error),
  }
  match client.head("stream-internal-access-write", &[INTERNAL_STREAM, TENANT]).await {
    Ok(_) => println!("stream-internal-access-write -> Ok"),
    Err(error) => println!("stream-internal-access-write -> {:?}", error),
  }

  match client.head("stream-public-access-read", &[PUBLIC_STREAM, TENANT]).await {
    Ok(_) => println!("stream-public-access-read -> Ok"),
    Err(error) => println!("stream-public-access-read -> {:?}", error),
  }
  match client.head("stream-public-access-write", &[PUBLIC_STREAM, TENANT]).await {
    Ok(_) => println!("stream-public-access-write -> Ok"),
    Err(error) => println!("stream-public-access-write -> {:?}", error),
  }

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
