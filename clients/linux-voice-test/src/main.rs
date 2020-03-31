use {
    async_std::task::block_on,
    netmod_udp::Endpoint,
    ratman::{Router, Identity},
};

async fn run() {
    let endpoint = Endpoint::spawn("127.0.0.1:4040");

    let router = Router::new();
    router.add_endpoint(endpoint).await;

    let user = Identity::random();
    println!("I am {}", user);

    router.add_user(user).await.unwrap();
    router.online(user).await.unwrap();

    println!("Online");
    loop {
        println!("Found {}", router.discover().await);
    }
}

fn main() {
    block_on(run())
}
