#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::{AllocationStatus, Bucket, BucketStatus};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let bucket_id = "schema-registry";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("get_bucket");
  let bucket: BucketStatus = client.get_bucket(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket);

  print_header("get_bucket_actual_configuration");
  let bucket_actual: Bucket = client.get_bucket_actual(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_actual);

  print_header("get_bucket_allocation_status");
  let bucket_status: AllocationStatus = client.get_bucket_status(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_status);

  print_header("get_bucket_configuration");
  let bucket_configuration: Bucket = client.get_bucket_configuration(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_configuration);

  print_header("get_buckets");
  let buckets: HashMap<String, BucketStatus> = client.get_buckets().await?;
  for (bucket_id, bucket_status) in buckets {
    println!("{} -> {}", bucket_id, bucket_status);
  }

  print_header("list_bucket_ids");
  let bucket_ids: Vec<String> = client.list_bucket_ids().await?;
  for bucket_id in bucket_ids {
    println!("{}", bucket_id);
  }

  print_header("list_buckets");
  let buckets: Vec<(String, BucketStatus)> = client.list_buckets().await?;
  for (bucket_id, bucket) in buckets {
    println!("{} -> {}", bucket_id, bucket);
  }

  Ok(())
}
