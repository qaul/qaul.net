use futures::join;
use libqaul::{error::Result, helpers::TagSet, messages::Mode, Qaul};
use ratman::Router;

#[async_std::main]
async fn main() -> Result<()> {
    let r = Router::new();
    // TDOD: Add network drivers

    let dir = tempfile::tempdir().unwrap();
    let q = Qaul::new(r, dir.path());
    let user = q.users().create("password").await?;

    let msg = q.messages();
    let send = msg.send(
        user.clone(),
        Mode::Flood,
        "de.spacekookie.myapp",
        TagSet::empty(),
        vec![1, 2, 3, 4],
    );
    let subscriber = msg
        .subscribe(user, "de.spacekookie.myapp", TagSet::empty())
        .await?;

    join!(send, subscriber.next(),).0.unwrap();
    Ok(())
}
