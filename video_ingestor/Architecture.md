# Video Ingestor: The Vision Engine

## Purpose
The `video_ingestor` is a high-performance Rust service responsible for the real-time ingestion, decoding, and analysis of video streams. It acts as the "eyes" of the system.

## 1. Internal Task Architecture
The engine is built on an asynchronous, multi-threaded task model using `tokio` to ensure zero-latency processing and isolation.

### A. Ingestor Task (The "Receiver")
*   **Technology:** LiveKit Rust SDK.
*   **Role:** Maintains a single connection to a LiveKit Room.
*   **Multi-Track Support:** Dynamically spawns internal pipelines for every `TrackSubscribed` event (supporting multiple cameras/drones).

### B. Buffer & Context Task (The "Short-term Memory")
*   **Mechanism:** `RingBuffer<Arc<VideoFrame>>`.
*   **Temporal Context:** Maintains a sliding window of the last 10 seconds (adjustable) of raw frames.
*   **Isolation:** If ML inference slows down, the buffer continues to ingest frames at full FPS without blocking the network.

### C. Inference Task (The "Reflexes")
*   **Engine:** `ort` (ONNX Runtime) with YOLOv8.
*   **Inference Chain:**
    *   **Primary:** YOLOv8 (Detects People, Vehicles, etc.).
    *   **Secondary (Future):** Specialized models (Face, Pose) triggered only on specific object crops.
*   **Concurrency:** Utilizes GPU/NPU acceleration via CUDA/TensorRT/CoreML providers.

### D. The gRPC Bridge (The "Nerve")
*   **Event Dispatcher (Push):** 
    *   **Main Agent (Python):** Subscribes to a continuous stream of `ObjectDetected` events.
    *   **Payload:** Includes the latest low-res frame, bounding box, and `TrackID`.
    *   **Goal:** Provides the "Reflex" path for the Main Agent to decide if a Sub-Agent is needed.
*   **Context Streamer (Pull/Query):**
    *   **Sub-Agents (Python):** Spawned by the Main Agent; they "dial back" into the Rust Vision Engine via gRPC.
    *   **Request:** `GetContextFrames(track_id, duration_ms)`.
    *   **Payload:** A high-resolution stream of historical frames from the **Sliding Window Buffer**.
    *   **Goal:** Provides the "Cognitive" path for specialized agents to perform deep temporal analysis (e.g., VLM reasoning, facial verification).

---

## 2. Refined Data Flow Diagram

```text
[ LiveKit Media Server ]
           │
           ▼ (WebRTC / RTP)
    ┌──────────────────────┐
    │    Ingestor Task     │
    └──────────┬───────────┘
               │ (Raw Frames / Arc)
    ┌──────────▼───────────┐      ┌─────────────────────────┐
    │ Sliding Ring Buffer  │ <─── │   gRPC Context Server   │
    └──────────┬───────────┘      └───────────▲─────────────┘
               │ (Latest Frame)               │ (Pull: Context Clips)
    ┌──────────▼───────────┐                  │
    │    Inference Task    │      ┌───────────┴─────────────┐
    │   (YOLOv8 Reflex)    │ ───> │    Agentic Brain (PY)   │
    └──────────────────────┘      └─────────────────────────┘
          (Push: Event Stream)
```


---

## 3. Performance & Scaling Strategies
*   **Zero-Copy Logic:** Uses `Arc<T>` to pass frame pointers between the Buffer and Inference tasks, avoiding expensive memory copies of high-resolution video data.
*   **Deterministic Latency:** By separating the "Ingestor" from the "Inference," we ensure that network jitters don't crash the ML loop, and ML lag doesn't cause WebRTC packet loss.
*   **Horizontal Growth:** To add new layers of analysis (e.g., facial recognition), we simply plug a new `SecondaryInference` task into the existing "Proposal" stream from the Primary YOLO task.

---

## 4. MVP Technical Stack
*   **Runtime:** `tokio` (Async/Await)
*   **WebRTC:** `livekit-rs`
*   **Inference:** `ort` (ONNX Runtime)
*   **Communication:** `tonic` (gRPC / Protobuf)
*   **Data Structures:** `crossbeam` (Lock-free channels)
