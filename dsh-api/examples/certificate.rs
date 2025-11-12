#[allow(unused_imports)]
#[path = "common.rs"]
mod common;

use crate::common::{initialize_logger, print_header};
use dsh_api::dsh_api_client_factory::DshApiClientFactory;
use dsh_api::types::AllocationStatus;
use dsh_api::types::{Certificate, CertificateStatus};
use dsh_api::DependantApp;

static CERTIFICATE_ID: &str = "broker-kafka-proxy-certificate";

#[tokio::main]
async fn main() -> Result<(), String> {
  initialize_logger();

  let client = DshApiClientFactory::default().client().await?;

  print_header("get_certificate");
  let certificate: CertificateStatus = client.get_certificate(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate);

  print_header("get_certificate_allocation_status");
  let allocation_status: AllocationStatus = client.get_certificate_status(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", allocation_status);

  print_header("get_certificate_configuration");
  let certificate: Certificate = client.get_certificate_configuration(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate);

  print_header("get_certificate_ids");
  let certificate_ids: Vec<String> = client.get_certificate_ids().await.unwrap();
  println!("{:#?}", certificate_ids);

  print_header("get_certificate_with_usage");
  let certificate_with_dependants: (CertificateStatus, Vec<DependantApp>) = client.certificate_with_dependant_apps(CERTIFICATE_ID).await.unwrap();
  println!("{:#?}", certificate_with_dependants);

  print_header("list_certificates_with_usage");
  let certificates_with_dependants = client.certificates_with_dependant_apps().await.unwrap();
  println!("{:#?}", certificates_with_dependants);

  Ok(())
}
