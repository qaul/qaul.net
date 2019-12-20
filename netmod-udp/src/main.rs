//! netmod-udp is a UDP overlay for RATMAN

use async_std::{net::UdpSocket, task};
use identity::Identity;
use std::{
    collections::{BTreeMap, BTreeSet},
    net::IpAddr,
    sync::{Arc, Mutex},
};

enum UdpCommand {
    /// Used to announce a netmod endpoint via broadcasts
    Announce,
    /// Send an ID announcement to known UDP endpoints
    Id(Identity),
}

/// An internal envelope that is used as a transfer protocol
enum UdpEnvelope {
    /// A tunneled data payload
    Data(Vec<u8>),
    /// An overlay command payload
    Internal(UdpCommand),
}

/// Represents an IP network tunneled via UDP
pub struct Endpoint {
    ips: Arc<Mutex<BTreeSet<IpAddr>>>,
    nat: Arc<Mutex<BTreeMap<Identity, IpAddr>>>,
}

impl Endpoint {
    /// Create a new UDP endpoint handler
    pub fn new() -> Self {
        Self {
            ips: Default::default(),
            nat: Default::default(),
        }
    }

    /// Blocking call that runs
    pub fn run() {}
}

fn main() {
    task::block_on(async {
        let socsender = UdpSocket::bind("0.0.0.0:1312").await.unwrap();        
        let socreceiver = UdpSocket::bind("0.0.0.0:1313").await.unwrap();
        let addrsender = socsender.local_addr().unwrap();
        let addrrecvr = socreceiver.local_addr().unwrap();
        dbg!(addrsender);
        dbg!(addrrecvr);

        let mut buf = [0; 16];

        dbg!(socsender.connect("127.0.0.1:1313").await.unwrap());
        dbg!(socsender.send(b"Hello, qaul").await.unwrap());
        dbg!(socreceiver.recv(&mut buf).await.unwrap());
        dbg!(buf);
    });
}


// /// The following is the server code running on my NAS
// fn main() {
//     task::block_on(async {
//         let socket = UdpSocket::bind("0.0.0.0:1312").await.unwrap();
//         let addr = socket.local_addr().unwrap();
//         // socket.connect("10.7.1.123:1312").await.unwrap();
//         dbg!(addr);
        
//         let mut buf = vec![0u8; 1024];
//         loop {
//             dbg!(socket.recv(&mut buf).await.unwrap());
//             dbg!(&buf);
//         }
//     });
// }
