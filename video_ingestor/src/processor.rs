use anyhow::{Context, Result};
use livekit::webrtc::video_frame::{VideoBuffer, VideoFrame};
// Use the re-exported opencv from od_opencv 0.2
pub use od_opencv::opencv::{core, imgproc};

/// Processes a raw LiveKit VideoFrame into an OpenCV Mat.
pub fn process_frame(frame: &VideoFrame<Box<dyn VideoBuffer>>) -> Result<core::Mat> {
    let width = frame.buffer.width() as i32;
    let height = frame.buffer.height() as i32;

    let i420 = frame
        .buffer
        .as_i420()
        .context("Failed to convert frame buffer to I420 format")?;

    // Fix: In the latest libwebrtc-sys, the getter is simply data()
    let y_data = i420.data();
    
    // Create OpenCV Mat from Y-plane (first width*height bytes of I420)
    let y_mat = core::Mat::new_rows_cols_with_data(
        height,
        width,
        &y_data[0..(width*height) as usize],
    ).map_err(|e| anyhow::anyhow!("Failed to create OpenCV Mat: {}", e))?;

    let mut rgb_mat = core::Mat::default();
    imgproc::cvt_color(&y_mat, &mut rgb_mat, imgproc::COLOR_GRAY2RGB, 0)
        .context("Failed to convert YUV to RGB")?;

    let mut resized = core::Mat::default();
    imgproc::resize(
        &rgb_mat,
        &mut resized,
        core::Size::new(640, 640),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .context("Failed to resize frame")?;

    Ok(resized)
}
