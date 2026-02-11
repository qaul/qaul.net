// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Unix socket server for qauld

use futures::{stream::StreamExt, SinkExt};
use futures_ticker::Ticker;
use prost::Message;
use std::{collections::HashMap, fs, path::PathBuf, sync::Arc};
use tokio::{
    net::UnixListener,
    signal,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    time::Duration,
};
use tokio_util::{bytes::Bytes, codec::LengthDelimitedCodec};

/// include generated protobuf RPC rust definition file
pub mod proto {
    include!("../../../libqaul/src/rpc/protobuf_generated/rust/qaul.rpc.rs");
}

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

    let client_request_register: Arc<Mutex<HashMap<String, Sender<Bytes>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    //  Central RPC poller. Polls libqaul for a response
    let register_clone = client_request_register.clone();
    tokio::spawn(async move {
        let mut futures_ticker = Ticker::new(Duration::from_millis(10));
        loop {
            futures_ticker.next().await;
            match libqaul::api::receive_rpc() {
                Ok(data) => match proto::QaulRpc::decode(&data[..]) {
                    Ok(msg) => {
                        let client_id = msg.request_id;
                        let sender;
                        println!("received response for client: {client_id}");
                        {
                            let register = register_clone.lock().await;
                            sender = register.get(&client_id).cloned();
                        }
                        if let Some(rx) = sender {
                            if let Err(err) = rx.send(data.into()).await {
                                eprintln!("failed to send data to receiver: {err:#?}");
                                log::error!("failed to send data to receiver: {err:#?}");
                            }
                        } else {
                            eprintln!("client ID not found in register");
                            log::warn!("client ID not found in register");
                        };
                    }
                    _ => {}
                },
                // move on to the next tick
                _ => {}
            }
        }
    });

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (stream, addr) = res?;
                let register_clone = client_request_register.clone();
                tokio::spawn(async move {
                    println!("client connected: {addr:#?}");

                    let (tx, mut rx) = mpsc::channel(100);

                    let framed_stream = LengthDelimitedCodec::builder().length_field_offset(0)
                        .length_field_type::<u16>()
                        .length_adjustment(0)
                        .new_framed(stream);
                    let (mut writer, mut reader) = framed_stream.split();

                    loop {
                        tokio::select! {
                            message = reader.next() => {
                                match message {
                                    Some(msg) => {
                                        let data = msg?.to_vec();
                                        match proto::QaulRpc::decode(&data[..]) {
                                            Ok(rpc_msg) => {
                                                let client_request_id = rpc_msg.request_id;
                                                println!("message received from client: {client_request_id}");
                                                {
                                                    let mut register = register_clone.lock().await;
                                                    register.insert(client_request_id.clone(), tx.clone());
                                                }
                                            }
                                            Err(error) => {
                                                eprintln!("{error:?}");
                                                log::error!("{:?}", error);
                                            }
                                        }
                                        libqaul::api::send_rpc(data);
                                    }
                                    None => { break; }
                                }

                            },
                            res = rx.recv() => {
                                if let Some(data) = res {
                                    writer.send(data.into()).await?;
                                } else {
                                    break;
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
