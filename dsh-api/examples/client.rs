use dsh_api::dsh_api_client::DshApiClient;

fn main() {
  println!("{}", DshApiClient::openapi_spec())
}
