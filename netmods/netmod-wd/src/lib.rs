//! Netmod driver for Android WiFi Direct

use async_std::{
    sync::{channel, Arc, Mutex, Receiver, Sender},
    task,
};
use async_trait::async_trait;
use netmod::{Endpoint, Error, Frame, Result, Target};
use std::{collections::VecDeque, ffi::c_void};

pub struct WdMod {
    recv_queue: (Sender<(Frame, Target)>, Receiver<(Frame, Target)>),
    send_queue: (Sender<(Frame, Target)>, Receiver<(Frame, Target)>),
}

impl WdMod {
    pub fn new() -> Self {
        Self {
            recv_queue: channel(1),
            send_queue: channel(1),
        }
    }

    /// Give some data to this netmod, receiving it on the device
    ///
    /// This function is called by the java-android driver stack in
    /// android-support which is called by any app that implements the
    /// WifiDirect mode, It could also be used as a general FFI shim
    /// for other drivers.
    pub fn give(self: &Arc<Self>, f: Frame, t: Target) {
        let this = Arc::clone(self);
        task::spawn(async move { this.recv_queue.0.send((f, t)).await });
    }

    /// Block on taking a new
    pub fn take(self: &Arc<Self>) -> (Frame, Target) {
        task::block_on(async { self.send_queue.1.recv().await.unwrap() })
    }
}

// /// Give a Frame to the driver state
// ///
// /// This function will append to a queue that is polled from the
// /// ratman runtime.  The name of the function is written from the
// /// perspective of the ffi components (giving to Rust)
// pub extern "C" fn give(this: *mut wifid_t, f: *const c_void, len: usize) {
//     let this = unsafe { Box::from_raw(this) };
//     let buf = unsafe { *(f as *const &[u8]) };
//     let vec: Vec<u8> = buf.into_iter().take(len).cloned().collect();
//     let frame = bincode::deserialize(&vec).unwrap();

//     task::spawn(async move {
//         this.inc.lock().await.push_back(Ok(frame));
//     });
// }

// extern "C" {
//     /// Send off a frame over a specific interface
//     ///
//     /// Hands off a const buffer with a length and target specifier.
//     /// Is not responsible for encoding data.
//     fn send_raw(f: *const c_void, length: usize, target: i16) -> u16;
// }

#[async_trait]
impl Endpoint for WdMod {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&self, frame: Frame, t: Target) -> Result<()> {
        self.send_queue.0.send((frame, t)).await;
        Ok(())

        // let buf = Box::new(bincode::serialize(&frame).unwrap());
        // let len = buf.len();
        // let c = unsafe { send_raw(Box::into_raw(buf) as *const c_void, len, 0) };
        // match c {
        //     0 => Ok(()),
        //     // TODO: disambiguate errors here (create error mapping?)
        //     _ => Err(Error::ConnectionLost),
        // }
    }

    async fn next(&self) -> Result<(Frame, Target)> {
        Ok(self.recv_queue.1.recv().await.unwrap())

        // let inc = Arc::clone(&self.inc);
        // future::poll_fn(|ctx| {
        //     let lock = &mut inc.lock();
        //     match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
        //         Poll::Ready(mut inc) => match inc.pop_front() {
        //             Some(Ok(f)) => Poll::Ready(Ok((f, Target::default()))),
        //             Some(Err(e)) => Poll::Ready(Err(e)),
        //             None => Poll::Pending,
        //         },
        //         Poll::Pending => Poll::Pending,
        //     }
        // })
        // .await
    }
}
