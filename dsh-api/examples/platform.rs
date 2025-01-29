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
  println!("url key cloak               {}", platform.key_cloak_url());
  println!("realm                       {}", platform.realm());
  println!("public domain               {}", platform.public_domain());
  match platform.private_domain() {
    Some(private_domain) => println!("private domain              {}", private_domain),
    None => println!("private domain              not specified"),
  }
  println!();
  println!("client id                   {}", platform.client_id());
  println!("client id tenant            {}", platform.tenant_client_id(TENANT_NAME));
  println!("domain console              {}", platform.console_domain());
  println!("domain internal service     {}", platform.internal_service_domain(SERVICE_NAME));
  println!("domain rest api             {}", platform.rest_api_domain());
  println!("endpoint access token       {}", platform.access_token_endpoint());
  println!("endpoint mqtt token         {}", platform.mqtt_token_endpoint());
  println!("endpoint rest api           {}", platform.rest_api_endpoint());
  match platform.tenant_private_vhost_domain(TENANT_NAME, VHOST) {
    Ok(private_vhost_domain) => println!("private domain vhost        {}", private_vhost_domain),
    Err(error) => println!("private domain vhost        {}", error),
  }
  println!("public domain app           {}", platform.tenant_public_app_domain(TENANT_NAME, APP_NAME));
  println!("public domain apps          {}", platform.tenant_public_apps_domain(TENANT_NAME));
  println!("public domain vhost         {}", platform.public_vhost_domain(VHOST));
  println!("url app catalog tenant      {}", platform.tenant_app_catalog_url(TENANT_NAME));
  println!("url app catalog tenant app  {}", platform.tenant_app_catalog_app_url(TENANT_NAME, "kpn", APP_NAME));
  println!("url console                 {}", platform.console_url());
  println!("url console tenant          {}", platform.tenant_console_url(TENANT_NAME));
  println!("url console tenant app      {}", platform.tenant_app_console_url(TENANT_NAME, SERVICE_NAME));
  println!("url console tenant service  {}", platform.tenant_service_console_url(TENANT_NAME, SERVICE_NAME));
  println!("url data catalog tenant     {}", platform.tenant_data_catalog_url(TENANT_NAME));
  println!("url monitoring tenant       {}", platform.tenant_monitoring_url(TENANT_NAME));
  println!("url swagger                 {}", platform.swagger_url());
  println!("url tracing                 {}", platform.tracing_url());
}
