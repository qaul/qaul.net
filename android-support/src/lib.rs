//! qaul.net android interop library

#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;
extern crate android_logger;

use async_std::task::{block_on, spawn};
use jni::objects::{JList, JObject, JString};
use jni::sys::{jboolean, jint};
use jni::JNIEnv;
use std::{
    ffi::{CStr, CString},
    sync::Arc,
};

use android_logger::{Config, FilterBuilder};
use log::Level;

use libqaul::Qaul;
use libqaul_http::{stream, HttpServer};
use libqaul_rpc::Responder;
use qaul_chat::Chat;
use ratman_configure::{EpBuilder, NetBuilder};

struct AndroidState {
    libqaul: Arc<Qaul>,
}

fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(unsafe { CStr::from_ptr(env.get_string(s).unwrap().as_ptr()) })
        .to_str()
        .unwrap()
        .into()
}

// Set a panic handler that will logcan print stacktraces
fn init_panic_handling_once() {
    use std::sync::Once;
    static INIT_BACKTRACES: Once = Once::new();
    INIT_BACKTRACES.call_once(move || {
        std::panic::set_hook(Box::new(move |panic_info| {
            let (file, line) = if let Some(loc) = panic_info.location() {
                (loc.file(), loc.line())
            } else {
                ("<unknown>", 0)
            };
            let reason = panic_info.to_string();

            log::error!(
                "### Rust `panic!` hit at file '{}', line {}: `{}`",
                file,
                line,
                reason
            );
        }));
    });
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_startServer(
    env: JNIEnv,
    this: JObject,
    port: jint,
    path: JString,
) {
    android_logger::init_once(
        Config::default()
            .with_filter(FilterBuilder::new().parse("async-std=error").build())
            .with_min_level(Level::Trace),
    );
    init_panic_handling_once();

    trace!("Hello from Rust, about to bootstrap the code, yo");

    let port = port as u16;
    let path = conv_jstring(&env, path);

    let net = NetBuilder::new()
        .endpoint(EpBuilder::tcp("0.0.0.0".into(), port + 1, false))
        //.endpoint(EpBuilder::wifi_direct())
        .build();

    trace!("Network builder done: {:?}", net);

    let router = net.into_router();
    trace!("Router done");

    let libqaul = Qaul::new(router);

    let chat = block_on(async { Chat::new(Arc::clone(&libqaul)).await }).unwrap();

    trace!("Chat service done");

    let http = HttpServer::set_paths(
        path,
        Responder {
            streamer: stream::setup_streamer(),
            qaul: Arc::clone(&libqaul),
            chat: chat,
        },
    );

    trace!("Http server done");

    // Spawn the http server off into the background
    spawn(async move { http.listen(&format!("127.0.0.1:{}", port)) });

    trace!("Chat service listening done");

    // let boxed = Box::new(AndroidState { libqaul });
    // Box::into_raw(boxed) as i64
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_checkLogin(
    env: JNIEnv,
    _: JObject,
) -> jboolean {
    0
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_userRegister<'this>(
    env: JNIEnv<'this>,
    _: JObject,
    name: JString,
    pw: JString,
) -> JObject<'this> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatList<'this>(
    env: JNIEnv<'this>,
    _: JObject,
) -> JObject<'this> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatStart<'this>(
    env: JNIEnv<'this>,
    _: JObject,
    name: JString,
    friends: JList,
) -> JObject<'this> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatSendMessage<'this>(
    env: JNIEnv<'this>,
    _: JObject,
    room_id: JString,
    content: JString,
) -> JObject<'this> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatLoadMessages<'this>(
    env: JNIEnv<'this>,
    _: JObject,
    room_id: JString,
) -> JObject<'this> {
    JObject::null()
}
