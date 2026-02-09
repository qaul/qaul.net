// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Unix socket server for qauld

use futures::{stream::StreamExt, FutureExt, SinkExt};
use futures_ticker::Ticker;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::{net::UnixListener, signal, sync::Mutex, time::Duration};
use tokio_util::codec::LengthDelimitedCodec;
use uuid::Uuid;

/// Starts the qauld unix socket server.
/// Runs infinitely until a shutdown signal is receibed.
/// It accepts connections on `qauld.sock` in cwd,
/// forwards requests to libqaul, and sends responses back to clients.
pub async fn start_server(socket_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let socket_path = socket_dir.join("qauld.sock");
    if socket_path.exists() {
        fs::remove_file(&socket_path)?;
    }

    let listener = UnixListener::bind(&socket_path)?;
    println!("qauld unix socket server started");

    let client_request_register = Arc::new(Mutex::new(HashMap::new()));

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (stream, addr) = res?;
                let register_clone = client_request_register.clone();
                tokio::spawn(async move {
                    println!("client connected");

                    let client_request_id = Uuid::new_v4().to_string();

                    {
                        let mut register = register_clone.lock().await;
                        register.insert(client_request_id.clone(), addr);
                    }

                    let framed_stream = LengthDelimitedCodec::builder().length_field_offset(0)
                        .length_field_type::<u16>()
                        .length_adjustment(0)
                        .new_framed(stream);
                    let (mut writer, mut reader) = framed_stream.split();
                    let mut futures_ticker = Ticker::new(Duration::from_millis(10));

                    loop {
                        let rpc_fut = futures_ticker.next().fuse();
                        tokio::select! {
                            message = reader.next() => {
                                match message {
                                    Some(msg) => {
                                        let msg = msg?.to_vec();
                                        libqaul::api::send_rpc(msg);
                                    }
                                    None => { break; }
                                }

                            },
                            _rpc_ticker = rpc_fut => {
                                match libqaul::api::receive_rpc() {
                                    Ok(data) => {
                                        writer.send(data.into()).await?;
                                    }
                                    _ => {}
                                }
                            }
                        };
                    };

                   Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                });
            },
            _ = signal::ctrl_c() => {
                println!("shutdown triggered");
                break;
            }
        }
    }

    fs::remove_file(socket_path)?;

    Ok(())
}
