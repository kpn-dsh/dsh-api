use dsh_api_build_helpers::generate_generic::generate_generic;
use dsh_api_build_helpers::update_openapi::update_openapi;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let original_openapi_spec_file_name = "dsh-api/openapi_spec/openapi_1_10_0.json";
  let original_openapi_spec_file = File::open(original_openapi_spec_file_name).unwrap();
  let mut openapi_spec: OpenAPI = serde_json::from_reader(original_openapi_spec_file).unwrap();
  update_openapi(&mut openapi_spec, true, true)?;
  let mut writer: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
  // let mut writer: BufWriter<std::io::Sink> = BufWriter::new(std::io::sink());
  generate_generic(&mut writer, &openapi_spec)?;
  Ok(())
}
