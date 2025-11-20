use dsh_api::vhost::VhostString;
use std::str::FromStr;

#[test]
fn test_parse_vhost_string() {
  assert_eq!(
    VhostString::from_str("{ vhost('my-vhost-name') }"),
    Ok(VhostString::new("my-vhost-name", false, None::<String>, None::<String>))
  );
  assert_eq!(
    VhostString::from_str("{ vhost('my-vhost-name.kafka.my-tenant','public') }"),
    Ok(VhostString::new("my-vhost-name", true, Some("my-tenant"), Some("public")))
  );
}
