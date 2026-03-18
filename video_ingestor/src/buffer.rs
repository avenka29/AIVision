use std::{collections::VecDeque, sync::Arc};
use livekit::webrtc::video_frame::{VideoFrame, VideoBuffer};

/// Sliding window buffer that keeps track of past frames

pub struct SlidingWindow{
  frames: VecDeque<Arc<VideoFrame<Box<dyn VideoBuffer>>>>,
  capacity: usize

}