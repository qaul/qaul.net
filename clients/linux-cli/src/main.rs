use futures::{
    executor::block_on,
    join,
    stream::{Stream, StreamExt},
};
use libqaul::{messages::Mode, Qaul};
use ratman::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Router::new();
    // TDOD: Add network drivers

    let dir = tempfile::tempdir().unwrap();
    let q = Qaul::new(r, dir.path());
    let user = block_on(async { q.users().create("password").await })?;

    let msg = q.messages();
    let send = msg.send(
        user.clone(),
        Mode::Flood,
        "de.spacekookie.myapp",
        vec![],
        vec![1, 2, 3, 4],
    );
    let mut subscriber = msg.subscribe(user, "de.spacekookie.myapp", None)?;

    block_on(async { join!(send, subscriber.next(),) });

    Ok(())
}
