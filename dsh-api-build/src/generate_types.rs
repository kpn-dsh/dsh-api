//! Generate types from an openapi string

use schemars::schema::RootSchema;
use serde_json::Value;
use serde_json::Value::Object;
use std::io::Write;
use syn::File;
use typify::{TypeSpace, TypeSpaceSettings};

pub fn generate_types(writer: &mut dyn Write, openapi_spec_json: &str) -> Result<(), String> {
  match serde_json::from_str::<Value>(openapi_spec_json) {
    Ok(Object(openapi)) => match openapi.get("components") {
      Some(Object(components)) => match components.get("schemas") {
        Some(Object(schemas)) => {
          let mut schema_map = serde_json::map::Map::<String, Value>::new();
          schema_map.insert("$defs".to_string(), Object(schemas.clone()));
          let defs = Value::Object(schema_map);
          let schema_string = serde_json::to_string_pretty(&defs).unwrap();
          let root_schema = serde_json::from_str::<RootSchema>(&schema_string).unwrap();
          let mut type_space_settings = TypeSpaceSettings::default();
          type_space_settings.with_struct_builder(true);
          type_space_settings.with_derive("PartialEq".to_string());
          let mut type_space = TypeSpace::new(&type_space_settings);
          type_space.add_root_schema(root_schema).unwrap();
          let types_code = prettyplease::unparse(&syn::parse2::<File>(type_space.to_stream()).unwrap());
          write!(writer, "{}", &types_code).map_err(|error| format!("could not write schema ({})", error))
        }
        _ => Err("openapi does not contain schemas".to_string()),
      },
      _ => Err("openapi does not contain components".to_string()),
    },
    _ => Err("could not parse openapi file".to_string()),
  }
}
