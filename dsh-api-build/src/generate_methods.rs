//! Generate the generic client code

use crate::dsh_api_operation::{method_api_operations, DshApiOperation};
use crate::openapi_utils::{method_path_operations, OpenApiOperationKind};
use crate::{capitalize, PathElement, RequestBodyType, ResponseBodyType, MANAGED_PARAMETERS, METHODS};
use indoc::{formatdoc, indoc};
use itertools::Itertools;
use openapiv3::{OpenAPI, Operation};
use std::error::Error;
use std::io::Write;

pub fn generate_methods(writer: &mut dyn Write, openapi: &OpenAPI) -> Result<(), Box<dyn Error>> {
  let mut operations: Vec<DshApiOperation> = vec![];
  for method in &METHODS {
    let path_operations: Vec<(&String, &Operation)> = method_path_operations(method, openapi);
    let method_generic_operations = method_api_operations(method, &path_operations)?;
    operations.extend(method_generic_operations);
  }
  operations.sort_by(|operation_a, operation_b| operation_a.selector.cmp(&operation_b.selector));
  writeln!(writer, "#[rustfmt::skip]")?;
  write_imports(writer)?;
  writeln!(writer)?;
  writeln!(writer, "/// # API methods")?;
  writeln!(writer, "///")?;
  writeln!(writer, "/// Module that contains all methods to call the API methods.")?;
  writeln!(writer, "impl DshApiClient {{")?;
  write_api_version_method(writer, openapi)?;
  for operation in &operations {
    writeln!(writer)?;
    write_method(writer, operation)?;
  }
  writeln!(writer)?;
  write_encode_path_method(writer)?;
  writeln!(writer)?;
  write_debug_error_method(writer)?;
  writeln!(writer)?;
  write_debug_error_prefix_method(writer)?;
  writeln!(writer, "}}")?;
  Ok(())
}

fn write_imports(writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "{}",
    indoc!(
      r#"
    use crate::dsh_api_client::DshApiClient;
    use crate::error::DshApiResult;
    use crate::types::*;
    use log::{{debug, trace}};
    use percent_encoding::{{AsciiSet, CONTROLS, PercentEncode, utf8_percent_encode}};
    use reqwest::header::{{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION}};
    use std::collections::HashMap;
    use std::str::FromStr;"#
    )
  )?;
  Ok(())
}

fn write_api_version_method(writer: &mut dyn Write, openapi: &OpenAPI) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  {}",
    formatdoc!(
      r#"
        /// # Returns the version of the openapi spec
          ///
          /// Version number of the openapi file that the crate has been generated from.
          pub fn api_version() -> &'static Version {{
            static API_VERSION: LazyLock<Version> = LazyLock::new(|| Version::from_str("{}").unwrap());
            &API_VERSION
          }}"#,
      openapi.info.version
    ),
  )?;
  Ok(())
}

fn write_encode_path_method(writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  {}",
    indoc!(
      r#"
        #[doc(hidden)]
          fn encode_path(pc: &str) -> PercentEncode {{
            const PATH_SET: &AsciiSet = &CONTROLS
              .add(b' ')
              .add(b'\"')
              .add(b'#')
              .add(b'<')
              .add(b'>')
              .add(b'?')
              .add(b'`')
              .add(b'{')
              .add(b'}')
              .add(b'/')
              .add(b'%');
            utf8_percent_encode(pc, PATH_SET)
          }}"#
    )
  )?;
  Ok(())
}

fn write_debug_error_method(writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  {}",
    indoc!(
      r#"
        #[doc(hidden)]
          fn debug_error<T>(error: T) -> T
          where
            T: Display
          {
            debug!("error: {}", error);
            error
          }"#
    )
  )?;
  Ok(())
}

fn write_debug_error_prefix_method(writer: &mut dyn Write) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  {}",
    indoc!(
      r#"
        #[doc(hidden)]
          fn debug_error_prefix<T>(prefix: &str, error: T) -> T
          where
            T: Display
          {
            debug!("{}: {}", prefix, error);
            error
          }"#
    )
  )?;
  Ok(())
}

fn write_method(writer: &mut dyn Write, operation: &DshApiOperation) -> Result<(), Box<dyn Error>> {
  writeln!(
    writer,
    "  /// # {} {}",
    capitalize(operation.method.to_string()),
    operation.selector.to_lowercase().replace('-', " ")
  )?;
  writeln!(writer, "  ///")?;
  if let Some(ref description) = operation.description {
    writeln!(writer, "  /// {}", description)?;
  }
  if operation.ok_response == ResponseBodyType::Ids {
    writeln!(writer, "  /// The returned list will be sorted alphabetically.")?;
  }
  writeln!(writer, "  ///")?;
  writeln!(writer, "  /// `{}` `{}`", operation.method.to_string().as_str().to_uppercase(), operation.path)?;
  let mut parameters_header_written = false;
  for (parameter_name, parameter_type, parameter_description) in &operation.parameters {
    if !MANAGED_PARAMETERS.contains(&parameter_name.as_str()) {
      if !parameters_header_written {
        writeln!(writer, "  ///")?;
        writeln!(writer, "  /// # Parameters")?;
        parameters_header_written = true;
      }
      if let Some(description) = parameter_description {
        writeln!(writer, "  /// * `{}` - {}", parameter_name, description)?;
      } else {
        writeln!(writer, "  /// * `{}` : `{}`", parameter_name, parameter_type)?;
      }
    }
  }
  if let Some(ref request_body) = operation.request_body {
    if !parameters_header_written {
      writeln!(writer, "  ///")?;
      writeln!(writer, "  /// # Parameters")?;
    }
    match request_body {
      RequestBodyType::String => writeln!(writer, "  /// * `body` : &str")?,
      RequestBodyType::SerializableType(serializable_type) => writeln!(writer, "  /// * `body` : &[`{}`]", serializable_type)?,
    }
  }
  match operation.kind {
    OpenApiOperationKind::Allocation | OpenApiOperationKind::AppCatalog => {}
    OpenApiOperationKind::Manage | OpenApiOperationKind::Robot => {
      writeln!(writer, "  ///")?;
      writeln!(writer, "  /// _This method is only available when the `{}` feature is enabled._", operation.kind)?
    }
  }
  writeln!(writer, "  {}", method(operation))?;
  Ok(())
}

fn method(dsh_api_operation: &DshApiOperation) -> String {
  let signature_parameters = signature_parameters(dsh_api_operation);
  let url_format = url_format(dsh_api_operation);
  let url_parameters = url_parameters(dsh_api_operation);
  let method = &dsh_api_operation.method;
  let method_name = &dsh_api_operation.method_name();
  let return_type = return_value_type(&dsh_api_operation.ok_response);
  let (must_be_sorted, processing_function) = processing_function(dsh_api_operation);
  let (body, add_body) = body(dsh_api_operation);
  let header_map = header_map(dsh_api_operation);
  let sort = if must_be_sorted { "\n    let processed_response = processed_response.map(|mut ids| {{\n      ids.sort();\n      ids\n    }});" } else { "" };

  formatdoc!(
    r#"
      pub async fn {method_name}(&self{signature_parameters}) -> DshApiResult<{return_type}> {{
          let url = format!(
            "{{}}/{url_format}",
            self.platform().rest_api_endpoint(),
            Self::encode_path(self.tenant_name()){url_parameters}
          );{body}
          let bearer_token = self.bearer_token().await.map_err(|error| Self::debug_error_prefix("token error", error))?;{header_map}
          debug!("{method} {{}}", url);
          let mut request_builder = self.client.{method}(url);
          request_builder = request_builder.headers(header_map);{add_body}
          let request = request_builder.build().map_err(|error| Self::debug_error_prefix("request builder error", error))?;
          trace!("{method_name}() -> {{:#?}}", request);
          let response = self.client.execute(request).await.map_err(|error| Self::debug_error_prefix("request execute error", error));
          trace!("{method_name}() -> {{:#?}}", response);
          let processed_response = self.{processing_function}(response).await;{sort}
          trace!("{method_name}() -> {{:#?}}", processed_response);
          processed_response.map_err(Self::debug_error)
        }}"#
  )
}

fn body(dsh_api_operation: &DshApiOperation) -> (&str, &str) {
  if let Some(ref request_body_type) = dsh_api_operation.request_body {
    match request_body_type {
      RequestBodyType::String => ("", "\n    request_builder = request_builder.body(body);"),
      RequestBodyType::SerializableType(_) => (
        "\n    let serialized_body = serde_json::to_string(body)?;",
        "\n    request_builder = request_builder.body(serialized_body);",
      ),
    }
  } else {
    ("", "")
  }
}

fn header_map(dsh_api_operation: &DshApiOperation) -> &str {
  if let Some(ref request_bode_type) = dsh_api_operation.request_body {
    match request_bode_type {
      RequestBodyType::String => "\n    let mut header_map = HeaderMap::with_capacity(2usize);\n    header_map.append(AUTHORIZATION, HeaderValue::try_from(bearer_token)?);\n    header_map.append(CONTENT_TYPE, HeaderValue::from_static(\"text/plain\"));",
      RequestBodyType::SerializableType(_) => "\n    let mut header_map = HeaderMap::with_capacity(2usize);\n    header_map.append(AUTHORIZATION, HeaderValue::try_from(bearer_token)?);\n    header_map.append(CONTENT_TYPE, HeaderValue::from_static(\"application/json\"));"
    }
  } else {
    "\n    let mut header_map = HeaderMap::with_capacity(1usize);\n    header_map.append(AUTHORIZATION, HeaderValue::try_from(bearer_token)?);"
  }
}

fn url_format(dsh_api_operation: &DshApiOperation) -> String {
  dsh_api_operation
    .path_elements
    .iter()
    .map(|path_element| match path_element {
      PathElement::Literal(literal) => literal,
      PathElement::Variable(_) => "{}",
    })
    .join("/")
}

fn url_parameters(dsh_api_operation: &DshApiOperation) -> String {
  let mut path_elements_iter = dsh_api_operation.path_elements.iter();
  path_elements_iter.next().expect("error reading path elements");
  path_elements_iter.next().expect("error reading path elements, expected target");
  let mut url_parameters = path_elements_iter
    .flat_map(|path_element| match path_element {
      PathElement::Literal(_) => None,
      PathElement::Variable(variable) => Some(format!("Self::encode_path({}.as_ref())", variable.to_lowercase())),
    })
    .join(",\n      ");
  if !url_parameters.is_empty() {
    url_parameters = format!(",\n      {}", url_parameters);
  }
  url_parameters
}

fn signature_parameters(dsh_api_operation: &DshApiOperation) -> String {
  let mut signature_parameters = dsh_api_operation
    .parameters
    .iter()
    .filter_map(
      |(parameter_name, _, _)| {
        if parameter_name != "Authorization" {
          Some(format!("{}: impl AsRef<str>", parameter_name.to_lowercase()))
        } else {
          None
        }
      },
    )
    .collect_vec();
  if let Some(ref request_body_type) = dsh_api_operation.request_body {
    match request_body_type {
      RequestBodyType::String => signature_parameters.push("body: String".to_string()),
      RequestBodyType::SerializableType(serializable_type) => signature_parameters.push(format!("body: &{}", serializable_type)),
    }
  }
  let signature_parameters = if signature_parameters.is_empty() { "".to_string() } else { format!(", {}", signature_parameters.join(", ")) };
  signature_parameters
}

fn processing_function(dsh_api_operation: &DshApiOperation) -> (bool, String) {
  match &dsh_api_operation.ok_response {
    ResponseBodyType::Ids => (true, format!("process_{}_deserializable::<Vec<String>>", dsh_api_operation.method)),
    ResponseBodyType::Ok(_) => (false, format!("process_{}", dsh_api_operation.method)),
    ResponseBodyType::SerializableMap(_) => (false, format!("process_{}_deserializable", dsh_api_operation.method)),
    ResponseBodyType::SerializableScalar(_) => (false, format!("process_{}_deserializable", dsh_api_operation.method)),
    ResponseBodyType::SerializableVector(_) => (false, format!("process_{}_deserializable", dsh_api_operation.method)),
    ResponseBodyType::String => (false, format!("process_{}_string", dsh_api_operation.method)),
  }
}

fn return_value_type(response_body_type: &ResponseBodyType) -> String {
  match response_body_type {
    ResponseBodyType::Ids => "Vec<String>".to_string(),
    ResponseBodyType::Ok(_) => "()".to_string(),
    ResponseBodyType::SerializableMap(value_type) => format!("HashMap<String, {}>", value_type),
    ResponseBodyType::SerializableScalar(scalar_type) => scalar_type.to_string(),
    ResponseBodyType::SerializableVector(element_type) => format!("Vec<{}>", element_type),
    ResponseBodyType::String => "String".to_string(),
  }
}
