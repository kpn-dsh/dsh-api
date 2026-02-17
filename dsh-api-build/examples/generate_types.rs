use dsh_api_build_helpers::generate_types::generate_types;
use std::error::Error;
use std::fs;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let openapi_spec_file_name = "dsh-api/openapi_spec/openapi_1_11_1.json";
  let openapi_spec_str = fs::read_to_string(openapi_spec_file_name).unwrap();
  let mut writer: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
  // let mut writer: BufWriter<std::io::Sink> = BufWriter::new(std::io::sink());
  generate_types(&mut writer, &openapi_spec_str)?;
  Ok(())
}
