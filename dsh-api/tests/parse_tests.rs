use dsh_api::parse::{parse_basic_authentication_string, parse_function, parse_function1, parse_function2, AuthString, ImageString};
use std::str::FromStr;

#[test]
fn test_display_auth_string() {
  assert_eq!(
    AuthString::basic(Some("my-realm"), "my-username").to_string(),
    "basic@my-realm:my-username".to_string()
  );
  assert_eq!(AuthString::basic(None::<String>, "my-username").to_string(), "basic@my-username".to_string());
  assert_eq!(
    AuthString::fwd("https://my-authentication-service.com", Some("my-headers")).to_string(),
    "fwd@https://my-authentication-service.com@my-headers".to_string()
  );
  assert_eq!(
    AuthString::fwd("https://my-authentication-service.com", None::<String>).to_string(),
    "fwd@https://my-authentication-service.com".to_string()
  );
  assert_eq!(AuthString::system_fwd("view,manage").to_string(), "sys-fwd@view,manage".to_string());
}

#[test]
fn test_parse_auth_string() {
  assert_eq!(
    AuthString::from_str("basic-auth@my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    AuthString::from_str("basic-auth@my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
  assert_eq!(
    AuthString::from_str("my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    AuthString::from_str("my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
  assert_eq!(
    AuthString::from_str("fwd-auth@https://my-authentication-service.com@my-headers"),
    Ok(AuthString::fwd("https://my-authentication-service.com", Some("my-headers".to_string())))
  );
  assert_eq!(AuthString::from_str("system-fwd-auth@view,manage"), Ok(AuthString::system_fwd("view,manage")));
}

#[test]
fn test_image_string() {
  let registry_image = ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string());
  assert_eq!(registry_image.id(), "my-image".to_string());
  assert_eq!(registry_image.source(), "harbor");
  assert_eq!(registry_image.tenant(), "my-tenant".to_string());
  assert_eq!(registry_image.version(), "0.0.1".to_string());
  let app_image = ImageString::app(
    "draft".to_string(),
    "kpn".to_string(),
    "my-tenant".to_string(),
    "my-image".to_string(),
    "0.0.1".to_string(),
  );
  assert_eq!(app_image.id(), "my-image".to_string());
  assert_eq!(app_image.source(), "app-catalog");
  assert_eq!(app_image.tenant(), "my-tenant".to_string());
  assert_eq!(app_image.version(), "0.0.1".to_string());
}

#[test]
fn test_display_image_string() {
  let registry_image = ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string());
  assert_eq!(registry_image.to_string(), "registry:my-tenant:my-image:0.0.1".to_string());
  let app_image = ImageString::app(
    "draft".to_string(),
    "kpn".to_string(),
    "my-tenant".to_string(),
    "my-image".to_string(),
    "0.0.1".to_string(),
  );
  assert_eq!(app_image.to_string(), "app:draft:kpn:my-tenant:my-image:0.0.1".to_string());
}

#[test]
fn test_parse_image_string() {
  assert_eq!(
    ImageString::from("registry.cp.kpn-dsh.com/my-tenant/my-image:0.0.1"),
    ImageString::registry("my-tenant".to_string(), "my-image".to_string(), "0.0.1".to_string())
  );
  assert_eq!(
    ImageString::from("APPCATALOG_REGISTRY/dsh-appcatalog/tenant/my-tenant/1234/1234/draft/kpn/my-image:0.0.1"),
    ImageString::app(
      "draft".to_string(),
      "kpn".to_string(),
      "my-tenant".to_string(),
      "my-image".to_string(),
      "0.0.1".to_string()
    )
  );
  assert_eq!(
    ImageString::from("APPCATALOG_REGISTRY/dsh-appcatalog/tenant/my-tenant/1234/1234/release/klarrio/whoami:1.6.1"),
    ImageString::app(
      "release".to_string(),
      "klarrio".to_string(),
      "my-tenant".to_string(),
      "whoami".to_string(),
      "1.6.1".to_string()
    )
  );
  assert_eq!(
    ImageString::from("registry.cp.kpn-dsh.com/greenbox-dev/postgres:pooria.20241211.1"),
    ImageString::registry("greenbox-dev".to_string(), "postgres".to_string(), "pooria.20241211.1".to_string())
  );
  assert_eq!(
    ImageString::from("registry/greenbox-dev/postgres:pooria.20241211.1"),
    ImageString::Unrecognized("registry/greenbox-dev/postgres:pooria.20241211.1".to_string())
  );
}

#[test]
fn test_parse_function1() {
  let valids_under_test = vec![
    ("{function('par')}", "function", "par"),
    ("{function('p.a_r-1')}", "function", "p.a_r-1"),
    ("{ function( 'par' ) }", "function", "par"),
    ("{function('')}", "function", ""),
  ];
  for (valid_string, function, parameter) in valids_under_test {
    assert_eq!(parse_function1(valid_string, function), Ok(parameter));
  }
  let invalids_under_test = vec![
    ("{function('par')}", "other"),
    ("{('par')}", ""),
    ("{function(par)}", "function"),
    ("{function('p$ar')}", "function"),
    ("{function()}", "function"),
    ("{function('par1','par2')}", "function"),
  ];
  for (invalid_string, function) in invalids_under_test {
    assert!(parse_function1(invalid_string, function).is_err_and(|error| error == format!("invalid {} string (\"{}\")", function, invalid_string)));
  }
}

#[test]
fn test_parse_function2() {
  let valids_under_test = vec![
    ("{function('par1','par2')}", "function", ("par1", "par2")),
    ("{function('p.a_r-1','p.a_r-2')}", "function", ("p.a_r-1", "p.a_r-2")),
    ("{ function( 'par1' , 'par2' ) }", "function", ("par1", "par2")),
    ("{function('','')}", "function", ("", "")),
  ];
  for (valid_string, function, parameter) in valids_under_test {
    assert_eq!(parse_function2(valid_string, function), Ok(parameter));
  }
  let invalids_under_test = vec![
    ("{function('par1','par2')}", "other"),
    ("{('par1','par2')}", ""),
    ("{function(par1,par2)}", "function"),
    ("{function('par1',par2)}", "function"),
    ("{function(par1,'par2')}", "function"),
    ("{function('p$ar1','p$ar2')}", "function"),
    ("{function('par1','p$ar2')}", "function"),
    ("{function('p$ar1','par2')}", "function"),
    ("{function()}", "function"),
    ("{function('par')}", "function"),
  ];
  for (invalid_string, function) in invalids_under_test {
    assert!(parse_function2(invalid_string, function).is_err_and(|error| error == format!("invalid {} string (\"{}\")", function, invalid_string)));
  }
}

#[test]
fn test_parse_function() {
  let valids_under_test = vec![
    ("{function('par')}", "function", ("par", None)),
    ("{function('p.a_r-1')}", "function", ("p.a_r-1", None)),
    ("{ function( 'par' ) }", "function", ("par", None)),
    ("{function('')}", "function", ("", None)),
    ("{function('par1','par2')}", "function", ("par1", Some("par2"))),
    ("{function('p.a_r-1','p.a_r-2')}", "function", ("p.a_r-1", Some("p.a_r-2"))),
    ("{ function( 'par1' , 'par2' ) }", "function", ("par1", Some("par2"))),
    ("{function('','')}", "function", ("", Some(""))),
  ];
  for (valid_string, function, parameter) in valids_under_test {
    assert_eq!(parse_function(valid_string, function), Ok(parameter));
  }
  let invalids_under_test = vec![
    ("{function('par')}", "other"),
    ("{('par')}", ""),
    ("{function(par)}", "function"),
    ("{function('p$ar')}", "function"),
    ("{function()}", "function"),
    ("{function('par1','par2')}", "other"),
    ("{('par1','par2')}", ""),
    ("{function(par1,par2)}", "function"),
    ("{function('par1',par2)}", "function"),
    ("{function(par1,'par2')}", "function"),
    ("{function('p$ar1','p$ar2')}", "function"),
    ("{function('par1','p$ar2')}", "function"),
    ("{function('p$ar1','par2')}", "function"),
    ("{function()}", "function"),
  ];
  for (invalid_string, function) in invalids_under_test {
    assert!(parse_function(invalid_string, function).is_err_and(|error| error == format!("invalid {} string (\"{}\")", function, invalid_string)));
  }
}

#[test]
fn test_parse_basic_authentication_string() {
  assert_eq!(
    parse_basic_authentication_string("my-realm:my-username:$password-hash/"),
    Ok(AuthString::basic(Some("my-realm"), "my-username"))
  );
  assert_eq!(
    parse_basic_authentication_string("my-username:$password-hash/"),
    Ok(AuthString::basic(None::<String>, "my-username"))
  );
}
