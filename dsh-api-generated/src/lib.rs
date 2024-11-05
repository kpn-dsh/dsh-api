pub use crate::generated::types;

pub mod generated {
  include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}
