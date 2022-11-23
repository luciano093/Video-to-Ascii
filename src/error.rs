use std::fmt::Display;

pub enum FileError {
    NoFile,
    InvalidFile,
}

impl Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NoFile => "No such file or directory",
            Self::InvalidFile => "File is not a valid video",
        };

        write!(f, "{}", msg)
    }
}