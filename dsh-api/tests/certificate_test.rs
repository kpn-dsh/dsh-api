use crate::common::{get_client, print_header};

#[path = "common.rs"]
mod common;

use dsh_api::UsedBy;
use dsh_api_generated::types::{AllocationStatus, Certificate, CertificateStatus};

// create_certificate(certificate_id, certificate)
// delete_certificate(certificate_id)

static CERTIFICATE_ID: &str = "broker-kafka-proxy-certificate";

#[tokio::test]
async fn test_get_certificate() {
  if let Ok(client) = get_client().await {
    let certificate: CertificateStatus = client.get_certificate(CERTIFICATE_ID).await.unwrap();
    print_header("get_certificate");
    println!("{:#?}", certificate);
  }
}

#[tokio::test]
async fn test_get_certificate_allocation_status() {
  if let Ok(client) = get_client().await {
    let allocation_status: AllocationStatus = client.get_certificate_allocation_status(CERTIFICATE_ID).await.unwrap();
    print_header("get_certificate_allocation_status");
    println!("{:#?}", allocation_status);
  }
}

#[tokio::test]
async fn test_get_certificate_configuration() {
  if let Ok(client) = get_client().await {
    let certificate: Certificate = client.get_certificate_configuration(CERTIFICATE_ID).await.unwrap();
    print_header("get_certificate_configuration");
    println!("{:#?}", certificate);
  }
}

#[tokio::test]
async fn test_get_certificate_ids() {
  if let Ok(client) = get_client().await {
    let certificate_ids: Vec<String> = client.get_certificate_ids().await.unwrap();
    print_header("get_certificate_ids");
    println!("{:#?}", certificate_ids);
  }
}

#[tokio::test]
async fn test_get_certificate_with_usage() {
  if let Ok(client) = get_client().await {
    let certificate_with_usage: (CertificateStatus, Vec<UsedBy>) = client.get_certificate_with_usage(CERTIFICATE_ID).await.unwrap();
    print_header("get_certificate_with_usage");
    println!("{:#?}", certificate_with_usage);
  }
}

#[tokio::test]
async fn test_list_certificates_with_usage() {
  if let Ok(client) = get_client().await {
    let certificates_with_usage: Vec<(String, CertificateStatus, Vec<UsedBy>)> = client.list_certificates_with_usage().await.unwrap();
    print_header("list_certificates_with_usage");
    println!("{:#?}", certificates_with_usage);
  }
}
