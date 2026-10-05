use std::{io, path::PathBuf};

#[derive(Debug, thiserror::Error)]
pub enum LlvmToBinaryError {
    #[error(transparent)]
    TempFile(#[from] TempFileError),

    #[error(transparent)]
    ClangCompilationExecution(#[from] ClangCompilationExecutionError),
}

#[derive(Debug, thiserror::Error)]
pub enum TempFileError {
    #[error(transparent)]
    Create(#[from] TempFileCreateError),

    #[error(transparent)]
    Open(#[from] TempFileOpenError),

    #[error(transparent)]
    Read(#[from] TempFileReadError),

    #[error(transparent)]
    Write(#[from] TempFileWriteError),
}

#[derive(Debug, thiserror::Error)]
#[error("Problem creating the temp file '{path:?}': {source}")]
pub struct TempFileCreateError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Problem opening the temp file '{path:?}': {source}")]
pub struct TempFileOpenError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Problem reading the temp file '{path:?}': {source}")]
pub struct TempFileReadError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Problem writing to the temp file '{path:?}': {source}")]
pub struct TempFileWriteError {
    pub path: PathBuf,

    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
pub enum ClangCompilationExecutionError {
    #[error(transparent)]
    Io(#[from] ClangExecutionIoError),

    #[error(transparent)]
    CompilationFailed(#[from] ClangCompilationFailedError),
}

#[derive(Debug, thiserror::Error)]
#[error("Failed while executing the LLVM -> binary compilation: {source}")]
pub struct ClangExecutionIoError {
    #[source]
    pub source: io::Error,
}

#[derive(Debug, thiserror::Error)]
#[error("Clang compilation failed with status {status:#?}: {stderr}")]
pub struct ClangCompilationFailedError {
    pub status: Option<i32>,
    pub stderr: String,
}
