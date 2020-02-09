use async_std::task;
use libqaul::{messages::Mode, Qaul};
use ratman::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Router::new();
    // TDOD: Add network drivers

    let q = Qaul::new(r);
    let user = q.users().create("password")?;

    // Register a service
    q.services().register("de.spacekookie.myapp")?;
    task::block_on(async {
        q.messages()
            .send(
                user.clone(),
                Mode::Flood,
                "de.spacekokie.myapp",
                vec![],
                vec![1, 2, 3, 4],
            )
            .await
    })?;

    q.messages().listen(user, "de.spacekookie.myapp", |msg| {
        println!("Received message: {:?}", msg);
        Ok(()) // Return error if parsing fails
    })?;

    Ok(())
}
