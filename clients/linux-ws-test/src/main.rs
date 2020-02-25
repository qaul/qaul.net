use libqaul::Qaul;
use libqaul_ws::WsServer;
use qaul_chat::Chat;
use ratman::Router;
use async_std::sync::Arc;

fn main() {

    // Init a basic libqaul stack with no interfaces
    let rat = Router::new();
    let qaul = Qaul::new(rat);
    let chat = Chat::new(Arc::clone(&qaul)).unwrap();

    // Start the websocket server
    let ws = WsServer::new("127.0.0.1:9900", qaul, chat);
    ws.block();
}
