use dsh_api::dsh_api_client::DshApiClient;
use dsh_api::dsh_api_client_factory::{DshApiClientFactory, TRY_DEFAULT_DSH_API_CLIENT_FACTORY};
use dsh_api::DshApiError;

#[allow(dead_code)]
fn main() {}

#[allow(unused)]
pub fn print_header(header: &str) {
  let bar = (0..header.len()).map(|_| "-").collect::<String>();
  println!("\n{}\n{}\n{}", bar, header, bar);
}

#[allow(unused)]
pub(crate) async fn get_client<'a>() -> Result<DshApiClient<'a>, ()> {
  // Explicit try_factory variable declaration is important since type inference will not work here
  let try_factory: &Result<DshApiClientFactory, DshApiError> = &TRY_DEFAULT_DSH_API_CLIENT_FACTORY;
  match try_factory {
    Ok(factory) => match factory.client().await {
      Ok(client) => Ok(client),
      Err(error) => {
        println!("could not create client ({})", error);
        Err(())
      }
    },
    Err(error) => {
      println!("could not create client factory ({})", error);
      Err(())
    }
  }
}
