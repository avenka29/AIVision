# AI Vision Architecture: "The Sentinel"

## System Overview
"The Sentinel" is a hybrid AI vision system designed for real-time video analysis and agentic response. It splits high-performance "reflex" processing from complex "strategic" reasoning.

### 1. High-Level Architecture
The system consists of two primary layers connected by a high-speed gRPC bridge:

*   **The Vision Engine (Rust):**
    *   Handles the "Fast Path" (high FPS, low latency).
    *   Manages WebRTC ingestion, hardware-accelerated decoding, and local ML inference.
    *   Filters raw video into high-signal "Events."

*   **The Agentic Brain (Python):**
    *   Handles the "Slow Path" (complex logic, LLMs).
    *   Maintains a "Main Agent" that subscribes to Vision Events.
    *   Spawns specialized "Sub-Agents" for deeper analysis or actions (e.g., Security Agent, Analytics Agent).

---

## 2. Technical Stack & Core Components

### A. The Connection Manager
*   **Registry:** Manages active WebRTC/LiveKit sessions.
*   **Scalability:** Designed to handle `N` streams (MVP limited to 1).
*   **Health Check:** Monitors stream vitals (bitrate, frame-drop) and restarts ingestors on failure.

### B. The Sliding Window Buffer (Temporal Context)
*   **Mechanism:** A high-performance Ring Buffer (in-memory) storing the last `X` seconds of raw frames.
*   **Purpose:** Provides "Pre-Event Context" to Agents. When a detection occurs at `T`, the Agent can request frames from `T - 5s` to `T` to analyze intent/behavior.
*   **Memory Management:** Strictly bounded to prevent OOM (Out of Memory) errors.

### C. The ML Pipeline (Multi-Layered)
*   **Layer 1 (Reflex):** YOLOv8 runs on every frame for generic object detection.
*   **Layer 2 (Cognitive):** Specialized models (Face, Pose, LPR) run only on "Proposals" from Layer 1.
*   **Inference Queue:** A "Latest-Only" queue to ensure zero-latency for live analysis.

---

## 3. System Bootstrap Sequence
1.  **Service Init:** Initialize gRPC/Messaging bus and Buffer services.
2.  **ML Warmup:** Load ONNX models into GPU/NPU memory and run a dummy inference pass.
3.  **Signaling Start:** Open the WebRTC/LiveKit signaling bridge to accept incoming connections.
4.  **Monitoring:** Start the health-check loop for stream stability.

---

## 3. Data Flow
1.  **Stream In:** WebRTC video enters the **Vision Engine**.
2.  **Inference:** Every frame is processed by a local **YOLOv8** model (Person, Car, etc.).
3.  **Tracking:** Objects are tracked over time to prevent "event storms."
4.  **Emit Event:** When a new object is identified, a **gRPC** message is sent to the **Agentic Brain**.
5.  **Agent Spawn:** The **Main Agent** evaluates the event and spawns a specialized **Sub-Agent**.
6.  **Loopback:** The Sub-Agent can request specific high-res frames from the Vision Engine for VLM analysis.
