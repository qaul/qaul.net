//! # qaul-hubd server
//!
//! A modular and configurable internet overlay server for qaul.net.

mod cfg;
mod log;
mod state;
mod upnp;

use async_std::{future, task::Poll};
use state::State;

pub(crate) fn elog<S: Into<String>>(msg: S, code: u16) -> ! {
    tracing::error!("{}", msg.into());
    std::process::exit(code.into());
}

#[async_std::main]
async fn main() {
    log::parse_log_level();

    let app = cfg::cli();
    let cfg = cfg::match_fold(app);
    let _state = State::new(&cfg).await;

    // !no_upnp means upnp has _not_ been disabled
    if !cfg.no_upnp {
        upnp::open_port(cfg.port);
    }

    // Never return the main thread or it all dies
    let _: () = future::poll_fn(|_| Poll::Pending).await;
}
