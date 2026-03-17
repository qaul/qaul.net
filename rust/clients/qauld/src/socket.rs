// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Socket server for qauld

use futures::{stream::StreamExt, SinkExt};
use futures_ticker::Ticker;
use prost::Message;
use std::{
    collections::HashMap,
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    sync::Arc,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    signal,
    sync::{
        mpsc::{self, Sender},
        Mutex,
    },
    time::Duration,
};
use tokio_util::{bytes::Bytes, codec::LengthDelimitedCodec};

#[cfg(windows)]
use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;

/// protobuf RPC definition
pub use qaul_proto::qaul_rpc as proto;

/// Default TCP address used on Windows where Unix sockets are unavailable.
#[cfg(windows)]
const DEFAULT_TCP_ADDR: &str = "127.0.0.1:9199";

async fn handle_client<T>(
    stream: T,
    register: Arc<Mutex<HashMap<String, Sender<Bytes>>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let (tx, mut rx) = mpsc::channel(100);

    let framed_stream = LengthDelimitedCodec::builder()
        .length_field_offset(0)
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
                                log::trace!("message received from client: {client_request_id}");
                                {
                                    let mut reg = register.lock().await;
                                    reg.insert(client_request_id.clone(), tx.clone());
                                }
                            }
                            Err(error) => {
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
    }

    Ok(())
}

/// RPC poller which forwards libqaul response
fn spawn_rpc_poller(register: Arc<Mutex<HashMap<String, Sender<Bytes>>>>) {
    tokio::spawn(async move {
        let mut futures_ticker = Ticker::new(Duration::from_millis(10));
        loop {
            futures_ticker.next().await;
            match libqaul::rpc::Rpc::receive_from_libqaul(&*instance_clone.state) {
                Ok(data) => match proto::QaulRpc::decode(&data[..]) {
                    Ok(msg) => {
                        let client_id = msg.request_id;
                        let sender;
                        log::info!("received response for client: {client_id}");
                        {
                            let reg = register.lock().await;
                            sender = reg.get(&client_id).cloned();
                        }
                        if let Some(rx) = sender {
                            if let Err(err) = rx.send(data.into()).await {
                                log::error!("failed to send data to receiver: {err:#?}");
                            }
                        } else {
                            log::warn!("client ID not found in register");
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    });
}

/// Starts the qauld socket server.
/// Runs infinitely until a shutdown signal is received.
pub async fn start_server(socket_dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let client_request_register: Arc<Mutex<HashMap<String, Sender<Bytes>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    spawn_rpc_poller(client_request_register.clone());

    #[cfg(unix)]
    {
        let socket_path = socket_dir.join("qauld.sock");
        if socket_path.exists() {
            fs::remove_file(&socket_path)?;
        }

        let listener = UnixListener::bind(&socket_path)?;
        fs::set_permissions(&socket_path, Permissions::from_mode(0o666))?;
        println!("qauld unix socket server started");

        loop {
            tokio::select! {
                res = listener.accept() => {
                    let (stream, addr) = res?;
                    let register_clone = client_request_register.clone();
                    tokio::spawn(async move {
                        log::info!("client connected: {addr:#?}");
                        if let Err(e) = handle_client(stream, register_clone).await {
                            log::error!("client error: {e:#?}");
                        }
                    });
                },
                _ = signal::ctrl_c() => {
                    log::info!("shutdown triggered");
                    break;
                }
            }
        }

        fs::remove_file(socket_path)?;
    }

    #[cfg(windows)]
    {
        let listener = TcpListener::bind(DEFAULT_TCP_ADDR).await?;
        println!("qauld TCP socket server started on {DEFAULT_TCP_ADDR}");

        loop {
            tokio::select! {
                res = listener.accept() => {
                    let (stream, addr) = res?;
                    let register_clone = client_request_register.clone();
                    tokio::spawn(async move {
                        log::info!("client connected: {addr:#?}");
                        if let Err(e) = handle_client(stream, register_clone).await {
                            log::error!("client error: {e:#?}");
                        }
                    });
                },
                _ = signal::ctrl_c() => {
                    log::info!("shutdown triggered");
                    break;
                }
            }
        }
    }

    Ok(())
}
