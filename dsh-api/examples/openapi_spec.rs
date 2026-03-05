#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::initialize_logger;
use dsh_api::dsh_api_client::DshApiClient;

fn main() {
  initialize_logger();

  println!("{}", DshApiClient::openapi_spec())
}
