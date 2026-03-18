use anyhow::Result;
use futures_util::StreamExt;
use livekit::prelude::*;
use livekit::webrtc::video_frame::{VideoFrame, VideoBuffer};
use livekit::webrtc::video_stream::native::NativeVideoStream;
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error};

/// A stateful video ingestor that manages the connection to LiveKit.
pub struct LiveListener {
    pub url: String,   // LiveKit server URL
    pub token: String, // JWT token for authentication
}

impl LiveListener {
    /// Constructor
    pub fn new(url: String, token: String) -> Self {
        Self { url, token }
    }

    /// Connects to the LiveKit server and begins the ingestion loop.
    pub async fn run(&self, frame_tx: mpsc::Sender<Arc<VideoFrame<Box<dyn VideoBuffer>>>>) -> Result<()> {
        info!("Attempting to connect to LiveKit at: {}", self.url);

        let (room, mut room_events) = Room::connect(
            &self.url,
            &self.token,
            RoomOptions::default(),
        )
        .await?;

        info!("Successfully joined room: {}", room.name());

        while let Some(event) = room_events.recv().await {
            match event {
                RoomEvent::TrackSubscribed { track, publication: _, participant } => {
                    if let RemoteTrack::Video(video_track) = track {
                        info!(
                            "New video track discovered from participant: {}",
                            participant.identity()
                        );

                        let tx = frame_tx.clone();
                        
                        tokio::spawn(async move {
                            // Using NativeVideoStream as per official documentation
                            let mut video_stream = NativeVideoStream::new(video_track.rtc_track());
                            while let Some(frame) = video_stream.next().await {
                                if let Err(_) = tx.send(Arc::new(frame)).await {
                                    error!("Pipeline channel closed, stopping ingestion.");
                                    break;
                                }
                            }
                            info!("Video stream ended for participant: {}", participant.identity());
                        });
                    }
                }
                RoomEvent::Disconnected { reason } => {
                    info!("Disconnected from LiveKit room: {:?}", reason);
                    break;
                }
                _ => {} 
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[test]
    fn test_ingestor_initialization() {
        let url = "http://localhost:7880".to_string();
        let token = "test-token".to_string();
        let ingestor = LiveListener::new(url.clone(), token.clone());
        
        assert_eq!(ingestor.url, url);
        assert_eq!(ingestor.token, token);
    }

    #[tokio::test]
    async fn test_ingestor_connection_failure() {
        let url = "http://invalid-url:7880".to_string();
        let token = "invalid-token".to_string();
        let ingestor = LiveListener::new(url, token);
        
        let (tx, _rx) = mpsc::channel(1);
        let result = ingestor.run(tx).await;
        
        assert!(result.is_err());
    }
}
