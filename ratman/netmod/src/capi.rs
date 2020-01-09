//! The native platform ABI wrapper for netmod
//!
//! This API is designed to create a coherent interface to write
//! netmod drivers in languages that are not Rust, via the C ABI.

use crate::{Endpoint as EndpointExt, Frame, Result};
use async_std::task;
use async_trait::async_trait;

/// A zero sized type to hold type info for native interface bridge
#[repr(C)]
pub struct Endpoint;

extern "C" {

    /// Let's the native driver provide a size hint for frame delivery
    #[no_mangle]
    pub fn size_hint() -> usize;

    /// Dispatch a Frame through this interface
    #[no_mangle]
    pub fn send(frame: Frame, target: i16);

    /// Get the next available frame from the interface driver
    #[no_mangle]
    pub fn next() -> (Frame, i16);
}

#[async_trait]
impl EndpointExt for Endpoint {
    fn size_hint(&self) -> usize {
        unsafe { size_hint() }
    }

    async fn send(&mut self, frame: Frame, target: i16) -> Result<()> {
        task::block_on(async { unsafe { send(frame, target) } });
        Ok(())
    }

    async fn next(&mut self) -> Result<(Frame, i16)> {
        Ok(task::block_on(async { unsafe { next() } }))
    }
}
