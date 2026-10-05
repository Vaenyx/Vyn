use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    process::{Command, ExitStatus},
};

use crate::lib::llvm_to_binary_error;

fn create_temp_file(extension: &str) -> Result<PathBuf, llvm_to_binary_error::TempFileError> {
    let temp_dir = env::temp_dir();
    let temp_file_name = format!("{}.{extension}", uuid::Uuid::new_v4());

    let temp_file_path = temp_dir.join(temp_file_name);

    fs::File::create_new(&temp_file_path).map_err(|source| {
        llvm_to_binary_error::TempFileCreateError {
            path: temp_file_path.clone(),
            source,
        }
    })?;

    Ok(temp_file_path)
}

fn open_temp_file(path: &PathBuf) -> Result<File, llvm_to_binary_error::TempFileError> {
    let file = fs::File::open(path).map_err(|source| llvm_to_binary_error::TempFileOpenError {
        path: path.clone(),
        source,
    })?;

    Ok(file)
}

fn read_temp_file(path: &PathBuf) -> Result<Vec<u8>, llvm_to_binary_error::TempFileError> {
    fs::read(path).map_err(|source| {
        llvm_to_binary_error::TempFileReadError {
            path: path.clone(),
            source,
        }
        .into()
    })
}

fn write_temp_file(path: &PathBuf, input: &str) -> Result<(), llvm_to_binary_error::TempFileError> {
    fs::write(path, input).map_err(|source| {
        llvm_to_binary_error::TempFileWriteError {
            path: path.clone(),
            source,
        }
        .into()
    })
}

fn execute_clang_command(
    ll_path: &PathBuf,
    bin_path: &PathBuf,
) -> Result<(), llvm_to_binary_error::ClangCompilationExecutionError> {
    let output = Command::new("clang")
        .arg(ll_path)
        .arg("-o")
        .arg(bin_path)
        .output()
        .map_err(|source| llvm_to_binary_error::ClangExecutionIoError { source })?;

    if !output.status.success() {
        return Err(llvm_to_binary_error::ClangCompilationFailedError {
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
        .into());
    }

    Ok(())
}

pub fn llvm_to_binary(input: String) -> Result<Vec<u8>, llvm_to_binary_error::LlvmToBinaryError> {
    let tmp_ll_path = create_temp_file("ll")?;
    let tmp_bin_path = create_temp_file("bin")?;

    write_temp_file(&tmp_ll_path, &input)?;

    execute_clang_command(&tmp_ll_path, &tmp_bin_path)?;

    let compiled_content = read_temp_file(&tmp_bin_path)?;

    Ok(compiled_content)
}
