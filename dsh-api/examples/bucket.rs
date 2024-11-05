use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api_generated::types::{AllocationStatus, Bucket, BucketStatus};

#[tokio::main]
async fn main() -> Result<(), String> {
  let bucket_id = "schema-registry";

  let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
  let client = client_factory.client().await?;

  // let bucket = client.delete_bucket(bucket_id).await?;
  // let bucket = client.update_bucket(bucket_id).await?;

  let mut bucket_ids: Vec<String> = client.get_bucket_ids().await?;
  bucket_ids.sort();
  println!("get_bucket_ids()");
  for bucket_id in bucket_ids {
    println!("{}", bucket_id);
  }

  let bucket: BucketStatus = client.get_bucket(bucket_id).await?;
  println!("get_bucket_status({})\n{}", bucket_id, serde_json::to_string_pretty(&bucket).unwrap());

  let bucket_status: AllocationStatus = client.get_bucket_allocation_status(bucket_id).await?;
  println!(
    "get_bucket_allocation_status({})\n{}",
    bucket_id,
    serde_json::to_string_pretty(&bucket_status).unwrap()
  );

  let bucket_actual: Bucket = client.get_bucket_actual_configuration(bucket_id).await?;
  println!("get_bucket_actual({})\n{}", bucket_id, serde_json::to_string_pretty(&bucket_actual).unwrap());

  let bucket_configuration: Bucket = client.get_bucket_configuration(bucket_id).await?;
  println!(
    "get_bucket_configuration({})\n{}",
    bucket_id,
    serde_json::to_string_pretty(&bucket_configuration).unwrap()
  );

  Ok(())
}
