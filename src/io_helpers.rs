use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

use crate::main_error::{
    InputFileError, InputFileOpenError, InputFileReadError, OutputFileCreateError, OutputFileError,
    OutputFileWriteError, VynCompilationError,
};

fn open_input_file(path: &PathBuf) -> Result<File, VynCompilationError> {
    let file = File::open(path).map_err(|source| {
        InputFileError::Open(InputFileOpenError {
            path: path.clone(),
            source,
        })
    })?;

    Ok(file)
}

pub fn read_input_file(path: &PathBuf) -> Result<String, VynCompilationError> {
    let mut file = open_input_file(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).map_err(|source| {
        InputFileError::Read(InputFileReadError {
            path: path.clone(),
            source,
        })
    })?;

    Ok(buffer)
}

pub fn create_output_file(path: &PathBuf) -> Result<(), VynCompilationError> {
    File::create_new(path).map_err(|source| {
        OutputFileError::Create(OutputFileCreateError {
            path: path.clone(),
            source,
        })
    })?;

    Ok(())
}

pub fn write_output_file(
    path: &PathBuf,
    input: impl AsRef<[u8]>,
) -> Result<(), VynCompilationError> {
    fs::write(path, input).map_err(|source| {
        OutputFileError::Write(OutputFileWriteError {
            path: path.clone(),
            source,
        })
        .into()
    })
}
