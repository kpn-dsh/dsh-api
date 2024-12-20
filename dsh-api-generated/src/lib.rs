pub use crate::generated::types;

pub mod generated {
  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}

pub mod display;

pub static OPENAPI_SPEC: &str = include_str!("open-api.json");
