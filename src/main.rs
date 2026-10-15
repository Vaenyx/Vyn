use clap::Parser;
use std::path::PathBuf;
use vyn::{
    desugaring::desugar, generating::generate, llvm_compilation::llvm_to_binary, parsing::parse,
    tokenizing::tokenize, validating::validate,
};

use crate::{
    io_helpers::{create_output_file, read_input_file, write_output_file},
    main_error::VynCompilationError,
};

mod args;
mod io_helpers;
mod main_error;

fn main() -> Result<(), VynCompilationError> {
    let args = args::Args::parse();
    let output = args.output.unwrap_or(PathBuf::from(match args.llvm {
        false => "out.bin",
        true => "out.ll",
    }));

    let llvm_content = {
        let input_file_content = read_input_file(&args.input)?;

        let tokens = tokenize(&input_file_content);
        let nodes = parse(&tokens);
        let desugared_nodes = desugar(&nodes);

        validate(&desugared_nodes);

        generate(&desugared_nodes)
    };

    let final_content = if args.llvm {
        llvm_content.into_bytes()
    } else {
        llvm_to_binary(llvm_content)?
    };

    create_output_file(&output)?;
    write_output_file(&output, &final_content)?;

    Ok(())
}
