//! Generate Progenitor client from an openapi specification

use progenitor::GenerationSettings;
use std::io::Write;

pub fn generate_client(writer: &mut dyn Write, updated_openapi_spec_json: &str) -> Result<(), String> {
  let updated_openapi_spec = serde_json::from_str(updated_openapi_spec_json).unwrap();
  let mut progenitor_generation_settings = GenerationSettings::default();
  progenitor_generation_settings.with_derive("PartialEq");
  let mut progenitor_generator = progenitor::Generator::new(&progenitor_generation_settings);
  let progenitor_generator_tokens = progenitor_generator.generate_tokens(&updated_openapi_spec).unwrap();
  let progenitor_generator_ast = syn::parse2(progenitor_generator_tokens).unwrap();
  let progenitor_generated_client_code = prettyplease::unparse(&progenitor_generator_ast);
  write!(writer, "{}", &progenitor_generated_client_code).unwrap();
  Ok(())
}
