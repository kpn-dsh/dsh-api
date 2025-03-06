use dsh_api_build_helpers::generate_client::generate_client;
use dsh_api_build_helpers::generate_generic::generate_generic;
use dsh_api_build_helpers::generate_wrapped::generate_wrapped;
use dsh_api_build_helpers::update_openapi::update_openapi;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let out_dir = std::env::var("OUT_DIR").unwrap();

  // Read the openapi specification
  let original_openapi_spec_file_name = "openapi_spec/openapi_1_9_0.json";
  println!("cargo:rerun-if-changed={}", original_openapi_spec_file_name);
  let original_openapi_spec_file = File::open(original_openapi_spec_file_name).unwrap();
  let mut openapi_spec: OpenAPI = serde_json::from_reader(original_openapi_spec_file).unwrap();

  // Update openapi specification, add authorization and operation ids
  let prune_manage = std::env::var("CARGO_FEATURE_MANAGE").is_err();
  let prune_robot = std::env::var("CARGO_FEATURE_ROBOT").is_err();
  update_openapi(&mut openapi_spec, prune_manage, prune_robot)?;

  // Make updated openapi spec available to the crate code
  let updated_openapi_spec_json = serde_json::to_string_pretty(&openapi_spec).unwrap();
  let mut embedded_updated_openapi_file_name = std::path::Path::new(&out_dir).to_path_buf();
  embedded_updated_openapi_file_name.push("openapi.json");
  fs::write(embedded_updated_openapi_file_name, &updated_openapi_spec_json).unwrap();

  // Build Progenitor client code
  let mut generated_progenitor_client_file_name = std::path::Path::new(&out_dir).to_path_buf();
  generated_progenitor_client_file_name.push("progenitor_client.rs");
  let mut writer: BufWriter<File> = BufWriter::new(File::create(generated_progenitor_client_file_name).unwrap());
  generate_client(&mut writer, &updated_openapi_spec_json)?;

  // Create wrapped client code
  let mut wrapped_client_file_name = std::path::Path::new(&out_dir).to_path_buf();
  wrapped_client_file_name.push("wrapped.rs");
  let mut writer: BufWriter<File> = BufWriter::new(File::create(wrapped_client_file_name).unwrap());
  generate_wrapped(&mut writer, &openapi_spec)?;

  // If enabled, create generic client code
  if std::env::var("CARGO_FEATURE_GENERIC").is_ok() {
    let mut generic_client_file_name = std::path::Path::new(&out_dir).to_path_buf();
    generic_client_file_name.push("generic.rs");
    let mut writer: BufWriter<File> = BufWriter::new(File::create(generic_client_file_name).unwrap());
    generate_generic(&mut writer, &openapi_spec)?;
  }

  Ok(())
}
