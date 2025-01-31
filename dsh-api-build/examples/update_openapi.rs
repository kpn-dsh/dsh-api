use dsh_api_build_helpers::update_openapi::update_openapi;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
  let original_openapi_spec_file_name = "dsh-api/openapi_spec/openapi_1_9_0.json";
  let original_openapi_spec_file = File::open(original_openapi_spec_file_name).unwrap();
  let mut openapi_spec: OpenAPI = serde_json::from_reader(original_openapi_spec_file).unwrap();
  update_openapi(&mut openapi_spec)?;
  let updated_openapi_spec_json = serde_json::to_string_pretty(&openapi_spec).unwrap();
  println!("{}", updated_openapi_spec_json);
  Ok(())
}
