use crate::common::{get_client, print_header};

mod common;

static CERTIFICATE_ID: &str = "broker-kafka-proxy-certificate";

#[tokio::test]
async fn test_get_certificate() {
  if let Ok(client) = get_client().await {
    print_header("get_certificate");
    match client.get_certificate(CERTIFICATE_ID).await {
      Ok(certificate) => println!("{:#?}", certificate),
      Err(_) => println!("certificate {} does not exist", CERTIFICATE_ID),
    }
  }
}

#[tokio::test]
async fn test_get_certificate_allocation_status() {
  if let Ok(client) = get_client().await {
    print_header("get_certificate_allocation_status");
    match client.get_certificate_status(CERTIFICATE_ID).await {
      Ok(allocation_status) => println!("{:#?}", allocation_status),
      Err(_) => println!("certificate {} does not exist", CERTIFICATE_ID),
    }
  }
}

#[tokio::test]
async fn test_get_certificate_configuration() {
  if let Ok(client) = get_client().await {
    print_header("get_certificate_configuration");
    match client.get_certificate_configuration(CERTIFICATE_ID).await {
      Ok(certificate) => println!("{:#?}", certificate),
      Err(_) => println!("certificate {} does not exist", CERTIFICATE_ID),
    }
  }
}

#[tokio::test]
async fn test_get_certificate_ids() {
  if let Ok(client) = get_client().await {
    print_header("get_certificate_ids");
    match client.get_certificate_ids().await {
      Ok(certificate_ids) => println!("{:#?}", certificate_ids),
      Err(_) => println!("could not get certificate ids"),
    }
  }
}

#[tokio::test]
async fn test_certificate_with_dependants() {
  if let Ok(client) = get_client().await {
    print_header("get_certificate_with_usage");
    match client.certificate_with_dependant_apps(CERTIFICATE_ID).await {
      Ok(certificate_with_usage) => println!("{:#?}", certificate_with_usage),
      Err(_) => println!("certificate {} does not exist", CERTIFICATE_ID),
    }
  }
}

#[tokio::test]
async fn test_certificates_with_dependants() {
  if let Ok(client) = get_client().await {
    print_header("list_certificates_with_usage");
    match client.certificates_with_dependant_apps().await {
      Ok(certificates_with_usage) => println!("{:#?}", certificates_with_usage),
      Err(_) => println!("could not get certificates"),
    }
  }
}
