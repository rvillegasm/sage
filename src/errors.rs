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
