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
  pub tenant_permissions: Vec<DshPermission>,
}

impl DshJwt {
  pub fn expires_in(&self) -> i64 {
    self.payload.expires_in()
  }

  pub fn expired(&self) -> bool {
    self.payload.expired()
  }

  pub fn authorized_tenants(&self) -> Vec<&str> {
    self.tenant_permissions.iter().map(|permission| permission.tenant.as_str()).collect_vec()
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
  // Rfc7519
  #[serde(rename = "iss")]
  pub issuer: Option<String>,
  #[serde(rename = "sub")]
  pub subject: Option<String>,
  #[serde(rename = "aud")]
  pub audience: Option<String>,
  #[serde(rename = "exp")]
  pub expiration_time: Option<i64>,
  #[serde(rename = "nbf")]
  pub not_before: Option<i64>,
  #[serde(rename = "iat")]
  pub issued_at: Option<i64>,
  #[serde(rename = "jti")]
  pub jwt_id: Option<String>,

  // Dsh specific
  #[serde(rename = "auth_time")]
  pub authentication_time: Option<i64>,
  #[serde(rename = "azp")]
  pub authorized_party: Option<String>,
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
  pub const ISSUER: &'static str = "iss";
  pub const SUBJECT: &'static str = "sub";
  pub const AUDIENCE: &'static str = "aud";
  pub const EXPIRATION_TIME: &'static str = "exp";
  pub const NOT_BEFORE: &'static str = "nbf";
  pub const ISSUED_AT: &'static str = "iat";
  pub const JWT_ID: &'static str = "jti";

  pub const REGISTERED_CLAIM_TYPES: [&'static str; 7] = [Self::ISSUER, Self::SUBJECT, Self::AUDIENCE, Self::EXPIRATION_TIME, Self::NOT_BEFORE, Self::ISSUED_AT, Self::JWT_ID];

  pub fn registered_claims(&self) -> Vec<(&str, String)> {
    vec![
      (Self::ISSUER, self.issuer.clone().map(|issuer| issuer.to_string())),
      (Self::SUBJECT, self.subject.clone().map(|subject| subject.to_string())),
      (Self::AUDIENCE, self.audience.clone().map(|audience| audience.to_string())),
      (Self::EXPIRATION_TIME, self.expiration_time.map(|expiration_time| expiration_time.to_string())),
      (Self::NOT_BEFORE, self.not_before.map(|not_before| not_before.to_string())),
      (Self::ISSUED_AT, self.issued_at.map(|issued_at| issued_at.to_string())),
      (Self::JWT_ID, self.jwt_id.clone().map(|jwt_id| jwt_id.to_string())),
    ]
    .into_iter()
    .filter_map(|(claim, value)| value.map(|v| (claim, v)))
    .collect_vec()
  }

  pub fn expires_in(&self) -> i64 {
    self.expiration_time.unwrap_or_default() - (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64)
  }

  pub fn expired(&self) -> bool {
    self.expires_in() <= 0
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

impl FromStr for DshJwt {
  type Err = DshApiError;

  fn from_str(token: &str) -> DshApiResult<Self> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
      Err(DshApiError::Conversion("invalid jwt token".to_string()))
    } else {
      let header = STANDARD_NO_PAD
        .decode(parts[0].as_bytes())
        .map_err(|_| DshApiError::Conversion("could not decode header".to_string()))
        .and_then(|decoded_header| String::from_utf8(decoded_header).map_err(|_| DshApiError::Conversion("header contains invalid utf8".to_string())))
        .and_then(|json_header| {
          serde_json::from_str::<DshJwtHeader>(&json_header).map_err(|json_error| DshApiError::Conversion(format!("payload contains invalid json ({})", json_error)))
        })?;

      let payload = STANDARD_NO_PAD
        .decode(parts[1].as_bytes())
        .map_err(|_| DshApiError::Conversion("could not decode payload".to_string()))
        .and_then(|decoded_payload| String::from_utf8(decoded_payload).map_err(|_| DshApiError::Conversion("payload contains invalid utf8".to_string())))
        .and_then(|json_payload| {
          serde_json::from_str::<DshJwtPayload>(&json_payload).map_err(|json_error| DshApiError::Conversion(format!("payload contains invalid json ({})", json_error)))
        })?;

      let mut tenant_permissions_map: HashMap<String, DshPermission> = HashMap::new();
      if let Some(ref permission_representations) = &payload.dsh_permission_representations {
        for permission_representation in permission_representations {
          DshPermission::from_str(permission_representation).map(|dsh_permission| {
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
      }
      let mut tenant_permissions: Vec<DshPermission> = Vec::from_iter(tenant_permissions_map.into_values());
      tenant_permissions.sort_by(|dsh_permission_a, dsh_permission_b| dsh_permission_a.tenant.cmp(&dsh_permission_b.tenant));
      Ok(DshJwt { header, payload, tenant_permissions })
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
      write!(
        f,
        "{}:{}:{}",
        self.token_type.as_deref().unwrap_or(""),
        self.preferred_username.as_deref().unwrap_or(""),
        self.expires_in()
      )
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
      None => Err(DshApiError::Conversion("illegal permission representation".to_string())),
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
