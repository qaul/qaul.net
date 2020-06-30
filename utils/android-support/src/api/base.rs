//! Base android (java ffi) API
//!
//! This module provides some simple utilities for setting up the
//! libqaul and router state, and adding new TCP routes to the driver.

use crate::utils::{self, GcWrapped};
use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, jint, jlong},
    JNIEnv,
};

use android_logger::{Config, FilterBuilder};
use libqaul::{users::UserAuth, Qaul};
use log::Level;
use qaul_chat::Chat;
use qaul_voice::Voice;
use ratman::Router;

/// Setup the main database and router state
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_setupState(
    _: JNIEnv,
    _: JObject,
    port: jint,
) -> jlong {
    info!("Rust FFI setupState");
    println!("Setting up android logger and panic hook!");
    android_logger::init_once(
        Config::default()
            .with_tag("rust")
            .with_min_level(Level::Debug),
    );
    utils::init_panic_handling_once();

    // tracing::subscriber::set_global_default(subscriber).unwrap();
    // let subscriber = android_tracing::AndroidSubscriber::new(true)
    //     .with(EnvFilter::new("android_support=trace,[]=warn"));

    info!("Running ratman-configure and libqaul bootstrap code...");

    let router = Router::new();

    let tcp = block_on(async {
        use netmod_tcp::Endpoint;
        let ep = Endpoint::new("0.0.0.0", port as u16, "qauld")
            .await
            .unwrap();
        router.add_endpoint(Arc::clone(&ep)).await;
        ep
    });

    let wd = block_on(async {
        use netmod_wd::WdMod;
        let ep = WdMod::new();
        router.add_endpoint(Arc::clone(&ep)).await;
        ep
    });
    info!("Router init...done");

    let libqaul = Qaul::new(router);
    info!("libqaul init...done");

    block_on(async {
        use libqaul::users::UserUpdate;
        let auth = libqaul.users().create("1234").await.unwrap();
        libqaul
            .users()
            .update(
                auth.clone(),
                UserUpdate::RealName(Some("Alice Anonymous".into())),
            )
            .await;
        libqaul
            .users()
            .update(auth.clone(), UserUpdate::DisplayName(Some("alice".into())))
            .await;
    });

    let chat = block_on(async { Chat::new(Arc::clone(&libqaul)).await }).unwrap();
    let voice = block_on(async { Voice::new(Arc::clone(&libqaul)).await }).unwrap();
    info!("Service init: done");

    // We just return the state pointer here because for some reason
    // storing the state directly in the instance variable doesn't
    // work, or didn't work when I last tried it.  Patches to change
    // this very welcome, if they work!
    GcWrapped::new(tcp, wd, libqaul, chat, voice).into_ptr()
}

/// Check if an auth token is still valid
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_checkLogin(
    _: JNIEnv,
    _: JObject,
    qaul: jlong,
) -> jboolean {
    info!("Rust FFI checkLogin");
    let state = GcWrapped::from_ptr(qaul as i64);
    match state.get_auth() {
        None => false,
        Some(auth) => block_on(async {
            let w = state.get_inner();
            w.qaul()
                .users()
                .is_authenticated(auth)
                .await
                .map(|_| true)
                .unwrap_or(false)
        }),
    }
    .into()
}

/// Check if an auth token is still valid
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_connectTcp(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
    addr: JString,
    port: jint,
) {
    info!("Rust FFI connectTcp");
    let state = GcWrapped::from_ptr(qaul as i64);
    let tcp = state.get_tcp();

    let addr: Vec<u8> = utils::conv_jstring(&env, addr)
        .split(".")
        .map(|s| s.parse().unwrap())
        .collect();
    let port = port as u16;

    block_on(async {
        use std::net::{Ipv4Addr, SocketAddrV4};
        tcp.load_peers(vec![SocketAddrV4::new(
            Ipv4Addr::new(addr[0], addr[1], addr[2], addr[3]),
            port,
        )])
        .await;
    });
}
