#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::bucket::BucketInjection;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::{AllocationStatus, Bucket, BucketStatus};
use dsh_api::{Dependant, DependantApp, DependantApplication};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let bucket_id = "cpr";

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  print_header("get_bucket_ids");
  let bucket_ids: Vec<String> = client.get_bucket_ids().await?;
  for bucket_id in bucket_ids {
    println!("{}", bucket_id);
  }

  print_header("buckets");
  let buckets: Vec<(String, BucketStatus)> = client.buckets().await?;
  for (bucket_id, bucket_status) in buckets {
    println!("{} -> {}", bucket_id, bucket_status);
  }

  print_header("bucket_name");
  let bucket_name: String = client.bucket_name(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_name);

  print_header("get_bucket");
  let bucket: BucketStatus = client.get_bucket(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket);

  print_header("get_bucket_actual");
  let bucket_actual: Bucket = client.get_bucket_actual(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_actual);

  print_header("get_bucket_status");
  let bucket_status: AllocationStatus = client.get_bucket_status(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_status);

  print_header("get_bucket_configuration");
  let bucket_configuration: Bucket = client.get_bucket_configuration(bucket_id).await?;
  println!("{} -> {}", bucket_id, bucket_configuration);

  print_header("bucket_ids_with_dependants");
  let bucket_ids_with_dependants: Vec<(String, Vec<Dependant<BucketInjection>>)> = client.bucket_ids_with_dependants().await?;
  for (bucket_id, dependants) in bucket_ids_with_dependants {
    println!("{} -> {}", bucket_id, bucket_status);
    for dependant in dependants {
      println!("  {}", dependant);
    }
  }

  print_header("bucket_map");
  let bucket_map: HashMap<String, BucketStatus> = client.bucket_map().await?;
  println!("{:?}", bucket_map);

  print_header("buckets_with_dependants");
  let buckets_with_dependants: Vec<(String, BucketStatus, Vec<Dependant<BucketInjection>>)> = client.buckets_with_dependants().await?;
  for (bucket_id, bucket_status, dependants) in buckets_with_dependants {
    println!("{} -> {}", bucket_id, bucket_status);
    for dependant in dependants {
      println!("  {}", dependant);
    }
  }

  print_header("buckets_with_dependant_applications");
  let buckets_with_dependant_applications: Vec<(String, BucketStatus, Vec<DependantApplication<BucketInjection>>)> = client.buckets_with_dependant_applications().await?;
  for (bucket_id, _, dependants) in buckets_with_dependant_applications {
    println!("{}", bucket_id);
    for dependant in dependants {
      println!("  {}", dependant);
    }
  }

  print_header("buckets_with_dependant_apps");
  let buckets_with_dependant_apps: Vec<(String, BucketStatus, Vec<DependantApp>)> = client.buckets_with_dependant_apps().await?;
  for (bucket_id, _, dependants) in buckets_with_dependant_apps {
    println!("{}", bucket_id);
    for dependant in dependants {
      println!("  {}", dependant);
    }
  }

  print_header("bucket_secrets");
  let (access_key_id, _) = client.bucket_secrets().await?;
  println!("{}, ********", access_key_id);

  Ok(())
}
