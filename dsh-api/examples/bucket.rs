use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api::types::{AllocationStatus, Bucket, BucketStatus};
use std::collections::HashMap;

#[path = "common.rs"]
mod common;

// delete_bucket(bucket_id)
// get_bucket(bucket_id)
// get_bucket_actual_configuration(bucket_id)
// get_bucket_allocation_status(bucket_id)
// get_bucket_configuration(bucket_id)
// get_buckets()
// list_bucket_ids()
// list_buckets()
// update_bucket(bucket_id, bucket)

#[tokio::main]
async fn main() -> Result<(), String> {
  let bucket_id = "schema-registry";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  // delete_bucket(bucket_id)

  print_header("get_bucket");
  let bucket: BucketStatus = client.get_bucket(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket);

  #[cfg(feature = "actual")]
  {
    print_header("get_bucket_actual_configuration");
    let bucket_actual: Bucket = client.get_bucket_actual_configuration(bucket_id).await?;
    println!("{} -> {}", bucket_id, bucket_actual);
  }

  print_header("get_bucket_allocation_status");
  let bucket_status: AllocationStatus = client.get_bucket_allocation_status(bucket_id).await?;
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

  // update_bucket(bucket_id)

  Ok(())
}
