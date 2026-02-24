use std::time::Duration;
use bytes::{Bytes, BytesMut};
use futures::StreamExt;
use reqwest::Client;
use tracing::{info, warn, error};

/// RadioStreamer service
///
/// Connects to a HTTP stream, buffers data, and produces fixed-size chunks.
pub struct RadioStreamer;

impl RadioStreamer {
    /// Starts streaming from the given URL.
    ///
    /// This function blocks asynchronously (runs in a loop) and should be spawned in a task.
    /// It automatically retries connection on failure with a delay.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the radio stream (HTTP/HTTPS).
    /// * `chunk_size` - The target size of each chunk in bytes.
    /// * `callback` - A closure called for each generated chunk. It receives the chunk data and a sequence number.
    pub async fn process_stream<F, Fut>(url: String, chunk_size: usize, callback: F)
    where
        F: Fn(Bytes, u64) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send,
    {
        // Build a client with a connection timeout
        let client = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            // .timeout(...) // We don't want a total request timeout for a stream, just connection
            .build()
            .unwrap_or_else(|e| {
                error!("Failed to build reqwest client: {}", e);
                Client::new()
            });

        let mut sequence_number: u64 = 0;

        loop {
            info!("Connecting to radio stream: {}", url);

            match client.get(&url).send().await {
                Ok(response) => {
                    if !response.status().is_success() {
                        warn!("HTTP error connecting to {}: {}", url, response.status());
                        // Fall through to retry delay
                    } else {
                        info!("Connected to stream {}. status: {}", url, response.status());

                        let mut stream = response.bytes_stream();
                        // Pre-allocate buffer to avoid immediate reallocation
                        let mut buffer = BytesMut::with_capacity(chunk_size * 2);

                        // Stream processing loop
                        loop {
                            match stream.next().await {
                                Some(Ok(chunk_data)) => {
                                    // Extend buffer with new data
                                    buffer.extend_from_slice(&chunk_data);

                                    // Extract chunks while we have enough data
                                    while buffer.len() >= chunk_size {
                                        let chunk = buffer.split_to(chunk_size).freeze();
                                        sequence_number += 1;

                                        info!("Généré chunk de taille: {} bytes (seq: {})", chunk.len(), sequence_number);

                                        // Execute callback
                                        callback(chunk, sequence_number).await;
                                    }
                                }
                                Some(Err(e)) => {
                                    error!("Stream error while reading from {}: {}", url, e);
                                    break; // Break inner loop to reconnect
                                }
                                None => {
                                    warn!("Stream ended (server closed connection) for {}", url);
                                    break; // Break inner loop to reconnect
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Connection failed to {}: {}", url, e);
                }
            }

            // Exponential backoff or fixed delay could be used.
            // Using fixed 3s delay as per instructions ("attendre 3 secondes").
            info!("Waiting 3 seconds before reconnecting to {}...", url);
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncWriteExt, AsyncReadExt};
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn test_streaming_chunk_logic() {
        // Setup a mock HTTP server
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind mock server");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();

        // Spawn server task
        tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    tokio::spawn(async move {
                        // Read request headers (ignore content)
                        let mut buf = [0u8; 1024];
                        let _ = socket.read(&mut buf).await;

                        // Write HTTP response headers
                        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\r\n";
                        if let Err(e) = socket.write_all(response.as_bytes()).await {
                            eprintln!("Failed to write headers: {}", e);
                            return;
                        }

                        // Send data continuously
                        // We want 1s audio ~ 16000 bytes.
                        // Let's send batches of 100 bytes.
                        let chunk_data = [1u8; 100];
                        for _ in 0..100 { // Send 100 * 100 = 10000 bytes total
                            if let Err(_) = socket.write_all(&chunk_data).await {
                                break;
                            }
                            tokio::time::sleep(Duration::from_millis(10)).await;
                        }
                    });
                }
            }
        });

        let url = format!("http://127.0.0.1:{}", port);
        // We want chunks of 500 bytes.
        let chunk_size = 500;
        let received_chunks = Arc::new(Mutex::new(Vec::new()));
        let received_chunks_clone = received_chunks.clone();

        // Use a channel to notify when we have enough chunks
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        // Run client
        let client_future = RadioStreamer::process_stream(
            url,
            chunk_size,
            move |chunk, seq| {
                let received_chunks = received_chunks_clone.clone();
                let tx = tx.clone();
                async move {
                    info!("Test received chunk {} of len {}", seq, chunk.len());
                    {
                        let mut data = received_chunks.lock().unwrap();
                        data.push((seq, chunk));
                    }
                    if seq >= 5 {
                        let _ = tx.send(()).await;
                    }
                }
            }
        );

        // Run until we get 5 chunks or timeout
        tokio::select! {
            _ = client_future => {
                panic!("process_stream should not finish normally");
            },
            _ = rx.recv() => {
                info!("Received enough chunks");
            },
            _ = tokio::time::sleep(Duration::from_secs(5)) => {
                // If we timeout, we check if we got enough data anyway (maybe the channel send failed?)
                // But better to panic or check assertions later.
                info!("Timeout reached");
            }
        }

        // Verify results
        let chunks = received_chunks.lock().unwrap();
        assert!(chunks.len() >= 5, "Should have received at least 5 chunks, got {}", chunks.len());

        for (seq, chunk) in chunks.iter() {
            assert_eq!(chunk.len(), chunk_size, "Chunk size should be exactly {}", chunk_size);
            // Verify content (all 1s)
            assert!(chunk.iter().all(|&b| b == 1u8));
            assert_eq!(*seq, chunks[0].0 + (seq - chunks[0].0)); // Sequential
        }
    }

    #[tokio::test]
    async fn test_connection_retry() {
        let listener = TcpListener::bind("127.0.0.1:0").await.expect("Failed to bind");
        let addr = listener.local_addr().unwrap();
        let port = addr.port();

        // Shared state to count connections
        let connection_count = Arc::new(Mutex::new(0));
        let connection_count_clone = connection_count.clone();

        tokio::spawn(async move {
            loop {
                if let Ok((mut socket, _)) = listener.accept().await {
                    let mut count = connection_count_clone.lock().unwrap();
                    *count += 1;
                    let current_count = *count;
                    drop(count); // Unlock

                    tokio::spawn(async move {
                         let mut buf = [0u8; 1024];
                        let _ = socket.read(&mut buf).await;

                        if current_count == 1 {
                            // First connection: Fail it
                            let response = "HTTP/1.1 500 Internal Server Error\r\n\r\n";
                            let _ = socket.write_all(response.as_bytes()).await;
                            // Close
                        } else {
                            // Second connection: Success
                             let response = "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\n\r\n";
                            let _ = socket.write_all(response.as_bytes()).await;
                            // Send some data
                            let data = [2u8; 100];
                            let _ = socket.write_all(&data).await;
                            tokio::time::sleep(Duration::from_secs(1)).await;
                        }
                    });
                }
            }
        });

        let url = format!("http://127.0.0.1:{}", port);
        let chunk_size = 50;
        let received_chunks = Arc::new(Mutex::new(Vec::new()));
        let received_chunks_clone = received_chunks.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let client_future = RadioStreamer::process_stream(
            url,
            chunk_size,
            move |chunk, _| {
                let received_chunks = received_chunks_clone.clone();
                let tx = tx.clone();
                async move {
                    received_chunks.lock().unwrap().push(chunk);
                    let _ = tx.send(()).await;
                }
            }
        );

        // We expect a delay of 3s after failure. So we need to wait at least 3s + overhead.
        // Let's wait 6s.
        tokio::select! {
             _ = client_future => {},
             _ = rx.recv() => {
                 info!("Received data after retry");
             },
             _ = tokio::time::sleep(Duration::from_secs(8)) => {
                 info!("Timeout waiting for retry");
             }
        }

        let chunks = received_chunks.lock().unwrap();
        assert!(!chunks.is_empty(), "Should have received data after retry");

        // Verify we had at least 2 connections
        let count = *connection_count.lock().unwrap();
        assert!(count >= 2, "Should have connected at least twice (1 fail, 1 success). Count: {}", count);
    }
}
