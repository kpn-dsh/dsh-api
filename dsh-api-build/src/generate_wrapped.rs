//! Generate the generic client code

use crate::dsh_api_operation::{method_api_operations, DshApiOperation, ParameterType};
use crate::openapi_utils::{method_path_operations, OpenApiOperationKind};
use crate::{capitalize, RequestBodyType, ResponseBodyType, MANAGED_PARAMETERS, METHODS};
use indoc::formatdoc;
use openapiv3::{OpenAPI, Operation};
use std::error::Error;
use std::io::Write;

pub fn generate_wrapped(writer: &mut dyn Write, openapi: &OpenAPI) -> Result<(), Box<dyn Error>> {
  let mut wrapped_operations: Vec<DshApiOperation> = vec![];
  for method in &METHODS {
    let path_operations: Vec<(&String, &Operation)> = method_path_operations(method, openapi);
    let method_generic_operations = method_api_operations(method, &path_operations)?;
    wrapped_operations.extend(method_generic_operations);
  }
  wrapped_operations.sort_by(|operation_a, operation_b| operation_a.selector.cmp(&operation_b.selector));
  writeln!(writer, "#[cfg_attr(rustfmt, rustfmt_skip)]")?;
  writeln!(writer, "use crate::dsh_api_client::DshApiClient;")?;
  writeln!(writer, "use crate::types::*;")?;
  writeln!(writer, "use std::collections::HashMap;")?;
  writeln!(writer)?;
  writeln!(writer, "/// # API methods")?;
  writeln!(writer, "///")?;
  writeln!(writer, "/// Module that contains all methods to call the API methods.")?;
  writeln!(writer, "/// These methods are wrappers around the methods generated from the `progenitor` library.")?;
  writeln!(writer, "impl DshApiClient<'_> {{")?;
  let mut first = true;
  for operation in &wrapped_operations {
    if !first {
      writeln!(writer)?;
    }
    write_wrapped_operation(writer, operation)?;
    first = false;
  }
  writeln!(writer, "}}")?;
  Ok(())
}

fn write_wrapped_operation(writer: &mut dyn Write, operation: &DshApiOperation) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  /// # {} {}",
    capitalize(operation.method.to_string()),
    operation.selector.to_lowercase().replace('-', " ")
  )?;
  writeln!(writer, "  ///")?;
  if let Some(ref description) = operation.description {
    writeln!(writer, "  /// {}", description)?;
    writeln!(writer, "  ///")?;
  }
  writeln!(writer, "  /// `{}` `{}`", operation.method.to_string().as_str().to_uppercase(), operation.path)?;
  if !&operation.parameters.is_empty() || operation.request_body.is_some() {
    writeln!(writer, "  ///")?;
    writeln!(writer, "  /// # Parameters")?;
  }
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
    match request_body {
      RequestBodyType::String => writeln!(writer, "  /// * `body` : &str")?,
      RequestBodyType::SerializableType(serializable_type) => writeln!(writer, "  /// * `body` : &[`{}`]", serializable_type)?,
    }
  }
  writeln!(writer, "  ///")?;
  match operation.kind {
    OpenApiOperationKind::Allocation => {}
    OpenApiOperationKind::AppCatalog | OpenApiOperationKind::Manage | OpenApiOperationKind::Robot => {
      writeln!(writer, "  /// _This method is only available when the `{}` feature is enabled._", operation.kind)?
    }
  }
  writeln!(writer, "  {}", wrapped_method(operation))?;
  Ok(())
}

fn wrapped_method(dsh_api_operation: &DshApiOperation) -> String {
  let mut signature_parameters = dsh_api_operation
    .parameters
    .iter()
    .filter_map(
      |(parameter_name, parameter_type, _)| {
        if parameter_name != "Authorization" {
          Some(wrapper_signature_parameter(&parameter_type, parameter_name.to_lowercase().as_str()))
        } else {
          None
        }
      },
    )
    .collect::<Vec<_>>();
  if let Some(ref request_body_type) = dsh_api_operation.request_body {
    match request_body_type {
      RequestBodyType::String => signature_parameters.push("body: String".to_string()),
      RequestBodyType::SerializableType(serializable_type) => signature_parameters.push(format!("body: &{}", serializable_type)),
    }
  }
  let signature_parameters = if signature_parameters.is_empty() { "".to_string() } else { format!(", {}", signature_parameters.join(", ")) };
  let mut call_parameters = dsh_api_operation
    .parameters
    .iter()
    .map(
      |(parameter_name, _, _)| {
        if parameter_name == "Authorization" {
          "self.token().await?.as_str()".to_string()
        } else {
          parameter_name.to_lowercase()
        }
      },
    )
    .collect::<Vec<_>>();
  if dsh_api_operation.request_body.is_some() {
    call_parameters.push("body".to_string());
  }
  let call_parameters = if call_parameters.is_empty() { "".to_string() } else { format!(", {}", call_parameters.join(", ")) };
  let method = &dsh_api_operation.method.to_string();
  let selector = &dsh_api_operation.selector.to_lowercase().replace('-', "_");
  let operation_id = &dsh_api_operation.operation_id;
  let return_type = wrapped_return_value_type(&dsh_api_operation.ok_response);
  let processing_function = dsh_api_operation.ok_response.processing_function();
  let map_childlist = if dsh_api_operation.ok_response == ResponseBodyType::Ids { "\n      .map(|ids| ids.iter().map(|id| id.to_string()).collect())" } else { "" };
  formatdoc!(
    r#"
          pub async fn {method}_{selector}(&self{signature_parameters}) -> DshApiResult<{return_type}> {{
              self
                .{processing_function}(
                  self
                    .generated_client
                    .{operation_id}(self.tenant_name(){call_parameters})
                    .await
                )
                .await
                .map(|(_, result)| result){map_childlist}
            }}"#
  )
}

fn wrapped_return_value_type(response_body_type: &ResponseBodyType) -> String {
  match response_body_type {
    ResponseBodyType::Ids => "Vec<String>".to_string(),
    ResponseBodyType::Ok(_) => "()".to_string(),
    ResponseBodyType::SerializableMap(value_type) => format!("HashMap<String, {}>", value_type),
    ResponseBodyType::SerializableScalar(scalar_type) => scalar_type.to_string(),
    ResponseBodyType::SerializableVector(element_type) => format!("Vec<{}>", element_type),
    ResponseBodyType::String => "String".to_string(),
  }
}

fn wrapper_signature_parameter(parameter_type: &ParameterType, parameter_name: &str) -> String {
  match parameter_type {
    ParameterType::ConstructedTypeOwned(constructed_type) => format!("{}: {}", parameter_name, constructed_type),
    ParameterType::ConstructedTypeRef(constructed_type) => format!("{}: &{}", parameter_name, constructed_type),
    ParameterType::SerializableType(serializable_type) => format!("{}: &{}", parameter_name, serializable_type),
    ParameterType::RefStr => format!("{}: &str", parameter_name),
  }
}
