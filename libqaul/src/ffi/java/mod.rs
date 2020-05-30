//! Java JNI wrappers for libqaul
#![allow(non_snake_case)]

use jni::{objects::JString, JNIEnv};
use std::ffi::{CStr, CString};

/// A utility function to convert java strings to Rust
pub(self) fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(unsafe { CStr::from_ptr(env.get_string(s).unwrap().as_ptr()) })
        .to_str()
        .unwrap()
        .into()
}

pub mod users;
