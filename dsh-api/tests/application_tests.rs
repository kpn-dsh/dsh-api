use dsh_api::application_types::{ApplicationValues, EnvVarInjection};
use dsh_api::secret::{secret_env_vars_from_application, secret_env_vars_from_applications, secrets_from_application, secrets_from_applications};
use dsh_api::types::{Application, ApplicationSecret, ApplicationVolumes, PortMapping};
use dsh_api::volume::{volumes_from_application, volumes_from_applications};
use std::collections::HashMap;
use std::fmt::Display;

#[test]
fn test_secrets_from_application() {
  let application = application(None);
  let application_secrets = secrets_from_application(&application);
  assert_eq!(application_secrets.len(), 2);
  assert_eq!(
    application_secrets.first().unwrap(),
    &EnvVarInjection::new("secret1", vec!["SECRET_KEY11", "SECRET_KEY12"])
  );
  assert_eq!(application_secrets.get(1).unwrap(), &EnvVarInjection::new("secret2", vec!["SECRET_KEY2"]));
}

#[test]
fn test_secrets_from_applications() {
  let mut application1 = application(Some("A1".to_string()));
  add_secret(&mut application1, "joined_secret", "A1_JOINED_SECRET_KEY1");
  add_secret(&mut application1, "joined_secret", "A1_JOINED_SECRET_KEY2");
  let mut application2 = application(Some("A2".to_string()));
  add_secret(&mut application2, "joined_secret", "A2_JOINED_SECRET_KEY");
  let applications = HashMap::from([("A1".to_string(), application1), ("A2".to_string(), application2)]);

  let application_secrets = secrets_from_applications(&applications);

  assert_eq!(application_secrets.len(), 2);

  let ApplicationValues { id: id1, values: values1, .. } = application_secrets.first().unwrap();
  assert_eq!(*id1, "A1");
  assert_eq!(values1.len(), 3);
  assert_eq!(
    values1.first().unwrap(),
    &EnvVarInjection::new("a1_secret1", vec!["A1_SECRET_KEY11", "A1_SECRET_KEY12"])
  );
  assert_eq!(values1.get(1).unwrap(), &EnvVarInjection::new("a1_secret2", vec!["A1_SECRET_KEY2"]));
  assert_eq!(
    values1.get(2).unwrap(),
    &EnvVarInjection::new("joined_secret", vec!["A1_JOINED_SECRET_KEY1", "A1_JOINED_SECRET_KEY2"])
  );

  let ApplicationValues { id: id2, values: values2, .. } = application_secrets.get(1).unwrap();
  assert_eq!(*id2, "A2");
  assert_eq!(values2.len(), 3);
  assert_eq!(
    values2.first().unwrap(),
    &EnvVarInjection::new("a2_secret1", vec!["A2_SECRET_KEY11", "A2_SECRET_KEY12"])
  );
  assert_eq!(values2.get(1).unwrap(), &EnvVarInjection::new("a2_secret2", vec!["A2_SECRET_KEY2"]));
  assert_eq!(values2.get(2).unwrap(), &EnvVarInjection::new("joined_secret", vec!["A2_JOINED_SECRET_KEY"]));
}

#[test]
fn test_secret_env_vars_from_application() {
  let application = application(None);
  let secrets_env_vars = secret_env_vars_from_application("secret1", &application);
  assert_eq!(secrets_env_vars, vec!["SECRET_KEY11", "SECRET_KEY12"]);
}

#[test]
fn test_secret_env_vars_from_applications() {
  let mut application1 = application(Some("A1".to_string()));
  add_secret(&mut application1, "joined_secret", "A1_JOINED_SECRET_KEY1");
  add_secret(&mut application1, "joined_secret", "A1_JOINED_SECRET_KEY2");
  let mut application2 = application(Some("A2".to_string()));
  add_secret(&mut application2, "joined_secret", "A2_JOINED_SECRET_KEY");
  let applications = HashMap::from([("A1".to_string(), application1), ("A2".to_string(), application2)]);

  let applications_a1_secret1 = secret_env_vars_from_applications("a1_secret1", &applications);
  assert_eq!(applications_a1_secret1.len(), 1);
  let first = applications_a1_secret1.first().unwrap();
  assert_eq!(first.id, "A1");
  assert_eq!(first.values, vec!["A1_SECRET_KEY11", "A1_SECRET_KEY12"]);

  let applications_a1_secret2 = secret_env_vars_from_applications("a1_secret2", &applications);
  assert_eq!(applications_a1_secret2.len(), 1);
  let first = applications_a1_secret2.first().unwrap();
  assert_eq!(first.id, "A1");
  assert_eq!(first.values, vec!["A1_SECRET_KEY2"]);

  let applications_a2_secret1 = secret_env_vars_from_applications("a2_secret1", &applications);
  assert_eq!(applications_a2_secret1.len(), 1);
  let first = applications_a2_secret1.first().unwrap();
  assert_eq!(first.id, "A2");
  assert_eq!(first.values, vec!["A2_SECRET_KEY11", "A2_SECRET_KEY12"]);

  let applications_a2_secret2 = secret_env_vars_from_applications("a2_secret2", &applications);
  assert_eq!(applications_a2_secret2.len(), 1);
  let first = applications_a2_secret2.first().unwrap();
  assert_eq!(first.id, "A2");
  assert_eq!(first.values, vec!["A2_SECRET_KEY2"]);

  let applications_joined_secret = secret_env_vars_from_applications("joined_secret", &applications);
  assert_eq!(applications_joined_secret.len(), 2);
  let first = applications_joined_secret.first().unwrap();
  assert_eq!(first.id, "A1");
  assert_eq!(first.values, vec!["A1_JOINED_SECRET_KEY1", "A1_JOINED_SECRET_KEY2"]);
  let second = applications_joined_secret.get(1).unwrap();
  assert_eq!(second.id, "A2");
  assert_eq!(second.values, vec!["A2_JOINED_SECRET_KEY"]);
}

#[test]
fn test_volumes_from_application() {
  let application = application(None);
  let application_volumes: Vec<(&str, &str)> = volumes_from_application(&application);
  assert_eq!(application_volumes.len(), 2);
  let first = application_volumes.first().unwrap();
  assert_eq!(first.0, "volume1");
  assert_eq!(first.1, "/path1".to_string());
  let second = application_volumes.get(1).unwrap();
  assert_eq!(second.0, "volume2");
  assert_eq!(second.1, "/path2");
}

#[test]
fn test_volumes_from_applications() {
  let application1 = application(Some("A1".to_string()));
  let application2 = application(Some("A2".to_string()));
  let applications = HashMap::from([("A1".to_string(), application1), ("A2".to_string(), application2)]);
  let application_volumes: Vec<ApplicationValues<(&str, &str)>> = volumes_from_applications(&applications);
  assert_eq!(application_volumes.len(), 2);
  let first = application_volumes.first().unwrap();
  assert_eq!(first.id, "A1");
  assert_eq!(first.values.first().unwrap().0, "a1_volume1");
  assert_eq!(first.values.first().unwrap().1, "/a1_path1");
  assert_eq!(first.values.get(1).unwrap().0, "a1_volume2");
  assert_eq!(first.values.get(1).unwrap().1, "/a1_path2");
  let second = application_volumes.get(1).unwrap();
  assert_eq!(second.id, "A2");
  assert_eq!(second.values.first().unwrap().0, "a2_volume1");
  assert_eq!(second.values.first().unwrap().1, "/a2_path1");
  assert_eq!(second.values.get(1).unwrap().0, "a2_volume2");
  assert_eq!(second.values.get(1).unwrap().1, "/a2_path2");
}

fn application(id: Option<String>) -> Application {
  let prefix = match id {
    Some(id) => format!("{}_", id),
    None => "".to_string(),
  };
  let pfl = |s: &str| format!("{}{}", prefix.to_lowercase(), s);
  let pfu = |s: &str| format!("{}{}", prefix.to_uppercase(), s);
  Application {
    cpus: 0.0,
    env: HashMap::from([
      (pfu("BUCKETNAME"), format!("{{ bucket_name('{}bucketname') }}", prefix.to_lowercase())),
      (pfu("HOSTNAME"), format!("{{ database_host('{}database1') }}", prefix.to_lowercase())),
      (pfu("DATABASE"), format!("{{ database_id('{}database1') }}", prefix.to_lowercase())),
      (pfu("USERNAME"), format!("{{ database_user('{}database1') }})", prefix.to_lowercase())),
      (pfu("KEY1"), pfu("VALUE1")),
      (pfu("KEY2"), pfu("VALUE2")),
    ]),
    exposed_ports: HashMap::from([("8000".to_string(), port_mapping(pfl("vhost1"), None::<String>)), ("8001".to_string(), port_mapping(pfl("vhost2"), Some("public")))]),
    health_check: None,
    image: "".to_string(),
    instances: 0,
    mem: 0,
    metrics: None,
    needs_token: false,
    readable_streams: vec![],
    secrets: vec![
      secret(pfl("secret1"), pfu("SECRET_KEY12")),
      secret(pfl("secret2"), pfu("SECRET_KEY2")),
      secret(pfl("secret1"), pfu("SECRET_KEY11")),
      // secret("system/objectstore/access_key_id", "SYSTEM_OBJECTSTORE_ACCESS_KEY_ID"),
      // secret("system/objectstore/secret_access_key", "SYSTEM_OBJECTSTORE_SECRET_ACCESS_KEY"),
      // secret("system/dbaas/example1_password", "PASSWORD"),
    ],
    single_instance: false,
    spread_group: None,
    topics: vec![pfl("topic1"), pfl("topic2"), pfl("topic3")],
    user: "".to_string(),
    volumes: HashMap::from([(format!("/{}", pfl("path1")), volume(pfl("volume1"))), (format!("/{}", pfl("path2")), volume(pfl("volume2")))]),
    writable_streams: vec![],
  }
}

fn port_mapping<S, T>(vhost: S, zone: Option<T>) -> PortMapping
where
  S: Display,
  T: Display,
{
  PortMapping {
    vhost: match zone {
      Some(zone) => Some(format!("{{ vhost('{}','{}') }}", vhost, zone)),
      None => Some(format!("{{ vhost('{}') }}", vhost)),
    },
    ..PortMapping::default()
  }
}

fn secret<S, T>(secret_id: S, env_key: T) -> ApplicationSecret
where
  S: Into<String>,
  T: ToString,
{
  ApplicationSecret::new(secret_id, &[env_key])
}

fn volume<T>(volume_id: T) -> ApplicationVolumes
where
  T: Display,
{
  ApplicationVolumes::new(format!("{{ volume('{}') }}", volume_id))
}

fn add_secret<S, T>(application: &mut Application, id: S, key: T)
where
  S: Into<String>,
  T: ToString,
{
  application.secrets.push(ApplicationSecret::new(id, &[key]));
}
