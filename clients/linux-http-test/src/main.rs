use async_std::sync::Arc;
use ratman::Router;
use {libqaul::Qaul, libqaul_http::HttpServer, libqaul_rpc::Responder, qaul_chat::Chat};

fn main() {
    // Init a basic libqaul stack with no interfaces
    let rat = Router::new();
    let qaul = Qaul::new(rat);
    let chat = Chat::new(Arc::clone(&qaul)).unwrap();

    // Start the websocket server
    HttpServer::block("127.0.0.1:9900", Responder { qaul, chat });
}
