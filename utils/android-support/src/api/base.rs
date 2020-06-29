//! Base android (java ffi) API
//!
//! This module provides some simple utilities for setting up the
//! libqaul and router state, and adding new TCP routes to the driver.

use crate::utils::{self, StateWrapped};
use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::JObject,
    sys::{jboolean, jint, jlong},
    JNIEnv,
};

use libqaul::{users::UserAuth, Qaul};
use qaul_chat::Chat;
use qaul_voice::Voice;
use ratman_configure::{EpBuilder, NetBuilder};

/// Setup the main database and router state
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_setupState(
    env: JNIEnv,
    this: JObject,
    port: jint,
) -> jlong {
    //android_logger::init_once(Config::default().with_min_level(Level::Info));
    utils::init_panic_handling_once();
    // let subscriber = android_tracing::AndroidSubscriber::new(true)
    //     .with(EnvFilter::new("android_support=trace,[]=warn"));
    // tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Running ratman-configure and libqaul bootstrap code...");

    let port = port as u16;
    let net = NetBuilder::new()
        .endpoint(EpBuilder::tcp("0.0.0.0".into(), port, false))
        //.endpoint(EpBuilder::wifi_direct())
        .build();
    info!("Network builder...done: {:?}", net);

    let router = net.into_router();
    info!("Router init...done");

    let libqaul = Qaul::new(router);
    info!("libqaul init...done");

    let chat = block_on(async { Chat::new(Arc::clone(&libqaul)).await }).unwrap();
    let voice = block_on(async { Voice::new(Arc::clone(&libqaul)).await }).unwrap();
    info!("Service init: done");

    // We just return the state pointer here because for some reason
    // storing the state directly in the instance variable doesn't
    // work, or didn't work when I last tried it.  Patches to change
    // this very welcome, if they work!
    StateWrapped::new(libqaul).into_ptr()
}

/// Check if an auth token is still valid
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_checkLogin(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
) -> jboolean {
    let state = StateWrapped::from_ptr(qaul as i64);
    match state.get_auth() {
        None => false,
        Some(auth) => block_on(async {
            let qaul = state.get_inner();
            qaul.users()
                .is_authenticated(auth)
                .await
                .map(|_| true)
                .unwrap_or(false)
        }),
    }
    .into()
}
