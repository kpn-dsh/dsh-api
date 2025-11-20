use dsh_api::types::{ApplicationSecret, ManagedStreamId, ManagedTenant, ManagedTenantServices, ManagedTenantServicesName};

#[test]
fn test_application_secret_new() {
  let secret = ApplicationSecret::new("secret_name", &["KEY1", "KEY2"]);
  assert_eq!(secret.name, "secret_name");
  assert_eq!(secret.injections.len(), 2);
  assert_eq!(secret.injections.first().unwrap().get("env").unwrap(), "KEY1");
  assert_eq!(secret.injections.get(1).unwrap().get("env").unwrap(), "KEY2");
}

#[test]
fn test_managed_stream_id_new() {
  let managed_stream_id = ManagedStreamId::new("manager", "stream-id");
  assert_eq!(*managed_stream_id, "manager---stream-id");
}

#[test]
#[should_panic]
fn test_managed_stream_id_new_panic_on_empty_manager() {
  ManagedStreamId::new("", "stream-id");
}

#[test]
#[should_panic(expected = "manager---stream_id is not a valid managed stream id")]
fn test_managed_stream_id_new_panic_on_underscore() {
  ManagedStreamId::new("manager", "stream_id");
}

#[test]
fn test_managed_tenant_new() {
  let managed_tenant = ManagedTenant::new("manager", "tenant");
  assert_eq!(managed_tenant.manager, "manager");
  assert_eq!(managed_tenant.name, "tenant");
  assert_eq!(managed_tenant.services.len(), 3);
  assert_eq!(
    managed_tenant.services,
    vec![
      ManagedTenantServices { enabled: true, name: ManagedTenantServicesName::Monitoring },
      ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Tracing },
      ManagedTenantServices { enabled: false, name: ManagedTenantServicesName::Vpn },
    ]
  );
}
