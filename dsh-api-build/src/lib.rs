#![doc(
  html_favicon_url = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeD0iMHB4IiB5PSIwcHgiCiAgICAgdmlld0JveD0iMCAwIDE3MS4zIDE4Mi45IiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCAxNzEuMyAxODIuOTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPgogICAgPHN0eWxlPgoJCSNrcG5fbG9nbyB7CgkJCWZpbGw6IGJsYWNrOwoJCX0KCgkJQG1lZGlhIChwcmVmZXJzLWNvbG9yLXNjaGVtZTogZGFyaykgewoJCQkja3BuX2xvZ28gewoJCQkJZmlsbDogd2hpdGU7CgkJCX0KCQl9Cgk8L3N0eWxlPgogICAgPGcgaWQ9Imtwbl9sb2dvIj4KCQk8cGF0aCBkPSJNMTYxLjcsNzIuMWMtNS40LTUuNC0xNS4zLTExLjgtMzIuMi0xMS44Yy0zLjEsMC02LjIsMC4yLTkuMSwwLjZsLTAuOSwwLjFsMC4zLDAuOWMwLjgsMi42LDEuNCw1LjUsMS44LDguNGwwLjEsMC44CgkJCWwwLjgtMC4xYzIuNC0wLjMsNC43LTAuNCw3LTAuNGMxMy40LDAsMjEsNC44LDI1LDguOGM0LjIsNC4yLDYuNSw5LjYsNi41LDE1YzAsNi45LTMuNiwxNS42LTcuMiwyNC4xYy0xLjcsNC4yLTQuOSwxMi4zLTYuNywxOS4yCgkJCWMtMy4zLDEzLjEtOC44LDM1LTIxLjksMzVjLTQuMywwLTkuNC0yLjQtMTUuNS03LjJjLTMuMywxLjktNi44LDMuNC0xMC41LDQuNmM5LjgsOC43LDE4LjEsMTIuOCwyNiwxMi44CgkJCWMyMS4yLDAsMjguMS0yNy44LDMxLjgtNDIuN2MxLjEtNC42LDMuMy0xMC44LDYuMi0xNy43YzMuOS05LjQsOC0xOS4xLDgtMjhDMTcxLjMsODYuMywxNjcuOCw3OC4yLDE2MS43LDcyLjF6Ii8+CgkJPHBhdGggZD0iTTExNiw1Mi4ybDAuOS0wLjJjMi45LTAuNSw1LjktMC44LDkuMS0xYzAuMywwLDAuNiwwLDAuOSwwQzExMi45LDE3LjcsNzcuMiwwLDU2LjcsMEMyOS42LDAsMjAsMjcuNiwyMCw1My40CgkJCWMwLDEyLDQuMSwyNC42LDcuNSwzM2wwLjMsMC44bDAuOC0wLjNjMi40LTEuMSw1LTIuMSw4LTMuMmwwLjgtMC4zTDM3LDgyLjZjLTQuMy0xMC42LTYuOC0yMS4zLTYuOC0yOS4yYzAtMTYuNSw0LTMwLDExLjEtMzcKCQkJYzQuMS00LjEsOS4xLTYuMSwxNS40LTYuMUM3Mi44LDEwLjMsMTAzLDI1LjIsMTE2LDUyLjJ6Ii8+CgkJPHBhdGggZD0iTTk0LjksMTUxLjNsLTAuNC0wLjRsLTAuNSwwLjJjLTUuNSwyLTExLjEsMi45LTE3LjIsMi45Yy0yMCwwLTQxLjgtOC45LTU1LjYtMjIuOGMtNi45LTYuOS0xMC45LTE0LjMtMTAuOS0yMC4yCgkJCWMwLTguMSwzLTE0LjEsOS40LTE5Yy0xLjItMi45LTIuNi02LjMtMy44LTkuOUM1LjIsODkuMiwwLDk4LjcsMCwxMTFjMCw4LjcsNC45LDE4LjUsMTMuOSwyNy41YzEyLjQsMTIuNSwzNS41LDI1LjgsNjIuOSwyNS44CgkJCWM4LjYsMCwxNi44LTEuNywyNC40LTVsMS4xLTAuNWwtMC44LTAuOEM5OS4xLDE1NS43LDk2LjksMTUzLjQsOTQuOSwxNTEuM3oiLz4KCQk8cGF0aCBkPSJNODMuMiw3OS45di05QzgxLDcwLjMsNzguNSw3MCw3NS45LDcwYy0xMC41LDAtMTUuNiw3LjEtMTUuNiwxNC4yYzAsNi44LDIuNSwxMy4zLDExLjksMjcuOWMzLjgtMC41LDcuNi0wLjgsMTEuNC0wLjkKCQkJYy04LjItMTUuMi0xMC4yLTIwLjYtMTAuMi0yNC41YzAtNC41LDIuNi02LjgsNy45LTYuOEM4Miw3OS44LDgyLjYsNzkuOSw4My4yLDc5Ljl6Ii8+CgkJPHBhdGggZD0iTTU0LjcsOTMuMWMtMC44LTItMS4zLTUuMy0xLjYtNy43Yy04LjMtMC4zLTE0LjYsNC41LTE0LjYsMTEuMmMwLDUuNCwyLjgsMTAuMiwxNC4yLDE5LjljMi45LTEsNi44LTIuMSwxMC4xLTIuOAoJCQljLTExLjItMTAuNS0xMy0xMy4zLTEzLTE2LjRDNTAsOTUuMSw1MS42LDkzLjYsNTQuNyw5My4xeiIvPgoJCTxwYXRoIGQ9Ik05MC45LDc5Ljl2LTljMi4xLTAuNiw0LjctMC45LDcuMy0wLjljMTAuNCwwLDE1LjYsNy4xLDE1LjYsMTQuMmMwLDYuOC0yLjUsMTMuMy0xMS45LDI3LjljLTMuOC0wLjUtNy42LTAuOC0xMS40LTAuOQoJCQljOC4yLTE1LjIsMTAuMi0yMC42LDEwLjItMjQuNWMwLTQuNS0yLjYtNi44LTcuOS02LjhDOTIsNzkuOCw5MS40LDc5LjksOTAuOSw3OS45eiIvPgoJCTxwYXRoIGQ9Ik0xMTkuMyw5My4xYzAuOC0yLDEuMy01LjMsMS42LTcuN2M4LjMtMC4zLDE0LjYsNC41LDE0LjYsMTEuMmMwLDUuNC0yLjgsMTAuMi0xNC4yLDE5LjljLTIuOS0xLTYuOC0yLjEtMTAuMS0yLjgKCQkJYzExLjItMTAuNSwxMy0xMy4zLDEzLTE2LjRDMTI0LjEsOTUuMSwxMjIuNSw5My42LDExOS4zLDkzLjF6Ii8+CgkJPHBhdGggZD0iTTg3LDEzMC4yYzguNCwwLDE3LDEuMSwyNS45LDMuOGwzLTEwYy0xMC0zLTE5LjgtNC4yLTI5LTQuMmMtOS4yLDAtMTguOSwxLjItMjksNC4ybDMsMTBDNzAsMTMxLjMsNzguNiwxMzAuMiw4NywxMzAuMnoiCgkJLz4KCQk8cmVjdCB4PSI4MC41IiB5PSI0OS4zIiB0cmFuc2Zvcm09Im1hdHJpeCgwLjcwNzIgLTAuNzA3MSAwLjcwNzEgMC43MDcyIC0xMy45OTkyIDc3Ljg3NDQpIiB3aWR0aD0iMTMuMSIKCQkJICBoZWlnaHQ9IjEzLjEiLz4KCTwvZz4KPC9zdmc+Cg=="
)]
#![doc(
  html_logo_url = "data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0idXRmLTgiPz4KPHN2ZyB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeD0iMHB4IiB5PSIwcHgiCiAgICAgdmlld0JveD0iMCAwIDE3MS4zIDE4Mi45IiBzdHlsZT0iZW5hYmxlLWJhY2tncm91bmQ6bmV3IDAgMCAxNzEuMyAxODIuOTsiIHhtbDpzcGFjZT0icHJlc2VydmUiPgogICAgPHN0eWxlPgoJCSNrcG5fbG9nbyB7CgkJCWZpbGw6IGJsYWNrOwoJCX0KCgkJQG1lZGlhIChwcmVmZXJzLWNvbG9yLXNjaGVtZTogZGFyaykgewoJCQkja3BuX2xvZ28gewoJCQkJZmlsbDogd2hpdGU7CgkJCX0KCQl9Cgk8L3N0eWxlPgogICAgPGcgaWQ9Imtwbl9sb2dvIj4KCQk8cGF0aCBkPSJNMTYxLjcsNzIuMWMtNS40LTUuNC0xNS4zLTExLjgtMzIuMi0xMS44Yy0zLjEsMC02LjIsMC4yLTkuMSwwLjZsLTAuOSwwLjFsMC4zLDAuOWMwLjgsMi42LDEuNCw1LjUsMS44LDguNGwwLjEsMC44CgkJCWwwLjgtMC4xYzIuNC0wLjMsNC43LTAuNCw3LTAuNGMxMy40LDAsMjEsNC44LDI1LDguOGM0LjIsNC4yLDYuNSw5LjYsNi41LDE1YzAsNi45LTMuNiwxNS42LTcuMiwyNC4xYy0xLjcsNC4yLTQuOSwxMi4zLTYuNywxOS4yCgkJCWMtMy4zLDEzLjEtOC44LDM1LTIxLjksMzVjLTQuMywwLTkuNC0yLjQtMTUuNS03LjJjLTMuMywxLjktNi44LDMuNC0xMC41LDQuNmM5LjgsOC43LDE4LjEsMTIuOCwyNiwxMi44CgkJCWMyMS4yLDAsMjguMS0yNy44LDMxLjgtNDIuN2MxLjEtNC42LDMuMy0xMC44LDYuMi0xNy43YzMuOS05LjQsOC0xOS4xLDgtMjhDMTcxLjMsODYuMywxNjcuOCw3OC4yLDE2MS43LDcyLjF6Ii8+CgkJPHBhdGggZD0iTTExNiw1Mi4ybDAuOS0wLjJjMi45LTAuNSw1LjktMC44LDkuMS0xYzAuMywwLDAuNiwwLDAuOSwwQzExMi45LDE3LjcsNzcuMiwwLDU2LjcsMEMyOS42LDAsMjAsMjcuNiwyMCw1My40CgkJCWMwLDEyLDQuMSwyNC42LDcuNSwzM2wwLjMsMC44bDAuOC0wLjNjMi40LTEuMSw1LTIuMSw4LTMuMmwwLjgtMC4zTDM3LDgyLjZjLTQuMy0xMC42LTYuOC0yMS4zLTYuOC0yOS4yYzAtMTYuNSw0LTMwLDExLjEtMzcKCQkJYzQuMS00LjEsOS4xLTYuMSwxNS40LTYuMUM3Mi44LDEwLjMsMTAzLDI1LjIsMTE2LDUyLjJ6Ii8+CgkJPHBhdGggZD0iTTk0LjksMTUxLjNsLTAuNC0wLjRsLTAuNSwwLjJjLTUuNSwyLTExLjEsMi45LTE3LjIsMi45Yy0yMCwwLTQxLjgtOC45LTU1LjYtMjIuOGMtNi45LTYuOS0xMC45LTE0LjMtMTAuOS0yMC4yCgkJCWMwLTguMSwzLTE0LjEsOS40LTE5Yy0xLjItMi45LTIuNi02LjMtMy44LTkuOUM1LjIsODkuMiwwLDk4LjcsMCwxMTFjMCw4LjcsNC45LDE4LjUsMTMuOSwyNy41YzEyLjQsMTIuNSwzNS41LDI1LjgsNjIuOSwyNS44CgkJCWM4LjYsMCwxNi44LTEuNywyNC40LTVsMS4xLTAuNWwtMC44LTAuOEM5OS4xLDE1NS43LDk2LjksMTUzLjQsOTQuOSwxNTEuM3oiLz4KCQk8cGF0aCBkPSJNODMuMiw3OS45di05QzgxLDcwLjMsNzguNSw3MCw3NS45LDcwYy0xMC41LDAtMTUuNiw3LjEtMTUuNiwxNC4yYzAsNi44LDIuNSwxMy4zLDExLjksMjcuOWMzLjgtMC41LDcuNi0wLjgsMTEuNC0wLjkKCQkJYy04LjItMTUuMi0xMC4yLTIwLjYtMTAuMi0yNC41YzAtNC41LDIuNi02LjgsNy45LTYuOEM4Miw3OS44LDgyLjYsNzkuOSw4My4yLDc5Ljl6Ii8+CgkJPHBhdGggZD0iTTU0LjcsOTMuMWMtMC44LTItMS4zLTUuMy0xLjYtNy43Yy04LjMtMC4zLTE0LjYsNC41LTE0LjYsMTEuMmMwLDUuNCwyLjgsMTAuMiwxNC4yLDE5LjljMi45LTEsNi44LTIuMSwxMC4xLTIuOAoJCQljLTExLjItMTAuNS0xMy0xMy4zLTEzLTE2LjRDNTAsOTUuMSw1MS42LDkzLjYsNTQuNyw5My4xeiIvPgoJCTxwYXRoIGQ9Ik05MC45LDc5Ljl2LTljMi4xLTAuNiw0LjctMC45LDcuMy0wLjljMTAuNCwwLDE1LjYsNy4xLDE1LjYsMTQuMmMwLDYuOC0yLjUsMTMuMy0xMS45LDI3LjljLTMuOC0wLjUtNy42LTAuOC0xMS40LTAuOQoJCQljOC4yLTE1LjIsMTAuMi0yMC42LDEwLjItMjQuNWMwLTQuNS0yLjYtNi44LTcuOS02LjhDOTIsNzkuOCw5MS40LDc5LjksOTAuOSw3OS45eiIvPgoJCTxwYXRoIGQ9Ik0xMTkuMyw5My4xYzAuOC0yLDEuMy01LjMsMS42LTcuN2M4LjMtMC4zLDE0LjYsNC41LDE0LjYsMTEuMmMwLDUuNC0yLjgsMTAuMi0xNC4yLDE5LjljLTIuOS0xLTYuOC0yLjEtMTAuMS0yLjgKCQkJYzExLjItMTAuNSwxMy0xMy4zLDEzLTE2LjRDMTI0LjEsOTUuMSwxMjIuNSw5My42LDExOS4zLDkzLjF6Ii8+CgkJPHBhdGggZD0iTTg3LDEzMC4yYzguNCwwLDE3LDEuMSwyNS45LDMuOGwzLTEwYy0xMC0zLTE5LjgtNC4yLTI5LTQuMmMtOS4yLDAtMTguOSwxLjItMjksNC4ybDMsMTBDNzAsMTMxLjMsNzguNiwxMzAuMiw4NywxMzAuMnoiCgkJLz4KCQk8cmVjdCB4PSI4MC41IiB5PSI0OS4zIiB0cmFuc2Zvcm09Im1hdHJpeCgwLjcwNzIgLTAuNzA3MSAwLjcwNzEgMC43MDcyIC0xMy45OTkyIDc3Ljg3NDQpIiB3aWR0aD0iMTMuMSIKCQkJICBoZWlnaHQ9IjEzLjEiLz4KCTwvZz4KPC9zdmc+Cg=="
)]
//! Helper functions for the build phase of the `dsh_api` crate.
//!
//! * Generate the generic client code
//! * Generate Progenitor client from an openapi specification
//! * Update the openapi specification

use std::fmt::{Display, Formatter};

pub mod dsh_api_operation;
pub mod generate_client;
pub mod generate_generic;
pub mod generate_wrapped;
pub mod openapi_utils;
pub mod update_openapi;

#[derive(Debug, PartialEq)]
pub(crate) enum PathElement {
  Literal(String),
  Variable(String),
}

impl PathElement {
  pub(crate) fn vec_from_str(string: &str) -> Vec<PathElement> {
    string
      .split('/')
      .collect::<Vec<_>>()
      .iter()
      .filter_map(|element| if element.is_empty() { None } else { Some(PathElement::from(*element)) })
      .collect::<Vec<_>>()
  }
}

impl From<&str> for PathElement {
  fn from(string: &str) -> Self {
    if string.starts_with('{') && string.ends_with('}') {
      PathElement::Variable(string[1..string.len() - 1].to_string())
    } else {
      PathElement::Literal(string.to_string())
    }
  }
}

impl Display for PathElement {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      PathElement::Literal(literal) => write!(f, "{}", literal),
      PathElement::Variable(variable) => write!(f, "{}", variable),
    }
  }
}

impl From<&PathElement> for String {
  fn from(value: &PathElement) -> Self {
    match value {
      PathElement::Literal(literal) => literal.to_string(),
      PathElement::Variable(variable) => variable.to_string(),
    }
  }
}

#[derive(Clone, Debug)]
pub(crate) enum RequestBodyType {
  String,
  SerializableType(String),
}

impl Display for RequestBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::String => write!(f, "String"),
      Self::SerializableType(serializable_type) => write!(f, "{}", serializable_type),
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ResponseBodyType {
  Ids,
  Ok(String),
  SerializableMap(String),
  SerializableScalar(String),
  SerializableVector(String),
  String,
}

impl ResponseBodyType {
  pub(crate) fn processing_function(&self) -> &str {
    match self {
      Self::String => "process_string",
      _ => "process",
    }
  }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Method {
  Delete,
  Get,
  Head,
  Patch,
  Post,
  Put,
}

pub const METHODS: [Method; 6] = [Method::Delete, Method::Get, Method::Head, Method::Patch, Method::Post, Method::Put];

impl Method {
  pub fn has_body_argument(&self) -> bool {
    match self {
      Self::Get | Self::Delete | Self::Head => false,
      Self::Patch | Self::Post | Self::Put => true,
    }
  }
}

impl Display for Method {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Delete => write!(f, "delete"),
      Self::Get => write!(f, "get"),
      Self::Head => write!(f, "head"),
      Self::Patch => write!(f, "patch"),
      Self::Post => write!(f, "post"),
      Self::Put => write!(f, "put"),
    }
  }
}

pub const MANAGED_PARAMETERS: [&str; 1] = ["Authorization"];

pub fn capitalize<T: AsRef<str>>(string: T) -> String {
  let mut chars = string.as_ref().chars();
  match chars.next() {
    None => String::new(),
    Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

fn revise<T: Into<String>>(description: T) -> String {
  let description = description.into();
  if description.is_empty() {
    description
  } else {
    let trimmed = description.trim();
    match (trimmed.chars().collect::<Vec<_>>()[0].is_uppercase(), trimmed.ends_with('.')) {
      (false, false) => format!("{}.", capitalize(trimmed)),
      (false, true) => capitalize(trimmed),
      (true, false) => format!("{}.", trimmed),
      (true, true) => description,
    }
  }
}

fn article(noun: &str) -> &'static str {
  const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
  if VOWELS.iter().any(|&vowel| noun.to_lowercase().starts_with(vowel)) {
    "an"
  } else {
    "a"
  }
}
