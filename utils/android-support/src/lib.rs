//! qaul.net android interop library

#![cfg(target_os = "android")]
#![allow(non_snake_case)]

#[macro_use]
extern crate tracing;
extern crate android_logger;

pub mod api;
mod utils;

// use async_std::{
//     sync::RwLock,
//     task::{block_on, spawn},
// };
// use jni::objects::{JList, JObject, JString, JValue};
// use jni::sys::{jboolean, jint, jlong, jobject, jcharArray};
// use jni::JNIEnv;
// use std::{
//     ffi::{CStr, CString},
//     sync::Arc,
// };

// use android_logger::{Config, FilterBuilder};
// use log::Level;

// use libqaul::{users::UserAuth, Qaul};
// use libqaul_http::{stream, HttpServer};
// use libqaul_rpc::Responder;
// use qaul_chat::Chat;
// use qaul_voice::Voice;
// use ratman_configure::{EpBuilder, NetBuilder};

// use tracing_subscriber::{layer::SubscriberExt, EnvFilter};


// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatList<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
// ) -> JObject<'env> {
//     JObject::null()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatStart<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     name: JString,
//     friends: JList,
// ) -> JObject<'env> {
//     JObject::null()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatSendMessage<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     room_id: JString,
//     content: JString,
// ) -> JObject<'env> {
//     JObject::null()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_chatLoadMessages<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     room_id: JString,
// ) -> JObject<'env> {
//     JObject::null()
// }

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_wdReceiveFrame<'env>(
//     env: JNIEnv<'env>,
//     _: JObject,
//     qaul: jlong,
//     target: jint,
//     data: jchatArray,
// ){
    
// }


