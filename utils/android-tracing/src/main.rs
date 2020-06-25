#[macro_use] extern crate tracing;

use android_tracing::AndroidSubscriber;

fn main() {
    let sub = AndroidSubscriber::new(true);
    tracing::subscriber::set_global_default(sub).unwrap();

    info!("Hey");
    trace!("Well i guess this part works");
    let span = span!(tracing::Level::TRACE, "test", boop = 1);
    let _e = span.enter();
    warn!("And in spans?");
}
