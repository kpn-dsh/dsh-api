extern crate dsh_api_build;
extern crate openapiv3;

use dsh_api_build::generate_generic::generate_generic;
use openapiv3::OpenAPI;
use std::error::Error;
use std::io::BufWriter;

fn main() -> Result<(), Box<dyn Error>> {
  let openapi_spec_original_file = "dsh-api-build/examples/openapi-updated.json";
  let file = std::fs::File::open(openapi_spec_original_file).unwrap();
  let openapi_spec: OpenAPI = serde_json::from_reader(file).unwrap();
  let mut w: BufWriter<std::io::Stdout> = BufWriter::new(std::io::stdout());
  generate_generic(&mut w, openapi_spec)?;
  Ok(())
}
