use anyhow::{Context, Result};
use livekit::webrtc::video_frame::{VideoBuffer, VideoFrame};
// CRITICAL: Use the Mat type from od_opencv to avoid "mismatched types"
use od_opencv::opencv::core::Mat; 
use od_opencv::model_ultralytics::ModelUltralyticsV8;
use std::path::Path;
use std::sync::Arc;
use tracing::info;

use crate::processor;

pub struct InferenceEngine {
    model: ModelUltralyticsV8,
}

impl InferenceEngine {
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let path_str = model_path.as_ref().to_str().context("Invalid model path")?;
        info!("Loading YOLOv8 via od-opencv from: {}", path_str);

        let model = ModelUltralyticsV8::new_from_onnx_file(
            path_str,
            (640, 640),
            0, // Backend ID
            0, // Target ID
            vec![], // Filter classes
        ).map_err(|e| anyhow::anyhow!("Failed to load model: {}", e))?;

        info!("ML Inference Engine (od-opencv) initialized.");
        Ok(Self { model })
    }

    pub async fn process(&mut self, frame: Arc<VideoFrame<Box<dyn VideoBuffer>>>) -> Result<Vec<Detection>> {
        let mat: Mat = processor::process_frame(&frame)?;

        // Now the types will match perfectly!
        let (bboxes, class_ids, confidences) = self.model
            .forward(&mat, 0.4, 0.5)
            .map_err(|e| anyhow::anyhow!("Inference failed: {}", e))?;

        let mut detections = Vec::new();
        for i in 0..bboxes.len() {
            detections.push(Detection {
                class_id: class_ids[i] as usize,
                confidence: confidences[i],
                bbox: [
                    bboxes[i].x as f32,
                    bboxes[i].y as f32,
                    bboxes[i].width as f32,
                    bboxes[i].height as f32,
                ],
            });
        }

        Ok(detections)
    }
}

#[derive(Debug, Clone)]
pub struct Detection {
    pub class_id: usize,
    pub confidence: f32,
    pub bbox: [f32; 4],
}

impl Detection {
    pub fn label(&self) -> &'static str {
        COCO_CLASSES.get(self.class_id).unwrap_or(&"unknown")
    }
}

pub const COCO_CLASSES: [&str; 80] = [
    "person", "bicycle", "car", "motorcycle", "airplane", "bus", "train", "truck", "boat",
    "traffic light", "fire hydrant", "stop sign", "parking meter", "bench", "bird", "cat",
    "dog", "horse", "sheep", "cow", "elephant", "bear", "zebra", "giraffe", "backpack",
    "umbrella", "handbag", "tie", "suitcase", "frisbee", "skis", "snowboard", "sports ball",
    "kite", "baseball bat", "sports glove", "skateboard", "surfboard", "tennis racket",
    "bottle", "wine glass", "cup", "fork", "knife", "spoon", "bowl", "banana", "apple",
    "sandwich", "orange", "broccoli", "carrot", "hot dog", "pizza", "donut", "cake",
    "chair", "couch", "potted plant", "bed", "dining table", "toilet", "tv", "laptop",
    "mouse", "remote", "keyboard", "cell phone", "microwave", "oven", "toaster", "sink",
    "refrigerator", "book", "clock", "vase", "scissors", "teddy bear", "hair drier", "toothbrush",
];
