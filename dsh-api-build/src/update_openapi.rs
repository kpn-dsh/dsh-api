//! Update the openapi specification
//!
//! This method will make the following (in place) updates to an `OpenApi` object:
//! * Add authorization header to each operation
//! * Add operation id to each operation
//! * Add a description

use crate::openapi_utils::OpenApiOperationKind;
use crate::PathElement;
use itertools::Itertools;
use openapiv3::{OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr};

pub fn update_openapi(original_openapi_spec: &mut OpenAPI, prune_manage: bool, prune_robot: bool) -> Result<(), String> {
  // If feature manage is not enabled, prune all /manage/... paths
  if prune_manage {
    prune_paths(original_openapi_spec, |path| path.starts_with("/manage/"))?;
  }
  // If feature robot is not enabled, prune all /robot/... paths
  if prune_robot {
    prune_paths(original_openapi_spec, |path| path.starts_with("/robot/"))?;
  }
  // Add authorization headers to original openapi spec
  add_authorization_parameters(original_openapi_spec)?;
  // Add operation ids to original openapi spec
  add_operation_ids(original_openapi_spec)?;
  // Add description to original openapi spec
  add_description(original_openapi_spec);
  Ok(())
}

fn prune_paths(openapi: &mut OpenAPI, predicate: fn(&str) -> bool) -> Result<(), String> {
  let paths = openapi.paths.paths.keys().map(|path| path.to_string()).collect_vec();
  for path in paths {
    if predicate(path.as_str()) {
      openapi.paths.paths.shift_remove(path.as_str());
    }
  }
  Ok(())
}

fn add_authorization_parameters(openapi: &mut OpenAPI) -> Result<(), String> {
  for (_, path_item) in openapi.paths.paths.iter_mut() {
    if let ReferenceOr::Item(path_item) = path_item {
      if let Some(ref mut delete) = path_item.delete {
        add_authorization_parameter(delete);
      }
      if let Some(ref mut get) = path_item.get {
        add_authorization_parameter(get);
      }
      if let Some(ref mut head) = path_item.head {
        add_authorization_parameter(head);
      }
      if let Some(ref mut options) = path_item.options {
        add_authorization_parameter(options);
      }
      if let Some(ref mut patch) = path_item.patch {
        add_authorization_parameter(patch);
      }
      if let Some(ref mut post) = path_item.post {
        add_authorization_parameter(post);
      }
      if let Some(ref mut put) = path_item.put {
        add_authorization_parameter(put);
      }
      if let Some(ref mut trace) = path_item.trace {
        add_authorization_parameter(trace);
      }
    }
  }
  Ok(())
}

fn add_authorization_parameter(operation: &mut Operation) {
  const STRING_SCHEMA_JSON: &str = "{ \"schema\": { \"type\": \"string\" } }";

  let schema_content: ParameterSchemaOrContent = serde_json::from_str::<ParameterSchemaOrContent>(STRING_SCHEMA_JSON).unwrap();
  operation.parameters.push(ReferenceOr::Item(Parameter::Header {
    parameter_data: ParameterData {
      name: "Authorization".to_string(),
      description: Some("Authorization header (bearer token)".to_string()),
      required: true,
      deprecated: Some(false),
      format: schema_content,
      example: None,
      examples: Default::default(),
      explode: None,
      extensions: Default::default(),
    },
    style: Default::default(),
  }));
}

fn add_operation_ids(openapi: &mut OpenAPI) -> Result<(), String> {
  for (path, path_item) in openapi.paths.paths.iter_mut() {
    let path_elements = PathElement::vec_from_str(path);
    if let ReferenceOr::Item(path_item) = path_item {
      if let Some(ref mut delete) = path_item.delete {
        add_operation_id(delete, "delete", &path_elements);
      }
      if let Some(ref mut get) = path_item.get {
        add_operation_id(get, "get", &path_elements);
      }
      if let Some(ref mut head) = path_item.head {
        add_operation_id(head, "head", &path_elements);
      }
      if let Some(ref mut options) = path_item.options {
        add_operation_id(options, "options", &path_elements);
      }
      if let Some(ref mut patch) = path_item.patch {
        add_operation_id(patch, "patch", &path_elements);
      }
      if let Some(ref mut post) = path_item.post {
        add_operation_id(post, "post", &path_elements);
      }
      if let Some(ref mut put) = path_item.put {
        add_operation_id(put, "put", &path_elements);
      }
      if let Some(ref mut trace) = path_item.trace {
        add_operation_id(trace, "trace", &path_elements);
      }
    }
  }
  Ok(())
}

fn add_operation_id(operation: &mut Operation, method: &str, path_elements: &[PathElement]) {
  operation.operation_id = Some(OpenApiOperation::new(method, path_elements).operation_id())
}

fn add_description(openapi: &mut OpenAPI) {
  const DESC: &str = "Updated from original version (added authorization parameters and operation ids)";
  if let Some(ref description) = openapi.info.description {
    openapi.info.description = Some(format!("{}\n{}", description, DESC));
  } else {
    openapi.info.description = Some(DESC.to_string());
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
      .collect_vec();
    let by_parameters = path_elements
      .iter()
      .filter_map(|element| match element {
        PathElement::Literal(_) => None,
        PathElement::Variable(variable) => Some(variable.to_lowercase().replace('-', "_").to_string()),
      })
      .collect_vec();
    OpenApiOperation { method: method.to_string(), kind, subjects, by_parameters }
  }

  fn operation_id(&self) -> String {
    let kind = match self.kind {
      OpenApiOperationKind::AppCatalog => "_appcatalog",
      _ => "",
    };
    let subjects: String = if self.subjects.len() >= 2 {
      let mut subjects_iter = self.subjects.iter();
      let first = subjects_iter.next().unwrap();
      let mut second = subjects_iter.next().unwrap().as_str();
      if let Some(stripped) = second.strip_prefix(first) {
        second = stripped;
      }
      format!("{}_{}{}", first, second, subjects_iter.map(|subject| format!("_{}", subject)).join(""))
    } else {
      self.subjects.join("_")
    };
    let parameters = if self.by_parameters.is_empty() { format!("_{}", subjects) } else { format!("_{}_by_{}", subjects, self.by_parameters.join("_by_")) };
    format!("{}{}{}", self.method, kind, parameters)
  }
}
