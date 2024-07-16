use std::collections::HashMap;
use std::env;

use dsh_rest_api_client::Client;
use dsh_sdk::{RestTokenFetcher, RestTokenFetcherBuilder};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;
use uuid::Uuid;

use crate::processor::application::platform::DshPlatform;
use crate::processor::processor_config::PlaceHolder;

pub mod application;
pub mod application_config;
pub mod application_registry;
pub mod dsh_api;
pub mod platform;

const TRIFONIUS_CONFIG_DIR: &str = "TRIFONIUS_CONFIG_DIR";

const TRIFONIUS_TARGET_TENANT: &str = "TRIFONIUS_TARGET_TENANT";
const TRIFONIUS_TARGET_TENANT_USER: &str = "TRIFONIUS_TARGET_TENANT_USER";
const TRIFONIUS_TARGET_TENANT_SECRET: &str = "TRIFONIUS_TARGET_TENANT_SECRET";
const TRIFONIUS_TARGET_PLATFORM: &str = "TRIFONIUS_TARGET_PLATFORM";

pub struct TargetClientFactory {
  pub dsh_platform: DshPlatform,
  pub tenant: String,
  pub user: String,
  pub client: Client,
  pub token_fetcher: RestTokenFetcher,
}

lazy_static! {
  pub static ref DEFAULT_TARGET_CLIENT_FACTOR: TargetClientFactory = {
    let tenant_name = get_env(TRIFONIUS_TARGET_TENANT);
    let tenant_user = get_env(TRIFONIUS_TARGET_TENANT_USER);
    let client_secret = get_env(TRIFONIUS_TARGET_TENANT_SECRET);
    let platform_name = get_env(TRIFONIUS_TARGET_PLATFORM);
    let dsh_platform = DshPlatform::try_from(platform_name.as_str()).unwrap();
    TargetClientFactory::create(tenant_name, tenant_user, client_secret, &dsh_platform).expect("could not create static target client factory")
  };
}

pub struct TargetClient<'a> {
  pub tenant: &'a String,
  pub user: &'a String,
  pub client: &'a Client,
  pub token: String,
}

impl TargetClientFactory {
  pub fn create(tenant: String, user: String, client_secret: String, dsh_platform: &DshPlatform) -> Result<Self, String> {
    match RestTokenFetcherBuilder::new(dsh_platform.sdk_platform())
      .tenant_name(tenant.clone())
      .client_secret(client_secret)
      .build()
    {
      Ok(token_fetcher) => {
        Ok(TargetClientFactory { dsh_platform: dsh_platform.clone(), tenant, user, client: Client::new(dsh_platform.endpoint_rest_api().as_str()), token_fetcher })
      }
      Err(e) => Err(format!("could not create token fetcher ({})", e)),
    }
  }

  pub async fn get(&self) -> Result<TargetClient, String> {
    match self.token_fetcher.get_token().await {
      Ok(token) => Ok(TargetClient { tenant: &self.tenant, user: &self.user, client: &self.client, token }),
      Err(e) => Err(format!("could not create token ({})", e)),
    }
  }
}

impl From<&TargetClientFactory> for TemplateMapping {
  fn from(value: &TargetClientFactory) -> Self {
    let mut mapping = TemplateMapping::new();
    if let Some(app_domain) = value.dsh_platform.app_domain(value.tenant.as_str()) {
      mapping.insert(PlaceHolder::AppDomain, app_domain);
    }
    if let Some(console_url) = value.dsh_platform.console_url() {
      mapping.insert(PlaceHolder::ConsoleUrl, console_url);
    }
    if let Some(dsh_internal_domain) = value.dsh_platform.dsh_internal_domain(value.tenant.as_str()) {
      mapping.insert(PlaceHolder::DshInternalDomain, dsh_internal_domain);
    }
    if let Some(monitoring_url) = value.dsh_platform.monitoring_url(value.tenant.as_str()) {
      mapping.insert(PlaceHolder::MonitoringUrl, monitoring_url);
    }
    mapping.insert(PlaceHolder::Platform, value.dsh_platform.to_string());
    if let Some(public_vhosts_domain) = value.dsh_platform.public_vhosts_domain() {
      mapping.insert(PlaceHolder::PublicVhostsDomain, public_vhosts_domain);
    }
    mapping.insert(PlaceHolder::Random, format!("{:x}", rand::thread_rng().gen_range(0x10000000_u64..=0xffffffff_u64)));
    mapping.insert(PlaceHolder::RandomUuid, Uuid::new_v4().to_string());
    mapping.insert(PlaceHolder::Realm, value.dsh_platform.realm().to_string());
    mapping.insert(PlaceHolder::RestAccessTokenUrl, value.dsh_platform.endpoint_rest_access_token().to_string());
    mapping.insert(PlaceHolder::RestApiUrl, value.dsh_platform.endpoint_rest_api().to_string());
    mapping.insert(PlaceHolder::Tenant, value.tenant.to_string());
    mapping.insert(PlaceHolder::User, value.user.to_string());
    mapping
  }
}

fn get_env(name: &str) -> String {
  match env::var(name) {
    Ok(value) => value,
    Err(_) => panic!("environment variable {} not set", name),
  }
}

pub type TemplateMapping = HashMap<PlaceHolder, String>;

lazy_static! {
  static ref TEMPLATE_REGEX: Regex = Regex::new("\\$\\{([A-Z][A-Z0-9_]*)\\}").unwrap();
}

pub(crate) fn template_resolver(template: &str, template_mapping: &TemplateMapping) -> Result<String, String> {
  let mut new = String::with_capacity(template.len());
  let mut last_match = 0;
  for caps in TEMPLATE_REGEX.captures_iter(template) {
    let m = caps.get(0).unwrap();
    new.push_str(&template[last_match..m.start()]);
    let place_holder = PlaceHolder::try_from(caps.get(1).unwrap().as_str())?;
    match template_mapping.get(&place_holder) {
      Some(value) => {
        new.push_str(value);
      }
      None => return Err(format!("template resolution failed because placeholder '{}' has no value", place_holder)),
    }
    last_match = m.end();
  }
  new.push_str(&template[last_match..]);
  Ok(new)
}

pub(crate) fn validate_template(template: &str, template_mapping: &[PlaceHolder]) -> Result<(), String> {
  for caps in TEMPLATE_REGEX.captures_iter(template) {
    let place_holder = PlaceHolder::try_from(caps.get(1).unwrap().as_str())?;
    if !template_mapping.contains(&place_holder) {
      return Err(format!("invalid template because placeholder '{}' is not allowed", place_holder));
    }
  }
  Ok(())
}

#[test]
fn resolve_template_successfully() {
  let template = "abcd${TENANT}def${USER}ghi";
  let tenant = "tenant";
  let user = "user";
  let template_mapping: TemplateMapping = HashMap::from([(PlaceHolder::Tenant, tenant.to_string()), (PlaceHolder::User, user.to_string())]);
  assert_eq!(template_resolver(template, &template_mapping).unwrap(), "abcdtenantdefuserghi");
}

#[test]
fn validate_template_succesfully() {
  assert!(validate_template("abcd${TENANT}def${USER}ghi", &[PlaceHolder::Tenant, PlaceHolder::User]).is_ok());
  assert!(validate_template("abcd${TENANT}def${USER}ghi", &[PlaceHolder::Tenant]).is_err());
  assert!(validate_template("abcd{TENANT}def{USER}ghi", &[PlaceHolder::Tenant]).is_ok());
  assert!(validate_template("abcdefghijkl", &[PlaceHolder::Tenant]).is_ok());
  assert!(validate_template("", &[PlaceHolder::Tenant]).is_ok());
}
