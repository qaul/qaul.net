//! Netmod driver for Android WiFi Direct

use async_std::{
    future::{self, Future},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{self, Poll},
};
use async_trait::async_trait;
use std::{collections::VecDeque, ffi::c_void, slice};

use conjoiner;
use netmod::{Endpoint, Frame, Result};

/// The wifi direct state holder that implements the endpoint trait
#[repr(C)]
#[derive(Default)]
pub struct wifid_t {
    /// Queue incoming frames for ratman to poll
    inc: Arc<Mutex<VecDeque<Result<Frame>>>>,
}

/// Create a new state object, allocating it on the heap
pub extern "C" fn new() -> *mut wifid_t {
    Box::into_raw(Box::new(wifid_t::default()))
}

/// Give a Frame to the driver state
///
/// This function will append to a queue that is polled from the
/// ratman runtime.  The name of the function is written from the
/// perspective of the ffi components (giving to Rust)
pub extern "C" fn give(this: *mut wifid_t, f: *const c_void, len: usize) {
    let this = unsafe { Box::from_raw(this) };
    let buf = unsafe { *(f as *const &[u8]) };
    let vec: Vec<u8> = buf.into_iter().take(len).collect();
    
    buf.len();
    
    // let buf: &[u8] = unsafe { slice::from_raw_parts(f, len) };
    // let frame = conjoiner::deserialise(buf);

    // task::spawn(async move {
    //     this.inc.lock().await.push_back(Ok(f));
    // });
}

extern "C" {
    /// Send off a frame over a specific interface
    ///
    /// Hands off a const buffer with a length and target specifier.
    /// Is not responsible for encoding data.
    fn send(f: *const c_void, length: u32, target: i16) -> u16;
}

#[async_trait]
impl Endpoint for wifid_t {
    fn size_hint(&self) -> usize {
        0
    }

    async fn send(&mut self, frame: Frame, target: i16) -> Result<()> {
        unimplemented!()
    }

    async fn next(&mut self) -> Result<(Frame, i16)> {
        let inc = Arc::clone(&self.inc);
        future::poll_fn(|ctx| {
            let lock = &mut inc.lock();
            match unsafe { Pin::new_unchecked(lock).poll(ctx) } {
                Poll::Ready(mut inc) => match inc.pop_front() {
                    Some(Ok(f)) => Poll::Ready(Ok((f, 0))),
                    Some(Err(e)) => Poll::Ready(Err(e)),
                    None => Poll::Pending,
                },
                Poll::Pending => Poll::Pending,
            }
        })
        .await
    }
}
