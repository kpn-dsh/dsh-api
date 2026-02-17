use dsh_api::dsh_api_client::DshApiClient;
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::error::DshApiResult;
use log::LevelFilter;
use std::env;
use std::io::Write;
use std::str::FromStr;

#[allow(unused)]
pub async fn get_client<'a>() -> Result<DshApiClient, String> {
  // Explicit try_factory type declaration is important since type inference will not work here
  let try_factory: DshApiResult<DshApiClientFactory> = DshApiClientFactory::try_default();
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
  const ENV_VAR_DSH_API_LOG_LEVEL: &str = "DSH_API_LOG_LEVEL";
  let log_level = match env::var(&ENV_VAR_DSH_API_LOG_LEVEL) {
    Ok(log_level) => LevelFilter::from_str(&log_level).expect("illegal log level"),
    Err(_) => LevelFilter::from(LevelFilter::Info),
  };
  env_logger::builder()
    .filter_module("dsh_api", log_level)
    .format(|f, r| {
      writeln!(
        f,
        "[{:5}] {} [{}:{}]",
        r.level(),
        r.args(),
        r.target(),
        r.line().map(|line| line.to_string()).unwrap_or_default()
      )
    })
    .try_init();
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
