#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

const APP_NAME: &str = "my-app";
const PLATFORM_NAME: &str = "np-aws-lz-dsh";
const PROXY: &str = "my-proxy";
const SERVICE_NAME: &str = "my-service";
const TENANT_NAME: &str = "my-tenant";
const VHOST: &str = "my-vhost";

use crate::common::initialize_logger;
use dsh_api::platform::DshPlatform;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  initialize_logger();

  match DshPlatform::try_default() {
    Ok(default_platform) => println!("default platform is {}", default_platform.name()),
    Err(error) => println!("no default platform, {}", error),
  }

  println!();

  match DshPlatform::try_from(PLATFORM_NAME) {
    Ok(default_platform) => print_platform(default_platform),
    Err(error) => println!("{}", error),
  }

  println!();

  for platform in DshPlatform::all() {
    println!("{} / {} -> {}", platform.name(), platform.alias(), platform.description());
  }

  Ok(())
}

fn print_platform(platform: DshPlatform) {
  println!("full name                        {}", platform.name());
  println!("description                      {}", platform.description());
  println!("alias                            {}", platform.alias());
  println!("is production                    {}", platform.is_production());
  println!("cloud provider                   {}", platform.cloud_provider());
  println!("access token endpoint            {}", platform.access_token_endpoint());
  println!("realm                            {}", platform.realm());
  println!("public domain                    {}", platform.public_domain());
  match platform.private_domain() {
    Some(private_domain) => println!("private domain                   {}", private_domain),
    None => println!("private domain                   not specified"),
  }
  println!();
  println!("client id                        {}", platform.client_id());
  println!("client id tenant                 {}", platform.tenant_client_id(TENANT_NAME));
  println!("domain console                   {}", platform.console_domain());
  println!("domain internal                  {}", platform.internal_domain(TENANT_NAME));
  println!("domain internal service          {}", platform.internal_service_domain(TENANT_NAME, SERVICE_NAME));
  println!("domain rest api                  {}", platform.rest_api_domain());
  println!("endpoint mqtt token              {}", platform.mqtt_token_endpoint());
  println!("endpoint rest api                {}", platform.rest_api_endpoint());
  match platform.tenant_private_vhost_domain(TENANT_NAME, VHOST) {
    Ok(private_vhost_domain) => println!("private domain vhost             {}", private_vhost_domain),
    Err(error) => println!("private domain vhost             {}", error),
  }
  println!("public domain app                {}", platform.tenant_public_app_domain(TENANT_NAME, APP_NAME));
  println!("public domain vhost              {}", platform.public_vhost_domain(VHOST));

  println!("url app catalog tenant           {}", platform.tenant_app_catalog_url(TENANT_NAME));
  println!(
    "url app catalog tenant app       {}",
    platform.tenant_app_catalog_app_url(TENANT_NAME, "kpn", APP_NAME)
  );
  println!("url console                      {}", platform.console_url());
  println!("url console tenant               {}", platform.tenant_console_url(TENANT_NAME));
  println!("url console tenant app           {}", platform.tenant_app_console_url(TENANT_NAME, SERVICE_NAME));
  println!(
    "url console tenant service       {}",
    platform.tenant_service_console_url(TENANT_NAME, SERVICE_NAME)
  );
  println!("url data catalog tenant          {}", platform.tenant_data_catalog_url(TENANT_NAME));
  println!("url monitoring tenant            {}", platform.tenant_monitoring_url(TENANT_NAME));
  println!("url swagger                      {}", platform.swagger_url());
  println!("url tracing                      {}", platform.tracing_url());

  match platform.tenant_private_domain(TENANT_NAME) {
    Ok(tenant_private_domain) => println!("private domain tenant            {}", tenant_private_domain),
    Err(error) => println!("private domain tenant            {}", error),
  }
  match platform.tenant_proxy_private_bootstrap_servers(TENANT_NAME, PROXY) {
    Ok(boostrap_servers) => println!("proxy private bootstrap servers  {}", boostrap_servers.join("\n                                 ")),
    Err(error) => println!("proxy private bootstrap servers  {}", error),
  }
  match platform.tenant_proxy_private_schema_store_host(TENANT_NAME, PROXY) {
    Ok(schema_store_host) => println!("proxy private schema store host  {}", schema_store_host),
    Err(error) => println!("proxy private schema store host  {}", error),
  }

  println!("public domain tenant             {}", platform.tenant_public_domain(TENANT_NAME));
  println!(
    "proxy public bootstrap servers   {}",
    platform
      .tenant_proxy_public_bootstrap_servers(TENANT_NAME, PROXY)
      .join("\n                                 ")
  );
  println!(
    "proxy public schema store host   {}",
    platform.tenant_proxy_public_schema_store_host(TENANT_NAME, PROXY)
  );
}
