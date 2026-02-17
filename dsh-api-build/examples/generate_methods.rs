use dsh_api_build_helpers::generate_methods::generate_methods;
use openapiv3::OpenAPI;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let original_openapi_spec_file_name = "dsh-api/openapi_spec/openapi_1_11_1.json";
  // let original_openapi_spec_file_name = "dsh-api-build/examples/openapi_reduced.json";
  let original_openapi_spec_file = File::open(original_openapi_spec_file_name).unwrap();
  let openapi_spec: OpenAPI = serde_json::from_reader(original_openapi_spec_file).unwrap();
  let mut writer: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
  // let mut writer: BufWriter<std::io::Sink> = BufWriter::new(std::io::sink());
  generate_methods(&mut writer, &openapi_spec)?;
  Ok(())
}
