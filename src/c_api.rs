//! An API file that exports C-compatible headers

#[allow(warnings)]

#[repr(C)]
pub struct qlendpoint_t {}

#[repr(C)]
pub struct qlrouter_t {}

///
#[no_mangle]
extern "C" fn qaul_router_initialise(slf: *mut *mut qlrouter_t, eps: *const *const qlendpoint_t) -> u32 {
    0
}

///
#[no_mangle]
extern "C" fn qaul_router_shutdown(slf: *mut qlrouter_t) -> u32 {
    0
}