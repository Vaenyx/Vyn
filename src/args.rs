use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Compile Vyn code")]
pub struct Args {
    #[arg(help = "Input file")]
    pub input: PathBuf,

    #[arg(help = "Output file")]
    pub output: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Creates an llvm IR file instead of a binary",
        default_value_t = false
    )]
    pub llvm: bool,
}
