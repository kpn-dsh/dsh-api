use crate::openapi_utils::{reference_to_string, OpenApiOperationKind};
use crate::{capitalize, revise, Method, PathElement, RequestBodyType, ResponseBodyType};
use itertools::Itertools;
use openapiv3::{Operation, Parameter, ParameterSchemaOrContent, ReferenceOr, SchemaKind, StatusCode, Type};
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub(crate) struct DshApiOperation {
  pub(crate) method: Method,
  pub(crate) selector: String,
  pub(crate) path: String,
  pub(crate) path_elements: Vec<PathElement>,
  pub(crate) description: Option<String>,
  pub(crate) parameters: Vec<(String, ParameterType, Option<String>)>,
  pub(crate) request_body: Option<RequestBodyType>,
  pub(crate) operation_id: String,
  pub(crate) ok_response: ResponseBodyType,
  #[allow(dead_code)]
  pub(crate) ok_responses: Vec<(u16, ResponseBodyType)>,
  #[allow(dead_code)]
  pub(crate) error_responses: Vec<(u16, ResponseBodyType)>,
  pub(crate) kind: OpenApiOperationKind,
}

pub(crate) fn method_api_operations(method: &Method, path_operations: &Vec<(&String, &Operation)>) -> Result<Vec<DshApiOperation>, Box<dyn Error>> {
  let mut method_generic_operations: Vec<DshApiOperation> = vec![];
  let mut selectors: HashSet<String> = HashSet::new();
  for (path, operation) in path_operations {
    let mut generic_operation = create_api_operation(method.clone(), path.to_string(), operation)?;
    if selectors.contains(&generic_operation.selector) {
      generic_operation.selector = selector_from_path_elements(&generic_operation.path_elements, &generic_operation.ok_response, true);
    }
    selectors.insert(generic_operation.selector.clone());
    method_generic_operations.push(generic_operation);
  }
  check_duplicate_selectors(&method_generic_operations, method)?;
  Ok(method_generic_operations)
}

fn create_api_operation(method: Method, path: String, operation: &Operation) -> Result<DshApiOperation, Box<dyn Error>> {
  let operation_id = operation.operation_id.clone().expect("missing operation id");
  let parameters: Vec<(String, ParameterType, Option<String>)> = operation
    .parameters
    .iter()
    .skip(1)
    .map(|parameter| parameter_to_parameter_type(parameter, &operation_id))
    .collect::<Vec<_>>();
  let request_body = operation.request_body.clone().map(|request_body| match request_body {
    ReferenceOr::Reference { reference } => RequestBodyType::SerializableType(reference_to_string(reference.as_ref())),
    ReferenceOr::Item(request_body_item) => RequestBodyType::from(&request_body_item),
  });
  let mut ok_responses: Vec<(u16, ResponseBodyType)> = vec![];
  let mut error_responses: Vec<(u16, ResponseBodyType)> = vec![];
  for (status_code, response) in operation.responses.responses.clone() {
    if let StatusCode::Code(numerical_status_code) = status_code {
      if (200..300).contains(&numerical_status_code) {
        ok_responses.push((numerical_status_code, ResponseBodyType::from(&response)))
      } else {
        error_responses.push((numerical_status_code, ResponseBodyType::from(&response)))
      }
    }
  }
  let ok_response = ok_responses.iter().min_by_key(|(status_code, _)| status_code).ok_or("")?.1.clone();
  let path_elements = PathElement::vec_from_str(&path);
  let selector = selector_from_path_elements(&path_elements, &ok_response, false);
  let kind = match path_elements.first().unwrap() {
    PathElement::Literal(first) => OpenApiOperationKind::from(first.as_str()),
    PathElement::Variable(_) => unreachable!(),
  };
  Ok(DshApiOperation {
    method,
    selector,
    path,
    path_elements,
    description: operation.summary.clone().map(revise),
    parameters,
    request_body,
    operation_id,
    ok_response,
    ok_responses,
    error_responses,
    kind,
  })
}

// Method will check whether duplicate selectors exist
fn check_duplicate_selectors(method_operations: &[DshApiOperation], method: &Method) -> Result<(), Box<dyn Error>> {
  let mut selectors = method_operations.iter().map(|operation| operation.selector.clone()).collect::<Vec<_>>();
  selectors.sort();
  let mut duplicate_selectors = Vec::new();
  for (selector, chunk) in &selectors.into_iter().chunk_by(|b| b.clone()) {
    if chunk.collect::<Vec<_>>().len() > 1 {
      duplicate_selectors.push(selector);
    }
  }
  if !duplicate_selectors.is_empty() {
    Err(Box::from(format!(
      "duplicate selectors for {} method ({})",
      method,
      duplicate_selectors.into_iter().join(", ")
    )))
  } else {
    Ok(())
  }
}

fn parameter_to_parameter_type(parameter: &ReferenceOr<Parameter>, operation_id: &str) -> (String, ParameterType, Option<String>) {
  match parameter {
    ReferenceOr::Reference { .. } => unimplemented!(),
    ReferenceOr::Item(parameter_item) => {
      let parameter_data = parameter_item.clone().parameter_data();
      match parameter_data.format {
        ParameterSchemaOrContent::Schema(ref schema) => match schema {
          ReferenceOr::Reference { reference } => (
            parameter_data.name,
            ParameterType::SerializableType(reference_to_string(reference)),
            parameter_data.description.map(capitalize),
          ),
          ReferenceOr::Item(item) => match &item.schema_kind {
            SchemaKind::Type(schema_kind_type) => match schema_kind_type {
              Type::String(string_type) => {
                let has_pattern = string_type.pattern.is_some();
                let has_enumeration = !string_type.enumeration.is_empty();
                match (has_pattern, has_enumeration) {
                  (false, false) => (parameter_data.name, ParameterType::RefStr, parameter_data.description.map(capitalize)), // No pattern, no enumeration -> &str
                  (false, true) => (
                    parameter_data.name.clone(),
                    ParameterType::ConstructedTypeOwned(to_type_name(operation_id, parameter_data.name.as_str())),
                    parameter_data.description.map(capitalize),
                  ), // No pattern, enumeration -> Constructed owned type
                  (true, false) => (
                    parameter_data.name.clone(),
                    ParameterType::ConstructedTypeRef(to_type_name(operation_id, parameter_data.name.as_str())),
                    parameter_data.description.map(capitalize),
                  ), // Pattern, no enumeration -> Constructed ref type
                  (true, true) => unimplemented!(),                                                                           // Pattern and enumeration -> Should not occur
                }
              }
              _ => unimplemented!(),
            },
            _ => unimplemented!(),
          },
        },
        ParameterSchemaOrContent::Content(_) => unimplemented!(),
      }
    }
  }
}

fn to_type_name(operation_id: &str, name: &str) -> String {
  format!("{}{}", operation_id.split('_').map(capitalize).collect::<Vec<_>>().join(""), capitalize(name))
}

fn selector_from_path_elements(path_elements: &[PathElement], ok_response: &ResponseBodyType, include_variables: bool) -> String {
  let mut selector = path_elements
    .iter()
    .filter_map(|path_element| match path_element {
      PathElement::Literal(literal) => {
        if literal != "allocation" {
          Some(literal.to_lowercase())
        } else {
          None
        }
      }
      PathElement::Variable(variable) => {
        if variable == "tenant" || variable == "manager" {
          None
        } else if include_variables || (variable != "id" && variable != "appid") {
          Some(variable.to_lowercase())
        } else {
          None
        }
      }
    })
    .collect::<Vec<_>>()
    .join("-");
  match ok_response {
    ResponseBodyType::Ids => selector = format!("{}-ids", selector),
    ResponseBodyType::Ok(_) => {}
    ResponseBodyType::SerializableMap(_) => selector = format!("{}-map", selector),
    ResponseBodyType::SerializableScalar(_) => {}
    ResponseBodyType::SerializableVector(_) => selector = format!("{}s", selector),
    ResponseBodyType::String => {}
  }
  selector
}

pub enum ParameterType {
  ConstructedTypeOwned(String),
  ConstructedTypeRef(String),
  SerializableType(String),
  RefStr,
}

impl Display for ParameterType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ConstructedTypeOwned(parameter_type) => write!(f, "{}", parameter_type),
      Self::ConstructedTypeRef(parameter_type) => write!(f, "&{}", parameter_type),
      Self::SerializableType(parameter_type) => write!(f, "&{}", parameter_type),
      Self::RefStr => write!(f, "&str"),
    }
  }
}
