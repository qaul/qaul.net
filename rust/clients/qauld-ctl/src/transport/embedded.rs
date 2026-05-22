// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! In-process embedded transport. Starts a libqaul instance in the
//! current process and talks to it directly via the `Rpc::send_to_libqaul`
//! / `Rpc::receive_from_libqaul` channels — no socket involved.
//!
//! Designed for one-shot CLI use (à la the legacy `qaul-cli` single
//! binary). Spins up libqaul in a background thread, polls its receive
//! channel for the matching `request_id`, drops everything when the
//! `Drop` impl fires.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use prost::Message;

use crate::cli::Cli;
use crate::proto;
use crate::transport::{encode_envelope, RpcTransport};

pub struct EmbeddedTransport {
    instance: Arc<libqaul::Libqaul>,
}

impl EmbeddedTransport {
    /// Bring up a libqaul instance in this process. `storage_path` is
    /// taken from `--dir`, then current working directory.
    pub async fn start(cli: &Cli) -> Result<Self, Box<dyn std::error::Error>> {
        let storage_path = if let Some(dir) = &cli.dir {
            dir.clone()
        } else {
            std::env::current_dir()?
                .to_string_lossy()
                .into_owned()
        };

        let instance = libqaul::api::start_instance_in_thread(storage_path, None);

        // Spin until the libqaul thread has initialised.
        let init_deadline = std::time::Instant::now() + Duration::from_secs(30);
        while !instance.is_initialized() {
            if std::time::Instant::now() >= init_deadline {
                return Err("embedded libqaul failed to initialise within 30s".into());
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        // Best-effort: create a default user account if none exists.
        // Mirrors qauld's own bootstrap so the embedded mode is
        // immediately usable for one-shot commands.
        if libqaul::node::user_accounts::UserAccounts::len(&*instance.state) == 0 {
            let name = format!(
                "Community Node {}",
                libqaul::utilities::timestamp::Timestamp::get_timestamp()
            );
            libqaul::node::user_accounts::UserAccounts::create(&*instance.state, name, None);
        }

        Ok(Self { instance })
    }
}

#[async_trait]
impl RpcTransport for EmbeddedTransport {
    async fn request(
        &mut self,
        envelope: proto::QaulRpc,
        timeout: Duration,
        expect_response: bool,
    ) -> Result<Option<proto::QaulRpc>, Box<dyn std::error::Error>> {
        let want_id = envelope.request_id.clone();
        let bytes = encode_envelope(&envelope);
        libqaul::rpc::Rpc::send_to_libqaul(&*self.instance.state, bytes);

        if !expect_response {
            return Ok(None);
        }

        let deadline = std::time::Instant::now() + timeout;
        loop {
            match libqaul::rpc::Rpc::receive_from_libqaul(&*self.instance.state) {
                Ok(data) => match proto::QaulRpc::decode(&data[..]) {
                    Ok(resp) if resp.request_id == want_id => return Ok(Some(resp)),
                    Ok(_) => {
                        // Response for a different request (e.g. a
                        // pushed event); drop and keep polling.
                        continue;
                    }
                    Err(e) => return Err(format!("malformed RPC envelope: {e}").into()),
                },
                Err(_) => {
                    if std::time::Instant::now() >= deadline {
                        return Err("timed out waiting for libqaul response".into());
                    }
                    tokio::time::sleep(Duration::from_millis(5)).await;
                }
            }
        }
    }
}
