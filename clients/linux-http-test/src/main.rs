use async_std::sync::Arc;
use ratman::Router;
use {
    libqaul::Qaul, libqaul_http::HttpServer, libqaul_rpc::Responder, 
    qaul_chat::Chat, qaul_voices::Voices
};
use std::{env, process};

fn main() {

    let assets = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: linux-http-test <path>");
            process::exit(2);
        }
    };
    
    // Init a basic libqaul stack with no interfaces
    let rat = Router::new();
    let qaul = Qaul::new(rat);
    let chat = Chat::new(Arc::clone(&qaul)).unwrap();
    let voices = Voices::new(Arc::clone(&qaul)).unwrap();

    // print the path to the static 
    println!("Path to static web content: {}", assets);
    println!("Open http://127.0.0.1:9900 in your web browser");

    // Start the websocket server
    HttpServer::block("127.0.0.1:9900", assets, Responder { qaul, chat, voices });
}
