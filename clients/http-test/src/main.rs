use libqaul::*;
use qaul_http::*;

fn main() {
    let qaul = Qaul::start();
    qaul.user_create("acab").expect("Failed to create test user!");
    
    let _server = ApiServer::new(&qaul, "0.0.0.0:9090")
        .expect("Failed to start qaul.net API server (0.0.0.0:9090)!");

    #[allow(deprecated)]
    loop { std::thread::sleep_ms(500) };
}
