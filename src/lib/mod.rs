mod desugarer;
mod generator;
mod parser;
mod tokenizer;
mod validator;

mod llvm_to_binary;
mod llvm_to_binary_error;

pub use desugarer::desugar;
pub use generator::generate;
pub use parser::parse;
pub use tokenizer::tokenize;
pub use validator::validate;

pub use llvm_to_binary::llvm_to_binary;
pub use llvm_to_binary_error::LlvmToBinaryError;
