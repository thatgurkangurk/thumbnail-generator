use crate::error::AppError;
use crate::ffmpeg;
use crate::image_processing;
use anyhow::Result;
use axum::{
    extract::Query,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThumbnailQuery {
    pub video_url: String,
}

pub async fn generate_thumbnail(video_url: &str) -> Result<Bytes> {
    let raw_frame_bytes = ffmpeg::extract_frame(video_url).await?;
    let compressed_webp =
        image_processing::compress_image(&raw_frame_bytes, 320, image::ImageFormat::WebP)?;
    Ok(Bytes::from(compressed_webp))
}

pub async fn get_thumbnail_handler(
    Query(query): Query<ThumbnailQuery>,
) -> Result<Response, AppError> {
    let thumbnail_bytes = generate_thumbnail(&query.video_url).await?;

    Ok((
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "image/webp"),
            (header::CACHE_CONTROL, "public, max-age=31536000, immutable"),
        ],
        thumbnail_bytes,
    )
        .into_response())
}
