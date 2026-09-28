use anyhow::{Context, Result, anyhow};
use std::process::Stdio;
use tokio::process::Command;

pub async fn extract_frame(video_url: &str) -> Result<Vec<u8>> {
    let output = Command::new("ffmpeg")
        .args([
            "-ss",
            "00:00:00.100",
            "-i",
            video_url,
            "-vframes",
            "1",
            "-f",
            "image2pipe",
            "-vcodec",
            "png",
            "pipe:1",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .context("failed to execute ffmpeg process")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("ffmpeg error: '{stderr}'"));
    }

    Ok(output.stdout)
}
