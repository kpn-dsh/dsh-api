use dsh_api_build_helpers::generate_generic::generate_generic;
use dsh_api_build_helpers::generate_methods::generate_methods;
use dsh_api_build_helpers::generate_types::generate_types;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let out_dir = std::env::var("OUT_DIR").unwrap();

  const OPENAPI_SPEC_FILE_NAME: &str = "openapi_spec/openapi_1_11_1.json";

  println!("cargo:rerun-if-changed={}", OPENAPI_SPEC_FILE_NAME);

  // Make openapi spec available to the crate code
  let openapi_spec_json = read_to_string(OPENAPI_SPEC_FILE_NAME).unwrap();
  let mut embedded_openapi_file_name = std::path::Path::new(&out_dir).to_path_buf();
  embedded_openapi_file_name.push("openapi.json");
  fs::write(embedded_openapi_file_name, &openapi_spec_json).unwrap();

  // Read openapi spec
  let openapi_spec_file = File::open(OPENAPI_SPEC_FILE_NAME).unwrap();
  let mut openapi_spec: OpenAPI = serde_json::from_reader(&openapi_spec_file).unwrap();

  // If feature manage is not enabled, prune all /manage/... paths
  if std::env::var("CARGO_FEATURE_MANAGE").is_err() {
    prune_paths(&mut openapi_spec, |path| path.starts_with("/manage/"));
  }
  // If feature robot is not enabled, prune all /robot/... paths
  if std::env::var("CARGO_FEATURE_ROBOT").is_err() {
    prune_paths(&mut openapi_spec, |path| path.starts_with("/robot/"));
  }

  // Generate types
  let mut types_file_name = std::path::Path::new(&out_dir).to_path_buf();
  types_file_name.push("types.rs");
  let mut types_writer: BufWriter<File> = BufWriter::new(File::create(types_file_name).unwrap());
  generate_types(&mut types_writer, &openapi_spec_json)?;

  // Build methods
  let mut methods_file_name = std::path::Path::new(&out_dir).to_path_buf();
  methods_file_name.push("methods.rs");
  let mut methods_writer: BufWriter<File> = BufWriter::new(File::create(methods_file_name).unwrap());
  generate_methods(&mut methods_writer, &openapi_spec)?;

  // If enabled, create generic client code
  if std::env::var("CARGO_FEATURE_GENERIC").is_ok() {
    let mut generic_file_name = std::path::Path::new(&out_dir).to_path_buf();
    generic_file_name.push("generic.rs");
    let mut generic_writer: BufWriter<File> = BufWriter::new(File::create(generic_file_name).unwrap());
    generate_generic(&mut generic_writer, &openapi_spec)?;
  }

  Ok(())
}

fn prune_paths(openapi: &mut OpenAPI, predicate: fn(&str) -> bool) {
  let paths = openapi.paths.paths.keys().map(|path| path.to_string()).collect::<Vec<_>>();
  for path in paths {
    if predicate(path.as_str()) {
      openapi.paths.paths.shift_remove(path.as_str());
    }
  }
}
