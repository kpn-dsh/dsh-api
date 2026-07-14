use dsh_api::error::{DshApiError, DshApiResult};
use dsh_api::platform::{DshPlatform, VhostZone};
use std::sync::LazyLock;

static DEVLZ: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("devlz"));
static NPLZ: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("nplz"));
static POC: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("poc"));
static PROD: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("prod"));
static PRODAZ: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("prodaz"));
static PRODLZ: LazyLock<DshPlatform> = LazyLock::new(|| DshPlatform::new("prodlz"));

#[test]
fn test_platform_from_domain() {
  assert_eq!(
    DshPlatform::from_domain("dsh-k8s.np.aws.kpn.com").unwrap().unwrap(),
    (DEVLZ.clone(), VhostZone::Public)
  );
  assert_eq!(
    DshPlatform::from_domain("dsh-k8s.np.aws.kpn.org").unwrap().unwrap(),
    (DEVLZ.clone(), VhostZone::Private)
  );
  assert_eq!(DshPlatform::from_domain("dsh.np.aws.kpn.com").unwrap().unwrap(), (NPLZ.clone(), VhostZone::Public));
  assert_eq!(DshPlatform::from_domain("dsh.np.aws.kpn.org").unwrap().unwrap(), (NPLZ.clone(), VhostZone::Private));
  assert_eq!(DshPlatform::from_domain("poc.kpn-dsh.com").unwrap().unwrap(), (POC.clone(), VhostZone::Public));
  assert_eq!(DshPlatform::from_domain("prod.aws.kpn.com").unwrap().unwrap(), (PRODLZ.clone(), VhostZone::Public));
  assert_eq!(DshPlatform::from_domain("prod.aws.kpn.org").unwrap().unwrap(), (PRODLZ.clone(), VhostZone::Private));
  assert_eq!(DshPlatform::from_domain("az.kpn-dsh.com").unwrap().unwrap(), (PRODAZ.clone(), VhostZone::Public));
  assert!(DshPlatform::from_domain("google.com").unwrap().is_none());
  assert!(DshPlatform::from_domain("google.org").unwrap().is_none());
}

#[test]
fn test_platform_from_domain_error() {
  assert!(DshPlatform::from_domain("np.aws.kpn.com").is_err());
  assert!(DshPlatform::from_domain("np.aws.kpn.org").is_err());
  assert!(DshPlatform::from_domain("kpn-dsh.com").is_err());
}

#[test]
fn test_domain() {
  assert_eq!(DEVLZ.domain(VhostZone::Private).unwrap(), "dev.dsh-k8s.np.aws.kpn.org");
  assert_eq!(DEVLZ.domain(VhostZone::Public).unwrap(), "dev.dsh-k8s.np.aws.kpn.com");
  assert_eq!(NPLZ.domain(VhostZone::Private).unwrap(), "dsh-dev.dsh.np.aws.kpn.org");
  assert_eq!(NPLZ.domain(VhostZone::Public).unwrap(), "dsh-dev.dsh.np.aws.kpn.com");
  assert!(POC.domain(VhostZone::Private).is_err());
  assert_eq!(POC.domain(VhostZone::Public).unwrap(), "poc.kpn-dsh.com");
  assert!(PROD.domain(VhostZone::Private).is_err());
  assert_eq!(PROD.domain(VhostZone::Public).unwrap(), "kpn-dsh.com");
  assert!(PRODAZ.domain(VhostZone::Private).is_err());
  assert_eq!(PRODAZ.domain(VhostZone::Public).unwrap(), "az.kpn-dsh.com");
  assert_eq!(PRODLZ.domain(VhostZone::Private).unwrap(), "dsh-prod.dsh.prod.aws.kpn.org");
  assert_eq!(PRODLZ.domain(VhostZone::Public).unwrap(), "dsh-prod.dsh.prod.aws.kpn.com");
}

#[test]
fn test_proxy_common_name() {
  assert_eq!(
    NPLZ.proxy_common_name("my-proxy", "my-tenant", VhostZone::Private).unwrap(),
    "my-proxy.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  );
  assert_eq!(
    NPLZ.proxy_common_name("my-proxy", "my-tenant", VhostZone::Public).unwrap(),
    "my-proxy.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  );
  assert!(PROD.proxy_common_name("my-proxy", "my-tenant", VhostZone::Private).is_err());
  assert_eq!(
    PROD.proxy_common_name("my-proxy", "my-tenant", VhostZone::Public).unwrap(),
    "my-proxy.kafka.my-tenant.kpn-dsh.com"
  );
}

#[test]
fn test_proxy_broker_vhost() {
  assert_eq!(
    NPLZ.proxy_vhost("my-tenant", "my-proxy", VhostZone::Private, 2).unwrap(),
    "my-proxy-2.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  );
  assert_eq!(
    NPLZ.proxy_vhost("my-tenant", "my-proxy", VhostZone::Public, 2).unwrap(),
    "my-proxy-2.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  );
  assert!(PROD.proxy_vhost("my-tenant", "my-proxy", VhostZone::Private, 2).is_err());
  assert_eq!(
    PROD.proxy_vhost("my-tenant", "my-proxy", VhostZone::Public, 2).unwrap(),
    "my-proxy-2.kafka.my-tenant.kpn-dsh.com"
  );
}

#[test]
fn test_proxy_consumer_group() {
  assert_eq!(NPLZ.proxy_consumer_group("my-tenant", "my-proxy", 2), "my-tenant_my-proxy_2".to_string());
}

#[test]
fn test_proxy_consumer_name_acl_group() {
  assert_eq!(
    NPLZ.proxy_consumer_group_acl("my-tenant", "my-proxy", "my-acl-group", 2),
    "my-tenant.my-acl-group_my-proxy_2".to_string()
  );
}

#[test]
fn test_proxy_schema_store_vhost() {
  assert_eq!(
    NPLZ.proxy_schema_store_vhost("my-tenant", "my-proxy", VhostZone::Private).unwrap(),
    "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org".to_string()
  );
  assert_eq!(
    NPLZ.proxy_schema_store_vhost("my-tenant", "my-proxy", VhostZone::Public).unwrap(),
    "my-proxy-schema-store.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com".to_string()
  );
  assert!(PROD.proxy_schema_store_vhost("my-tenant", "my-proxy", VhostZone::Private).is_err());
  assert_eq!(
    PROD.proxy_schema_store_vhost("my-tenant", "my-proxy", VhostZone::Public).unwrap(),
    "my-proxy-schema-store.kafka.my-tenant.kpn-dsh.com".to_string()
  );
}

#[test]
fn test_proxy_vhost_domain() {
  assert_eq!(
    NPLZ.proxy_vhost_domain("my-tenant", VhostZone::Private).unwrap(),
    "kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org"
  );
  assert_eq!(
    NPLZ.proxy_vhost_domain("my-tenant", VhostZone::Public).unwrap(),
    "kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com"
  );
  assert!(PROD.proxy_vhost_domain("my-tenant", VhostZone::Private).is_err());
  assert_eq!(PROD.proxy_vhost_domain("my-tenant", VhostZone::Public).unwrap(), "kafka.my-tenant.kpn-dsh.com");
}

#[test]
fn test_public_domain() {
  assert_eq!(NPLZ.public_domain(), "dsh-dev.dsh.np.aws.kpn.com".to_string());
}

#[test]
fn test_private_domain() {
  assert_eq!(DEVLZ.private_domain().unwrap(), "dev.dsh-k8s.np.aws.kpn.org");
  assert_eq!(NPLZ.private_domain().unwrap(), "dsh-dev.dsh.np.aws.kpn.org");
  assert!(POC.private_domain().is_none());
  assert!(PROD.private_domain().is_none());
  assert!(PRODAZ.private_domain().is_none());
  assert_eq!(PRODLZ.private_domain().unwrap(), "dsh-prod.dsh.prod.aws.kpn.org");
}

#[test]
fn test_public_vhost_domain() {
  assert_eq!(DEVLZ.public_domain(), "dev.dsh-k8s.np.aws.kpn.com");
  assert_eq!(NPLZ.public_domain(), "dsh-dev.dsh.np.aws.kpn.com");
  assert_eq!(POC.public_domain(), "poc.kpn-dsh.com");
  assert_eq!(PROD.public_domain(), "kpn-dsh.com");
  assert_eq!(PRODAZ.public_domain(), "az.kpn-dsh.com");
  assert_eq!(PRODLZ.public_domain(), "dsh-prod.dsh.prod.aws.kpn.com");
}

#[test]
fn test_validate_vhost_domain() {
  static VHOST_DOMAINS: LazyLock<Vec<(&DshPlatform, &str, DshApiResult<(String, Option<String>, bool, VhostZone)>)>> = LazyLock::new(|| {
    vec![
      (
        &NPLZ,
        "my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Private)),
      ),
      (
        &NPLZ,
        "my-subdomain.my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Ok(("my-subdomain.my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Private)),
      ),
      (
        &NPLZ,
        "my-subsubdomain.my-subdomain.my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Ok((
          "my-subsubdomain.my-subdomain.my-vhost".to_string(),
          Some("my-tenant".to_string()),
          false,
          VhostZone::Private,
        )),
      ),
      (
        &NPLZ,
        "vhost.dsh-dev.dsh.np.aws.kpn.org",
        Ok(("vhost".to_string(), None, false, VhostZone::Private)),
      ),
      (
        &NPLZ,
        "my-broker-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Ok(("my-broker-0".to_string(), Some("my-tenant".to_string()), true, VhostZone::Private)),
      ),
      (
        &NPLZ,
        "my-broker-subdomain-0.my-broker-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Ok((
          "my-broker-subdomain-0.my-broker-0".to_string(),
          Some("my-tenant".to_string()),
          true,
          VhostZone::Private,
        )),
      ),
      (
        &NPLZ,
        "my-vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.com",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Public)),
      ),
      (&NPLZ, "vhost.dsh-dev.dsh.np.aws.kpn.com", Ok(("vhost".to_string(), None, false, VhostZone::Public))),
      (
        &NPLZ,
        "my-broker-0.kafka.my-tenant.dsh-dev.dsh.np.aws.kpn.com",
        Ok(("my-broker-0".to_string(), Some("my-tenant".to_string()), true, VhostZone::Public)),
      ),
      (
        &POC,
        "my-vhost.my-tenant.poc.kpn-dsh.com",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Public)),
      ),
      (
        &PROD,
        "my-vhost.my-tenant.kpn-dsh.com",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Public)),
      ),
      (
        &PRODLZ,
        "my-vhost.my-tenant.dsh-prod.dsh.prod.aws.kpn.com",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Public)),
      ),
      (
        &PRODAZ,
        "my-vhost.my-tenant.az.kpn-dsh.com",
        Ok(("my-vhost".to_string(), Some("my-tenant".to_string()), false, VhostZone::Public)),
      ),
      (
        &NPLZ,
        "my_vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org",
        Err(DshApiError::Conversion { message: "vhost domain 'my_vhost.my-tenant.dsh-dev.dsh.np.aws.kpn.org' not valid for platform np-aws-lz-dsh".to_string() }),
      ),
      (
        &NPLZ,
        "my-vhost.my-tenant.google.com",
        Err(DshApiError::Conversion { message: "vhost domain 'my-vhost.my-tenant.google.com' not valid for platform np-aws-lz-dsh".to_string() }),
      ),
    ]
  });

  for (platform_ut, vhost_domain_ut, result_ut) in VHOST_DOMAINS.iter() {
    assert_eq!(platform_ut.validate_vhost_domain(vhost_domain_ut), *result_ut);
  }
}
