#[path = "common.rs"]
mod common;

const APP_NAME: &str = "my-app";
const PLATFORM_NAME: &str = "np-aws-lz-dsh";
const SERVICE_NAME: &str = "my-service";
const TENANT_NAME: &str = "my-tenant";
const VHOST: &str = "my-vhost";

use dsh_api::platform::DshPlatform;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();

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
  println!("full name                   {}", platform.name());
  println!("description                 {}", platform.description());
  println!("alias                       {}", platform.alias());
  println!("is production               {}", platform.is_production());
  println!("cloud provider              {}", platform.cloud_provider());
  println!("url key cloak               {}", platform.url_key_cloak());
  println!("realm                       {}", platform.realm());
  println!("public domain               {}", platform.public_domain());
  match platform.private_domain() {
    Some(private_domain) => println!("private domain              {}", private_domain),
    None => println!("private domain              not specified"),
  }
  println!();
  println!("client id                   {}", platform.client_id());
  println!("client id tenant            {}", platform.client_id_tenant(TENANT_NAME));
  println!("domain console              {}", platform.domain_console());
  println!("domain internal service     {}", platform.domain_internal_service(SERVICE_NAME));
  println!("domain rest api             {}", platform.domain_rest_api());
  println!("endpoint access token       {}", platform.endpoint_access_token());
  println!("endpoint mqtt token         {}", platform.endpoint_mqtt_token());
  println!("endpoint rest api           {}", platform.endpoint_rest_api());
  match platform.private_domain_vhost(TENANT_NAME, VHOST) {
    Ok(private_vhost_domain) => println!("private domain vhost        {}", private_vhost_domain),
    Err(error) => println!("private domain vhost        {}", error),
  }
  println!("public domain app           {}", platform.public_domain_app(TENANT_NAME, APP_NAME));
  println!("public domain apps          {}", platform.public_domain_apps(TENANT_NAME));
  println!("public domain vhost         {}", platform.public_domain_vhost(VHOST));
  println!("url app catalog tenant      {}", platform.url_app_catalog_tenant(TENANT_NAME));
  println!("url app catalog tenant app  {}", platform.url_app_catalog_tenant_app(TENANT_NAME, "kpn", APP_NAME));
  println!("url console                 {}", platform.url_console());
  println!("url console tenant          {}", platform.url_console_tenant(TENANT_NAME));
  println!("url console tenant app      {}", platform.url_console_tenant_app(TENANT_NAME, SERVICE_NAME));
  println!("url console tenant service  {}", platform.url_console_tenant_service(TENANT_NAME, SERVICE_NAME));
  println!("url data catalog tenant     {}", platform.url_data_catalog_tenant(TENANT_NAME));
  println!("url monitoring tenant       {}", platform.url_monitoring_tenant(TENANT_NAME));
  println!("url swagger                 {}", platform.url_swagger());
  println!("url tracing                 {}", platform.url_tracing());
}
