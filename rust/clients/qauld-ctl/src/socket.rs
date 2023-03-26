use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};

pub fn run_socket() {
    let listener = UnixListener::bind("/tmp/qauld.sock").expect("Unable to bind to Unix socket");
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        // handle the request
                    }
                    Err(e) => println!("Error reading from socket: {}", e),
                }
            }
            Err(e) => println!("Error accepting incoming connection: {}", e),
        }
    }
}
