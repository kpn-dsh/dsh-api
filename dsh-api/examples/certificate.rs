use crate::common::print_header;
use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
use dsh_api::types::AllocationStatus;
use dsh_api::types::{Certificate, CertificateStatus};
use dsh_api::UsedBy;

#[path = "common.rs"]
mod common;

static CERTIFICATE_ID: &str = "broker-kafka-proxy-certificate";

#[tokio::main]
async fn main() -> Result<(), String> {
  let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;

  print_header("get_certificate");
  let certificate: CertificateStatus = client.get_certificate(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate);

  print_header("get_certificate_allocation_status");
  let allocation_status: AllocationStatus = client.get_certificate_allocation_status(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", allocation_status);

  print_header("get_certificate_configuration");
  let certificate: Certificate = client.get_certificate_configuration(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate);

  print_header("get_certificate_ids");
  let certificate_ids: Vec<String> = client.list_certificate_ids().await.unwrap();
  println!("{:#?}", certificate_ids);

  print_header("get_certificate_with_usage");
  let certificate_with_usage: (CertificateStatus, Vec<UsedBy>) = client.get_certificate_with_usage(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate_with_usage);

  print_header("list_certificates_with_usage");
  let certificates_with_usage: Vec<(String, CertificateStatus, Vec<UsedBy>)> = client.list_certificates_with_usage().await.unwrap();
  println!("{:#?}", certificates_with_usage);

  Ok(())
}
