use gp::GpError;
use ndarray_linalg::error::LinalgError;
use std::error::Error;
use std::fmt::{self, Display};

pub type Result<T> = std::result::Result<T, MoeError>;

/// An error when modeling a GMM algorithm
#[derive(Debug)]
pub enum MoeError {
    /// When linear algebra computation fails
    LinalgError(String),
    /// When clustering fails
    EmptyCluster(String),
    // /// When PLS fails
    // PlsError(String),
    /// When Gaussian Process fails
    GpError(String),
}

impl Display for MoeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LinalgError(message) => write!(f, "Linear Algebra error: {}", message),
            // Self::PlsError(message) => write!(f, "PLS error: {}", message),
            Self::EmptyCluster(message) => write!(f, "Empty cluster: {}", message),
            Self::GpError(message) => {
                write!(f, "Gaussian process computation error: {}", message)
            }
        }
    }
}

impl Error for MoeError {}

impl From<LinalgError> for MoeError {
    fn from(error: LinalgError) -> MoeError {
        MoeError::LinalgError(error.to_string())
    }
}

impl From<GpError> for MoeError {
    fn from(error: GpError) -> MoeError {
        MoeError::GpError(error.to_string())
    }
}
