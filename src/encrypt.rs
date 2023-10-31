use std::path::{Path, PathBuf};
use image::RgbImage;

use crate::error::{HexCryptResult, HexCryptError};

/// Encrypts the UTF-8 encoded text from the specified file and converts it into an RGB image.
///
/// # Arguments
///
/// * `path` - A reference to a path that points to the file containing the text to be encrypted.
/// * `size` - An optional `String` representing the custom size of the image (e.g., "16x32").
/// * `out_path` - An optional `PathBuf` representing the output path for the generated image. 
/// If `None`, `path` is used insted (with .png extension)
///
/// # Errors
///
/// This function can return a `HexCryptError` enum that represents different error cases, including I/O errors,
/// invalid image sizes, and issues related to image creation and processing.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use hexcrypt::encrypt;
///
/// let input_path = PathBuf::from("input.txt");
/// let output_path = PathBuf::from("output.png");
/// let size = Some("16x32".to_string());
///
/// match encrypt(input_path, size, Some(output_path)) {
///     Ok(_) => println!("Encryption successful!"),
///     Err(e) => eprintln!("Error: {:?}", e),
/// }
/// ```
pub fn encrypt(
    path: impl AsRef<Path> + Clone,
    size: Option<String>,
    out_path: Option<PathBuf>,
) -> HexCryptResult<()> {
    let text = std::fs::read_to_string(path.clone())?;
    let mut buf = text.as_bytes().to_owned();
    
    let size = match size {
        Some(s) => parse_size(&s)?,
        None => {
            let n = ((buf.len() / 3) as f32).sqrt().ceil() as u32;
            (n, n)
        },
    };

    let diff = (size.0 * size.1) as i32 - (buf.len() / 3) as i32;

    match diff {
        0 => {},
        1.. => {
            for _ in 0..diff {
                buf.extend(&[0, 0, 0]);
            }
        },
        _ => return Err(HexCryptError::CannotCreateImage(size.0, size.1)),
    }

    let image_path = match out_path {
        Some(path) => path.to_str().unwrap().to_owned(),
        None => format!("{}.png", path.as_ref().file_stem().expect("Cannot extract file path").to_str().unwrap()),
    };
    
    let image = RgbImage::from_raw(size.0, size.1, buf).ok_or(HexCryptError::CannotCreateImage(size.0, size.1))?;
    image.save(image_path)?;

    Ok(())
}

/// Parses the custom size string into a tuple of width and height.
///
/// # Arguments
///
/// * `s` - A reference to the custom size string, e.g., "16x32".
///
/// # Errors
///
/// This function can return a `HexCryptError` enum that represents an invalid image size.
///
/// # Examples
///
/// ```
/// use hexcrypt::parse_size;
///
/// match parse_size("16x32") {
///     Ok((width, height)) => println!("Parsed size: {}x{}", width, height),
///     Err(e) => eprintln!("Error: {:?}", e),
/// }
/// ```
fn parse_size(s: &str) -> HexCryptResult<(u32, u32)> {
    let (w, h) = s.split_once('x').ok_or(HexCryptError::InvalidImageSize(s.to_owned()))?;

    let w = w.parse::<u32>().map_err(|_| HexCryptError::InvalidImageSize(s.to_owned()))?;
    let h = h.parse::<u32>().map_err(|_| HexCryptError::InvalidImageSize(s.to_owned()))?;

    Ok((w, h))
}