use std::{
    fs::File,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AaptError {
    #[error("Bad path")]
    BadPath,
    #[error("32-bit OS detected with no 32-bit binaries available.")]
    No32BitBinaries,
    #[error("Unknown OS: {0}")]
    UnknownOS(String),
    #[error("aapt not found")]
    NotFound,
    #[error("issue reading or parsing the file")]
    FileError,
    #[error("aapt is not a file")]
    NotAFile,
    #[error("aapt failed to execute")]
    FailedToExecute,
    #[error("aapt version could not be identified")]
    VersionParseFailed,
}

type AaptResult<T> = Result<T, AaptError>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AaptVersion {
    V1,
    V2,
}

pub fn get_aapt1_path() -> AaptResult<PathBuf> {
    get_aapt_path(AaptVersion::V1)
}

pub fn get_aapt2_path() -> AaptResult<PathBuf> {
    get_aapt_path(AaptVersion::V2)
}

fn get_aapt_path(version: AaptVersion) -> AaptResult<PathBuf> {
    let aapt_path: PathBuf;

    let mut aapt_version = String::from("aapt")
        + match version {
            AaptVersion::V1 => "",
            AaptVersion::V2 => "2",
        };

    #[cfg(all(not(target_pointer_width = "64"), target_os = "macos"))]
    return Err(AaptError::No32BitBinaries).into_report();

    if cfg!(target_pointer_width = "64") {
        aapt_version += "_64"
    }

    if cfg!(target_os = "macos") {
        aapt_path = Path::new("/prebuilt/macosx").join(aapt_version);
    } else if cfg!(target_os = "linux") {
        aapt_path = Path::new("/prebuilt/linux").join(aapt_version);
    } else if cfg!(target_os = "windows") {
        aapt_path = Path::new("/prebuilt/windows").join(aapt_version + ".exe");
    } else {
        return Err(AaptError::UnknownOS(std::env::consts::OS.to_string()))
            .into_report()
            .attach_printable_lazy(|| {
                "aapt binaries are not available for this OS. \
                 Please build aapt from source and place it in the \
                 prebuilt directory for your OS."
            });
    }

    Ok(aapt_path)
}

pub fn get_aapt_execution_command(aapt_path: PathBuf) -> AaptResult<String> {
    let aapt_str = aapt_path.to_str().ok_or(AaptError::BadPath)?;
    if !aapt_str.is_empty() {
        let aapt_file = File::open(&aapt_path)
            .into_report()
            .attach_printable_lazy(|| {
                format!("aapt file not found at path: {}", aapt_path.display())
            })
            .change_context(AaptError::FileError)?;
        if aapt_file
            .metadata()
            .into_report()
            .change_context(AaptError::FileError)?
            .is_file()
        {
            Ok(aapt_str.to_string())
        } else {
            Err(AaptError::NotAFile).into_report()
        }
    } else {
        Ok(std::fs::canonicalize(aapt_path)
            .into_report()
            .change_context(AaptError::FileError)?
            .to_str()
            .ok_or(AaptError::BadPath)?
            .to_string())
    }
}

pub fn get_app_version(version: &str) -> AaptResult<i32> {
    if version.starts_with("Android Asset Packaging Tool (aapt) 2:")
        || version.starts_with("Android Asset Packaging Tool (aapt) 2.")
    {
        Ok(2)
    } else if version.starts_with("Android Asset Packaging Tool, v0.") {
        Ok(1)
    } else {
        Err(AaptError::VersionParseFailed).into_report()
    }
}

pub fn get_aapt_version(aapt_path: PathBuf) -> AaptResult<i32> {
    // execute aapt_path then parse version
    let output = String::from_utf8(
        std::process::Command::new(aapt_path)
            .arg("version")
            .output()
            .into_report()
            .change_context(AaptError::FailedToExecute)?
            .stdout,
    )
    .into_report()
    .change_context(AaptError::FailedToExecute)?;

    get_app_version(&output)
}
