use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
mod socket;
fn main() {
    socket::run_socket();
    let mut stream = UnixStream::connect("/tmp/qauld.sock").expect("Unable to connect to Unix socket");
    let request = "some request".as_bytes();
    match stream.write(request) {
        Ok(_) => {
            let mut buffer = [0; 1024];
            match stream.read(&mut buffer) {
                Ok(_) => {
                    // handle the response
                }
                Err(e) => println!("Error reading from socket: {}", e),
            }
        }
        Err(e) => println!("Error writing to socket: {}", e),
    }
}
