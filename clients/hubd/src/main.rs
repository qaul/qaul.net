//! # qaul-hubd server
//!
//! A modular and configurable internet overlay server for qaul.net.  

mod cfg;
mod state;
mod log;

use async_std::{future, task::Poll};
use state::State;

#[async_std::main]
async fn main() {
    log::parse_log_level();
    
    let app = cfg::cli();
    let cfg = cfg::match_fold(app);
    let _state = State::new(&cfg).await;

    // Never return the main thread or it all dies
    let _: () = future::poll_fn(|_| Poll::Pending).await;
}
