use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::{JObject, JString, JValue},
    JNIEnv,
};
use libqaul::{users::UserAuth, Qaul};
use std::ffi::{CStr, CString};

pub(crate) struct AndroidState {
    libqaul: Arc<Qaul>,
    auth: Option<UserAuth>,
}

/// A wrapped state making sure that auth information is thread-safe
pub(crate) struct StateWrapped(Arc<RwLock<AndroidState>>);

impl StateWrapped {
    pub(crate) fn new(libqaul: Arc<Qaul>) -> Box<Self> {
        Box::new(Self(Arc::new(RwLock::new(AndroidState {
            libqaul,
            auth: None,
        }))))
    }

    /// Dissolve the state box and turn it into a raw pointer
    pub(crate) fn into_ptr(self: Box<Self>) -> i64 {
        Box::into_raw(self) as i64
    }

    pub(crate) unsafe fn from_ptr(ptr: i64) -> Box<Self> {
        Box::from_raw(ptr as *mut Self)
    }

    /// Get the inner state representation from the wrapper
    pub(crate) fn get_inner(self: &Box<Self>) -> Arc<Qaul> {
        block_on(async { Arc::clone(&self.0.read().await.libqaul) })
    }

    /// Get current auth information from the FFI state
    pub(crate) fn get_auth(self: &Box<Self>) -> Option<UserAuth> {
        block_on(async { self.0.read().await.auth.clone() })
    }

    /// Set the auth information
    pub(crate) fn set_auth(self: &Box<Self>, auth: Option<UserAuth>) {
        block_on(async { self.0.write().await.auth = auth });
    }
}

impl Drop for StateWrapped {
    fn drop(&mut self) {
        // We need to forget the state instead of actually cleaning it
        // up!  If we clean it up after a function runs, we will loose
        // data and cause memory-unsafety in the next ffi call
        info!("Forgetting state object - not dropping it!");
        std::mem::forget(self);
    }
}

pub(crate) fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(unsafe { CStr::from_ptr(env.get_string(s).unwrap().as_ptr()) })
        .to_str()
        .unwrap()
        .into()
}

/// Don't call this function when the JValue isn't a string!
fn jvalue_to_jstring(val: JValue) -> JString {
    match val {
        JValue::Object(o) => o.into(),
        _ => unreachable!(), // don't be naughty
    }
}

pub(crate) struct JavaId(pub(crate) String);

impl JavaId {
    pub(crate) fn from_obj(env: &JNIEnv, obj: JObject) -> Self {
        let jval = env.get_field(obj, "inner", "Ljava/lang/String;").unwrap();
        let jstring = jvalue_to_jstring(jval);
        let id = conv_jstring(env, jstring);
        Self(id)
    }
}

/// Setup panic handling and logging for this library
pub(crate) fn init_panic_handling_once() {
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

            let err = format!(
                "### Rust `panic!` hit at file '{}', line {}: `{}`",
                file, line, reason,
            );

            android_tracing::AndroidWriter::log("panic".into(), err, &tracing::Level::ERROR);
        }));
    });
}
