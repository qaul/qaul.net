//! Main entry point to the hub daemon runner
//!
//! This server starts a Tcp overlay endpoint with the list of initial
//! peer nodes (must be provided at startup).

mod cfg;
mod state;

use state::State;

fn main() {
    let app = cfg::cli();
    let cfg = cfg::match_fold(app);
    let state = State::new(&cfg);
    

}

// use alexandria::utils::Tag;
// use async_std::prelude::*;
// use libqaul::{messages::Mode, users::UserUpdate, Qaul};
// use netmod_udp::Endpoint as UdpEndpoint;
// use ratman::Router;
// use std::sync::Arc;

// mod cli;
// mod config;
// mod spawn;

// const IPC_SERVICE_NAME: &'static str = "qaul.qauld.server-ipc";
// const IPC_SERVICE_TAG: &'static str = "ipc-control-message";

// #[async_std::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let options = cli::inflate_options().await;
//     dbg!(options);
//     let server = Arc::new(UdpEndpoint::spawn(12226));
//     let router = Router::new();
//     router
//         .add_endpoint(Arc::try_unwrap(server).unwrap_or_else(|_| panic!("Couldn't get endpoint.")))
//         .await;
//     let core = Qaul::new(router, std::path::Path::new("./store"));

//     let server_user = core.users().create("REPLACE ME").await?;
//     core.services().register(IPC_SERVICE_NAME, |e| ()).await?;
//     core.users()
//         .update(
//             server_user.clone(),
//             UserUpdate::AddService(IPC_SERVICE_NAME.into()),
//         )
//         .await?;
//     let subscription = core
//         .messages()
//         .subscribe(
//             server_user.clone(),
//             IPC_SERVICE_NAME,
//             Tag::empty(IPC_SERVICE_TAG),
//         )
//         .await?;

//     let hypoth_user = core.users().create("REPLACE ME TOO").await?;
//     let server_identity = server_user.clone().0;

//     println!("Sending message...");
//     let r = core
//         .messages()
//         .send(
//             hypoth_user.clone(),
//             Mode::Std(server_identity),
//             IPC_SERVICE_NAME,
//             Tag::empty(IPC_SERVICE_TAG),
//             "TEST TEST TEST".as_bytes().into(),
//         )
//         .await
//         .expect("Failed to send message.");
//     println!("Successfully sent message {}.", r);
//     while let Some(msg) = subscription.next().await {
//         println!("{:?}", msg);
//     }

//     Ok(())
// }
