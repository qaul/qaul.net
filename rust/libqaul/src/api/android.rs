// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Android-API for libqaul
//!
//! This is the Android FFI of libqaul.
//!
//! LibqaulKt references the name libqaul.kt, which is the kotlin file exposing the functions.

#![cfg(target_os = "android")]
#![allow(non_snake_case)]
extern crate android_logger;
extern crate log;

use lazy_static::*;
use std::{ffi::c_void, sync::Mutex};

/// Modules for integrating with JavaVM
use jni::objects::{GlobalRef, JByteArray, JClass, JObject, JString};
use jni::sys::{jint, jstring, JNI_ERR, JNI_VERSION_1_6};
use jni::JNIEnv;
use jni::{JavaVM, NativeMethod};

lazy_static! {
    // jvm
    static ref JVM_GLOBAL: Mutex<Option<JavaVM>> = Mutex::new(None);
    //callback
    static ref JNI_CALLBACK: Mutex<Option<GlobalRef>> = Mutex::new(None);
}

#[no_mangle]
pub fn nativeSetCallback(env: JNIEnv, _obj: JObject, callback: JObject) {
    let callback = env.new_global_ref(JObject::from(callback)).unwrap();

    let mut ptr_fn = JNI_CALLBACK.lock().unwrap();
    *ptr_fn = Some(callback);
}

#[no_mangle]
unsafe fn JNI_OnLoad(java_vm: JavaVM, _: *mut std::os::raw::c_void) -> jint {
    let class_name: &str = "net/qaul/ble/core/BleWrapperClass";

    let jni_methods = [jni::NativeMethod {
        name: jni::strings::JNIString::from("nativeSetCallback"),
        sig: jni::strings::JNIString::from(
            "(Lnet/qaul/ble/core/BleWrapperClass$ILibqaulCallback;)V",
        ),
        fn_ptr: nativeSetCallback as *mut c_void,
    }];
    let ok = Android::register_natives(&java_vm, class_name, jni_methods.as_ref());
    if ok == JNI_ERR {
        log::error!("android jni: register natives failed");
    }

    let mut ptr_jvm = JVM_GLOBAL.lock().unwrap();
    *ptr_jvm = Some(java_vm);

    JNI_VERSION_1_6
}

/// dummy function to test the functionality
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_hello(env: JNIEnv, _: JClass) -> jstring {
    // create a string
    let output = env
        .new_string(format!("Hello qaul!"))
        .expect("Couldn't create java string!");

    // return the raw pointer
    output.into_raw()
}

/// start libqaul from android
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_start(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) {
    // get path string
    let android_path: String = env
        .get_string(&path)
        .expect("Couldn't get Java path string!")
        .into();

    // start libqaul in an own thread
    super::start_android(android_path);
}

/// check if libqaul finished initializing
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_initialized(
    mut _env: JNIEnv,
    _: JClass,
) -> bool {
    super::initialization_finished()
}

/// get number of messages sent via RPC to libqaul from android
/// this function is only for debugging
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_sendcounter(
    mut _env: JNIEnv,
    _: JClass,
) -> jint {
    // return number of RPC messages sent to libqaul
    super::send_rpc_count() as jint
}

/// get number of messages queued to be received by this program
/// from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_receivequeue(
    mut _env: JNIEnv,
    _: JClass,
) -> jint {
    // return the number of RPC messages in the pipeline to be
    // received by the GUI
    super::receive_rpc_queued() as jint
}

/// send an rpc message from android to libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_send<'local>(
    env: JNIEnv<'local>,
    _: JClass,
    message: JByteArray<'local>,
) {
    // get the message out of java
    let binary_message: Vec<u8> = env.convert_byte_array(&message).unwrap();

    // send it to libqaul
    super::send_rpc(binary_message);
}

/// receive an rpc message on android from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_receive<'local>(
    env: JNIEnv<'local>,
    _: JClass,
) -> JByteArray<'local> {
    // check if there is an RPC message
    if let Ok(message) = super::receive_rpc() {
        // convert message to java byte array
        let byte_array = env.byte_array_from_slice(&message).unwrap();
        // return byte array
        return byte_array;
    }

    // there is no message and we return an empty array
    let buf: [u8; 0] = [0; 0];
    let empty_array = env.byte_array_from_slice(&buf).unwrap();
    empty_array
}

/// # BLE Module Functions
///
/// Set's up the system protobuf communication pipelines
/// between libqaul and the BLE module library.

/// send a sys protobuf message from BLE module to libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_syssend<'local>(
    env: JNIEnv<'local>,
    _: JClass,
    message: JByteArray<'local>,
) {
    // get the message out of java
    let binary_message: Vec<u8> = env.convert_byte_array(&message).unwrap();

    // send it to libqaul
    super::send_sys(binary_message);
}

/// receive a sys message on android from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_sysreceive<'local>(
    env: JNIEnv<'local>,
    _: JClass,
) -> JByteArray<'local> {
    // check if there is an RPC message
    if let Ok(message) = super::receive_sys() {
        // convert message to java byte array
        let byte_array = env.byte_array_from_slice(&message).unwrap();
        // return byte array
        return byte_array;
    }

    // there is no message and we return an empty array
    let buf: [u8; 0] = [0; 0];
    let empty_array = env.byte_array_from_slice(&buf).unwrap();
    empty_array
}

pub struct Android {}

impl Android {
    /// send an sys message to Android BLE Module
    /// This function will call the Android BLE Module's "receiveRequest" function.
    pub fn send_to_android(message: Vec<u8>) {
        Self::call_java_callback(message);
    }

    unsafe fn register_natives(jvm: &JavaVM, class_name: &str, methods: &[NativeMethod]) -> jint {
        let mut env: JNIEnv = jvm.get_env().unwrap();
        let jni_version = env.get_version().unwrap();
        let version: jint = jni_version.into();

        let clazz = match env.find_class(class_name) {
            Ok(clazz) => clazz,
            Err(e) => {
                log::error!("java class not found : {:?}", e);
                return JNI_ERR;
            }
        };
        let result = env.register_native_methods(clazz, &methods);

        if result.is_ok() {
            //   log::info!("register_natives : succeed");
            version
        } else {
            //   log::error!("register_natives : failed ");
            JNI_ERR
        }
    }

    fn call_jvm<F>(callback: &Mutex<Option<GlobalRef>>, run: F)
    where
        F: Fn(&JObject, &mut JNIEnv) + Send + 'static,
    {
        let ptr_jvm = JVM_GLOBAL.lock().unwrap();
        if (*ptr_jvm).is_none() {
            return;
        }
        let ptr_fn = callback.lock().unwrap();
        if (*ptr_fn).is_none() {
            return;
        }
        let jvm: &JavaVM = (*ptr_jvm).as_ref().unwrap();
        match jvm.attach_current_thread_permanently() {
            Ok(mut env) => {
                let obj = (*ptr_fn).as_ref().unwrap().as_obj();
                run(obj, &mut env);
                if let Ok(true) = env.exception_check() {
                    let _ = env.exception_describe();
                    let _ = env.exception_clear();
                }
            }
            Err(e) => {
                log::debug!("jvm attach_current_thread failed: {:?}", e);
            }
        }
    }

    pub fn call_java_callback(fun_type: Vec<u8>) {
        Self::call_jvm(&JNI_CALLBACK, move |obj: &JObject, env: &mut JNIEnv| {
            let jmessage = env.byte_array_from_slice(fun_type.as_slice()).unwrap();
            match env.call_method(
                obj,
                "OnLibqaulMessage",
                "([B)V",
                &[(&JObject::from(jmessage)).into()],
            ) {
                Ok(jvalue) => {
                    log::debug!("callback succeed: {:?}", jvalue);
                }
                Err(e) => {
                    log::error!("callback failed : {:?}", e);
                }
            }
        });
    }
}
