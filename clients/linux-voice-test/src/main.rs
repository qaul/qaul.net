use {
    std::env::args,
    async_std::task::block_on,
    netmod_udp::Endpoint,
    ratman::{Router, Identity},
};

async fn run() {
    let args = args().collect::<Vec<_>>();
    let endpoint = Endpoint::spawn(&args[1]);

    for i in 2..args.len() {
        endpoint.introduce(&args[i]).await;
    }

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
