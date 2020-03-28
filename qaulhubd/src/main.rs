use async_std::net::UdpSocket;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("127.0.0.1:20122").await?;
    println!("Bound 20122");
    let mut buf = vec![0u8; 1024];

    loop {
        let (n, peer) = socket.recv_from(&mut buf).await?;
        println!("recvd from {:?}", peer);
        socket.send_to(&buf[..n], &peer).await?;
    }
}
