use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv,
};
use libqaul::{users::UserAuth, Identity, Qaul};
use qaul_chat::Chat;
use qaul_voice::Voice;
use std::{
    ffi::{CStr, CString},
    ops::Deref,
};

pub(crate) struct AndroidState {
    libqaul: Arc<QaulWrapped>,
    auth: Option<UserAuth>,
}

pub(crate) struct QaulWrapped(Arc<Qaul>, pub(crate) Arc<Chat>, pub(crate) Arc<Voice>);

impl Drop for QaulWrapped {
    fn drop(&mut self) {
        debug!("Calling drop() on QaulWrapped: running std::mem::forget(...)");
        std::mem::forget(&self.0);
    }
}

impl QaulWrapped {
    pub(crate) fn qaul(&self) -> Arc<Qaul> {
        Arc::clone(&self.0)
    }

    pub(crate) fn chat(&self) -> Arc<Chat> {
        Arc::clone(&self.1)
    }

    pub(crate) fn voice(&self) -> Arc<Voice> {
        Arc::clone(&self.2)
    }
}

type StateWrapped = Arc<RwLock<AndroidState>>;

/// A state wrapper to provide thread and ffi-memory safety
pub(crate) struct GcWrapped(StateWrapped);

impl Drop for GcWrapped {
    fn drop(&mut self) {
        // We forget the wrapped state here instead of dropping it to
        // keep it valid for future Java FFI calls.
        debug!("Calling drop() on GcWrapped: running std::mem::forget(...)");
        std::mem::forget(&self.0);
    }
}

impl GcWrapped {
    pub(crate) fn new(libqaul: Arc<Qaul>, chat: Arc<Chat>, voice: Arc<Voice>) -> Self {
        Self(Arc::new(RwLock::new(AndroidState {
            libqaul: Arc::new(QaulWrapped(libqaul, chat, voice)),
            auth: None,
        })))
    }

    /// Dissolve the state box and turn it into a raw pointer
    pub(crate) fn into_ptr(self) -> i64 {
        debug!("Turning state into pointer...");
        Arc::into_raw(Arc::clone(&self.0)) as i64
    }

    pub(crate) unsafe fn from_ptr(ptr: i64) -> Self {
        debug!("Trying to get state object from pointer...");
        Self(Arc::from_raw(ptr as *const RwLock<AndroidState>))
    }

    /// Get the inner state representation from the wrapper
    pub(crate) fn get_inner(&self) -> Arc<QaulWrapped> {
        block_on(async { Arc::clone(&self.0.read().await.libqaul) })
    }

    /// Get current auth information from the FFI state
    pub(crate) fn get_auth(&self) -> Option<UserAuth> {
        block_on(async { self.0.read().await.auth.clone() })
    }

    /// Set the auth information
    pub(crate) fn set_auth(&self, auth: Option<UserAuth>) {
        block_on(async { self.0.write().await.auth = auth });
    }
}

#[deprecated]
pub(crate) fn conv_jstring(env: &JNIEnv, s: JString) -> String {
    CString::from(unsafe { CStr::from_ptr(env.get_string(s).unwrap().as_ptr()) })
        .to_str()
        .unwrap()
        .into()
}

pub(crate) fn maybe_conv_jstring(env: &JNIEnv, s: JString) -> Option<String> {
    match env.get_string(s) {
        Ok(ref s) => Some(
            CString::from(unsafe { CStr::from_ptr(s.as_ptr()) })
                .to_str()
                .unwrap()
                .into(),
        ),
        _ => None,
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
