use dsh_api::dsh_api_client::DshApiClient;
use indoc::formatdoc;
use openapiv3::{AdditionalProperties, OpenAPI, Operation, Parameter, ParameterSchemaOrContent, ReferenceOr, RequestBody, Response, Schema, SchemaKind, StatusCode, Type};
use serde_json;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

fn main() -> Result<(), String> {
  // let openapi_spec_original_file = "dsh-api/openapi_spec/openapi_1_9_0.json";
  // let file = std::fs::File::open(openapi_spec_original_file).unwrap();
  // let openapi_spec: OpenAPI = serde_json::from_reader(file).unwrap();

  let openapi_spec_string = DshApiClient::openapi_spec();
  let openapi_spec: OpenAPI = serde_json::from_str(openapi_spec_string).unwrap();
  println!("// openapi spec version: {}", openapi_spec.info.version);

  let api_paths = api_paths(openapi_spec)?;

  println!("use crate::dsh_api_client::DshApiClient;");
  println!("use crate::types::*;");
  println!("use crate::{{DshApiError, DshApiResult}};");
  println!("use erased_serde::Serialize as ErasedSerialize;");
  println!("use std::any::Any;");
  println!("use std::ops::Deref;");
  println!();

  println!("impl DshApiClient<'_> {{");

  print_get_operations(&api_paths);
  println!();
  print_put_operations(&api_paths);

  println!("}}");

  Ok(())
}

fn print_get_operations(api_paths: &Vec<ApiPath>) {
  println!("  pub async fn get(&self, path: &str, parameters: &[&str]) -> DshApiResult<Box<dyn ErasedSerialize>> {{");
  let get_operations: Vec<(&String, &ApiOperation)> = api_paths
    .iter()
    .filter_map(|api_path| api_path.operations.get(&ApiOperationType::Get).map(|api_operation| (&api_path.path, api_operation)))
    .collect::<Vec<_>>();

  // TODO For testing only
  // let get_operations: Vec<(&String, &ApiOperation)> = get_operations
  //   .into_iter()
  //   .filter(|(path, _)| *path == "/manage/{manager}/tenant/{tenant}/limit/{kind}")
  //   .collect::<Vec<_>>();

  let mut first = true;
  for (api_path, api_operation) in get_operations {
    if first {
      print!("    {}", api_operation.to_get(api_path))
    } else {
      print!(" else {}", api_operation.to_get(api_path));
    }
    first = false;
  }
  println!(" else {{");
  println!("      Err(DshApiError::Configuration(format!(\"get method '{{}}' not recognized\", path)))");
  println!("    }}");
  println!("  }}");
}

//   pub async fn post(&self, path: &str, parameters: &[Box<dyn Any>]) -> DshApiResult<()> {
//     if path == "secret_by_tenant" {
//       self
//         .process(
//           self
//             .generated_client
//             .post_secret_by_tenant(
//               self.tenant_name(),
//               self.token(),
//               parameters.get(0).unwrap().deref().downcast_ref::<Secret>().unwrap(),
//             )
//             .await,
//         )
//         .map(|(_, result)| result)
//     } else {
//       Err(DshApiError::Configuration(format!("post method '{}' not recognized", path)))
//     }
//   }

fn print_put_operations(api_paths: &Vec<ApiPath>) {
  println!("  pub async fn put(&self, path: &str, parameters: &[&str], body: &Box<dyn Any>) -> DshApiResult<()> {{");
  let put_operations: Vec<(&String, &ApiOperation)> = api_paths
    .iter()
    .filter_map(|api_path| api_path.operations.get(&ApiOperationType::Put).map(|api_operation| (&api_path.path, api_operation)))
    .collect::<Vec<_>>();

  // TODO For testing only
  // let put_operations: Vec<(&String, &ApiOperation)> = put_operations
  //   .into_iter()
  //   .filter(|(path, _)| *path == "/allocation/{tenant}/aclgroup/{id}/configuration")
  //   .collect::<Vec<_>>();

  let mut first = true;
  for (api_path, api_operation) in put_operations {
    if first {
      print!("    {}", api_operation.to_put(api_path))
    } else {
      print!(" else {}", api_operation.to_put(api_path));
    }
    first = false;
  }
  println!(" else {{");
  println!("      Err(DshApiError::Configuration(format!(\"get method '{{}}' not recognized\", path)))");
  println!("    }}");
  println!("  }}");
}

fn api_paths(openapi: OpenAPI) -> Result<Vec<ApiPath>, String> {
  let mut api_paths: Vec<ApiPath> = vec![];
  for (path, path_item) in openapi.paths.into_iter() {
    let _path_elements = PathElement::vec_from_str(&path);
    let mut api_path = ApiPath { path: path.clone(), _path_elements, operations: HashMap::new() };
    if let ReferenceOr::Item(item) = path_item {
      // println!("{}\n{:#?}", path, item.parameters);
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
    ReferenceOr::Reference { reference } => ref_to_string(reference.as_ref()),
    ReferenceOr::Item(item) => request_body_to_string(&item),
  });
  let mut _ok_responses: Vec<(u16, ResponseBodyType)> = vec![];
  let mut _error_responses: Vec<(u16, ResponseBodyType)> = vec![];
  for (status_code, response) in operation.responses.responses {
    if let StatusCode::Code(numerical_status_code) = status_code {
      if numerical_status_code >= 200 && numerical_status_code < 300 {
        _ok_responses.push((numerical_status_code, response_to_response_body_type(&response)))
      } else {
        _error_responses.push((numerical_status_code, response_to_response_body_type(&response)))
      }
    }
  }
  let ok_response = _ok_responses.iter().min_by_key(|(status_code, _)| status_code).ok_or("".to_string())?.1.clone();
  Ok(ApiOperation { _operation_type, parameters, request_body, operation_id, ok_response, _ok_responses, _error_responses })

  // let response_string = responses_to_string(&operation.responses);
  // match operation.request_body {
  //   Some(ref request_body) => match request_body {
  //     ReferenceOr::Reference { reference } => {
  //       println!("  {} -> {}", operation_type, response_string);
  //       println!("    {}", operation.operation_id.clone().unwrap_or(">>>>>>>>>>".to_string()));
  //       print_parameters(&operation.parameters);
  //       println!("    body: {}", ref_to_string(reference));
  //     }
  //     ReferenceOr::Item(body) => {
  //       println!("  {} -> {}", operation_type, response_string);
  //       println!("    {}", operation.operation_id.clone().unwrap_or(">>>>>>>>>>".to_string()));
  //       print_parameters(&operation.parameters);
  //       println!("    body: {}", request_body_to_string(body))
  //     }
  //   },
  //   None => {
  //     println!("  {} -> {}", operation_type, response_string);
  //     println!("    {}", operation.operation_id.clone().unwrap_or(">>>>>>>>>>".to_string()));
  //     print_parameters(&operation.parameters);
  //   }
  // }
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

// parameters
// .get(1)
// .unwrap()
// .downcast_ref::<GetManageTenantLimitByManagerByTenantByKindKind>()
// .unwrap()
// .deref()
// .to_owned(),

impl ParameterType {
  fn to_index_parameter(&self, index: isize, name: &str) -> String {
    match self {
      ParameterType::ConstructedTypeOwned(constructed_type) => format!(
        "/* {}: constructed owned */ {}::try_from(parameters.get({}).unwrap().deref())?",
        name, constructed_type, index
      ),
      ParameterType::ConstructedTypeRef(constructed_type) => format!(
        "/* {}: constructed ref */ &{}::try_from(parameters.get({}).unwrap().deref())?",
        name, constructed_type, index
      ),
      ParameterType::SerializableType(serializable_type) => format!(
        "/* {}: serializable */ &{}::try_from(parameters.get({}).unwrap().deref())?",
        name, serializable_type, index
      ),
      ParameterType::RefStr => format!("/* {}: &str */ parameters.get({}).unwrap()", name, index),
    }
  }
}

// schema with pattern -> add &

// GET

//  26 constructed  &GetAclgroupConfigurationByTenantByIdId::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "id",
//             "description": "Kafka ACL group id",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "pattern": "[a-z][a-z0-9-]{1,15}"
//             },
//             "example": "kafka-acl-group-id",
//             "explode": false,
//             "style": "simple"
//           },

// 417 constructed  GetDatacatalogAssetByTenantByKindKind::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "data catalog asset kind",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "bucket",
//                 "writablestream"
//               ]
//             },
//             "style": "simple"
//           },

// 432 constructed  GetDatacatalogAssetByTenantByKindByNameKind::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "data catalog asset kind",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "bucket",
//                 "writablestream"
//               ]
//             },
//             "style": "simple"
//           },

// 448 constructed  GetDatacatalogAssetConfigurationByTenantByKindByNameKind::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "data catalog asset kind",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "bucket",
//                 "writablestream"
//               ]
//             },
//             "style": "simple"
//           },

// 987 constructed  GetManageTenantLimitByManagerByTenantByKindKind::try_from(parameters.get(1).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "Limit request type",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "cpu",
//                 "mem",
//                 "certificatecount",
//                 "secretcount",
//                 "topiccount",
//                 "partitioncount",
//                 "consumerrate",
//                 "producerrate",
//                 "requestrate"
//               ]
//             },
//             "example": "cpu",
//             "explode": false,
//             "style": "simple"
//           },

// PUT

// 1019 constructed &PutAclgroupConfigurationByTenantByIdId::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "id",
//             "description": "Kafka ACL group id",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "pattern": "[a-z][a-z0-9-]{1,15}"
//             },
//             "example": "kafka-acl-group-id",
//             "explode": false,
//             "style": "simple"
//           },

// 1127 constructed PutDatacatalogAssetConfigurationByTenantByKindByNameKind::try_from(parameters.get(0).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "data catalog asset kind",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "bucket",
//                 "writablestream"
//               ]
//             },
//             "style": "simple"
//           },

// 1316 constructed PutManageTenantLimitByManagerByTenantByKindKind::try_from(parameters.get(1).unwrap().deref())?,
//           {
//             "in": "path",
//             "name": "kind",
//             "description": "Limit request type",
//             "required": true,
//             "schema": {
//               "type": "string",
//               "enum": [
//                 "cpu",
//                 "mem",
//                 "certificatecount",
//                 "secretcount",
//                 "topiccount",
//                 "partitioncount",
//                 "consumerrate",
//                 "producerrate",
//                 "requestrate"
//               ]
//             },
//             "example": "cpu",
//             "explode": false,
//             "style": "simple"
//           },

// impl ParameterType {
//   fn to_index_parameter(&self, index: isize) -> String {
//     match self {
//       ParameterType::ConstructedType(constructed_type) => format!(
//         "/* constructed */ parameters.get({}).unwrap().downcast_ref::<{}>().unwrap().to_owned()",
//         index, constructed_type
//       ),
//       ParameterType::SerializableType(serializable_type) => format!(
//         "/* serializable */ parameters.get({}).unwrap().downcast_ref::<&{}>().unwrap().to_owned()",
//         index, serializable_type
//       ),
//       ParameterType::RefStr => format!("/* &str */ parameters.get({}).unwrap().downcast_ref::<&str>().unwrap().to_owned()", index),
//     }
//   }
// }

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

// enum RequestBodyType {
//   String,
//   SerializableType(String),
// }

// impl Display for RequestBodyType {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     match self {
//       Self::String => write!(f, "str"),
//       Self::SerializableType(type_) => write!(f, "{}", type_),
//     }
//   }
// }

#[derive(Clone, Debug)]
enum ResponseBodyType {
  // Error(String),
  Ok(String),
  SerializableType(String),
  String,
}

impl ResponseBodyType {
  fn mapping(&self) -> &str {
    match self {
      // ResponseBodyType::Error(_) => {},
      ResponseBodyType::Ok(_) => ".map(|(_, result)| result)",
      ResponseBodyType::SerializableType(_) => ".map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)",
      ResponseBodyType::String => ".await.map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)",
    }
  }
}

// self
//   .process_string(
//     self
//       .generated_client
//       .get_secret_by_tenant_by_id(self.tenant_name(), parameters.get(0).unwrap(), self.token())
//       .await,
//   )
//   .await
//   .map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)

impl ResponseBodyType {
  fn processing_function(&self) -> &str {
    match self {
      // ResponseBodyType::Error(_) => {},
      ResponseBodyType::Ok(_) => "process",
      ResponseBodyType::SerializableType(_) => "process",
      ResponseBodyType::String => "process_string",
    }
  }
}

impl Display for ResponseBodyType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      // Self::Error(reference) => write!(f, "{}", reference),
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
  request_body: Option<String>,
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

  fn to_get(&self, api_path: &String) -> String {
    let comment1 = format!("GET {}", api_path);
    let comment2 = self
      .parameters
      .iter()
      .map(|(parameter_name, parameter_type)| format!("{}:{}", parameter_name, parameter_type))
      .collect::<Vec<_>>()
      .join(", ");
    let mut parameter_counter = -1;
    let parameters = self
      .parameters
      .iter()
      .enumerate()
      .map(|(_index, (parameter_name, parameter_type))| {
        if parameter_name == "tenant" {
          "self.tenant_name()".to_string()
        } else if parameter_name == "token" {
          "self.token()".to_string()
        } else {
          parameter_counter += 1;
          parameter_type.to_index_parameter(parameter_counter, parameter_name)
        }
      })
      .collect::<Vec<_>>()
      .join(",\n              ");
    formatdoc!(
      r#"
        if path == "{}" {{
              // {}
              // {}
              self
                .{}(
                  self
                    .generated_client
                    .{}(
                      {}
                    )
                    .await
                )
                {}
            }}"#,
      self.operation_id,
      comment1,
      comment2,
      self.ok_response.processing_function(),
      self.operation_id,
      parameters,
      self.ok_response.mapping()
    )
  }

  fn to_put(&self, api_path: &String) -> String {
    let comment1 = format!("PUT {}", api_path);
    let comment2 = self
      .parameters
      .iter()
      .map(|(parameter_name, parameter_type)| format!("{}:{}", parameter_name, parameter_type))
      .collect::<Vec<_>>()
      .join(", ");
    let comment3 = format!("{:?}", self.ok_response);
    let mut parameter_counter = -1;
    let mut parameters = self
      .parameters
      .iter()
      .enumerate()
      .map(|(_index, (parameter_name, parameter_type))| {
        if parameter_name == "tenant" {
          "self.tenant_name()".to_string()
        } else if parameter_name == "token" {
          "self.token()".to_string()
        } else {
          parameter_counter += 1;
          parameter_type.to_index_parameter(parameter_counter, parameter_name)
        }
      })
      .collect::<Vec<_>>();
    if let Some(ref request_body) = self.request_body {
      parameters.push(format!("body.downcast_ref::<&{}>().unwrap()", request_body));
    }
    let parameters = parameters.join(",\n              ");
    formatdoc!(
      r#"
        if path == "{}" {{
              // {}
              // {}
              // {}
              self
                .{}(
                  self
                    .generated_client
                    .{}(
                      {}
                    )
                    .await
                )
                {}
            }}"#,
      self.operation_id,
      comment1,
      comment2,
      comment3,
      self.ok_response.processing_function(),
      self.operation_id,
      parameters,
      self.ok_response.mapping()
    )
  }

  // fn to_put_old(&self, api_path: &String) -> String {
  //   let comment1 = format!("PUT {}", api_path);
  //   let comment2 = self
  //     .parameters
  //     .iter()
  //     .map(|(parameter_name, parameter_type)| format!("{}:{}", parameter_name, parameter_type))
  //     .collect::<Vec<_>>()
  //     .join(", ");
  //   let mut parameter_counter = -1;
  //   let mut parameters: Vec<String> = self
  //     .parameters
  //     .iter()
  //     .enumerate()
  //     .map(|(_index, (parameter_name, parameter_type))| {
  //       if parameter_name == "tenant" {
  //         "              self.tenant_name()".to_string()
  //       } else if parameter_name == "token" {
  //         "              self.token()".to_string()
  //       } else {
  //         parameter_counter += 1;
  //         format!("              {}", parameter_type.to_index_parameter(parameter_counter, parameter_name))
  //       }
  //     })
  //     .collect::<Vec<_>>();
  //   if let Some(ref request_body) = self.request_body {
  //     parameters.push(format!("              body.downcast_ref::<&{}>().unwrap()", request_body));
  //   }
  //   format!(
  //           "if path == \"{}\" {{\n      // {}\n      // {}\n      self\n        .process(\n          self\n            .generated_client\n            .{}(\n{}\n            )\n            .await\n        )\n        .map(|(_, result)| result)\n    }}",
  //           self.operation_id, comment1, comment2, self.operation_id, parameters.join(",\n")
  //       )
  // }
}

// parameters
// .get(1)
// .unwrap()
// .downcast_ref::<GetManageTenantLimitByManagerByTenantByKindKind>()
// .unwrap()
// .deref()
// .to_owned(),

//    if path == "aclgroup_configuration_by_tenant_by_id" {
//      self
//        .process(
//          self
//            .generated_client
//            .get_aclgroup_configuration_by_tenant_by_id(
//              self.tenant_name(),
//              parameters
//                .get(0)
//                .unwrap()
//                .deref()
//                .downcast_ref::<&GetAclgroupConfigurationByTenantByIdId>()
//                .unwrap(),
//              self.token(),
//            )
//            .await,
//        )
//        .map(|(_, result)| Box::new(result) as Box<dyn ErasedSerialize>)
//    } else if path == "application_configuration_by_tenant_by_appid" {

fn type_to_string(type_: &Type) -> String {
  match type_ {
    Type::String(_) => format!("str"),
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
    ReferenceOr::Reference { reference } => ref_to_string(reference),
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
    ReferenceOr::Reference { reference } => ref_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref type_) => type_to_string(type_),
        _ => unimplemented!(),
      }
    }
  }
}

// fn parameter_to_string(parameter: &ReferenceOr<Parameter>) -> String {
//   match parameter {
//     ReferenceOr::Reference { reference } => ref_to_string(reference),
//     ReferenceOr::Item(item) => {
//       let parameter_data = item.clone().parameter_data();
//       match parameter_data.format {
//         ParameterSchemaOrContent::Schema(schema) => {
//           format!("{}: {}", parameter_data.name, schema_to_string(&schema))
//         }
//         ParameterSchemaOrContent::Content(_) => {
//           format!(">>>>>>>>>> {}", parameter_data.name)
//         }
//       }
//     }
//   }
// }

// fn parameter_to_parameter_type(parameter: &ReferenceOr<Parameter>) -> (String, ParameterType) {
//   match parameter {
//     ReferenceOr::Reference { .. } => panic!(),
//     ReferenceOr::Item(item) => {
//       let parameter_data = item.clone().parameter_data();
//       match parameter_data.format {
//         ParameterSchemaOrContent::Schema(schema) => (parameter_data.name, ParameterType::SerializableType(schema_to_string(&schema))),
//         ParameterSchemaOrContent::Content(_) => (parameter_data.name, ParameterType::String),
//       }
//     }
//   }
// }

pub fn capitalize(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  }
}

fn to_type_name(operation_id: &str, name: &str) -> String {
  format!(
    "{}{}",
    operation_id.split('_').map(|part| capitalize(part)).collect::<Vec<_>>().join(""),
    capitalize(name)
  )
}

fn parameter_to_parameter_type(parameter: &ReferenceOr<Parameter>, operation_id: &String) -> (String, ParameterType) {
  match parameter {
    ReferenceOr::Reference { .. } => panic!(),
    ReferenceOr::Item(parameter_item) => {
      let parameter_data = parameter_item.clone().parameter_data();
      match parameter_data.format {
        ParameterSchemaOrContent::Schema(ref schema) => match schema {
          ReferenceOr::Reference { reference } => (parameter_data.name, ParameterType::SerializableType(ref_to_string(reference))),
          ReferenceOr::Item(item) => match &item.schema_kind {
            SchemaKind::Type(type_) => match type_ {
              Type::String(string_type) => {
                let has_pattern = string_type.pattern.is_some();
                let has_enumeration = !string_type.enumeration.is_empty();
                match (has_pattern, has_enumeration) {
                  (false, false) =>
                  // No pattern, no enumeration -> &str
                  {
                    (parameter_data.name, ParameterType::RefStr)
                  }
                  (false, true) =>
                  // No pattern, enumeration -> Owned type
                  {
                    let type_name = to_type_name(operation_id, parameter_data.name.as_str());
                    (parameter_data.name, ParameterType::ConstructedTypeOwned(type_name))
                  }
                  (true, false) =>
                  // Pattern, no enumeration -> Ref type
                  {
                    let type_name = to_type_name(operation_id, parameter_data.name.as_str());
                    (parameter_data.name, ParameterType::ConstructedTypeRef(type_name))
                  }
                  (true, true) =>
                  // Pattern and enumeration -> &str
                  {
                    unimplemented!()
                  }
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
    ReferenceOr::Reference { .. } => panic!(),
    ReferenceOr::Item(response) => match response.content.get("application/json") {
      Some(media_type) => match &media_type.schema {
        Some(schema) => ResponseBodyType::SerializableType(schema_to_string(&schema)),
        None => panic!(),
      },
      None => match response.content.get("text/plain") {
        Some(_) => ResponseBodyType::String,
        None => ResponseBodyType::Ok(response.description.clone()),
      },
    },
  }
}

// fn response_to_string(response: &ReferenceOr<Response>) -> String {
//   match response {
//     ReferenceOr::Reference { reference } => ref_to_string(reference),
//     ReferenceOr::Item(response) => match response.content.get("application/json") {
//       Some(media_type) => match &media_type.schema {
//         Some(schema) => schema_to_string(&schema),
//         None => ">>>>>>>>>> NO_SCHEMA".to_string(),
//       },
//       None => match response.content.get("text/plain") {
//         Some(_) => "String".to_string(),
//         None => response.description.to_string(),
//       },
//     },
//   }
// }

fn ref_to_string(reference: &str) -> String {
  match reference.strip_prefix("#/components/schemas/") {
    Some(type_) => type_.to_string(),
    None => format!("$ref: {}", reference),
  }
}

fn request_body_to_string(request_body: &RequestBody) -> String {
  match request_body.content.get("application/json") {
    Some(media_type) => match &media_type.schema {
      Some(schema) => schema_to_string(&schema),
      None => ">>>>>>>>>> NO_SCHEMA".to_string(),
    },
    None => match request_body.content.get("text/plain") {
      Some(_) => "String".to_string(),
      None => format!(">>>>>>>>>> NO_JSON_NO_TEXT {:?}", request_body),
    },
  }
}
