#[path = "common.rs"]
mod common;

#[tokio::main]
async fn main() -> Result<(), String> {
  #[cfg(feature = "actual")]
  {
    use dsh_api::dsh_api_client::DshApiClient;
    use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
    let client_factory = &DEFAULT_DSH_API_CLIENT_FACTORY;
    let client = client_factory.client().await?;

    let application_ids = client.list_application_ids().await?;
    for application_id in application_ids {
      let application = client.get_application(&application_id).await?;
      let application_actual = client.get_application_actual(&application_id).await?;
      let diff = DshApiClient::application_diff(&application, &application_actual);
      if !diff.is_empty() {
        println!("{}", application_id);
      }
    }
  }

  Ok(())
}
