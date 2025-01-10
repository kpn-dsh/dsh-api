use openapiv3::{OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr};
use {ApiOperation, PathElement};

pub fn update_openapi(original_openapi_spec: &mut OpenAPI) -> Result<(), String> {
  // Add authorization headers to original openapi spec
  add_authorization_parameters(original_openapi_spec)?;
  // Add operation ids to original openapi spec
  add_operation_ids(original_openapi_spec)?;
  // Add description to original openapi spec
  add_description(original_openapi_spec);
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

const STRING_SCHEMA_JSON: &str = "{ \"schema\": { \"type\": \"string\" } }";

fn add_authorization_parameter(operation: &mut Operation) {
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
  operation.operation_id = Some(ApiOperation::new(method, path_elements).to_string())
}

fn add_description(openapi: &mut OpenAPI) {
  const DESC: &str = "Updated from original version (added authorization parameters and operation ids)";
  if let Some(ref description) = openapi.info.description {
    openapi.info.description = Some(format!("{}\n{}", description, DESC));
  } else {
    openapi.info.description = Some(DESC.to_string());
  }
}
