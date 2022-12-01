use std::{fmt::Display, error::Error};
/// Enum used to represent errors originating from `ffmpeg` abstractions.\
/// Can be converted to the general `ConversionError` enum.
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ffmpegError {
    FormatContext(FormatContextError),
    FileError(FileError),
    CodecError(CodecError),
    CodecContextError(CodecContextError),
    ScalerError(ScalerError),
    FrameError(FrameError),
    PacketError(PacketError)
}

impl From<FileError> for ffmpegError {
    fn from(err: FileError) -> Self {
        ffmpegError::FileError(err)
    }
}

impl From<FormatContextError> for ffmpegError {
    fn from(err: FormatContextError) -> Self {
        ffmpegError::FormatContext(err)
    }
}

impl From<CodecError> for ffmpegError {
    fn from(err: CodecError) -> Self {
        ffmpegError::CodecError(err)
    }
}

impl From<CodecContextError> for ffmpegError {
    fn from(err: CodecContextError) -> Self {
        ffmpegError::CodecContextError(err)
    }
}

impl From<ScalerError> for ffmpegError {
    fn from(err: ScalerError) -> Self {
        ffmpegError::ScalerError(err)
    }
}

impl From<FrameError> for ffmpegError {
    fn from(err: FrameError) -> Self {
        ffmpegError::FrameError(err)
    }
}

impl From<PacketError> for ffmpegError {
    fn from(err: PacketError) -> Self {
        ffmpegError::PacketError(err)
    }
}

#[derive(Debug)]
pub enum FormatContextError {
    NoVideoStream
}

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
pub enum CodecError {
    UnsuportedCodec
}

#[derive(Debug)]
pub enum CodecContextError {
    AllocationError,
    CopyError,
    OpenError,
}

#[derive(Debug)]
pub enum ScalerError {
    ContextError
}

#[derive(Debug)]
pub enum FrameError {
    AllocationError,
    CopyError,
    MetadataCopyError,
}

#[derive(Debug)]
pub enum PacketError {
    /// Stores error code from `ffmpeg`
    SendError(i32)
}