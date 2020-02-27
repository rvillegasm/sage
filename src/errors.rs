use std::error::Error;
use std::fmt;

/// Error for when no versions of the specified package could be found
#[derive(Debug)]
pub struct NoVersionFoundError;

impl Error for NoVersionFoundError {}

impl fmt::Display for NoVersionFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No version of the specified package could be found.")
    }
}

/// Error for when no package version was sepcified by the user
#[derive(Debug)]
pub struct NoVersionSpecifiedError;

impl Error for NoVersionSpecifiedError {}

impl fmt::Display for NoVersionSpecifiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "No version was specified for the package. Please specify one."
        )
    }
}

/// Error for when the specified package could not be found
#[derive(Debug)]
pub struct PackageNotFoundError;

impl Error for PackageNotFoundError {}

impl fmt::Display for PackageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The specified package could not be found.")
    }
}

/// Error for when the file type is not supported
#[derive(Debug)]
pub struct FileTypeNotSupportedError;

impl Error for FileTypeNotSupportedError {}

impl fmt::Display for FileTypeNotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The specified file type is not supported.")
    }
}

/// Error for when the install type is not supported
#[derive(Debug)]
pub struct InstallTypeNotSupportedError;

impl Error for InstallTypeNotSupportedError {}

impl fmt::Display for InstallTypeNotSupportedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The specified install type is not supported.")
    }
}

/// Error for when a decoder was expected but not given
#[derive(Debug)]
pub struct DecoderNotFoundError;

impl Error for DecoderNotFoundError {}

impl fmt::Display for DecoderNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A decoder was Expected, but none was found.")
    }
}

/// Error for when a path was expexted but not given
#[derive(Debug)]
pub struct PathNotFoundError;

impl Error for PathNotFoundError {}

impl fmt::Display for PathNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A path was Expected, but none was found.")
    }
}
