use libqaul::{messages::Recipient, Qaul};
use ratman::Router;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Router::new();
    // TDOD: Add network drivers

    let q = Qaul::new(r);
    let user = q.users().create("password")?;

    // Register a service
    q.services().register("de.spacekookie.myapp")?;
    q.messages().send(
        user.clone(),
        Recipient::Flood,
        "de.spacekokie.myapp",
        vec![1, 2, 3, 4],
    )?;

    q.messages().listen(user, "de.spacekookie.myapp", |msg| {
        println!("Received message: {:?}", msg);
        Ok(()) // Return error if parsing fails
    }).unwrap();

    Ok(())
}
