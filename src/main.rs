//! `hexcrypt` is a CLI application to convert UTF-8 encoded text into RGB images.

#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

use std::path::PathBuf;
use anyhow::Result;
use clap::Parser;
use decrypt::*;
use encrypt::*;

/// The `decrypt` module contains functionality related to decrypting hex-encrypted images to text.
mod decrypt;
/// The `encrypt` module contains functionality related to encrypting text to RGB images.
mod encrypt;
/// The `error` module contains custom error types and error handling functionality.
mod error;

/// The `Args` struct represents the command-line arguments for the `hexcrypt` application.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the text file to be encrypted.
    #[arg(short, long, required = true, conflicts_with = "decrypt")]
    encrypt: Option<PathBuf>,
    /// Path to the image to be decrypted.
    #[arg(short, long, required = true, conflicts_with = "encrypt")]
    decrypt: Option<PathBuf>,
    /// Path to the output file (optional)
    #[arg(short, long)]
    output: Option<PathBuf>,
    /// Whether use custom size of an image. E.g. `-s 16x32`
    #[arg(short, long, conflicts_with = "decrypt")]
    size: Option<String>,
}

/// The main function of the `hexcrypt` application.
fn main() -> Result<()> {
    // Parse command-line arguments.
    let args = Args::parse();
    
    // Encrypt or decrypt image based on the provided arguments.
    if let Some(path) = args.encrypt {
        encrypt(path, args.size, args.output)?;
    } else if let Some(path) = args.decrypt {
        decrypt(path, args.output)?;
    }

    Ok(())
}
