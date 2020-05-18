//! Main entry point to the hub daemon runner
//!
//! This server starts a Tcp overlay endpoint with the list of initial
//! peer nodes (must be provided at startup).

mod cfg;
mod state;

use async_std::{future, task::Poll};
use state::State;
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[async_std::main]
async fn main() {
    let app = cfg::cli();
    let cfg = cfg::match_fold(app);

    // Initialise the logger after CLI validation
    let _subscriber = fmt().with_env_filter("async-std=error").with_max_level(Level::DEBUG).init();
    info!("Initialised logger: welcome to qauld!");

    let _state = State::new(&cfg).await;

    // Never return the main thread or it all dies
    let _: () = future::poll_fn(|_| Poll::Pending).await;
}
