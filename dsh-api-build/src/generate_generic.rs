use indoc::formatdoc;
use openapiv3::{AdditionalProperties, OpenAPI, Operation, Parameter, ParameterSchemaOrContent, ReferenceOr, RequestBody, Response, Schema, SchemaKind, StatusCode, Type};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

pub fn generate_generic(writer: &mut dyn Write, openapi_spec: OpenAPI) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "#[cfg_attr(rustfmt, rustfmt_skip)]")?;
  writeln!(writer, "// openapi spec version: {}", openapi_spec.info.version)?;
  writeln!(writer, "use crate::dsh_api_client::DshApiClient;")?;
  writeln!(writer, "use crate::types::*;")?;
  writeln!(writer, "use crate::{{DshApiError, DshApiResult}};")?;
  writeln!(writer, "use erased_serde::Serialize as ErasedSerialize;")?;
  writeln!(writer, "use std::str::FromStr;")?;
  writeln!(writer)?;

  writeln!(writer, "impl DshApiClient<'_> {{")?;

  let api_paths = api_paths(openapi_spec)?;
  print_get_operations(writer, &api_paths)?;
  writeln!(writer)?;
  print_put_operations(writer, &api_paths)?;

  writeln!(writer, "}}")?;

  Ok(())
}

fn print_get_operations(writer: &mut dyn Write, api_paths: &[ApiPath]) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  pub async fn get(&self, path: &str, parameters: &[&str]) -> DshApiResult<Box<dyn ErasedSerialize>> {{"
  )?;
  let get_operations: Vec<(&String, &ApiOperation)> = api_paths
    .iter()
    .filter_map(|api_path| api_path.operations.get(&ApiOperationType::Get).map(|api_operation| (&api_path.path, api_operation)))
    .collect::<Vec<_>>();
  let mut first = true;
  for (api_path, api_operation) in get_operations {
    if first {
      write!(writer, "    {}", api_operation.to_get_if_block(api_path))?;
    } else {
      write!(writer, " else {}", api_operation.to_get_if_block(api_path))?;
    }
    first = false;
  }
  writeln!(writer, " else {{")?;
  writeln!(writer, "      Err(DshApiError::Configuration(format!(\"get method '{{}}' not recognized\", path)))")?;
  writeln!(writer, "    }}")?;
  writeln!(writer, "  }}")?;
  Ok(())
}

fn print_put_operations(writer: &mut dyn Write, api_paths: &[ApiPath]) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  pub async fn put(&self, path: &str, parameters: &[&str], body: &str) -> DshApiResult<()> {{"
  )?;
  let put_operations: Vec<(&String, &ApiOperation)> = api_paths
    .iter()
    .filter_map(|api_path| api_path.operations.get(&ApiOperationType::Put).map(|api_operation| (&api_path.path, api_operation)))
    .collect::<Vec<_>>();
  let mut first = true;
  for (api_path, api_operation) in put_operations {
    if first {
      write!(writer, "    {}", api_operation.to_put_if_block(api_path))?;
    } else {
      write!(writer, " else {}", api_operation.to_put_if_block(api_path))?;
    }
    first = false;
  }
  writeln!(writer, " else {{")?;
  writeln!(writer, "      Err(DshApiError::Configuration(format!(\"get method '{{}}' not recognized\", path)))")?;
  writeln!(writer, "    }}")?;
  writeln!(writer, "  }}")?;
  Ok(())
}

fn api_paths(openapi: OpenAPI) -> Result<Vec<ApiPath>, String> {
  let mut api_paths: Vec<ApiPath> = vec![];
  for (path, path_item) in openapi.paths.into_iter() {
    let _path_elements = PathElement::vec_from_str(&path);
    let mut api_path = ApiPath { path: path.clone(), _path_elements, operations: HashMap::new() };
    if let ReferenceOr::Item(item) = path_item {
      if let Some(delete) = item.delete {
        api_path
          .operations
          .insert(ApiOperationType::Delete, create_api_operation(ApiOperationType::Delete, delete)?);
      }
      if let Some(get) = item.get {
        api_path.operations.insert(ApiOperationType::Get, create_api_operation(ApiOperationType::Get, get)?);
      }
      if let Some(head) = item.head {
        api_path
          .operations
          .insert(ApiOperationType::Head, create_api_operation(ApiOperationType::Head, head)?);
      }
      if let Some(patch) = item.patch {
        api_path
          .operations
          .insert(ApiOperationType::Patch, create_api_operation(ApiOperationType::Patch, patch)?);
      }
      if let Some(post) = item.post {
        api_path
          .operations
          .insert(ApiOperationType::Post, create_api_operation(ApiOperationType::Post, post)?);
      }
      if let Some(put) = item.put {
        api_path.operations.insert(ApiOperationType::Put, create_api_operation(ApiOperationType::Put, put)?);
      }
    }
    api_paths.push(api_path);
  }
  api_paths.sort_by(|a, b| a.path.cmp(&b.path));
  Ok(api_paths)
}

fn create_api_operation(_operation_type: ApiOperationType, operation: Operation) -> Result<ApiOperation, String> {
  let operation_id = operation.operation_id.ok_or("".to_string())?;
  let parameters: Vec<(String, ParameterType)> = operation
    .parameters
    .iter()
    .map(|parameter| parameter_to_parameter_type(parameter, &operation_id))
    .collect::<Vec<_>>();
  let request_body = operation.request_body.map(|request_body| match request_body {
    ReferenceOr::Reference { reference } => RequestBodyType::SerializableType(reference_to_string(reference.as_ref())),
    ReferenceOr::Item(request_body_item) => RequestBodyType::from(&request_body_item),
  });
  let mut _ok_responses: Vec<(u16, ResponseBodyType)> = vec![];
  let mut _error_responses: Vec<(u16, ResponseBodyType)> = vec![];
  for (status_code, response) in operation.responses.responses {
    if let StatusCode::Code(numerical_status_code) = status_code {
      if (200..300).contains(&numerical_status_code) {
        _ok_responses.push((numerical_status_code, response_to_response_body_type(&response)))
      } else {
        _error_responses.push((numerical_status_code, response_to_response_body_type(&response)))
      }
    }
  }
  let ok_response = _ok_responses.iter().min_by_key(|(status_code, _)| status_code).ok_or("".to_string())?.1.clone();
  Ok(ApiOperation { _operation_type, parameters, request_body, operation_id, ok_response, _ok_responses, _error_responses })
}

#[derive(Debug, PartialEq)]
enum PathElement {
  Literal(String),
  Variable(String),
}

impl Display for PathElement {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Literal(literal) => write!(f, "{}", literal),
      Self::Variable(variable) => write!(f, "{{{}}}", variable),
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

#[derive(Eq, Hash, PartialEq)]
enum ApiOperationType {
  Delete,
  Get,
  Head,
  Patch,
  Post,
  Put,
}

const _OPERATION_TYPES: [ApiOperationType; 6] =
  [ApiOperationType::Delete, ApiOperationType::Get, ApiOperationType::Head, ApiOperationType::Patch, ApiOperationType::Post, ApiOperationType::Put];

impl Display for ApiOperationType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Delete => write!(f, "DELETE"),
      Self::Get => write!(f, "GET"),
      Self::Head => write!(f, "HEAD"),
      Self::Patch => write!(f, "PATCH"),
      Self::Post => write!(f, "POST"),
      Self::Put => write!(f, "PUT"),
    }
  }
}

enum ParameterType {
  ConstructedTypeOwned(String),
  ConstructedTypeRef(String),
  SerializableType(String),
  RefStr,
}

impl ParameterType {
  fn to_index_parameter(&self, index: isize, name: &str) -> String {
    let get_or_first = if index == 0 { "first()".to_string() } else { format!("get({})", index) };
    match self {
      ParameterType::ConstructedTypeOwned(constructed_type) => format!(
        "/* {}: constructed owned */ {}::from_str(parameters.{}.unwrap())?",
        name, constructed_type, get_or_first
      ),
      ParameterType::ConstructedTypeRef(constructed_type) => format!(
        "/* {}: constructed ref */ &{}::from_str(parameters.{}.unwrap())?",
        name, constructed_type, get_or_first
      ),
      ParameterType::SerializableType(serializable_type) => format!(
        "/* {}: serializable */ &{}::from_str(parameters.{}.unwrap())?",
        name, serializable_type, get_or_first
      ),
      ParameterType::RefStr => format!("/* {}: &str */ parameters.{}.unwrap()", name, get_or_first),
    }
  }
}

impl Display for ParameterType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::ConstructedTypeOwned(parameter_type) => write!(f, "{}", parameter_type),
      Self::ConstructedTypeRef(parameter_type) => write!(f, "{}", parameter_type),
      Self::SerializableType(parameter_type) => write!(f, "{}", parameter_type),
      Self::RefStr => write!(f, "str"),
    }
  }
}

#[derive(Debug)]
enum RequestBodyType {
  String,
  SerializableType(String),
}

impl Display for RequestBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::String => write!(f, "String"),
      Self::SerializableType(type_) => write!(f, "{}", type_),
    }
  }
}

#[derive(Clone, Debug)]
enum ResponseBodyType {
  Ok(String),
  SerializableType(String),
  String,
}

impl ResponseBodyType {
  fn response_mapping(&self) -> &str {
    match self {
      ResponseBodyType::Ok(_) => ".map(|(_, result)| result)",
      ResponseBodyType::SerializableType(_) => ".map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)",
      ResponseBodyType::String => ".await.map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)",
    }
  }

  fn processing_function(&self) -> &str {
    match self {
      ResponseBodyType::Ok(_) => "process",
      ResponseBodyType::SerializableType(_) => "process",
      ResponseBodyType::String => "process_string",
    }
  }
}

impl Display for ResponseBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Ok(desc) => write!(f, "{}", desc),
      Self::SerializableType(type_) => write!(f, "{}", type_),
      Self::String => write!(f, "str"),
    }
  }
}

struct ApiPath {
  path: String,
  _path_elements: Vec<PathElement>,
  operations: HashMap<ApiOperationType, ApiOperation>,
}

struct ApiOperation {
  _operation_type: ApiOperationType,
  parameters: Vec<(String, ParameterType)>,
  request_body: Option<RequestBodyType>,
  operation_id: String,
  ok_response: ResponseBodyType,
  _ok_responses: Vec<(u16, ResponseBodyType)>,
  _error_responses: Vec<(u16, ResponseBodyType)>,
}

impl ApiPath {
  fn _print(&self) {
    println!(
      "{}",
      self
        ._path_elements
        .iter()
        .map(|path_element| path_element.to_string())
        .collect::<Vec<_>>()
        .join("/")
    );
    println!("  {}", self.path);
    for operation_type in _OPERATION_TYPES {
      if let Some(operation) = self.operations.get(&operation_type) {
        operation._print();
      }
    }
  }
}

impl ApiOperation {
  fn _print(&self) {
    println!("  {}", self._operation_type);
    for (parameter, parameter_type) in &self.parameters {
      println!("    parameter {}: {}", parameter, parameter_type);
    }
    if let Some(ref request_body) = self.request_body {
      println!("    request body: {}", request_body);
    }
    println!("    operation id: {}", self.operation_id);
    if !self._ok_responses.is_empty() {
      println!(
        "    ok responses: {}",
        self
          ._ok_responses
          .iter()
          .map(|(status_code, response_type)| format!("{}: {}", status_code, response_type))
          .collect::<Vec<_>>()
          .join(", ")
      );
    }
    if !self._error_responses.is_empty() {
      println!(
        "    error responses: {}",
        self
          ._error_responses
          .iter()
          .map(|(status_code, response_type)| format!("{}: {}", status_code, response_type))
          .collect::<Vec<_>>()
          .join(", ")
      );
    }
  }

  fn to_if_block(&self, comments: Vec<String>) -> String {
    let mut parameter_counter = -1;
    let mut parameters = self
      .parameters
      .iter()
      .map(|(parameter_name, parameter_type)| {
        if parameter_name == "tenant" {
          "self.tenant_name()".to_string()
        } else if parameter_name == "Authorization" {
          "self.token()".to_string()
        } else {
          parameter_counter += 1;
          parameter_type.to_index_parameter(parameter_counter, parameter_name)
        }
      })
      .collect::<Vec<_>>();
    if let Some(ref request_body_type) = self.request_body {
      match request_body_type {
        RequestBodyType::String => parameters.push(
          "serde_json::from_str::<String>(body).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid String\".to_string()))?.to_string()".to_string(),
        ),
        RequestBodyType::SerializableType(serializable_type) => parameters.push(format!(
          "&serde_json::from_str::<{}>(body).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid {}\".to_string()))?",
          serializable_type, serializable_type
        )),
      }
    }
    formatdoc!(
      r#"
        if path == "{}" {{
              // {}
              self
                .{}(
                  self
                    .generated_client
                    .{}(
                      {},
                    )
                    .await,
                )
                {}
            }}"#,
      self.operation_id,
      comments.join("\n      // "),
      self.ok_response.processing_function(),
      self.operation_id,
      parameters.join(",\n              "),
      self.ok_response.response_mapping()
    )
  }

  fn to_get_if_block(&self, api_path: &String) -> String {
    self.to_if_block(vec![
      format!("GET {}", api_path),
      self
        .parameters
        .iter()
        .map(|(parameter_name, parameter_type)| format!("{}:{}", parameter_name, parameter_type))
        .collect::<Vec<_>>()
        .join(", "),
      format!("{:?}", self.ok_response),
    ])
  }

  fn to_put_if_block(&self, api_path: &String) -> String {
    self.to_if_block(vec![
      format!("PUT {}", api_path),
      self
        .parameters
        .iter()
        .map(|(parameter_name, parameter_type)| format!("{}:{}", parameter_name, parameter_type))
        .collect::<Vec<_>>()
        .join(", "),
      format!("{:?} / {:?}", self.request_body, self.ok_response),
    ])
  }
}

fn type_to_string(type_: &Type) -> String {
  match type_ {
    Type::String(_) => "str".to_string(),
    Type::Number(_) => unimplemented!(),
    Type::Integer(_) => unimplemented!(),
    Type::Object(object_type) => match object_type.additional_properties {
      Some(ref additional_properties) => match additional_properties {
        AdditionalProperties::Any(_) => unimplemented!(),
        AdditionalProperties::Schema(schema) => format!("HashMap<String, {}>", schema_to_string(schema)),
      },
      None => unimplemented!(),
    },
    Type::Array(array_type) => match array_type.items {
      Some(ref items) => format!("Array of {}", boxed_schema_to_string(items)),
      None => unimplemented!(),
    },
    Type::Boolean(_) => unimplemented!(),
  }
}

fn schema_to_string(parameter: &ReferenceOr<Schema>) -> String {
  match parameter {
    ReferenceOr::Reference { reference } => reference_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref type_) => type_to_string(type_),
        _ => unimplemented!(),
      }
    }
  }
}

fn boxed_schema_to_string(parameter: &ReferenceOr<Box<Schema>>) -> String {
  match parameter {
    ReferenceOr::Reference { reference } => reference_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref type_) => type_to_string(type_),
        _ => unimplemented!(),
      }
    }
  }
}

fn capitalize(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

fn to_type_name(operation_id: &str, name: &str) -> String {
  format!("{}{}", operation_id.split('_').map(capitalize).collect::<Vec<_>>().join(""), capitalize(name))
}

fn parameter_to_parameter_type(parameter: &ReferenceOr<Parameter>, operation_id: &str) -> (String, ParameterType) {
  match parameter {
    ReferenceOr::Reference { .. } => unimplemented!(),
    ReferenceOr::Item(parameter_item) => {
      let parameter_data = parameter_item.clone().parameter_data();
      match parameter_data.format {
        ParameterSchemaOrContent::Schema(ref schema) => match schema {
          ReferenceOr::Reference { reference } => (parameter_data.name, ParameterType::SerializableType(reference_to_string(reference))),
          ReferenceOr::Item(item) => match &item.schema_kind {
            SchemaKind::Type(type_) => match type_ {
              Type::String(string_type) => {
                let has_pattern = string_type.pattern.is_some();
                let has_enumeration = !string_type.enumeration.is_empty();
                match (has_pattern, has_enumeration) {
                  (false, false) => (parameter_data.name, ParameterType::RefStr), // No pattern, no enumeration -> &str
                  (false, true) => (
                    parameter_data.name.clone(),
                    ParameterType::ConstructedTypeOwned(to_type_name(operation_id, parameter_data.name.as_str())),
                  ), // No pattern, enumeration -> Constructed owned type
                  (true, false) => (
                    parameter_data.name.clone(),
                    ParameterType::ConstructedTypeRef(to_type_name(operation_id, parameter_data.name.as_str())),
                  ), // Pattern, no enumeration -> Constructed ref type
                  (true, true) => unimplemented!(),                               // Pattern and enumeration -> Should not occur
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

fn response_to_response_body_type(response: &ReferenceOr<Response>) -> ResponseBodyType {
  match response {
    ReferenceOr::Reference { .. } => unimplemented!(),
    ReferenceOr::Item(response) => match response.content.get("application/json") {
      Some(media_type) => match &media_type.schema {
        Some(schema) => ResponseBodyType::SerializableType(schema_to_string(schema)),
        None => unimplemented!(),
      },
      None => match response.content.get("text/plain") {
        Some(_) => ResponseBodyType::String,
        None => ResponseBodyType::Ok(response.description.clone()),
      },
    },
  }
}

fn reference_to_string(reference: &str) -> String {
  match reference.strip_prefix("#/components/schemas/") {
    Some(type_) => type_.to_string(),
    None => format!("$ref: {}", reference),
  }
}

impl From<&RequestBody> for RequestBodyType {
  fn from(request_body: &RequestBody) -> Self {
    match request_body.content.get("application/json") {
      Some(media_type) => match &media_type.schema {
        Some(schema) => RequestBodyType::SerializableType(schema_to_string(schema)),
        None => unimplemented!(),
      },
      None => match request_body.content.get("text/plain") {
        Some(_) => RequestBodyType::String,
        None => unimplemented!(),
      },
    }
  }
}
