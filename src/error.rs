use std::{fmt::Display, error::Error};

#[derive(Debug)]
pub enum FileError {
    NoFile,
    InvalidFile,
    /// Error where the file exists but isn't able to be opened.\
    /// Contains a `String` indicating the name of the file or more information about the error.
    UnableToOpenFile(String)
}

impl Error for FileError { }

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NoFile => "No such file or directory",
            Self::InvalidFile => "File is not a valid video",
            Self::UnableToOpenFile(msg) => return write!(f, "Unable to open file: {}", msg),
        };

        write!(f, "{}", msg)
    }
}

#[derive(Debug)]
pub enum CodecContextError {
    AllocationError,
    CopyError,
    OpenError,
}

pub enum ConversionError {
    #[allow(non_camel_case_types)]
    ffmpegError
}