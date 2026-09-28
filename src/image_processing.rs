use anyhow::{Context, Result};
use image::ImageFormat;
use std::io::Cursor;

pub fn compress_image(
    image_bytes: &[u8],
    target_width: u32,
    format: ImageFormat,
) -> Result<Vec<u8>> {
    let img = image::load_from_memory(image_bytes).context("failed to decode image from memory")?;

    let resized = img.resize(
        target_width,
        u32::MAX,
        image::imageops::FilterType::Triangle,
    );

    let mut compressed_bytes = Vec::new();
    resized
        .write_to(&mut Cursor::new(&mut compressed_bytes), format)
        .context("failed to encode image")?;

    Ok(compressed_bytes)
}
