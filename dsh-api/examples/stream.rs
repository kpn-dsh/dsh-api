use crate::common::{get_client, initialize_logger, print_header};
use dsh_api::types::ManagedStreamId;
use std::str::FromStr;

// Tenant needs manage rights

#[path = "common.rs"]
mod common;

const INTERNAL_MANAGED_STREAM_UNDER_TEST: &str = "ajuc---internal";
const PUBLIC_MANAGED_STREAM_UNDER_TEST: &str = "ajuc---public";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
  let client = get_client().await?;

  let internal_managed_stream_id = ManagedStreamId::from_str(INTERNAL_MANAGED_STREAM_UNDER_TEST).unwrap();
  let public_managed_stream_id = ManagedStreamId::from_str(PUBLIC_MANAGED_STREAM_UNDER_TEST).unwrap();

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
