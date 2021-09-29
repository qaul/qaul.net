use async_std;
use libqaul;


#[async_std::main]
async fn main() {
    libqaul::start_cli().await;
}

