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

use async_std::task::{block_on, spawn};
use jni::objects::{JObject, JString};
use jni::sys::{jint, jlong, jstring};
use jni::JNIEnv;
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};
use tempfile::tempdir;

use libqaul::Qaul;
use libqaul_http::HttpServer;
use libqaul_rpc::Responder;
use qaul_chat::Chat;
use ratman_configure::{EpBuilder, NetBuilder};

struct AndroidState {
    libqaul: Arc<Qaul>,
}

unsafe fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(CStr::from_ptr(env.get_string(s).unwrap().as_ptr()))
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
    let recipient = conv_jstring(&env, j_recipient);

    let output = env
        .new_string("Hello ".to_owned() + recipient.as_str())
        .unwrap();
    output.into_inner()
}
/// Function "start_server" that takes a port and path
///
/// The port is used to listen on for the http api, the path is the
/// location of the compiled webui assets.  This function bootstraps
/// the qaul.net stack via ratman-configure and libqaul-http.
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_MainActivity_startServer(
    env: JNIEnv,
    _: JObject,
    port: jint,
    path: JString,
) -> jlong {
    let port = port as u16;
    let path = conv_jstring(&env, path);

    let net = NetBuilder::new()
        //         .endpoint(EpBuilder::tcp("0.0.0.0".into(), port + 1, vec![], false))
        .endpoint(EpBuilder::wifi_direct())
        .build();

    let tmp_dir = tempdir().unwrap();
    let router = net.into_router();
    let libqaul = Qaul::new(router, tmp_dir.path());

    let chat = block_on(async { Chat::new(Arc::clone(&libqaul)).await }).unwrap();
    let http = HttpServer::set_paths(
        path,
        Responder {
            qaul: Arc::clone(&libqaul),
            chat: chat,
        },
    );

    // Spawn the http server off into the background
    spawn(async move { http.listen(&format!("127.0.0.1:{}", port)) });

    let boxed = Box::new(AndroidState { libqaul });
    Box::into_raw(boxed) as i64
}
