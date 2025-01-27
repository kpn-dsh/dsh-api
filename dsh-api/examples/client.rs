use dsh_api::dsh_api_client::DshApiClient;

fn main() {
  env_logger::init();

  println!("{}", DshApiClient::openapi_spec())
}
