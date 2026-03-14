# AI Vision Implementation Plan: "The Sentinel"

This roadmap outlines the step-by-step development of a high-performance vision system with a Rust-based ingestor and a Python-based agentic layer.

## Phase 1: Local Vision Engine (The "Eyes")
*Goal: Successfully process a video stream locally using YOLOv8 and Rust.*

1.  **Environment Setup:**
    *   Initialize Rust project in `video_ingestor/`.
    *   Download pre-trained YOLOv8-Nano ONNX model.
2.  **Inference Pipeline:**
    *   Implement `onnxruntime-rs` loader.
    *   Write preprocessing logic (resize, normalize, RGB/BGR conversion).
    *   Perform inference on static images/local video files.
3.  **Basic Tracking:**
    *   Implement simple centroid tracking to avoid duplicate events for the same object.

## Phase 2: The Control Plane (The "Nerve")
*Goal: Establish a gRPC bridge between Rust and Python.*

1.  **Protocol Definition:**
    *   Create `vision.proto` defining `ObjectDetectionEvent` and `FrameRequest`.
2.  **Rust gRPC Client:**
    *   Implement the client in `video_ingestor/` to stream detection events.
3.  **Python Mock Brain:**
    *   Create a simple Python gRPC server to receive and log events from the Rust engine.

## Phase 3: Real-time Ingestion (The "Network")
*Goal: Replace local files with a live WebRTC stream.*

1.  **LiveKit Setup:**
    *   Run a local LiveKit server (Docker).
    *   Create a simple browser-based publisher to stream webcam feed to LiveKit.
2.  **Rust LiveKit Client:**
    *   Integrate `livekit-rs` into the Vision Engine.
    *   Connect to the LiveKit room and pipe `VideoFrame` objects into the ML loop.

## Phase 4: Agentic Integration (The "Brain")
*Goal: Spawn agents based on vision events.*

1.  **Main Agent Implementation:**
    *   Build the Python "Main Agent" using an agentic framework (e.g., CrewAI, PydanticAI, or custom).
    *   Implement logic to evaluate incoming vision events.
2.  **Sub-Agent Spawning:**
    *   Implement the dynamic spawning of specialized agents (e.g., "Security Audit Agent").
3.  **Frame Grab Loopback:**
    *   Implement the "Frame Request" gRPC call: Sub-Agent requests high-res frames for VLM (Gemini/GPT-4o) analysis.

## Phase 5: Refinement & Validation
1.  **Performance Optimization:**
    *   Profile GPU utilization and frame-drop rates.
2.  **End-to-End Testing:**
    *   Simulate security incidents and verify agent responses.
3.  **Deployment Prep:**
    *   Containerize both services using Docker Compose.
