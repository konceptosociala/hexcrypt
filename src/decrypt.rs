use std::{path::{Path, PathBuf}, fs::File, io::Write};

use crate::error::HexCryptResult;

/// Decrypts the RGB image from the specified file and converts it back to UTF-8 encoded text.
///
/// # Arguments
///
/// * `path` - A reference to a path that points to the image file to be decrypted.
/// * `out_path` - An optional `PathBuf` representing the output path for the decrypted text.
/// If `None`, `path` is used insted (with .txt extension)
///
/// # Errors
///
/// This function can return a `HexCryptError` enum that represents different error cases, including I/O errors,
/// issues related to image processing, and errors during the conversion of image bytes to text.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use hexcrypt::decrypt;
///
/// let input_path = PathBuf::from("encrypted_image.png");
/// let output_path = PathBuf::from("decrypted.txt");
///
/// match decrypt(input_path, Some(output_path)) {
///     Ok(_) => println!("Decryption successful!"),
///     Err(e) => eprintln!("Error: {:?}", e),
/// }
/// ```
pub fn decrypt(
    path: impl AsRef<Path> + Clone,
    out_path: Option<PathBuf>,
) -> HexCryptResult<()> {
    let img = image::open(path.clone())?.into_rgb8();
    let buf = img.as_raw().to_owned();

    let text_nulled = String::from_utf8(buf)?;
    let text = text_nulled.trim_matches(char::from(0));

    let out_path = match out_path {
        Some(path) => path,
        None => format!("{}.txt", path.as_ref().file_stem().expect("Cannot extract file path").to_str().unwrap()).into(),
    };

    let mut file = File::create(out_path)?;
    file.write_all(text.as_bytes())?;

    Ok(())
}