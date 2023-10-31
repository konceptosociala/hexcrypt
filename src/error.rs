use std::io;
use std::string::FromUtf8Error;

use image::ImageError;
use thiserror::Error;

/// Custom error type for the `hexcrypt` application.
#[derive(Debug, Error)]
pub enum HexCryptError {
    /// Represents an I/O error.
    #[error("I/O error")]
    IoError(#[from] io::Error),

    /// Represents an error related to an invalid image size.
    #[error("Invalid image size `{0}`")]
    InvalidImageSize(String),

    /// Represents an error that occurs when an image cannot be created with the specified size.
    #[error("Cannot create image with size {0}x{1}")]
    CannotCreateImage(u32, u32),

    /// Represents an error related to image processing.
    #[error("Error processing image")]
    ImageError(#[from] ImageError),

    /// Represents an error that occurs when image bytes cannot be converted to a string.
    #[error("Cannot convert image bytes to string")]
    BytesToString(#[from] FromUtf8Error),
}

/// Alias for a `Result` that uses the `HexCryptError` enum as the error type.
pub type HexCryptResult<T> = Result<T, HexCryptError>;