use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;

pub fn run_socket() {

    let socket_dir = "/tmp/";
    let socket_file = "/tmp/qauld.sock";
    // 0700 (read, write, execute for owner only)
    fs::set_permissions(socket_dir, Permissions::from_mode(0o700));
    // 0600 (read, write for owner only)
    fs::set_permissions(socket_file, Permissions::from_mode(0o600));
    
    let listener = UnixListener::bind(socket_file).expect("Unable to bind to Unix socket");
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
