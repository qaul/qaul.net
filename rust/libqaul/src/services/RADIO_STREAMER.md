# RadioStreamer Service

## 1. Vision & Humanitarian Context (V.I.A Project)

This service is a direct implementation of the "Radio: audio stream of local radio" feature envisioned in the original **Vital Information Agency (V.I.A)** proposal (2018). Its mission is to bypass internet censorship or lack of connectivity for end-users by allowing a connected qaul node to fetch a web-radio stream and rebroadcast it across the P2P mesh network.

In crisis zones like Syria (at least till 2024), providing access to trustworthy local audio information (health bulletins, emergency alerts) is a vital resilience factor for internally displaced people (IDPs).

## 2. Technical Architecture

The implementation is designed to be lightweight and resilient, fitting the constraints of older mobile devices.

* **Asynchronous Streaming**: Uses `reqwest` with `rustls-tls` to fetch streams over HTTPS without relying on heavy system-level OpenSSL dependencies.
* **Zero-Copy Chunking**: Incoming data is buffered using `BytesMut`. Chunks of a fixed size (e.g., 32KB) are extracted using `split_to`, which provides high-performance, zero-copy segmentation.
* **Network Resilience**: An automatic reconnection loop with a **3-second delay** ensures that the service recovers from network drops common in unstable environments.
* **Lifecycle Management**: Integrated into qaul's `ServicesModule` using `tokio::task::AbortHandle`, allowing safe and immediate stopping of the streaming task.

## 3. P2P Mesh Integration

Audio segments are injected into the **Feed** service as asynchronous flood messages.

* **Format**: `radio_chunk:<sequence_number>:<base64_data>`.
* **Encoding**: Chunks are Base64-encoded to ensure binary audio data can be safely propagated through the current text-based P2P messaging infrastructure.

## 4. Verification

The module's core logic is verified by unit tests:

* `test_streaming_chunk_logic`: Validates exact chunk sizing and sequence.
* `test_connection_retry`: Confirms the 3-second retry behavior after a connection failure.
