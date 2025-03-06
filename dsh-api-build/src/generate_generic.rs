//! Generate the generic client code

use crate::dsh_api_operation::{method_api_operations, DshApiOperation, ParameterType};
use crate::openapi_utils::{method_path_operations, OpenApiOperationKind};
use crate::{article, revise, Method, RequestBodyType, ResponseBodyType, MANAGED_PARAMETERS, METHODS};
use indoc::formatdoc;
use openapiv3::{OpenAPI, Operation};
use std::error::Error;
use std::io::Write;

pub fn generate_generic(writer: &mut dyn Write, openapi_spec: &OpenAPI) -> Result<(), Box<dyn Error>> {
  let mut generic_operations: Vec<(Method, Vec<DshApiOperation>)> = vec![];
  for method in &METHODS {
    let path_operations: Vec<(&String, &Operation)> = method_path_operations(method, openapi_spec);
    generic_operations.push((method.to_owned(), method_api_operations(method, &path_operations)?));
  }
  writeln!(writer, "#[cfg_attr(rustfmt, rustfmt_skip)]")?;
  writeln!(writer, "{}", USE)?;
  writeln!(writer)?;
  writeln!(writer, "{}", COMMENT_OUTER)?;
  writeln!(writer, "impl DshApiClient {{")?;
  let mut first = true;
  for (method, operations) in &generic_operations {
    if !first {
      writeln!(writer)?;
    }
    if operations.is_empty() {
      write_empty_method_operations(writer, method)?;
    } else {
      write_method_operations(writer, method, operations)?;
    }
    first = false;
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

fn write_method_operations(writer: &mut dyn Write, method: &Method, operations: &[DshApiOperation]) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "  /// # Generic `{}` operations", method)?;
  writeln!(writer, "  ///")?;
  writeln!(writer, "  /// _This function is only available when the `generic` feature is enabled._")?;
  writeln!(writer, "  ///")?;
  writeln!(writer, "{}", method_comment(method))?;
  writeln!(writer, "  ///")?;
  writeln!(writer, "  /// ## Supported operation selectors for the `{}` method", method)?;
  for operation in operations.iter() {
    writeln!(writer, "  ///")?;
    writeln!(writer, "  /// # __`{}`__", operation.selector)?;
    if let Some(ref description) = operation.description {
      writeln!(writer, "  /// * {}", description)?;
    }
    writeln!(writer, "  /// * `{} {}`", method.to_string().as_str().to_uppercase(), operation.path)?;
    let mut parameter_index = 0;
    for (parameter_name, _, description) in &operation.parameters {
      if !MANAGED_PARAMETERS.contains(&parameter_name.as_str()) {
        if let Some(description) = description {
          writeln!(writer, "  /// * `parameters[{}] = {}` - {}", parameter_index, parameter_name, description)?;
        } else {
          writeln!(writer, "  /// * `parameters[{}] = {}`", parameter_index, parameter_name)?;
        }
        parameter_index += 1;
      }
    }
    if let Some(ref request_body) = operation.request_body {
      match request_body {
        RequestBodyType::String => writeln!(
          writer,
          "  /// * `body` : `Into<String>` yielding a quoted (e.g. valid `json`) string (e.g. `\"ABCDEF\"`)"
        )?,
        RequestBodyType::SerializableType(serializable_type) => writeln!(
          writer,
          "  /// * `body` : `Into<String>` yielding `json` text that deserializes to {} [`{}`]",
          article(serializable_type),
          serializable_type
        )?,
      }
    }
    match &operation.ok_response {
      ResponseBodyType::Ids => writeln!(
        writer,
        "  /// * On success a trait object is returned that will deserialize to a vector of id `String`s."
      )?,
      ResponseBodyType::Ok(_) => writeln!(writer, "  /// * On success `Ok(())` is returned.")?,
      ResponseBodyType::SerializableMap(value_type) => writeln!(
        writer,
        "  /// * On success a trait object is returned that will deserialize to a `HashMap<String, `[`{}`]`>`.",
        value_type
      )?,
      ResponseBodyType::SerializableScalar(scalar_type) => writeln!(
        writer,
        "  /// * On success a trait object is returned that will deserialize to {} [`{}`].",
        article(scalar_type),
        scalar_type
      )?,
      ResponseBodyType::SerializableVector(element_type) => writeln!(
        writer,
        "  /// * On success a trait object is returned that will deserialize to a `Vec<`[`{}`]`>`.",
        element_type
      )?,
      ResponseBodyType::String => writeln!(writer, "  /// * On success a trait object is returned that will deserialize to a `String`.")?,
    }
    match operation.kind {
      OpenApiOperationKind::Allocation => {}
      OpenApiOperationKind::AppCatalog => {}
      OpenApiOperationKind::Manage => writeln!(writer, "  /// * _This selector is only available when the `manage` feature is enabled._")?,
      OpenApiOperationKind::Robot => writeln!(writer, "  /// * _This selector is only available when the `robot` feature is enabled._")?,
    }
  }
  writeln!(writer, "  {} {{", method_signature(method, ""))?;
  let mut first = true;
  for operation in operations.iter() {
    if first {
      write!(writer, "    {}", if_block(operation))?;
    } else {
      write!(writer, " else {}", if_block(operation))?;
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

fn method_signature(method: &Method, prefix: &str) -> String {
  match method {
    Method::Delete => format!(
      "pub async fn delete(&self, {}selector: &str, {}parameters: &[&str]) -> DshApiResult<()>",
      prefix, prefix
    ),
    Method::Get => format!(
      "pub async fn get(&self, {}selector: &str, {}parameters: &[&str]) -> DshApiResult<Box<dyn erased_serde::Serialize>>",
      prefix, prefix
    ),
    Method::Head => format!(
      "pub async fn head(&self, {}selector: &str, {}parameters: &[&str]) -> DshApiResult<()>",
      prefix, prefix
    ),
    Method::Patch => format!(
      "pub async fn patch<T: Into<String>>(&self, {}selector: &str, {}parameters: &[&str], {}body: Option<T>) -> DshApiResult<()>",
      prefix, prefix, prefix
    ),
    Method::Post => format!(
      "pub async fn post<T: Into<String>>(&self, {}selector: &str, {}parameters: &[&str], {}body: Option<T>) -> DshApiResult<()>",
      prefix, prefix, prefix
    ),
    Method::Put => format!(
      "pub async fn put<T: Into<String>>(&self, {}selector: &str, {}parameters: &[&str], {}body: Option<T>) -> DshApiResult<()>",
      prefix, prefix, prefix
    ),
  }
}

fn method_comment(method: &Method) -> &str {
  match method {
    Method::Delete => DELETE_COMMENT,
    Method::Get => GET_COMMENT,
    Method::Head => HEAD_COMMENT,
    Method::Patch => PATCH_COMMENT,
    Method::Post => POST_COMMENT,
    Method::Put => PUT_COMMENT,
  }
}

fn comments(operation: &DshApiOperation) -> Vec<String> {
  let mut comments = vec![];
  comments.push(format!("{} {}", operation.method.to_string().as_str().to_uppercase(), operation.path));
  for (parameter_name, parameter_type, description) in &operation.parameters {
    if !MANAGED_PARAMETERS.contains(&parameter_name.as_str()) {
      match description {
        Some(description) => comments.push(format!("{}:{}, {}", parameter_name, parameter_type, revise(description.to_string()))),
        None => comments.push(format!("{}:{}", parameter_name, parameter_type)),
      }
    }
  }
  if let Some(request_body) = operation.request_body.clone().map(|request_body| request_body.to_string()) {
    comments.push(format!("body: {}", request_body));
  }
  comments.push(generic_doc_return_value(&operation.ok_response).to_string());
  comments
}

fn if_block(operation: &DshApiOperation) -> String {
  let mut parameter_counter = -1;
  let mut parameters = operation
    .parameters
    .iter()
    .map(|(parameter_name, parameter_type, _)| {
      if parameter_name == "Authorization" {
        "self.token().await?.as_str()".to_string()
      } else {
        parameter_counter += 1;
        parameter_type_to_index_parameter(parameter_type, parameter_counter, parameter_name)
      }
    })
    .collect::<Vec<_>>();
  if let Some(ref request_body_type) = operation.request_body {
    match request_body_type {
            RequestBodyType::String => parameters.push(
                "serde_json::from_str::<String>(body.unwrap().into().as_str()).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid String\".to_string()))?.to_string()"
                    .to_string(),
            ),
            RequestBodyType::SerializableType(serializable_type) => parameters.push(format!(
                "&serde_json::from_str::<{}>(body.unwrap().into().as_str()).map_err(|_| DshApiError::Parameter(\"json body could not be parsed as a valid {}\".to_string()))?",
                serializable_type, serializable_type
            )),
        }
  }
  let number_of_expected_parameters = if operation.request_body.is_none() { parameters.len() as i64 - 1 } else { parameters.len() as i64 - 2 };
  let (parameter_length_check, wrong_parameter_length_error) = match number_of_expected_parameters {
    0 => ("!parameters.is_empty()".to_string(), "none expected".to_string()),
    1 => ("parameters.len() != 1".to_string(), "one parameter expected".to_string()),
    _ => (
      format!("parameters.len() != {}", number_of_expected_parameters),
      format!("{} parameters expected", number_of_expected_parameters),
    ),
  };
  let body_check: String = if operation.method.has_body_argument() {
    match operation.request_body {
      Some(ref request_body) => format!(
        r#"}} else if body.is_none() {{
        Err(DshApiError::Parameter("body expected ({})".to_string()))
      "#,
        request_body
      ),
      None => r#"} else if body.is_some() {
        Err(DshApiError::Parameter("no body expected".to_string()))
      "#
      .to_string(),
    }
  } else {
    "".to_string()
  };
  let selector = &operation.selector;
  let path = &operation.path;
  let comments = comments(operation).join("\n      // ");
  let ok_response_processing_function = operation.ok_response.processing_function();
  let operation_id = &operation.operation_id;
  let parameters = parameters.join(",\n                ");
  let ok_response_response_mapping = generic_response_mapping(&operation.ok_response, &operation.method);
  formatdoc!(
    r#"
        if selector == "{selector}" || selector == "{path}" {{
              // {comments}
              if {parameter_length_check} {{
                Err(DshApiError::Parameter("wrong number of parameters ({wrong_parameter_length_error})".to_string()))
              {body_check}}} else {{
                self
                  .{ok_response_processing_function}(
                    self
                      .generated_client
                      .{operation_id}(
                        self.tenant_name(),
                        {parameters},
                      )
                      .await,
                  )
                  {ok_response_response_mapping}
              }}
            }}"#
  )
}

fn write_empty_method_operations(writer: &mut dyn Write, method: &Method) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "  /// # Generic `{}` operations", method)?;
  writeln!(writer, "  ///")?;
  writeln!(writer, "  /// _This function is only available when the `generic` feature is enabled._")?;
  writeln!(writer, "  ///")?;
  writeln!(writer, "  /// ## There are no supported operations for the `{}` method", method)?;
  writeln!(writer, "  {} {{", method_signature(method, "_"))?;
  writeln!(writer, "    Err(DshApiError::Configuration(\"no {} methods available\".to_string()))", method)?;
  writeln!(writer, "  }}")?;
  Ok(())
}

fn write_method_operations_descriptors(writer: &mut dyn Write, method: &Method, operations: &[DshApiOperation]) -> Result<(), Box<dyn Error>> {
  writeln!(writer, "/// `{}` method descriptors", method)?;
  writeln!(writer, "///")?;
  writeln!(writer, "/// _This constant is only available when the `generic` feature is enabled._")?;
  writeln!(writer, "///")?;
  writeln!(writer, "/// Vector that describes all available `{}` methods.", method)?;
  writeln!(writer, "///")?;
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
      let parameters = create_parameters(operation);
      writeln!(writer, "    \"{}\",", operation.selector)?;
      writeln!(writer, "    MethodDescriptor {{")?;
      writeln!(writer, "      path: \"{}\",", operation.path)?;
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
      writeln!(writer, "      response_type: Some(\"{}\")", generic_return_descriptor(&operation.ok_response))?;
      writeln!(writer, "    }}")?;
      writeln!(writer, "  ),")?;
    }
    writeln!(writer, "];")?;
  }

  Ok(())
}

fn generic_response_mapping(response_body_type: &ResponseBodyType, method: &Method) -> &'static str {
  match method {
    Method::Get => match response_body_type {
      ResponseBodyType::Ok(_) => ".await.map(|(_, result)| result)",
      ResponseBodyType::Ids
      | ResponseBodyType::SerializableMap(_)
      | ResponseBodyType::SerializableScalar(_)
      | ResponseBodyType::SerializableVector(_)
      | ResponseBodyType::String => ".await.map(|(_, result)| Box::new(result) as Box<dyn erased_serde::Serialize>)",
    },
    _ => ".await.map(|(_, _)| ())",
  }
}

fn generic_return_descriptor(response_body_type: &ResponseBodyType) -> String {
  match response_body_type {
    ResponseBodyType::Ids => "Vec<String>".to_string(),
    ResponseBodyType::Ok(desc) => desc.to_string(),
    ResponseBodyType::SerializableMap(value_type) => format!("HashMap<String, {}>", value_type),
    ResponseBodyType::SerializableScalar(scalar_type) => scalar_type.to_string(),
    ResponseBodyType::SerializableVector(element_type) => format!("Vec<{}>", element_type),
    ResponseBodyType::String => "String".to_string(),
  }
}

fn generic_doc_return_value(response_body_type: &ResponseBodyType) -> String {
  match response_body_type {
    ResponseBodyType::Ids => "`Vec<String>`".to_string(),
    ResponseBodyType::Ok(desc) => format!("`Ok(())` when {}", desc),
    ResponseBodyType::SerializableMap(value_type) => format!("`HashMap<String, `[`{}`]`>`", value_type),
    ResponseBodyType::SerializableScalar(scalar_type) => format!("[`{}`]", scalar_type),
    ResponseBodyType::SerializableVector(element_type) => format!("`Vec<`[`{}`]`>`", element_type),
    ResponseBodyType::String => "String".to_string(),
  }
}

fn create_parameters(operation: &DshApiOperation) -> Vec<String> {
  operation
    .parameters
    .iter()
    .filter(|(name, _, _)| !MANAGED_PARAMETERS.contains(&name.as_str()))
    .map(|(parameter, parameter_type, description)| {
      format!(
        "(\"{}\", \"{}\", {})",
        parameter,
        parameter_type,
        description.clone().map(|d| format!("Some(\"{}\")", d)).unwrap_or("None".to_string())
      )
    })
    .collect::<Vec<_>>()
}

fn parameter_type_to_index_parameter(parameter_type: &ParameterType, index: isize, name: &str) -> String {
  let get_or_first = if index == 0 { "first()".to_string() } else { format!("get({})", index) };
  match parameter_type {
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

const USE: &str = r#"use crate::dsh_api_client::DshApiClient;
use crate::types::*;
use crate::{DshApiError, DshApiResult};
use std::str::FromStr;"#;

const COMMENT_OUTER: &str = r#"/// # Generic API function calls
///
/// Module that contains methods to call the API methods in a generic way.
/// What this means is that the API functions can be called indirect,
/// where the path of the method must be provided as an argument.
///
/// This has a number of consequences which are caused by the limitations
/// of the `rust` language with respect to abstraction:
/// * The number and types of the required parameters for each method
///   are not known at compile time, which means that (emulated) dynamic typing is used
///   and parameter errors will occur at run-time instead of compile time.
///   * Path parameters must be provided as `&str`.
///   * Body parameters must be provided as a json formatted `String`
///     that can be deserialized at run-time into the expected type.
/// * The response type for each method is not known at compile time.
///   * For `GET` methods the responses will be returned as dynamic trait objects
///     that implement [`erased_serde::Serialize`], defined in the
///     [`erased_serde`](https://crates.io/crates/erased-serde) crate.
///     These objects can be serialized into `json`, `yaml` or `toml` without any type information.
///   * If `DELETE`, `HEAD`, `PATCH`, `POST` and `PUT` methods return data this will be ignored
///     and only errors will be returned.
///
/// # Examples
///
/// Get the configuration of the application `my-application` and print it as json.
///
/// ```ignore
/// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = DshApiClientFactory::default().client().await?;
/// let application = client.get("application_configuration", &["my-application"]).await?;
/// println!("{}", serde_json::to_string_pretty(&application)?);
/// # Ok(())
/// # }
/// ```
///
/// Update the secret `abcdef` to the value `ABCDEF`.
///
/// ```ignore
/// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = DshApiClientFactory::default().client().await?;
///  let secret = serde_json::to_string("ABCDEF")?;
///  client.put("secret", &["abcdef"], &secret).await?;
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

const DELETE_COMMENT: &str = r#"  /// The `delete` function enables the generic calling of all
  /// `DELETE` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The result of this method can only indicate whether the DSH API web service
  ///   has successfully accepted the call or not.
  ///
  /// ## Example
  ///
  /// Delete the secret `my-secret`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// client.delete("secret-configuration", &["my-secret"]).await?;
  /// # Ok(())
  /// # }
  /// ```"#;

const GET_COMMENT: &str = r#"  /// The `get` function enables the generic calling of all
  /// `GET` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// and the type of the response are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The results of this method will be returned as a dynamic trait object
  ///   that implements [`erased_serde::Serialize`].
  ///   This object can be used to serialize the result to json, yaml or toml or
  ///   any other compatible `rust` serialization solution,
  ///   without the need of any type information.
  ///   This will require an (implicit) dependency to the
  ///   [`erased_serde`](https://crates.io/crates/erased-serde) crate.
  ///
  /// ## Example
  ///
  /// Get the configuration of the application `my-service` and print it as json.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// let application = client.get("application-configuration", &["my-service"]).await?;
  /// println!("{}", serde_json::to_string_pretty(&application)?);
  /// # Ok(())
  /// # }
  /// ```"#;

const HEAD_COMMENT: &str = r#"  /// The `head` function enables the generic calling of all
  /// `HEAD` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The result of this method can only indicate whether the DSH API web service
  ///   has successfully accepted the call or not.
  ///
  /// ## Example
  ///
  /// Check whether the tenant `my-tenant` has write access to the topic `my-topic`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// match client.head(
  ///   "manage-stream-internal-streamid-access-write",
  ///   &["my-topic", "my-tenant"]
  /// ).await {
  ///   Ok(()) => println!("tenant has write access"),
  ///   Err(_) => println!("tenant does not have write access"),
  /// }
  /// # Ok(())
  /// # }
  /// ```"#;

const PATCH_COMMENT: &str = r#"  /// The `patch` function enables the generic calling of all
  /// `PATCH` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// and the type of the optional body parameter
  /// are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The body parameter must be provided as a string in the form of an optional `Into<String>`,
  ///   where the string must be deserializable into the expected type. This is checked at runtime.
  /// * The result of this method can only indicate whether the DSH API web service
  ///   has successfully accepted the call or not.
  ///
  /// ## Example
  ///
  /// For tenant `my-tenant`, set the cpu limit to `2.0` and the memory limit to `1000 MiB`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// let limit_values: Vec<LimitValue> =
  ///   vec![
  ///     LimitValue::Cpu(LimitValueCpu { name: LimitValueCpuName::Cpu, value: 2.0 }),
  ///     LimitValue::Mem(LimitValueMem { name: LimitValueMemName::Mem, value: 1000.0 })
  ///   ];
  /// let body = serde_json::to_string(&limit_values)?;
  /// client.patch("manage-tenant-limit", &["my-tenant"], Some(body)).await?;
  /// # Ok(())
  /// # }
  /// ```"#;

const POST_COMMENT: &str = r#"  /// The `post` function enables the generic calling of all
  /// `POST` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// and the type of the optional body parameter
  /// are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The body parameter must be provided as a string in the form of an optional `Into<String>`,
  ///   where the string must be deserializable into the expected type. This is checked at runtime.
  /// * The result of this method can only indicate whether the DSH API web service
  ///   has successfully accepted the call or not.
  ///
  /// ## Example
  ///
  /// Create a new secret `abcdef` with the value `ABCDEF`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// let secret: Secret = Secret {
  ///   name: "abcdef".to_string(),
  ///   value: "ABCDEF".to_string()
  /// };
  /// let body = serde_json::to_string(&secret)?;
  /// client.post("secret", &[], Some(body)).await?;
  /// # Ok(())
  /// # }
  /// ```"#;

const PUT_COMMENT: &str = r#"  /// The `put` function enables the generic calling of all
  /// `PUT` functions of the DSH API, where the specific function is
  /// selected by the `selector` parameter.
  /// By the generic nature of this function the number of parameters and their type
  /// and the type of the optional body parameter
  /// are not known at compile time. This has some consequences:
  /// * The method parameters must be provided as a list of strings in the form of a `&[&str]`.
  ///   Validation of the number of parameters and their type/syntax will be done at run-time.
  /// * The body parameter must be provided as a string in the form of an optional `Into<String>`,
  ///   where the string must be deserializable into the expected type. This is checked at runtime.
  /// * The result of this method can only indicate whether the DSH API web service
  ///   has successfully accepted the call or not.
  ///
  /// ## Example
  ///
  /// Set the existing secret `abcdef` to the value `ABCDEF`.
  ///
  /// ```ignore
  /// # use dsh_api::dsh_api_client_factory::DshApiClientFactory;
  /// # #[tokio::main]
  /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
  /// # let client = DshApiClientFactory::default().client().await?;
  /// let new_secret = "ABCDEF";
  /// let body = serde_json::to_string(new_secret)?;
  /// client.put("secret", &["abcdef"], Some(body)).await?;
  /// # Ok(())
  /// # }
  /// ```"#;

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
  pub path: &'static str,
  pub description: Option<&'static str>,
  pub parameters: &'static[(&'static str, &'static str, Option<&'static str>)],
  pub body_type: Option<&'static str>,
  pub response_type: Option<&'static str>
}"#;
