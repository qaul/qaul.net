//! Java JNI wrappers for libqaul
#![allow(non_snake_case)]

use crate::users::UserProfile;
use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv,
};
use std::ffi::{CStr, CString};

/// A utility function to convert java strings to Rust
pub(self) fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(unsafe { CStr::from_ptr(env.get_string(s).unwrap().as_ptr()) })
        .to_str()
        .unwrap()
        .into()
}

/// Create a jstring from an optional Rust string
///
/// `None` is mapped to `null`
pub(crate) fn to_jstring<'env>(env: &'env JNIEnv, s: Option<String>) -> JString<'env> {
    match s {
        Some(s) => env.new_string(s).unwrap(),
        None => JObject::null().into(),
    }
}

pub mod users;

pub(self) trait ToJObject {
    fn to_jobject<'env>(self, env: &'env JNIEnv) -> JObject<'env>;
}

impl ToJObject for UserProfile {
    fn to_jobject<'env>(self, env: &'env JNIEnv) -> JObject<'env> {
        let id = to_jstring(env, Some(self.id.to_string()));
        let display_name = to_jstring(env, self.display_name);
        let real_name = to_jstring(env, self.real_name);

        let class: JClass<'env> = env
            .find_class("net/qaul/app/ffi/models/UserProfile")
            .unwrap();

        env.new_object(
            class,
            "(L/lang/java/String;L/lang/java/String;L/lang/java/String;)V",
            &[
                JValue::Object(*id),
                JValue::Object(*display_name),
                JValue::Object(*real_name),
            ],
        )
        .unwrap()
    }
}
