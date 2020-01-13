use libqaul::*;
use libqaul_http::*;
use std::sync::Arc;
use qaul_messaging::Messaging;

fn main() {
    let qaul = Arc::new(Qaul::dummy());

    let messaging = Messaging::new(qaul.clone()); 

    let _server = ServerBuilder::new(qaul.clone())
        .messaging(&messaging)
        .start("0.0.0.0:9090")
        .expect("Failed to start qaul.net API server (0.0.0.0:9090)!");

    println!("Started qaul.net API server at http://0.0.0.0:9090");

    #[allow(deprecated)]
    loop { std::thread::sleep_ms(500) };
}
