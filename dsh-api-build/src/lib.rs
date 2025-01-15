extern crate indoc;
extern crate itertools;
extern crate lazy_static;
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

#[derive(Debug)]
enum OpenApiOperationKind {
  Allocation,
  AppCatalog,
  Manage,
  Robot,
}

impl From<&str> for OpenApiOperationKind {
  fn from(kind: &str) -> Self {
    match kind {
      "allocation" => Self::Allocation,
      "manage" => Self::Manage,
      "appcatalog" => Self::AppCatalog,
      "robot" => Self::Robot,
      _ => {
        panic!("unrecognized operation kind '{}'", kind)
      }
    }
  }
}

impl Display for OpenApiOperationKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Allocation => write!(f, "allocation"),
      Self::Manage => write!(f, "manage"),
      Self::AppCatalog => write!(f, "appcatalog"),
      Self::Robot => write!(f, "robot"),
    }
  }
}

#[derive(Debug)]
struct OpenApiOperation {
  method: String,
  kind: OpenApiOperationKind,
  subjects: Vec<String>,
  by_parameters: Vec<String>,
}

impl OpenApiOperation {
  fn new(method: &str, path_elements: &[PathElement]) -> Self {
    let kind: OpenApiOperationKind = OpenApiOperationKind::from(path_elements.first().unwrap().to_string().as_str());
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
    OpenApiOperation { method: method.to_string(), kind, subjects, by_parameters }
  }

  fn operation_id(&self) -> String {
    let kind = match self.kind {
      OpenApiOperationKind::AppCatalog => "_appcatalog",
      _ => "",
    };
    let parameters =
      if self.by_parameters.is_empty() { format!("_{}", self.subjects.join("_")) } else { format!("_{}_by_{}", self.subjects.join("_"), self.by_parameters.join("_by_")) };
    format!("{}{}{}", self.method, kind, parameters)
  }
}
