extern crate indoc;
extern crate openapiv3;
extern crate progenitor;
extern crate serde_json;

use std::fmt::{Display, Formatter};

pub mod generate_client;
pub mod generate_generic;
pub mod update_openapi;

#[derive(Debug, PartialEq)]
enum PathElement {
  Literal(String),
  Variable(String),
}

impl PathElement {
  fn vec_from_str(string: &str) -> Vec<PathElement> {
    string
      .split('/')
      .collect::<Vec<_>>()
      .into_iter()
      .filter_map(|element| {
        if element.is_empty() {
          None
        } else if element.starts_with('{') && element.ends_with('}') {
          Some(PathElement::Variable(element[1..element.len() - 1].to_string()))
        } else {
          Some(PathElement::Literal(element.to_string()))
        }
      })
      .collect::<Vec<_>>()
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

#[derive(Debug)]
struct ApiOperation {
  method: String,
  kind: String,
  subjects: Vec<String>,
  by_parameters: Vec<String>,
}

impl ApiOperation {
  fn new(method: &str, path_elements: &[PathElement]) -> Self {
    let kind: String = path_elements.first().unwrap().into();
    let subjects = path_elements
      .iter()
      .skip(1)
      .filter_map(|element| match element {
        PathElement::Literal(subject) => Some(subject.to_lowercase().replace('-', "_").to_string()),
        PathElement::Variable(_) => None,
      })
      .collect::<Vec<_>>();
    let by_parameters = path_elements
      .iter()
      .filter_map(|element| match element {
        PathElement::Literal(_) => None,
        PathElement::Variable(variable) => Some(variable.to_lowercase().replace('-', "_").to_string()),
      })
      .collect::<Vec<_>>();
    ApiOperation { method: method.to_string(), kind, subjects, by_parameters }
  }
}

impl Display for ApiOperation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.method)?;
    if self.kind != "allocation" {
      write!(f, "_{}", self.kind)?;
    }
    if self.by_parameters.is_empty() {
      write!(f, "_{}", self.subjects.join("_"))
    } else {
      write!(f, "_{}_by_{}", self.subjects.join("_"), self.by_parameters.join("_by_"))
    }
  }
}
