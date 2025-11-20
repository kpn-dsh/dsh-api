#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::application_types::{ApplicationDiff, ApplicationValues, EnvVarInjection};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::{Application, PortMapping};

const APPLICATION: &str = "keyring-dev";
const APPLICATION_BASELINE: &str = "keyring-063";
const APPLICATION_SAMPLE: &str = "installed-base";
const APPLICATION_THAT_USES_SECRET: &str = "installed-base";
const _BUCKET: &str = "flink-cluster-bucket";
const SECRET: &str = "boss-account-ids";
const TOPIC: &str = "cpr-blacklist-record";
const VOLUME: &str = "faas-volume";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client_factory = DshApiClientFactory::default();
  let client = client_factory.client().await?;

  let application = client.get_application_configuration(APPLICATION).await?;
  let application_baseline = client.get_application_configuration(APPLICATION_BASELINE).await?;
  let application_sample = client.get_application_configuration(APPLICATION_SAMPLE).await?;
  let applications = client.get_application_configuration_map().await?;

  print_header(&format!("buckets_from_application('{}') -> [(bucket, [env])]`]", APPLICATION));
  let buckets_from_application: Vec<EnvVarInjection> = dsh_api::bucket::buckets_from_application(&application);
  for EnvVarInjection { id, env_var_keys } in buckets_from_application {
    println!("{} -> {}", id, env_var_keys.join(", "));
  }

  print_header("buckets_from_applications(applications) -> [(app id, app, [(bucket, [inj])])]`]");
  let buckets_from_applications: Vec<ApplicationValues<EnvVarInjection>> = dsh_api::bucket::buckets_from_applications(&applications);
  for ApplicationValues { id, application, values } in buckets_from_applications {
    println!("{} -> {}", id, application.instances);
    for EnvVarInjection { id, env_var_keys } in values {
      println!("  {} -> {}", id, env_var_keys.join(", "));
    }
  }

  print_header(&format!(
    "differences_between_applications('{}', '{}') -> [diff]`]",
    APPLICATION_BASELINE, APPLICATION_SAMPLE
  ));
  let differences_between_applications: ApplicationDiff = ApplicationDiff::differences_between_applications(&application_baseline, &application_sample);
  println!("{:#?}", differences_between_applications);

  print_header(&format!(
    "find_applications_that_use_secret('{}', applications) -> [(app id, app, [inj])]`]",
    SECRET
  ));
  let applications_that_use_secret: Vec<ApplicationValues<&str>> = dsh_api::secret::secret_env_vars_from_applications(SECRET, &applications);
  for ApplicationValues { id, application, values } in applications_that_use_secret {
    println!("{} -> {} -> {}", id, application.instances, values.join(", "));
  }

  print_header(&format!(
    "find_applications_that_use_volume('{}', applications) -> [(app id, app, [inj])]`]",
    VOLUME
  ));
  let applications_that_use_volume: Vec<ApplicationValues<&str>> = dsh_api::volume::volume_paths_from_applications(VOLUME, &applications);
  for ApplicationValues { id, application, values } in applications_that_use_volume {
    println!("{} -> {} -> {}", id, application.instances, values.join(", "));
  }

  print_header(&format!("find_applications_that_use_topic('{}', applications) -> [(app id, app, [inj])]`]", TOPIC));
  let applications_that_use_topic: Vec<(&str, &Application)> = dsh_api::topic::topic_used_in_applications(TOPIC, &applications);
  for (application_id, application) in applications_that_use_topic {
    println!("{} -> {}", application_id, application.instances);
  }

  print_header(&format!("secret_from_application('{}', '{}') -> [inj]`]", SECRET, APPLICATION_THAT_USES_SECRET));
  let secret_from_application: Vec<&str> = dsh_api::secret::secret_env_vars_from_application(SECRET, &application);
  println!("{:#?}", secret_from_application);

  print_header(&format!("secrets_from_application('{}') -> [(secret, [inj])]`]", APPLICATION));
  let secrets_from_application: Vec<EnvVarInjection> = dsh_api::secret::secrets_from_application(&application);
  for EnvVarInjection { id, env_var_keys } in secrets_from_application {
    println!("{} -> {}", id, env_var_keys.join(", "));
  }

  print_header(&format!("vhosts_from_application('{}') -> [(vhost, inj)]`]", APPLICATION));
  let vhosts_from_application: Vec<(String, &str, &PortMapping)> = dsh_api::vhost::vhosts_from_application(&application);
  for (vhost_id, port, port_mapping) in vhosts_from_application {
    println!("{} -> {} -> {}", vhost_id, port, port_mapping);
  }

  print_header("vhosts_from_applications(applications) -> [(app id, app, [(vhost, inj)])]`]");
  let vhosts_from_applications: Vec<ApplicationValues<(String, &str, &PortMapping)>> = dsh_api::vhost::vhosts_from_applications(&applications);
  for ApplicationValues { id, application, values } in vhosts_from_applications {
    println!("{} -> {}", id, application.instances);
    for (vhost_id, port, port_mapping) in values {
      println!("{} -> {} -> {}", vhost_id, port, port_mapping);
    }
  }

  Ok(())
}
