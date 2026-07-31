pub mod error;
pub mod generator;
pub mod json;
pub mod merge;
pub mod typst;

pub use error::{AppError, Result};
pub use generator::{generate_combined_typ, generate_standalone_typ, sanitize_template_name};
pub use json::{escape_typst_string, json_to_typst_literal, json_to_typst_value};
pub use merge::merge_pdfs;
pub use typst::compile_entry_simple;

#[cfg(feature = "native")]
pub use typst::compile_entry;
