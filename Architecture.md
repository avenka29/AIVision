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

## 2. Technical Stack

| Layer | Component | Technology |
| :--- | :--- | :--- |
| **Ingestion** | WebRTC | `webrtc-rs` (Pure Rust) |
| **Inference** | Local ML | ONNX Runtime + YOLOv8 |
| **Bridge** | Communication | gRPC (Tonic / Protobuf) |
| **Brain** | Agentic Framework | Python (LangChain/CrewAI/Custom) |

---

## 3. Data Flow
1.  **Stream In:** WebRTC video enters the **Vision Engine**.
2.  **Inference:** Every frame is processed by a local **YOLOv8** model (Person, Car, etc.).
3.  **Tracking:** Objects are tracked over time to prevent "event storms."
4.  **Emit Event:** When a new object is identified, a **gRPC** message is sent to the **Agentic Brain**.
5.  **Agent Spawn:** The **Main Agent** evaluates the event and spawns a specialized **Sub-Agent**.
6.  **Loopback:** The Sub-Agent can request specific high-res frames from the Vision Engine for VLM analysis.
