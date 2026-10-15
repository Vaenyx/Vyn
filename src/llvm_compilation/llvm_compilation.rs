use std::{fs, path::PathBuf, process::Command};

use crate::llvm_compilation::llvm_compilation_error;

fn create_temp_file(extension: &str) -> Result<PathBuf, llvm_compilation_error::TempFileError> {
    let temp_dir = env::temp_dir();
    let temp_file_name = format!("{}.{extension}", uuid::Uuid::new_v4());

    let temp_file_path = temp_dir.join(temp_file_name);

    fs::File::create_new(&temp_file_path).map_err(|source| {
        llvm_compilation_error::TempFileCreateError {
            path: temp_file_path.clone(),
            source,
        }
    })?;

    Ok(temp_file_path)
}

fn read_temp_file(path: &PathBuf) -> Result<Vec<u8>, llvm_compilation_error::TempFileError> {
    fs::read(path).map_err(|source| {
        llvm_compilation_error::TempFileReadError {
            path: path.clone(),
            source,
        }
        .into()
    })
}

fn write_temp_file(
    path: &PathBuf,
    input: &str,
) -> Result<(), llvm_compilation_error::TempFileError> {
    fs::write(path, input).map_err(|source| {
        llvm_compilation_error::TempFileWriteError {
            path: path.clone(),
            source,
        }
        .into()
    })
}

fn delete_temp_file(path: PathBuf) -> Result<(), llvm_compilation_error::TempFileError> {
    fs::remove_file(&path)
        .map_err(|source| llvm_compilation_error::TempFileDeleteError { path, source }.into())
}

fn execute_clang_command(
    ll_path: &PathBuf,
    bin_path: &PathBuf,
) -> Result<(), llvm_compilation_error::ClangCompilationExecutionError> {
    let output = Command::new("clang")
        .arg(ll_path)
        .arg("-o")
        .arg(bin_path)
        .output()
        .map_err(|source| llvm_compilation_error::ClangExecutionIoError { source })?;

    if !output.status.success() {
        return Err(llvm_compilation_error::ClangCompilationFailedError {
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
        .into());
    }

    Ok(())
}

pub fn llvm_to_binary(input: String) -> Result<Vec<u8>, llvm_compilation_error::LlvmToBinaryError> {
    let tmp_ll_path = create_temp_file("ll")?;
    let tmp_bin_path = create_temp_file("bin")?;

    write_temp_file(&tmp_ll_path, &input)?;

    execute_clang_command(&tmp_ll_path, &tmp_bin_path)?;

    delete_temp_file(tmp_ll_path)?;

    let compiled_content = read_temp_file(&tmp_bin_path)?;

    delete_temp_file(tmp_bin_path)?;
    Ok(compiled_content)
}
