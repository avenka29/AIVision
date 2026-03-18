mod ingestor;
mod processor;
mod inference;

use anyhow::Result;
use ingestor::LiveListener;
use inference::InferenceEngine;
use std::env;
use std::sync::Arc;
use dotenvy::dotenv;
use livekit_api::access_token;
use livekit::webrtc::video_frame::{VideoFrame, VideoBuffer};
use tracing::{info, error};

/// Generates a temporary JWT token for development.
fn get_dev_token(api_key: &str, api_secret: &str, identity: &str, can_publish: bool) -> String {
    access_token::AccessToken::with_api_key(api_key, api_secret)
        .with_identity(identity)
        .with_name(identity)
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: "vision-test".to_string(),
            can_subscribe: true, 
            can_publish, 
            ..Default::default()
        })
        .to_jwt()
        .expect("Failed to generate LiveKit development token")
}

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize Environment & Logging
    dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Starting 'The Sentinel' Vision Engine...");

    // 2. Setup Configuration
    let url = env::var("LIVE_SERVER_URL").unwrap_or_else(|_| "http://localhost:7880".to_string());
    let api_key = env::var("LIVEKIT_API_KEY").unwrap_or_else(|_| "devkey".to_string());
    let api_secret = env::var("LIVEKIT_API_SECRET").unwrap_or_else(|_| "secret".to_string());

    // 3. Initialize the ML Engine (Load the model once!)
    let mut engine = InferenceEngine::new("models/yolov8n.onnx")?;

    // 4. Automatically Generate the JWTs
    let rust_token = get_dev_token(&api_key, &api_secret, "sentinel-01", false);
    let browser_token = get_dev_token(&api_key, &api_secret, "camera-user", true);
    
    println!("\n--- COPY THIS TOKEN FOR YOUR BROWSER ---");
    println!("{}\n", browser_token);
    println!("----------------------------------------\n");

    let listener = LiveListener::new(url, rust_token);

    // 5. Create the Pipe
    let (tx, mut rx) = tokio::sync::mpsc::channel::<Arc<VideoFrame<Box<dyn VideoBuffer>>>>(10);

    // 6. Start the Ingestor (The "Listener" Thread)
    tokio::spawn(async move {
        if let Err(e) = listener.run(tx).await {
            error!("Ingestor error: {}", e);
        }
    });

    info!("Sentinel is now watching room: 'vision-test'");

    // 7. MAIN CONSUMER LOOP: The Brain
    while let Some(frame) = rx.recv().await {
        // Live-Only Drain logic
        let mut newest_frame = frame;
        while let Ok(f) = rx.try_recv() {
            newest_frame = f;
        }

        // --- RUN INFERENCE ---
        match engine.process(newest_frame).await {
            Ok(detections) => {
                for det in detections {
                    info!(">>> ALERT: {} detected! (Confidence: {:.2})", det.label(), det.confidence);
                }
            },
            Err(e) => {
                error!("Inference error: {}", e);
            }
        }
    }

    Ok(())
}
