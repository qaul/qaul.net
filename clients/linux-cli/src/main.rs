use futures::{
    executor::block_on,
    stream::{StreamExt, Stream},
    join,
};
use libqaul::{messages::Mode, Qaul};
use ratman::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Router::new();
    // TDOD: Add network drivers

    let q = Qaul::new(r);
    let user = q.users().create("password")?;

    let msg = q.messages();
    let send = msg
        .send(
            user.clone(),
            Mode::Flood,
            "de.spacekookie.myapp",
            vec![],
            vec![1, 2, 3, 4],
        );
    let mut subscriber = msg
        .subscribe(user, "de.spacekookie.myapp", None)?;

    block_on(async { join!(
            send,
            subscriber.next(),
    )});

    Ok(())
}
