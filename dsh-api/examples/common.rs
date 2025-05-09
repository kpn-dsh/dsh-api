use dsh_api::dsh_api_client::DshApiClient;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::DshApiError;
use std::io::Write;

// DSH_API_PLATFORM=nplz
// DSH_API_TENANT=greenbox-dev
// DSH_API_SECRET_NPLZ_GREENBOX_DEV=...

#[allow(unused)]
pub async fn get_client<'a>() -> Result<DshApiClient, String> {
  // Explicit try_factory variable declaration is important since type inference will not work here
  let try_factory: Result<DshApiClientFactory, DshApiError> = DshApiClientFactory::try_default();
  match try_factory {
    Ok(factory) => match factory.client().await {
      Ok(client) => Ok(client),
      Err(error) => Err(format!("could not create client ({})", error)),
    },
    Err(error) => Err(format!("could not create client factory ({})", error)),
  }
}

#[allow(unused)]
pub fn initialize_logger() {
  env_logger::builder()
    .format(|f, r| writeln!(f, "[{:5}] {} [{}]", r.level(), r.args(), r.target()))
    .try_init();
  // env_logger::builder().format_target(false).format_timestamp(None).init();
}

#[allow(unused)]
pub fn print_header(header: &str) {
  let bar = (0..header.len()).map(|_| "-").collect::<String>();
  println!("\n{}\n{}\n{}", bar, header, bar);
}

#[allow(unused)]
pub fn initialize_example(header: Option<&str>) {
  header.inspect(|h| print_header(h));
  env_logger::builder()
    .format(|f, r| writeln!(f, "[{:5}] {} [{}]", r.level(), r.args(), r.target()))
    .try_init();
}

#[allow(dead_code)]
fn main() {}
