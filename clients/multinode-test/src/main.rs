use async_std::{sync::Arc, task};
//use async_std::future;
use futures::try_join;
use ratman_harness::{temp, Initialize, ThreePoint};
use std::{env, process};
use {
    libqaul::Qaul, libqaul_http::HttpServer, libqaul_rpc::Responder, qaul_chat::Chat,
    qaul_voices::Voices,
};

#[async_std::main]
async fn main() {
    let assets = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: linux-http-test <path-to-static-webgui-directory>");
            process::exit(2);
        }
    };
    let assets_b = assets.clone();

    // Initialize a 3 node local qaul network
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| Qaul::new(arc, temp().path()));

    // services for Node A
    let qaul_a = tp.a.1.unwrap().clone();
    let chat_a = Chat::new(Arc::clone(&qaul_a)).await.unwrap();
    let voices_a = Voices::new(Arc::clone(&qaul_a)).await.unwrap();

    // services for Node B
    let qaul_b = tp.b.1.unwrap().clone();
    let chat_b = Chat::new(Arc::clone(&qaul_b)).await.unwrap();
    let voices_b = Voices::new(Arc::clone(&qaul_b)).await.unwrap();

    // print information for the user
    println!("Path to static web content: {}", assets);
    println!("Open the UI in your web browser:");
    println!("  Node A: http://127.0.0.1:9900");
    println!("  Node B: http://127.0.0.1:9901");

    // configure the web servers
    let server_a = HttpServer::set_paths(
        assets,
        Responder {
            qaul: qaul_a,
            chat: chat_a,
            voices: voices_a,
        },
    );
    let server_b = HttpServer::set_paths(
        assets_b,
        Responder {
            qaul: qaul_b,
            chat: chat_b,
            voices: voices_b,
        },
    );

    // run the servers
    task::block_on(async move {
        let a = server_a.listen("127.0.0.1:9900");
        let b = server_b.listen("127.0.0.1:9901");
        try_join!(a, b).unwrap();
    });
}
