#[allow(unused)]
mod common;

use crate::common::{get_client, initialize_logger};
use dsh_api::dsh_api_client::DshApiClient;
use dsh_api::types::Secret;

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();
  let client = get_client().await?;

  test_delete_non_existing_secret(&client).await?;
  test_get_existing_secret(&client).await?;
  test_get_tenant_ids(&client).await?;
  test_get_tenant_configuration(&client).await?;
  test_get_non_existing_secret(&client).await?;
  test_post_existing_secret(&client).await?;
  test_post_invalid_secret(&client).await?;
  test_put_secret(&client).await?;
  test_get_existing_bucket(&client).await?;
  test_get_non_existing_bucket(&client).await?;

  Ok(())
}

#[allow(unused)]
async fn test_delete_non_existing_secret(client: &DshApiClient) -> Result<(), String> {
  let result = client.delete_secret_configuration("non-existing-secret").await;
  println!("client.delete_secret_configuration('{}') -> {:?}", "non-existing-secret", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_existing_secret(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_secret("test").await;
  println!("client.get_secret('{}') -> {:?}", "test", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_tenant_ids(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_tenant_ids().await;
  println!("client.get_tenant_ids() -> {:?}", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_tenant_configuration(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_tenant_configuration("ajuc-test").await;
  println!("client.get_tenant_configuration('{}') -> {:?}", "ajuc-test", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_non_existing_secret(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_secret("non-existing-secret").await;
  println!("client.get_secret('{}') -> {:?}", "non-existing-secret", result);
  Ok(())
}

#[allow(unused)]
async fn test_post_existing_secret(client: &DshApiClient) -> Result<(), String> {
  let secret = Secret::new("test", "doesn't matter");
  let result = client.post_secret(&secret).await;
  println!("client.post_secret('{}') -> {:?}", "test", result);
  Ok(())
}

#[allow(unused)]
async fn test_post_invalid_secret(client: &DshApiClient) -> Result<(), String> {
  let secret = Secret::new("TEST", "doesn't matter");
  let result = client.post_secret(&secret).await;
  println!("client.post_secret('{}') -> {:?}", "TEST", result);
  Ok(())
}

#[allow(unused)]
async fn test_put_secret(client: &DshApiClient) -> Result<(), String> {
  let result = client.put_secret("test", "doesn't matter".to_string()).await;
  println!("client.put_secret('{}', \"doesn't matter\") -> {:?}", "test", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_existing_bucket(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_bucket_configuration("cpr").await;
  println!("client.get_bucket_configuration('{}') -> {:?}", "cpr", result);
  Ok(())
}

#[allow(unused)]
async fn test_get_non_existing_bucket(client: &DshApiClient) -> Result<(), String> {
  let result = client.get_bucket_configuration("non-existing-bucket").await;
  println!("client.get_bucket_configuration('{}') -> {:?}", "non-existing-bucket", result);
  Ok(())
}
