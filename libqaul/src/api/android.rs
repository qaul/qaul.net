#![cfg(target_os = "android")]
#![allow(non_snake_case)]

use crate::api;
use jni::objects::{JClass, JString, JByteBuffer};
use jni::sys::{jstring, jbyteArray};
use jni::JNIEnv;
use std::ffi::CString;

// NOTE: LibqaulKt references the name libqaul.kt, which is the kotlin file exposing the functions.

/// dummy function to test the functionality
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_dummy(
  env: JNIEnv,
  _: JClass,
) -> jstring {
    // create a string
    let output = env
        .new_string(format!("Hello qaul!"))
        .expect("Couldn't create java string!");
    
    // return the raw pointer
    output.into_inner()
}

/// start libqaul from android
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_start(
  env: JNIEnv,
  _: JClass,
) {
    // start libqaul in an own thread
    super::start();
}

/// send an rpc message from android to libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_send(
    env: JNIEnv,
    _: JClass,
    message: jbyteArray,
) {
    // get the message out of java
    let binary_message: Vec<u8> = env.convert_byte_array(message).unwrap();

    // send it to libqaul
    super::send_rpc(binary_message);
}

/// receive an rpc message on android from libqaul
#[no_mangle]
pub extern "system" fn Java_net_qaul_libqaul_LibqaulKt_receive(
    env: JNIEnv,
    _: JClass,
) -> jbyteArray {
    // check if there is an RPC message
    if let Ok(message) = super::receive_rpc() {
        // convert message to java byte array
        let byte_array = env.byte_array_from_slice(&message).unwrap();
        // return byte array
        return byte_array
    }
    
    // there is no message and we return an empty array
    let buf: [u8; 0] = [0; 0];
    let empty_array = env.byte_array_from_slice(&buf).unwrap();
    empty_array
}
