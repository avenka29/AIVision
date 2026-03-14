# Video Ingestor: The Vision Engine

## Purpose
The `video_ingestor` is a high-performance Rust service responsible for the real-time ingestion, decoding, and analysis of video streams. It acts as the "eyes" of the system.

## Technical Specifications

### 1. Language & Runtime
*   **Language:** Rust (Safety, Concurrency, Performance).
*   **Async Runtime:** `tokio` (Handles high-volume IO and network tasks).

### 2. Media Pipeline
*   **WebRTC Stack:** LiveKit (MVP) / `webrtc-rs` (Long-term)
    *   **MVP:** Use the **LiveKit Rust SDK** to join a room and receive raw `VideoFrame` objects.
    *   **LiveKit Server:** Acts as the SFU (Selective Forwarding Unit), handling signaling, ICE/STUN/TURN, and adaptive bitrate.
    *   **Future:** Potential transition to a custom `webrtc-rs` implementation for zero-dependency edge deployments.
*   **Decoding:** Handled by the LiveKit SDK (internal `libwebrtc` bindings).
*   **Preprocessing:** Frame resizing and normalization (RGB/BGR) for ML input using the `image` or `ndarray` crates.

### 3. ML Inference (Local)
*   **Engine:** `onnxruntime-rs`
    *   Cross-platform hardware acceleration (CUDA, CoreML, DirectML).
*   **Model:** YOLOv8-Nano (Pre-trained on COCO).
    *   Input size: 640x640.
    *   Output: Bounding boxes and confidence scores.

### 4. Logic & Filtering
*   **Tracking:** Basic "SORT" or centroid tracking to assign unique IDs to detected objects.
*   **Event Generation:** 
    *   `TRACK_STARTED`: New unique object appeared.
    *   `TRACK_LOST`: Object left the scene.
    *   `HEARTBEAT`: Periodic updates for active objects.

### 5. Communication (gRPC)
*   **Service Definition:** `vision.proto`
*   **Role:** The Vision Engine acts as a **gRPC Client** (streaming events to the Brain) and a **gRPC Server** (listening for frame-grab requests from Sub-Agents).

## MVP Milestones
1.  **Phase 1:** Basic Rust app with gRPC boilerplate.
2.  **Phase 2:** Integrate `onnxruntime` and run inference on a local file/webcam.
3.  **Phase 3:** Integrate `webrtc-rs` for remote ingestion.
4.  **Phase 4:** Implement object tracking and Event Bridge.
