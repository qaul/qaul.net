//! Java JNI wrappers for libqaul
#![allow(non_snake_case)]

use crate::{users::UserProfile, Identity};
use jni::{
    objects::{JClass, JObject, JString, JValue},
    sys::jboolean,
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

pub trait ToJObject {
    fn to_jobject<'env>(self, env: &'env JNIEnv) -> JObject<'env>;
}

impl ToJObject for UserProfile {
    fn to_jobject<'env>(self, env: &'env JNIEnv) -> JObject<'env> {
        //let id = to_jstring(env, Some(self.id.to_string()));
        let id = JavaId::from_identity(self.id);
        let display_name = to_jstring(env, self.display_name);
        let real_name = to_jstring(env, self.real_name);

        let class: JClass<'env> = env
            .find_class("net/qaul/app/ffi/models/UserProfile")
            .unwrap();

        env.new_object(
            class,
            "(Lnet/qaul/app/ffi/models/Id;Ljava/lang/String;Ljava/lang/String;Z)V",
            &[
                JValue::Object(id.into_obj(env)),
                JValue::Object(*display_name),
                JValue::Object(*real_name),
                JValue::Bool(false as jboolean),
            ],
        )
        .unwrap()
    }
}

pub(crate) fn into_jstring<'a>(env: &'a JNIEnv, s: String) -> JString<'a> {
    env.new_string(s).unwrap()
}

/// Don't call this function when the JValue isn't a string!
fn jvalue_to_jstring(val: JValue) -> JString {
    match val {
        JValue::Object(o) => o.into(),
        _ => unreachable!(), // don't be naughty
    }
}

// FIXME: dedup with the android-support impl
pub(crate) struct JavaId(pub(crate) String);

impl JavaId {
    pub(crate) fn from_obj(env: &JNIEnv, obj: JObject) -> Self {
        let jval = env.get_field(obj, "inner", "Ljava/lang/String;").unwrap();
        let jstring = jvalue_to_jstring(jval);
        let id = conv_jstring(env, jstring);
        Self(id)
    }

    pub(crate) fn into_obj<'a>(self, env: &'a JNIEnv) -> JObject<'a> {
        let class: JClass<'a> = env.find_class("net/qaul/app/ffi/models/Id").unwrap();

        env.new_object(
            class,
            "(Ljava/lang/String;)V",
            &[JValue::Object(into_jstring(env, self.0).into())],
        )
        .unwrap()
    }

    pub(crate) fn from_identity(id: Identity) -> Self {
        Self(id.to_string())
    }

    pub(crate) fn into_identity(self) -> Identity {
        Identity::from_string(&self.0)
    }
}
