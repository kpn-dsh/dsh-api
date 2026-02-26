use dsh_api::secret::{is_system_id, is_system_name, normalize_secret_name};

#[test]
fn test_is_system_id() {
  assert!(is_system_id("!vpn-password"));
  assert!(is_system_id("!bucketaccess!greenbox-dev-bucket!identifier"));
  assert!(!is_system_id("system/vpn-password"));
  assert!(!is_system_id("system/bucketaccess/greenbox-dev-bucket/identifier"));
  assert!(!is_system_id("secretname"));
  assert!(!is_system_id("secret-name"));
  assert!(!is_system_id("secret_name"));
}

#[test]
fn test_is_system_name() {
  assert!(!is_system_name("!vpn-password"));
  assert!(!is_system_name("!bucketaccess!greenbox-dev-bucket!identifier"));
  assert!(is_system_name("system/vpn-password"));
  assert!(is_system_name("system/bucketaccess/greenbox-dev-bucket/identifier"));
  assert!(!is_system_name("secretname"));
  assert!(!is_system_name("secret-name"));
  assert!(!is_system_name("secret_name"));
}

#[test]
fn test_normalize_secret_name() {
  assert_eq!(
    normalize_secret_name("!vpn-password".to_string()),
    ("system/vpn-password".to_string(), Some("!vpn-password".to_string()))
  );
  assert_eq!(
    normalize_secret_name("!bucketaccess!greenbox-dev-bucket!identifier".to_string()),
    (
      "system/bucketaccess/greenbox-dev-bucket/identifier".to_string(),
      Some("!bucketaccess!greenbox-dev-bucket!identifier".to_string())
    )
  );
  assert_eq!(
    normalize_secret_name("system/vpn-password".to_string()),
    ("system/vpn-password".to_string(), Some("!vpn-password".to_string()))
  );
  assert_eq!(
    normalize_secret_name("system/bucketaccess/greenbox-dev-bucket/identifier".to_string()),
    (
      "system/bucketaccess/greenbox-dev-bucket/identifier".to_string(),
      Some("!bucketaccess!greenbox-dev-bucket!identifier".to_string())
    )
  );
  assert_eq!(normalize_secret_name("secretname".to_string()), ("secretname".to_string(), None));
  assert_eq!(normalize_secret_name("secret-name".to_string()), ("secret-name".to_string(), None));
  assert_eq!(normalize_secret_name("secret_name".to_string()), ("secret_name".to_string(), None));
}
