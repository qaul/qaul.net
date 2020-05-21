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
use jni::sys::{jint, jstring};
use jni::JNIEnv;
use std::ffi::{CStr, CString};

use ratman_configure::{EpBuilder, NetBuilder};
use libqaul::Qaul;
use tempfile::tempdir;
use libqaul_http;

fn conv_jstring(s: jstring) -> String {
    CString::from(CStr::from_ptr(
        env.get_string(j_recipient).unwrap().as_ptr(),
    ))
    .to_str()
    .unwrap()
    .into()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_MainActivity_hello(
    env: JNIEnv,
    _: JObject,
    j_recipient: JString,
) -> jstring {
    let recipient = conv_jstring(j_recipient);

    let output = env
        .new_string("Hello ".to_owned() + recipient.to_str().unwrap())
        .unwrap();
    output.into_inner()
}

/// Function "start_server" that takes a port and path
///
/// The port is used to listen on for the http api, the path is the
/// location of the compiled webui assets.  This function bootstraps
/// the qaul.net stack via ratman-configure and libqaul-http.
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_MainActivity_start_server(
    env: JNIEnv,
    port: jint,
    path: jstring,
) -> jint {
    let port = port as u16;
    let path = conv_jstring(path);

    let net = NetBuilder::new()
        .endpoint(EpBuilder::tcp("0.0.0.0", port + 1, peers: vec![], false).build())
        .endpoint(EpBuilder::wifi_direct().build())
        .build();

    let tmp_dir = tempdir().unwrap();
    let router = net.into_router();
    let libqaul = Qaul::new(router, tmp_dir.path());
    
    
    0.into()
}
