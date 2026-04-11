mod errors;
mod functions;
mod generator;
mod structs;
mod utils;

pub use errors::RustGeneratorError;
pub use generator::generate_files;
