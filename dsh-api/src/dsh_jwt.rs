//! # Models DSH tokens
//!
//! A `DshJwt` struct models some DSH specifics in the used Json Web Tokens.

use crate::error::{DshApiError, DshApiResult};
use base64::engine::general_purpose::STANDARD_NO_PAD;
use base64::Engine;
use itertools::Itertools;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DshJwt {
  pub header: DshJwtHeader,
  pub payload: DshJwtPayload,
  pub tenant_permissions: Option<Vec<DshPermission>>,
}

impl DshJwt {
  /// Returns expected time before token expires.
  pub fn expires_in(&self) -> Option<i64> {
    self.payload.expires_in()
  }

  /// Whether token is expired.
  pub fn expired(&self) -> Option<bool> {
    self.payload.expired()
  }

  /// Returns list of authorized tenants.
  pub fn authorized_tenants(&self) -> Option<Vec<&str>> {
    self
      .tenant_permissions
      .as_ref()
      .map(|permissions| permissions.iter().map(|permission| permission.tenant.as_str()).collect_vec())
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DshJwtHeader {
  // Rfc7519
  #[serde(rename = "typ")]
  pub typ: String,
  #[serde(rename = "alg")]
  pub algorithm: String,
  #[serde(rename = "kid")]
  pub kid: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DshJwtPayload {
  /// Issuer claim (rfc7519 "iss").
  #[serde(rename = "iss")]
  pub issuer: Option<String>,
  /// Subject claim (rfc7519 "sub").
  #[serde(rename = "sub")]
  pub subject: Option<String>,
  /// Audience claim (rfc7519 "aud").
  #[serde(rename = "aud")]
  pub audience: Option<String>,
  /// Expiration time claim (rfc7519 "exp").
  #[serde(rename = "exp")]
  pub expiration_time: Option<i64>,
  /// Not before claim (rfc7519 "nbf").
  #[serde(rename = "nbf")]
  pub not_before: Option<i64>,
  /// Issued at claim (rfc7519 "iat").
  #[serde(rename = "iat")]
  pub issued_at: Option<i64>,
  /// Jwt id claim (rfc7519 "jti").
  #[serde(rename = "jti")]
  pub jwt_id: Option<String>,

  #[serde(rename = "auth_time")]
  pub authentication_time: Option<i64>,
  #[serde(rename = "azp")]
  pub authorized_party: Option<String>,
  #[serde(rename = "clientAddress")]
  pub client_address: Option<String>,
  #[serde(rename = "clientHost")]
  pub client_host: Option<String>,
  pub client_id: Option<String>,
  /// Permission representations claim (dsh specific "dsh_perms").
  #[serde(rename = "dsh_perms")]
  pub dsh_permission_representations: Option<Vec<String>>,
  pub email: Option<String>,
  pub email_verified: Option<bool>,
  pub family_name: Option<String>,
  pub given_name: Option<String>,
  pub name: Option<String>,
  pub preferred_username: Option<String>,
  pub scope: Option<String>,
  #[serde(rename = "sid")]
  pub session_id: Option<String>,
  #[serde(rename = "typ")]
  pub token_type: Option<String>,
}

impl DshJwtPayload {
  /// Returns a list of the available rfc7519 claims with their values.
  ///
  /// # Returns
  /// * List of tuples consisting of the claim name and value.
  pub fn rfc7519_claims(&self) -> Vec<(&str, String)> {
    vec![
      ("iss", self.issuer.as_ref().map(|issuer| issuer.to_string())),
      ("sub", self.subject.as_ref().map(|subject| subject.to_string())),
      ("aud", self.audience.as_ref().map(|audience| audience.to_string())),
      ("exp", self.expiration_time.map(|expiration_time| expiration_time.to_string())),
      ("nbf", self.not_before.map(|not_before| not_before.to_string())),
      ("iat", self.issued_at.map(|issued_at| issued_at.to_string())),
      ("jti", self.jwt_id.as_ref().map(|jwt_id| jwt_id.to_string())),
    ]
    .into_iter()
    .filter_map(|(claim, value)| value.map(|v| (claim, v)))
    .collect_vec()
  }

  /// Returns expected time before token expires.
  pub fn expires_in(&self) -> Option<i64> {
    self
      .expiration_time
      .and_then(|expiration_time| SystemTime::now().duration_since(UNIX_EPOCH).ok().map(|now| expiration_time - now.as_secs() as i64))
  }

  /// Whether token is expired.
  pub fn expired(&self) -> Option<bool> {
    self.expires_in().map(|expires_in| expires_in <= 0)
  }

  /// Returns a list of permissions.
  pub fn permissions(&self) -> DshApiResult<Vec<DshPermission>> {
    match &self.dsh_permission_representations {
      Some(representations) => {
        let mut permissions = representations
          .iter()
          .map(|representation| DshPermission::from_str(representation))
          .collect::<Result<Vec<_>, _>>()?;
        permissions.sort_by(|permission_a, permission_b| permission_a.tenant.cmp(&permission_b.tenant));
        Ok(permissions)
      }
      None => Err(DshApiError::NotFound { message: Some("token does not contain permissions".to_string()) }),
    }
  }

  /// Returns a list with the names of authenticated tenants.
  pub fn authenticated_tenants(&self) -> DshApiResult<Vec<String>> {
    Ok(self.permissions()?.iter().map(|permission| permission.tenant.to_string()).collect_vec())
  }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DshPermission {
  pub realm: String,
  pub tenant: String,
  pub manage: bool,
  pub view: bool,
}

impl DshPermission {
  pub fn new(realm: String, tenant: String) -> Self {
    Self { realm, tenant, manage: false, view: false }
  }
}

impl Display for DshJwt {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      match serde_json::to_string_pretty(self) {
        Ok(json) => write!(f, "{}", json),
        Err(_) => write!(f, "[json-error]"),
      }
    } else {
      write!(f, "{}|{}", self.header, self.payload)
    }
  }
}

/// Split json web token in header and payload json.
///
/// # Parameter
/// * `jwt` - Json web token.
///
/// # Returns
/// * Ok((String, String)) - Tuple with header and payload json strings.
pub fn jwt_into_header_payload_json(jwt: &str) -> DshApiResult<(String, String)> {
  let (header_part, payload_part, _) = split_jwt_to_parts(jwt)?;
  let header_json = decode_part("header", header_part)?;
  let payload_json = decode_part("payload", payload_part)?;
  Ok((header_json, payload_json))
}

/// Parse json web token string in `DshJwtHeader` and `DshJwtPayload` structs.
///
/// # Parameter
/// * `jwt` - Json web token string.
///
/// # Returns
/// * Ok((DshJwtHeader, DshJwtPayload)) - Tuple with header and payload structs.
pub fn jwt_into_header_payload(jwt: &str) -> DshApiResult<(DshJwtHeader, DshJwtPayload)> {
  let (header_json, payload_json) = jwt_into_header_payload_json(jwt)?;
  let header = serde_json::from_str::<DshJwtHeader>(&header_json).map_err(|json_error| DshApiError::conversion(format!("header contains invalid json ({})", json_error)))?;
  let payload = serde_json::from_str::<DshJwtPayload>(&payload_json).map_err(|json_error| DshApiError::conversion(format!("payload contains invalid json ({})", json_error)))?;
  Ok((header, payload))
}

fn split_jwt_to_parts(jwt: &str) -> DshApiResult<(&str, &str, &str)> {
  let parts: Vec<&str> = jwt.split('.').collect();
  if parts.len() != 3 {
    Err(DshApiError::conversion("invalid jwt token"))
  } else {
    Ok((parts[0], parts[1], parts[2]))
  }
}

fn decode_part(kind: &str, part: &str) -> DshApiResult<String> {
  STANDARD_NO_PAD
    .decode(part.as_bytes())
    .map_err(|_| DshApiError::conversion(format!("could not decode {}", kind)))
    .and_then(|decoded_header| String::from_utf8(decoded_header).map_err(|_| DshApiError::conversion(format!("{} contains invalid utf8", kind))))
}

impl FromStr for DshJwt {
  type Err = DshApiError;

  fn from_str(token: &str) -> DshApiResult<Self> {
    let (header, payload) = jwt_into_header_payload(token)?;
    match &payload.dsh_permission_representations {
      Some(representations) => {
        let mut tenant_permissions_map: HashMap<String, DshPermission> = HashMap::new();
        for representation in representations {
          DshPermission::from_str(representation).map(|dsh_permission| {
            let manage = dsh_permission.manage;
            let view = dsh_permission.view;
            let mapped = tenant_permissions_map.entry(dsh_permission.tenant.to_string()).or_insert_with(|| dsh_permission);
            if manage {
              mapped.manage = true;
            }
            if view {
              mapped.view = true;
            }
          })?;
        }
        let mut tenant_permissions: Vec<DshPermission> = Vec::from_iter(tenant_permissions_map.into_values());
        tenant_permissions.sort_by(|dsh_permission_a, dsh_permission_b| dsh_permission_a.tenant.cmp(&dsh_permission_b.tenant));
        Ok(DshJwt { header, payload, tenant_permissions: Some(tenant_permissions) })
      }
      None => Ok(DshJwt { header, payload, tenant_permissions: None }),
    }
  }
}

impl Display for DshJwtHeader {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      match serde_json::to_string_pretty(self) {
        Ok(json) => write!(f, "{}", json),
        Err(_) => write!(f, "[json-error]"),
      }
    } else {
      write!(f, "{}:{}", self.typ, self.algorithm)
    }
  }
}

impl Display for DshJwtPayload {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if f.alternate() {
      match serde_json::to_string_pretty(self) {
        Ok(json) => write!(f, "{}", json),
        Err(_) => write!(f, "[json-error]"),
      }
    } else {
      match self.expires_in() {
        Some(expires_in) => write!(
          f,
          "{}:{}:{}",
          self.token_type.as_deref().unwrap_or(""),
          self.preferred_username.as_deref().unwrap_or(""),
          expires_in
        ),

        None => write!(
          f,
          "{}:{}",
          self.token_type.as_deref().unwrap_or(""),
          self.preferred_username.as_deref().unwrap_or(""),
        ),
      }
    }
  }
}

impl Display for DshPermission {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "manage:{}:{}:{}",
      self.realm,
      self.tenant,
      [if self.manage { Some("manage") } else { None }, if self.view { Some("view") } else { None }]
        .iter()
        .flatten()
        .join("+")
    )
  }
}

impl FromStr for DshPermission {
  type Err = DshApiError;

  fn from_str(permission_representation: &str) -> DshApiResult<Self> {
    static VALUE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^manage:([a-z][a-z0-9-]*):([a-z][a-z0-9-]*):(manage|view)$").unwrap());
    match VALUE_REGEX.captures(permission_representation) {
      Some(captures) => {
        let kind = captures.get(3).map(|tenant_match| tenant_match.as_str()).unwrap_or_default();
        Ok(Self {
          realm: captures.get(1).map(|realm_match| realm_match.as_str()).unwrap().to_string(),
          tenant: captures.get(2).map(|tenant_match| tenant_match.as_str()).unwrap().to_string(),
          manage: kind == "manage",
          view: kind == "view",
        })
      }
      None => Err(DshApiError::conversion("illegal permission representation")),
    }
  }
}

#[test]
fn test_dsh_permission_from_str() {
  let dsh_permission = DshPermission::from_str("manage:dev-lz-dsh:greenbox-dev:view").unwrap();
  assert_eq!(dsh_permission.realm, "dev-lz-dsh");
  assert_eq!(dsh_permission.tenant, "greenbox-dev");
  assert_eq!(dsh_permission.manage, false);
  assert_eq!(dsh_permission.view, true);
}

#[test]
fn test_dsh_permission_display() {
  assert_eq!(
    DshPermission::from_str("manage:dev-lz-dsh:greenbox-dev:view").unwrap().to_string(),
    "manage:dev-lz-dsh:greenbox-dev:view"
  );
  assert_eq!(
    DshPermission { realm: "my-realm".to_string(), tenant: "my-tenant".to_string(), manage: true, view: true }.to_string(),
    "manage:my-realm:my-tenant:manage+view"
  );
}
