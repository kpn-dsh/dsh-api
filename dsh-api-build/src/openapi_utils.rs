use crate::{Method, RequestBodyType, ResponseBodyType};
use openapiv3::{AdditionalProperties, OpenAPI, Operation, PathItem, ReferenceOr, RequestBody, Response, Schema, SchemaKind, Type};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum OpenApiOperationKind {
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

// Returns all (path, operation) pairs for a given method
pub(crate) fn method_path_operations<'a>(method: &Method, openapi: &'a OpenAPI) -> Vec<(&'a String, &'a Operation)> {
  let mut method_path_items: Vec<(&String, &Operation)> = vec![];
  for (path, path_item) in openapi.paths.iter() {
    if let ReferenceOr::Item(ref path_item) = path_item {
      if let Some(operation) = get_method_operation(method, path_item) {
        method_path_items.push((path, operation))
      }
    }
  }
  method_path_items
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

fn openapi_type_to_string(openapi_type: &Type) -> String {
  match openapi_type {
    Type::String(_) => "String".to_string(),
    Type::Number(_) => unimplemented!(),
    Type::Integer(_) => unimplemented!(),
    Type::Object(object_type) => match object_type.additional_properties {
      Some(ref additional_properties) => match additional_properties {
        AdditionalProperties::Any(_) => unimplemented!(),
        AdditionalProperties::Schema(schema) => format!("HashMap<String, {}>", openapi_schema_to_string(schema)),
      },
      None => unimplemented!(),
    },
    Type::Array(array_type) => match array_type.items {
      Some(ref items) => format!("Vec<{}>", boxed_openapi_schema_to_string(items)),
      None => unimplemented!(),
    },
    Type::Boolean(_) => unimplemented!(),
  }
}

fn openapi_schema_to_string(schema: &ReferenceOr<Schema>) -> String {
  match schema {
    ReferenceOr::Reference { reference } => reference_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref schema_kind_type) => openapi_type_to_string(schema_kind_type),
        SchemaKind::OneOf { .. } => unimplemented!(),
        SchemaKind::AllOf { .. } => unimplemented!(),
        SchemaKind::AnyOf { .. } => unimplemented!(),
        SchemaKind::Not { .. } => unimplemented!(),
        SchemaKind::Any(_) => unimplemented!(),
      }
    }
  }
}

fn boxed_openapi_schema_to_string(parameter: &ReferenceOr<Box<Schema>>) -> String {
  match parameter {
    ReferenceOr::Reference { reference } => reference_to_string(reference),
    ReferenceOr::Item(schema) => {
      let schema_kind = schema.schema_kind.clone();
      match schema_kind {
        SchemaKind::Type(ref schema_kind_type) => openapi_type_to_string(schema_kind_type),
        _ => unimplemented!(),
      }
    }
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

pub(crate) fn reference_to_string(reference: &str) -> String {
  match reference.strip_prefix("#/components/schemas/") {
    Some(reference_type) => reference_type.to_string(),
    None => format!("$ref: {}", reference),
  }
}

impl From<&RequestBody> for RequestBodyType {
  fn from(request_body: &RequestBody) -> Self {
    match request_body.content.get("application/json") {
      Some(media_type) => match &media_type.schema {
        Some(schema) => RequestBodyType::SerializableType(openapi_schema_to_string(schema)),
        None => unimplemented!(),
      },
      None => match request_body.content.get("text/plain") {
        Some(_) => RequestBodyType::String,
        None => unimplemented!(),
      },
    }
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
          AdditionalProperties::Schema(schema) => ResponseBodyType::SerializableMap(openapi_schema_to_string(schema)),
        },
        None => unimplemented!(),
      },
      Type::Array(array_type) => match array_type.items {
        Some(ref items) => ResponseBodyType::SerializableVector(boxed_openapi_schema_to_string(items)),
        None => unimplemented!(),
      },
      Type::Boolean(_) => unimplemented!(),
    }
  }
}
