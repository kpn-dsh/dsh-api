use progenitor::GenerationSettings;
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::fmt::{Display, Formatter};
use std::fs;

fn main() -> Result<(), String> {
  let openapi_spec_original_file = "openapi_spec/openapi_1_8_0.json";
  println!("cargo:rerun-if-changed={}", openapi_spec_original_file);
  let file = fs::File::open(openapi_spec_original_file).unwrap();
  let mut openapi_spec: Value = serde_json::from_reader(file).unwrap();
  // Add authorization headers and operationId to original openapi spec
  update_openapi_spec(&mut openapi_spec)?;
  // Make updated openapi spec available to the crate code
  let openapi_spec_updated_json = serde_json::to_string_pretty(&openapi_spec).unwrap();
  let mut embedded_openapi_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
  embedded_openapi_file.push("open-api.json");
  // println!("cargo:warning= embedded_file: {:?}", &embedded_openapi_file);
  fs::write(embedded_openapi_file, &openapi_spec_updated_json).unwrap();

  let spec = serde_json::from_str(&openapi_spec_updated_json).unwrap();
  let mut generator_settings = GenerationSettings::default();
  generator_settings.with_derive("PartialEq");
  let mut generator = progenitor::Generator::new(&generator_settings);
  let tokens = generator.generate_tokens(&spec).unwrap();
  let ast = syn::parse2(tokens).unwrap();
  let content = prettyplease::unparse(&ast);
  let mut out_file = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).to_path_buf();
  out_file.push("codegen.rs");
  // println!("cargo:warning= out_file: {:?}", &out_file);
  fs::write(out_file, &content).unwrap();

  Ok(())
}

#[derive(Serialize)]
struct AuthorizationParameterSchema {
  #[serde(rename = "type")]
  type_: String,
}

impl Default for AuthorizationParameterSchema {
  fn default() -> Self {
    Self { type_: "string".to_string() }
  }
}

#[derive(Serialize)]
struct AuthorizationParameter {
  name: String,
  #[serde(rename = "in")]
  in_: String,
  description: String,
  required: bool,
  deprecated: bool,
  schema: AuthorizationParameterSchema,
}

impl Default for AuthorizationParameter {
  fn default() -> Self {
    Self {
      name: "Authorization".to_string(),
      in_: "header".to_string(),
      description: "Authorization header (bearer token)".to_string(),
      required: true,
      deprecated: false,
      schema: Default::default(),
    }
  }
}

fn update_openapi_spec(openapi: &mut Value) -> Result<(), String> {
  const METHODS: [&str; 6] = ["delete", "get", "head", "patch", "post", "put"];
  let paths = openapi.get_mut("paths").unwrap().as_object_mut().unwrap();
  for (path, methods) in paths.iter_mut() {
    let path_elements = PathElement::vec_from_str(path);
    for (method, method_value) in methods.as_object_mut().unwrap().iter_mut() {
      if METHODS.contains(&method.to_lowercase().as_str()) {
        let operation = Operation::new(method, &path_elements);
        if let Some(method_object) = method_value.as_object_mut() {
          add_authorization_parameter(method_object)?;
          add_operation_id(method_object, operation)?;
        }
      }
    }
  }
  Ok(())
}

#[derive(Debug, PartialEq)]
enum PathElement {
  Literal(String),
  Variable(String),
}

impl From<&PathElement> for String {
  fn from(value: &PathElement) -> Self {
    match value {
      PathElement::Literal(literal) => literal.to_string(),
      PathElement::Variable(variable) => variable.to_string(),
    }
  }
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

struct Operation {
  method: String,
  kind: String,
  subjects: Vec<String>,
  bys: Vec<String>,
}

impl Operation {
  fn new(method: &str, path_elements: &[PathElement]) -> Self {
    let kind: String = path_elements.first().unwrap().into();
    let subjects = path_elements
      .iter()
      .skip(1)
      .filter_map(|element| match element {
        PathElement::Literal(subject) => Some(subject.to_lowercase().to_string()),
        PathElement::Variable(_) => None,
      })
      .collect::<Vec<_>>();
    let bys = path_elements
      .iter()
      .filter_map(|element| match element {
        PathElement::Literal(_) => None,
        PathElement::Variable(variable) => Some(variable.to_lowercase().to_string()),
      })
      .collect::<Vec<_>>();
    Operation { method: method.to_string(), kind, subjects, bys }
  }
}

impl Display for Operation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.method)?;
    if self.kind != "allocation" {
      write!(f, "_{}", self.kind)?;
    }
    if self.bys.is_empty() {
      write!(f, "_{}", self.subjects.join("_"))
    } else {
      write!(f, "_{}_by_{}", self.subjects.join("_"), self.bys.join("_by_"))
    }
  }
}

fn add_authorization_parameter(method_object: &mut Map<String, Value>) -> Result<(), String> {
  if let Some(parameters) = method_object.get_mut("parameters").and_then(|ps| ps.as_array_mut()) {
    parameters.push(json!(AuthorizationParameter::default()));
  } else {
    method_object.insert("parameters".to_string(), Value::from(vec![json!(AuthorizationParameter::default())]));
  }
  Ok(())
}

fn add_operation_id(method_object: &mut Map<String, Value>, operation: Operation) -> Result<(), String> {
  method_object.insert("operationId".to_string(), Value::String(operation.to_string()));
  Ok(())
}

fn _tag(method_object: &mut Map<String, Value>) -> Option<String> {
  method_object
    .get("tags")
    .and_then(|ts| ts.as_array())
    .and_then(|a| a.first())
    .and_then(|v| v.as_str())
    .map(|v| v.to_string())
}
