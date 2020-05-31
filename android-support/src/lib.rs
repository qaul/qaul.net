//! qaul.net android interop library

#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate log;
extern crate android_logger;

use async_std::task::{block_on, spawn};
use jni::objects::{JList, JObject, JString, JValue};
use jni::sys::{jboolean, jint, jobject, jlong};
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
use qaul_voice::Voice;
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

unsafe fn extract_state(ptr: i64) -> Arc<Qaul> {
    let b: Box<AndroidState> = Box::from_raw(ptr as *mut AndroidState);
    Arc::clone(&b.libqaul)
}

#[allow(unused)]
fn get_qaul_state(env: &JNIEnv, this: JObject) -> Arc<Qaul> {
    let class = env.get_object_class(this).unwrap();
    let ptr = match env.get_field(*class, "libqaulState", "J") {
        Ok(JValue::Long(l)) => l,
        e => panic!("Failed to get JNI data: {:?}", e),
    };

    unsafe { extract_state(ptr) }
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
) -> jlong {
    android_logger::init_once(Config::default().with_min_level(Level::Info));
    init_panic_handling_once();

    info!("Hello from Rust, about to bootstrap the code, yo");

    let port = port as u16;
    let path = conv_jstring(&env, path);

    let net = NetBuilder::new()
        .endpoint(EpBuilder::tcp("0.0.0.0".into(), port + 1, false))
        //.endpoint(EpBuilder::wifi_direct())
        .build();

    info!("Network builder done: {:?}", net);

    let router = net.into_router();
    info!("Router done");

    let libqaul = Qaul::new(router);
    block_on(async {
        use libqaul::users::UserUpdate;
        let auth = libqaul.users().create("1234").await.unwrap();
        libqaul
            .users()
            .update(auth, UserUpdate::DisplayName(Some("Alice".to_string())))
            .await
            .unwrap();
    });

    let chat = block_on(async { Chat::new(Arc::clone(&libqaul)).await }).unwrap();
    let voice = block_on(async { Voice::new(Arc::clone(&libqaul)).await }).unwrap();
    
    info!("Chat service done");

    let http = HttpServer::set_paths(
        path,
        Responder {
            streamer: stream::setup_streamer(),
            qaul: Arc::clone(&libqaul),
            chat,
            voice,
        },
    );

    trace!("Http server done");

    // Spawn the http server off into the background
    spawn(async move { http.listen(&format!("127.0.0.1:{}", port)) });

    trace!("Chat service listening done");

    // Store the libqaulState variable here
    let boxed = Box::new(AndroidState { libqaul });
    Box::into_raw(boxed) as i64
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_checkLogin(
    env: JNIEnv,
    _: JObject,
) -> jboolean {
    0
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersList(
    env: JNIEnv,
    this: JObject,
    qaul: jlong,
) -> jobject {
    let qaul = extract_state(qaul);
    let list: JObject<'_> = *libqaul::ffi::java::users::list(&env, qaul);
    list.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_userRegister<'env>(
    env: JNIEnv<'env>,
    _: JObject,
    name: JString,
    pw: JString,
) -> JObject<'env> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatList<'env>(
    env: JNIEnv<'env>,
    _: JObject,
) -> JObject<'env> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatStart<'env>(
    env: JNIEnv<'env>,
    _: JObject,
    name: JString,
    friends: JList,
) -> JObject<'env> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatSendMessage<'env>(
    env: JNIEnv<'env>,
    _: JObject,
    room_id: JString,
    content: JString,
) -> JObject<'env> {
    JObject::null()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatLoadMessages<'env>(
    env: JNIEnv<'env>,
    _: JObject,
    room_id: JString,
) -> JObject<'env> {
    JObject::null()
}
