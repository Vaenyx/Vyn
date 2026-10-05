use std::{io, path::PathBuf};

use crate::lib::LlvmToBinaryError;

#[derive(Debug, thiserror::Error)]
pub enum VynCompilationError {
    #[error(transparent)]
    InputFile(#[from] InputFileError),

    #[error(transparent)]
    OutputFile(#[from] OutputFileError),

    #[error(transparent)]
    LlvmBinary(#[from] LlvmToBinaryError),
}

#[derive(Debug, thiserror::Error)]
pub enum InputFileError {
    #[error(transparent)]
    Open(#[from] InputFileOpenError),

    #[error(transparent)]
    Read(#[from] InputFileReadError),
}

#[derive(Debug, thiserror::Error)]
#[error("Problem opening the input file '{path:?}': {source}")]
pub struct InputFileOpenError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Problem reading the input file '{path:?}': {source}")]
pub struct InputFileReadError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
pub enum OutputFileError {
    #[error(transparent)]
    Create(#[from] OutputFileCreateError),

    #[error(transparent)]
    Write(#[from] OutputFileWriteError),
}

#[derive(Debug, thiserror::Error)]
#[error("Problem creating the output file '{path:?}': {source}")]
pub struct OutputFileCreateError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Problem writing to the output file '{path:?}': {source}")]
pub struct OutputFileWriteError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}
