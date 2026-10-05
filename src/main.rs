use clap::Parser;
use std::path::PathBuf;

use crate::{
    io_helpers::{create_output_file, read_input_file, write_output_file},
    lib::{llvm_to_binary, vyn_to_llvm},
    main_error::VynCompilationError,
};

mod args;
mod io_helpers;
mod lib;
mod main_error;

fn main() -> Result<(), VynCompilationError> {
    let args = args::Args::parse();
    let output = args.output.unwrap_or(PathBuf::from(match args.llvm {
        false => "out.bin",
        true => "out.ll",
    }));

    let input_file_content = read_input_file(&args.input)?;

    let llvm_content = vyn_to_llvm(input_file_content);

    let final_content = if args.llvm {
        llvm_content.into_bytes()
    } else {
        llvm_to_binary(llvm_content)?
    };

    create_output_file(&output)?;
    write_output_file(&output, &final_content)?;

    Ok(())
}
