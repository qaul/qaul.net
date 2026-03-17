// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
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

use std::{ffi::c_void, sync::Mutex};

/// Modules for integrating with JavaVM
use jni::objects::{GlobalRef, JByteArray, JClass, JObject, JString};
use jni::sys::{jint, jstring, JNI_ERR, JNI_VERSION_1_6};
use jni::JNIEnv;
use jni::{JavaVM, NativeMethod};

/// JNI requires global statics — `extern "system"` functions cannot carry context.
static JVM_GLOBAL: Mutex<Option<JavaVM>> = Mutex::new(None);
static JNI_CALLBACK: Mutex<Option<GlobalRef>> = Mutex::new(None);

#[no_mangle]
pub fn nativeSetCallback(env: JNIEnv, _obj: JObject, callback: JObject) {
    let callback = match env.new_global_ref(JObject::from(callback)) {
        Ok(cb) => cb,
        Err(e) => {
            log::error!("nativeSetCallback: failed to create global ref: {:?}", e);
            return;
        }
    };

    let mut ptr_fn = match JNI_CALLBACK.lock() {
        Ok(guard) => guard,
        Err(e) => {
            log::error!("nativeSetCallback: failed to lock JNI_CALLBACK: {:?}", e);
            return;
        }
    };
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

    let mut ptr_jvm = match JVM_GLOBAL.lock() {
        Ok(guard) => guard,
        Err(e) => {
            log::error!("JNI_OnLoad: failed to lock JVM_GLOBAL: {:?}", e);
            return JNI_ERR;
        }
    };
    *ptr_jvm = Some(java_vm);

    JNI_VERSION_1_6
}

/// dummy function to test the functionality
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_hello(env: JNIEnv, _: JClass) -> jstring {
    // create a string
    match env.new_string(format!("Hello qaul!")) {
        Ok(output) => output.into_raw(),
        Err(e) => {
            log::error!("hello: failed to create java string: {:?}", e);
            std::ptr::null_mut()
        }
    }
}

/// start libqaul from android
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_start(
    mut env: JNIEnv,
    _: JClass,
    path: JString,
) {
    // get path string
    let android_path: String = match env.get_string(&path) {
        Ok(s) => s.into(),
        Err(e) => {
            log::error!("start: failed to get Java path string: {:?}", e);
            return;
        }
    };

    // start libqaul in an own thread
    super::start_android(android_path);
}

/// check if libqaul finished initializing
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_initialized(
    mut _env: JNIEnv,
    _: JClass,
) -> bool {
    let state = super::c::get_c_state();
    state.initialized.load(std::sync::atomic::Ordering::SeqCst)
}

/// get number of messages sent via RPC to libqaul from android
/// this function is only for debugging
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_sendcounter(
    mut _env: JNIEnv,
    _: JClass,
) -> jint {
    let state = super::c::get_c_state();
    crate::rpc::Rpc::send_rpc_count(&*state) as jint
}

/// get number of messages queued to be received by this program
/// from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_receivequeue(
    mut _env: JNIEnv,
    _: JClass,
) -> jint {
    let state = super::c::get_c_state();
    crate::rpc::Rpc::receive_from_libqaul_queue_length(&*state) as jint
}

/// send an rpc message from android to libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_send<'local>(
    env: JNIEnv<'local>,
    _: JClass,
    message: JByteArray<'local>,
) {
    // get the message out of java
    let binary_message: Vec<u8> = match env.convert_byte_array(&message) {
        Ok(msg) => msg,
        Err(e) => {
            log::error!("send: failed to convert byte array: {:?}", e);
            return;
        }
    };

    // send it to libqaul
    let state = super::c::get_c_state();
    crate::rpc::Rpc::send_to_libqaul(&*state, binary_message);
}

/// receive an rpc message on android from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_receive<'local>(
    env: JNIEnv<'local>,
    _: JClass,
) -> JByteArray<'local> {
    let state = super::c::get_c_state();
    // check if there is an RPC message
    if let Ok(message) = crate::rpc::Rpc::receive_from_libqaul(&*state) {
        // convert message to java byte array
        match env.byte_array_from_slice(&message) {
            Ok(byte_array) => return byte_array,
            Err(e) => {
                log::error!("receive: failed to create byte array from message: {:?}", e);
            }
        }
    }

    // there is no message (or conversion failed) and we return an empty array
    let buf: [u8; 0] = [0; 0];
    match env.byte_array_from_slice(&buf) {
        Ok(empty_array) => empty_array,
        Err(e) => {
            log::error!("receive: failed to create empty byte array: {:?}", e);
            JByteArray::default()
        }
    }
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
    let binary_message: Vec<u8> = match env.convert_byte_array(&message) {
        Ok(msg) => msg,
        Err(e) => {
            log::error!("syssend: failed to convert byte array: {:?}", e);
            return;
        }
    };

    // send it to libqaul
    let state = super::c::get_c_state();
    crate::rpc::sys::Sys::send_to_libqaul(&*state, binary_message);
}

/// receive a sys message on android from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_sysreceive<'local>(
    env: JNIEnv<'local>,
    _: JClass,
) -> JByteArray<'local> {
    let state = super::c::get_c_state();
    // check if there is an RPC message
    if let Ok(message) = crate::rpc::sys::Sys::receive_from_libqaul(&*state) {
        // convert message to java byte array
        match env.byte_array_from_slice(&message) {
            Ok(byte_array) => return byte_array,
            Err(e) => {
                log::error!("sysreceive: failed to create byte array from message: {:?}", e);
            }
        }
    }

    // there is no message (or conversion failed) and we return an empty array
    let buf: [u8; 0] = [0; 0];
    match env.byte_array_from_slice(&buf) {
        Ok(empty_array) => empty_array,
        Err(e) => {
            log::error!("sysreceive: failed to create empty byte array: {:?}", e);
            JByteArray::default()
        }
    }
}

pub struct Android {}

impl Android {
    /// send an sys message to Android BLE Module
    /// This function will call the Android BLE Module's "receiveRequest" function.
    pub fn send_to_android(message: Vec<u8>) {
        Self::call_java_callback(message);
    }

    unsafe fn register_natives(jvm: &JavaVM, class_name: &str, methods: &[NativeMethod]) -> jint {
        let mut env: JNIEnv = match jvm.get_env() {
            Ok(e) => e,
            Err(e) => {
                log::error!("register_natives: failed to get JNI env: {:?}", e);
                return JNI_ERR;
            }
        };
        let jni_version = match env.get_version() {
            Ok(v) => v,
            Err(e) => {
                log::error!("register_natives: failed to get JNI version: {:?}", e);
                return JNI_ERR;
            }
        };
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
        let ptr_jvm = match JVM_GLOBAL.lock() {
            Ok(guard) => guard,
            Err(e) => {
                log::error!("call_jvm: failed to lock JVM_GLOBAL: {:?}", e);
                return;
            }
        };
        if (*ptr_jvm).is_none() {
            return;
        }
        let ptr_fn = match callback.lock() {
            Ok(guard) => guard,
            Err(e) => {
                log::error!("call_jvm: failed to lock callback: {:?}", e);
                return;
            }
        };
        if (*ptr_fn).is_none() {
            return;
        }
        // Safe: we checked is_none() above
        let jvm: &JavaVM = (*ptr_jvm).as_ref().unwrap();
        match jvm.attach_current_thread_permanently() {
            Ok(mut env) => {
                // Safe: we checked is_none() above
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
            let jmessage = match env.byte_array_from_slice(fun_type.as_slice()) {
                Ok(arr) => arr,
                Err(e) => {
                    log::error!("call_java_callback: failed to create byte array: {:?}", e);
                    return;
                }
            };
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
