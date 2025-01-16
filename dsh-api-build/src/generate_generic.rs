use indoc::formatdoc;
use itertools::Itertools;
use openapiv3::{
  AdditionalProperties, OpenAPI, Operation, Parameter, ParameterSchemaOrContent, PathItem, ReferenceOr, RequestBody, Response, Schema, SchemaKind, StatusCode, Type,
};
use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use PathElement;

// TODO For development testing only
#[allow(dead_code)]
const PATHS_FILTER: [&str; 3] = [
  // "/allocation/{tenant}/application/configuration",
  // "/allocation/{tenant}/application/{appid}/configuration",
  // "/allocation/{tenant}/task",
  // "/allocation/{tenant}/task/{appid}",
  "/allocation/{tenant}/secret",
  "/allocation/{tenant}/secret/{id}/configuration",
  "/allocation/{tenant}/secret/{id}",
];
#[allow(dead_code)]
const METHODS_FILTER: [Method; 6] = [Method::Delete, Method::Get, Method::Head, Method::Patch, Method::Post, Method::Put];

pub fn generate_generic(writer: &mut dyn Write, openapi_spec: &OpenAPI) -> Result<(), Box<dyn Error>> {
  let mut generic_operations: Vec<(Method, Vec<GenericOperation>)> = vec![];
  for method in &METHODS {
    let path_operations: Vec<(&String, &Operation)> = method_path_operations(method, openapi_spec);
    generic_operations.push((method.to_owned(), method_generic_operations(method, &path_operations)?));
  }

  writeln!(writer, "#[cfg_attr(rustfmt, rustfmt_skip)]")?;
  writeln!(writer, "{}", USE)?;
  writeln!(writer)?;
  writeln!(writer, "{}", COMMENT_OUTER)?;
  writeln!(writer)?;
  writeln!(writer, "impl DshApiClient<'_> {{")?;

  let mut first = true;
  for (method, operations) in &generic_operations {
    if !operations.is_empty() {
      if !first {
        writeln!(writer)?;
      }
      write_method_operations(writer, method, operations)?;
      first = false;
    }
  }
  writeln!(writer, "}}")?;
  writeln!(writer)?;
  writeln!(writer, "{}", METHOD_DESCRIPTOR_STRUCT)?;

  for (method, operations) in &generic_operations {
    writeln!(writer)?;
    write_method_operations_descriptors(writer, method, operations)?;
  }
  Ok(())
}

fn write_method_operations(writer: &mut dyn Write, method: &Method, operations: &[GenericOperation]) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "  /// # Generic `{}` operations", method)?;
  if let Some(method_comment) = method.comment() {
    writeln!(writer, "  ///")?;
    writeln!(writer, "{}", method_comment)?;
    writeln!(writer, "  ///")?;
  }
  writeln!(writer, "  /// ## Supported operation selectors for the `{}` method", method)?;
  for operation in operations.iter() {
    writeln!(writer, "  ///")?;
    writeln!(writer, "  /// # __`{}`__", operation.selector)?;
    if let Some(ref description) = operation.description {
      writeln!(writer, "  /// {}", description)?;
      writeln!(writer, "  ///")?;
    }
    writeln!(writer, "  /// `{}` `{}`", method.to_string().as_str().to_uppercase(), operation.path)?;
    for (parameter_name, parameter_type, description) in &operation.parameters {
      if !MANAGED_PARAMETERS.contains(&parameter_name.as_str()) {
        if let Some(description) = description {
          writeln!(writer, "  /// * `{}` : `{}` - {}", parameter_name, parameter_type, description)?;
        } else {
          writeln!(writer, "  /// * `{}` : `{}`", parameter_name, parameter_type)?;
        }
      }
    }
    if let Some(ref request_body) = operation.request_body {
      writeln!(writer, "  /// * `body` : {}", request_body.doc_type())?;
    }
    writeln!(writer, "  ///")?;
    writeln!(writer, "  /// Returns {}.", operation.ok_response.doc_return_value())?;
  }
  writeln!(writer, "  {} {{", method.signature())?;
  let mut first = true;
  for operation in operations.iter() {
    if first {
      write!(writer, "    {}", operation.to_if_block())?;
    } else {
      write!(writer, " else {}", operation.to_if_block())?;
    }
    first = false;
  }
  writeln!(writer, " else {{")?;
  writeln!(
    writer,
    "      Err(DshApiError::Configuration(format!(\"{} method selector '{{}}' not recognized\", selector)))",
    method
  )?;
  writeln!(writer, "    }}")?;
  writeln!(writer, "  }}")?;
  Ok(())
}

const MANAGED_PARAMETERS: [&str; 1] = ["Authorization"];

fn write_method_operations_descriptors(writer: &mut dyn Write, method: &Method, operations: &[GenericOperation]) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "/// `{}` method descriptors", method)?;
  if operations.is_empty() {
    writeln!(
      writer,
      "pub const {}_METHODS: [(&str, MethodDescriptor); {}] = [];",
      method.to_string().as_str().to_uppercase(),
      operations.len()
    )?;
  } else {
    writeln!(
      writer,
      "pub const {}_METHODS: [(&str, MethodDescriptor); {}] = [",
      method.to_string().as_str().to_uppercase(),
      operations.len()
    )?;
    for operation in operations.iter() {
      writeln!(writer, "  (")?;
      let parameters = operation
        .parameters
        .iter()
        .filter(|(name, _, _)| !MANAGED_PARAMETERS.contains(&name.as_str()))
        .map(|(parameter, parameter_type, description)| {
          format!(
            "(\"{}\", \"{}\", {})",
            parameter,
            parameter_type,
            match description {
              None => "None".to_string(),
              Some(description) => format!("Some(\"{}\")", description),
            }
          )
        })
        .collect::<Vec<_>>();
      writeln!(writer, "    \"{}\",", operation.selector)?;
      writeln!(writer, "    MethodDescriptor {{")?;
      if let Some(ref description) = operation.description {
        writeln!(writer, "      description: Some(\"{}\"),", description)?;
      } else {
        writeln!(writer, "      description: None,")?;
      }
      if parameters.is_empty() {
        writeln!(writer, "      parameters: &[],")?;
      } else {
        writeln!(writer, "      parameters: &[")?;
        writeln!(writer, "        {},", parameters.join(",\n        "))?;
        writeln!(writer, "      ],")?;
      }
      if let Some(ref body_type) = operation.request_body {
        writeln!(writer, "      body_type: Some(\"{}\"),", body_type)?;
      } else {
        writeln!(writer, "      body_type: None,")?;
      }
      writeln!(writer, "      response_type: Some(\"{}\")", operation.ok_response)?;
      writeln!(writer, "    }}")?;
      writeln!(writer, "  ),")?;
    }
    writeln!(writer, "];")?;
  }

  Ok(())
}

fn get_method_operation<'a>(method: &Method, path_item: &'a PathItem) -> Option<&'a Operation> {
  match method {
    Method::Delete => path_item.delete.as_ref(),
    Method::Get => path_item.get.as_ref(),
    Method::Head => path_item.head.as_ref(),
    Method::Patch => path_item.patch.as_ref(),
    Method::Post => path_item.post.as_ref(),
    Method::Put => path_item.put.as_ref(),
  }
}

// Returns all (path, operation) pairs for a given method
fn method_path_operations<'a>(method: &Method, openapi: &'a OpenAPI) -> Vec<(&'a String, &'a Operation)> {
  let mut method_path_items: Vec<(&String, &Operation)> = vec![];
  for (path, path_item) in openapi.paths.iter() {
    // if !PATHS_FILTER.contains(&path.as_str()) {
    //   continue;
    // }
    if let ReferenceOr::Item(ref path_item) = path_item {
      if let Some(operation) = get_method_operation(method, path_item) {
        method_path_items.push((path, operation))
      }
    }
  }
  method_path_items
}

fn method_generic_operations(method: &Method, path_operations: &Vec<(&String, &Operation)>) -> Result<Vec<GenericOperation>, Box<dyn Error>> {
  let mut method_generic_operations: Vec<GenericOperation> = vec![];
  let mut selectors: HashSet<String> = HashSet::new();
  for (path, operation) in path_operations {
    let mut generic_operation = create_generic_operation(method.clone(), path.to_string(), operation)?;
    if selectors.contains(&generic_operation.selector) {
      generic_operation.selector = selector_from_path_elements(&generic_operation.path_elements, &generic_operation.ok_response, true);
    }
    selectors.insert(generic_operation.selector.clone());
    method_generic_operations.push(generic_operation);
  }
  check_duplicate_selectors(&method_generic_operations, method)?;
  Ok(method_generic_operations)
}

fn selector_from_path_elements(path_elements: &[PathElement], ok_response: &ResponseBodyType, include_variables: bool) -> String {
  let mut selector = path_elements
    .iter()
    .filter_map(|path_element| match path_element {
      PathElement::Literal(literal) => {
        if literal != "allocation" {
          Some(literal.to_string())
        } else {
          None
        }
      }
      PathElement::Variable(variable) => {
        if variable == "tenant" || variable == "manager" {
          None
        } else if include_variables || (variable != "id" && variable != "appid") {
          Some(variable.to_string())
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

fn create_generic_operation(method: Method, path: String, operation: &Operation) -> Result<GenericOperation, Box<dyn Error>> {
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
  Ok(GenericOperation {
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
  })
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Method {
  Delete,
  Get,
  Head,
  Patch,
  Post,
  Put,
}

const METHODS: [Method; 6] = [Method::Delete, Method::Get, Method::Head, Method::Patch, Method::Post, Method::Put];

impl Method {
  pub(crate) fn signature(&self) -> &str {
    match self {
      Self::Delete => "pub async fn delete(&self, selector: &str, parameters: &[&str]) -> DshApiResult<()>",
      Self::Get => "pub async fn get(&self, selector: &str, parameters: &[&str]) -> DshApiResult<Box<dyn erased_serde::Serialize>>",
      Self::Head => "pub async fn head(&self, selector: &str, parameters: &[&str]) -> DshApiResult<()>",
      Self::Patch => "pub async fn patch(&self, selector: &str, parameters: &[&str], body: Option<&str>) -> DshApiResult<()>",
      Self::Post => "pub async fn post(&self, selector: &str, parameters: &[&str], body: Option<&str>) -> DshApiResult<()>",
      Self::Put => "pub async fn put(&self, selector: &str, parameters: &[&str], body: Option<&str>) -> DshApiResult<()>",
    }
  }

  pub(crate) fn has_body_argument(&self) -> bool {
    match self {
      Self::Get | Self::Delete | Self::Head => false,
      Self::Patch | Self::Post | Self::Put => true,
    }
  }

  fn comment(&self) -> Option<&'static str> {
    match self {
      Method::Delete => None,
      Method::Get => Some(GET_COMMENT),
      Method::Head => None,
      Method::Patch => None,
      Method::Post => None,
      Method::Put => Some(PUT_COMMENT),
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

#[derive(Clone, Debug)]
enum RequestBodyType {
  String,
  SerializableType(String),
}

impl RequestBodyType {
  fn doc_type(&self) -> String {
    match self {
      Self::String => "`&str`".to_string(),
      Self::SerializableType(serializable_type) => format!("serialized [`{}`]", serializable_type),
    }
  }
}

impl Display for RequestBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::String => write!(f, "String"),
      Self::SerializableType(serializable_type) => write!(f, "{}", serializable_type),
    }
  }
}

#[derive(Clone, Debug)]
enum ResponseBodyType {
  Ids,
  Ok(String),
  SerializableMap(String),
  SerializableScalar(String),
  SerializableVector(String),
  String,
}

impl ResponseBodyType {
  fn response_mapping(&self, method: &Method) -> &str {
    match method {
      Method::Get => match self {
        Self::Ids => ".map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
        Self::Ok(_) => ".map(|(_, result)| result)",
        Self::SerializableMap(_) => ".map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
        Self::SerializableScalar(_) => ".map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
        Self::SerializableVector(_) => ".map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
        Self::String => ".await.map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
      },
      _ => ".map(|(_, _)| ())",
    }
  }

  fn processing_function(&self) -> &str {
    match self {
      Self::String => "process_string",
      _ => "process",
    }
  }

  fn doc_return_value(&self) -> String {
    match self {
      Self::Ids => "`Vec<String>`".to_string(),
      Self::Ok(desc) => format!("`Ok(())` when {}", desc),
      Self::SerializableMap(value_type) => format!("`HashMap<String, `[`{}`]`>`", value_type),
      Self::SerializableScalar(scalar_type) => format!("[`{}`]", scalar_type),
      Self::SerializableVector(element_type) => format!("`Vec<`[`{}`]`>`", element_type),
      Self::String => "String".to_string(),
    }
  }
}

impl Display for ResponseBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Ids => write!(f, "Vec<String>"),
      Self::Ok(desc) => write!(f, "{}", desc),
      Self::SerializableMap(value_type) => write!(f, "HashMap<String, {}>", value_type),
      Self::SerializableScalar(scalar_type) => write!(f, "{}", scalar_type),
      Self::SerializableVector(element_type) => write!(f, "Vec<{}>", element_type),
      Self::String => write!(f, "str"),
    }
  }
}

struct GenericOperation {
  method: Method,
  selector: String,
  path: String,
  path_elements: Vec<PathElement>,
  description: Option<String>,
  parameters: Vec<(String, ParameterType, Option<String>)>,
  request_body: Option<RequestBodyType>,
  operation_id: String,
  ok_response: ResponseBodyType,
  #[allow(dead_code)]
  ok_responses: Vec<(u16, ResponseBodyType)>,
  #[allow(dead_code)]
  error_responses: Vec<(u16, ResponseBodyType)>,
}

impl GenericOperation {
  fn comments(&self) -> Vec<String> {
    let mut comments = vec![];
    comments.push(format!("{} {}", self.method.to_string().as_str().to_uppercase(), self.path));
    for (parameter_name, parameter_type, description) in &self.parameters {
      if !MANAGED_PARAMETERS.contains(&parameter_name.as_str()) {
        match description {
          Some(description) => comments.push(format!("{}:{}, {}", parameter_name, parameter_type, revise(description.to_string()))),
          None => comments.push(format!("{}:{}", parameter_name, parameter_type)),
        }
      }
    }
    if let Some(request_body) = self.request_body.clone().map(|request_body| request_body.to_string()) {
      comments.push(format!("body: {}", request_body));
    }
    comments.push(self.ok_response.doc_return_value().to_string());
    comments
  }

  fn to_if_block(&self) -> String {
    let mut parameter_counter = -1;
    let mut parameters = self
      .parameters
      .iter()
      .map(|(parameter_name, parameter_type, _)| {
        if parameter_name == "Authorization" {
          "self.token().await?.as_str()".to_string()
        } else {
          parameter_counter += 1;
          parameter_type.to_index_parameter(parameter_counter, parameter_name)
        }
      })
      .collect::<Vec<_>>();
    if let Some(ref request_body_type) = self.request_body {
      match request_body_type {
        RequestBodyType::String => parameters.push(
          "serde_json::from_str::<String>(body.unwrap()).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid String\".to_string()))?.to_string()"
            .to_string(),
        ),
        RequestBodyType::SerializableType(serializable_type) => parameters.push(format!(
          "&serde_json::from_str::<{}>(body.unwrap()).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid {}\".to_string()))?",
          serializable_type, serializable_type
        )),
      }
    }
    let number_of_expected_parameters = if self.request_body.is_none() { parameters.len() as i64 - 1 } else { parameters.len() as i64 - 2 };
    let (parameter_length_check, wrong_parameter_length_error) = match number_of_expected_parameters {
      0 => ("!parameters.is_empty()".to_string(), "none expected".to_string()),
      1 => ("parameters.len() != 1".to_string(), "one parameter expected".to_string()),
      _ => (
        format!("parameters.len() != {}", number_of_expected_parameters),
        format!("{} parameters expected", number_of_expected_parameters),
      ),
    };
    let body_check: Option<String> = if self.method.has_body_argument() {
      match self.request_body {
        Some(ref request_body) => Some(format!(
          r#"}} else if body.is_none() {{
        Err(DshApiError::Parameter("body expected ({})".to_string()))
      "#,
          request_body
        )),
        None => Some(
          r#"} else if body.is_some() {
        Err(DshApiError::Parameter("no body expected".to_string()))
      "#
          .to_string(),
        ),
      }
    } else {
      None
    };
    formatdoc!(
      r#"
        if selector == "{}" || selector == "{}" {{
              // {}
              if {} {{
                Err(DshApiError::Parameter("wrong number of parameters ({})".to_string()))
              {}}} else {{
                self
                  .{}(
                    self
                      .generated_client
                      .{}(
                        self.tenant_name(),
                        {},
                      )
                      .await,
                  )
                  {}
              }}
            }}"#,
      self.selector,
      self.path,
      self.comments().join("\n      // "),
      parameter_length_check,
      wrong_parameter_length_error,
      body_check.unwrap_or_default(),
      self.ok_response.processing_function(),
      self.operation_id,
      parameters.join(",\n                "),
      self.ok_response.response_mapping(&self.method),
    )
  }
}

impl From<&Type> for ResponseBodyType {
  fn from(openapi_type: &Type) -> Self {
    match openapi_type {
      Type::String(_) => ResponseBodyType::String,
      Type::Number(_) => unimplemented!(),
      Type::Integer(_) => unimplemented!(),
      Type::Object(object_type) => match object_type.additional_properties {
        Some(ref additional_properties) => match additional_properties {
          AdditionalProperties::Any(_) => unimplemented!(),
          AdditionalProperties::Schema(schema) => ResponseBodyType::SerializableMap(schema_to_string(schema)),
        },
        None => unimplemented!(),
      },
      Type::Array(array_type) => match array_type.items {
        Some(ref items) => ResponseBodyType::SerializableVector(boxed_schema_to_string(items)),
        None => unimplemented!(),
      },
      Type::Boolean(_) => unimplemented!(),
    }
  }
}

fn type_to_string(openapi_type: &Type) -> String {
  match openapi_type {
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
      Some(ref items) => format!("Vec<{}>", boxed_schema_to_string(items)),
      None => unimplemented!(),
    },
    Type::Boolean(_) => unimplemented!(),
  }
}

impl From<&ReferenceOr<Schema>> for ResponseBodyType {
  fn from(schema: &ReferenceOr<Schema>) -> Self {
    match schema {
      ReferenceOr::Reference { reference } => {
        let scalar_type = reference_to_string(reference);
        if scalar_type == "ChildList" {
          ResponseBodyType::Ids
        } else {
          ResponseBodyType::SerializableScalar(reference_to_string(reference))
        }
      }
      ReferenceOr::Item(schema) => {
        let schema_kind = schema.schema_kind.clone();
        match schema_kind {
          SchemaKind::Type(ref schema_kind_type) => ResponseBodyType::from(schema_kind_type),
          SchemaKind::OneOf { .. } => unimplemented!(),
          SchemaKind::AllOf { .. } => unimplemented!(),
          SchemaKind::AnyOf { .. } => unimplemented!(),
          SchemaKind::Not { .. } => unimplemented!(),
          SchemaKind::Any(_) => unimplemented!(),
        }
      }
    }
  }
}

fn schema_to_string(schema: &ReferenceOr<Schema>) -> String {
  match schema {
    ReferenceOr::Reference { reference } => reference_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref schema_kind_type) => type_to_string(schema_kind_type),
        SchemaKind::OneOf { .. } => unimplemented!(),
        SchemaKind::AllOf { .. } => unimplemented!(),
        SchemaKind::AnyOf { .. } => unimplemented!(),
        SchemaKind::Not { .. } => unimplemented!(),
        SchemaKind::Any(_) => unimplemented!(),
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
        SchemaKind::Type(ref schema_kind_type) => type_to_string(schema_kind_type),
        _ => unimplemented!(),
      }
    }
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

fn capitalize<T: AsRef<str>>(string: T) -> String {
  let mut chars = string.as_ref().chars();
  match chars.next() {
    None => String::new(),
    Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

fn to_type_name(operation_id: &str, name: &str) -> String {
  format!("{}{}", operation_id.split('_').map(capitalize).collect::<Vec<_>>().join(""), capitalize(name))
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

impl From<&ReferenceOr<Response>> for ResponseBodyType {
  fn from(response: &ReferenceOr<Response>) -> Self {
    match response {
      ReferenceOr::Reference { .. } => unimplemented!(),
      ReferenceOr::Item(response) => match response.content.get("application/json") {
        Some(media_type) => match &media_type.schema {
          Some(schema) => ResponseBodyType::from(schema),
          None => unimplemented!(),
        },
        None => match response.content.get("text/plain") {
          Some(_) => ResponseBodyType::String,
          None => ResponseBodyType::Ok(response.description.clone()),
        },
      },
    }
  }
}

fn reference_to_string(reference: &str) -> String {
  match reference.strip_prefix("#/components/schemas/") {
    Some(reference_type) => reference_type.to_string(),
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

// Method will check whether duplicate selectors exist
fn check_duplicate_selectors(method_operations: &[GenericOperation], method: &Method) -> Result<(), Box<dyn Error>> {
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

const METHOD_DESCRIPTOR_STRUCT: &str = r#"/// # Describes one method
///
/// This structure is used to describe the available generic methods.
/// For each method type there is constant vector defined that consists of
/// `(&str, MethodDescriptor)` pairs,
/// listing the selectors and method descriptions for the method type.
///
/// # Example
///
/// This example will list all `get` selectors with a description of the
/// method indicated by the selector.
///
/// ```ignore
/// use dsh_api::generic::GET_METHODS;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// for (selector, method_descriptor) in GET_METHODS {
///   println!("{}: {}", selector, method_descriptor.description);
/// }
/// # }
/// ```
#[derive(Debug)]
pub struct MethodDescriptor {
  pub description: Option<&'static str>,
  pub parameters: &'static[(&'static str, &'static str, Option<&'static str>)],
  pub body_type: Option<&'static str>,
  pub response_type: Option<&'static str>
}"#;

const COMMENT_OUTER: &str = r#"/// # Generic API function calls
///
/// Module that contains methods to call the api methods in a generic way.
/// What this means is that the API functions can be called indirect,
/// where the path of the method must be provided as an argument.
///
/// This has a number of consequences which are caused by the limitations
/// of the `rust` language with respect to abstraction:
/// * The number and types of the required parameters for each method
///   are not known at compile time, which means that (emulated) dynamic typing is used
///   and parameters errors will occur at run-time instead of compile time.
///   * Path parameters must be provided as `&str`.
///   * Body parameters must be provided as a json `&str` that can be deserialized at runtime
///     into the expected type.
/// * The response type for each method is not known at compile time.
///   * For `GET` methods the responses will be returned as dynamic trait objects
///     that implement [`erased_serde::Serialize`], defined in the
///     [`erased_serde`](https://crates.io/crates/erased-serde) crate.
///     These objects can be serialized into json, yaml or toml without any type information.
///   * If `DELETE`, `POST` and `PUT` methods return data this will be ignored
///     and only errors will be returned.
///
/// # Examples
///
/// Get the configuration of the application `keyring-dev` and print it as json.
///
/// ```ignore
/// # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
/// let application = client.get(
///   "get_application_configuration_by_tenant_by_appid",
///   &["keyring-dev"]
/// ).await?;
/// println!("{}", serde_json::to_string_pretty(&application)?);
/// # Ok(())
/// # }
/// ```
///
/// Update the secret `abcdef` to the value `ABCDEF`.
///
/// ```ignore
/// # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
///  let secret = serde_json::to_string("ABCDEF")?;
///  client.put("put_secret_by_tenant_by_id", &["abcdef"], &secret).await?;
/// # Ok(())
/// # }
/// ```
///
/// # API functions
///
/// [`DshApiClient`] methods that call the DSH resource management API.
///
/// * [`delete(path, [parameters]) -> Ok`](DshApiClient::delete)
/// * [`get(path, [parameters]) -> serialized`](DshApiClient::get)
/// * [`head(path, [parameters], body) -> Ok`](DshApiClient::head)
/// * [`patch(path, [parameters], body) -> Ok`](DshApiClient::patch)
/// * [`post(path, [parameters], body) -> Ok`](DshApiClient::post)
/// * [`put(path, [parameters], body) -> Ok`](DshApiClient::put)"#;

const GET_COMMENT: &str = r#"  /// The`get` function enables the generic calling of all
  /// `get` functions of the Dsh API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function, the types of the parameters and the response
  /// are not known at compile time. This has some consequences.
  /// * All parameters must be provided as a list of strings in the form of a `&[&str]`.
  /// * The results of this method will be returned as a dynamic trait object
  ///   that implements [`erased_serde::Serialize`].
  ///   This object can be used to serialize the result to json, yaml or toml or
  ///   any other compatible `rust` serialization solution, without the need of any type information.
  ///
  /// This does however require a dependency to the
  /// [`erased_serde`](https://crates.io/crates/erased-serde) crate.
  /// Add the following line to the `dependencies` section of your `Cargo.toml` file.
  /// ```toml
  /// [dependencies]
  /// erased-serde = "0.4"
  /// ```
  /// ## Example
  ///
  /// Get the configuration of the application `my-service` and print it as json.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
  /// let application = client.get(
  ///   "get_application_configuration_by_tenant_by_appid",
  ///   &["my-service"]
  /// ).await?;
  /// println!("{}", serde_json::to_string_pretty(&application)?);
  /// # Ok(())
  /// # }
  /// ```"#;

const PUT_COMMENT: &str = r#"  /// ## Example
  /// Update the secret `abcdef` to the value `ABCDEF`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DEFAULT_DSH_API_CLIENT_FACTORY;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = &DEFAULT_DSH_API_CLIENT_FACTORY.client().await?;
  ///  let secret = serde_json::to_string("ABCDEF")?;
  ///  client.put("put_secret_by_tenant_by_id", &["abcdef"], &secret).await?;
  /// # Ok(())
  /// # }
  /// ```"#;

const USE: &str = r#"use crate::dsh_api_client::DshApiClient;
use crate::types::*;
use crate::{DshApiError, DshApiResult};
use std::str::FromStr;"#;
