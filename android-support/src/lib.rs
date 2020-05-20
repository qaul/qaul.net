//! qaul.net android interop library
//!
//! A lot of functions are handled internally, for example after
//! spawning the http server, the main way of communicating with the
//! libqaul stack is via the http api.  Some functions need to be
//! exposed from the services, for example for more efficient audio
//! streaming or notifications, and those are handled via this
//! library.
//!
//! It can depend on any library in the qaul.net ecosystem, and can
//! also handle initialisation for the hardware drivers on android.

#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::{CStr, CString};

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_MainActivity_hello(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ));

    let output = env
        .new_string("Hello ".to_owned() + recipient.to_str().unwrap())
        .unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_MainActivity_start_server(
    env: JNIEnv,
    _: JObject,
) -> jint{
    let output = env
        .new_string("Hello ".to_owned())
        .unwrap();
    output.into_inner()
}
