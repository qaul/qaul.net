//! Users API scope
//!
//! This API is responsible for creating, listing and managing local
//! and remote users.

use crate::utils::{self, JavaId, StateWrapped};
use async_std::{
    sync::{Arc, RwLock},
    task::block_on,
};
use jni::{
    objects::{JObject, JString},
    sys::{jlong, jobject},
    JNIEnv,
};

use libqaul::{users::UserAuth, Qaul};
use qaul_chat::Chat;
use qaul_voice::Voice;
use ratman_configure::{EpBuilder, NetBuilder};

/// Lits all local users that are available for login
#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersList(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
) -> jobject {
    let state = StateWrapped::from_ptr(qaul as i64).get_inner();
    (*libqaul::ffi::java::users::list(&env, Arc::clone(&state))).into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersCreate<'env>(
    env: JNIEnv,
    _: JObject,
    qaul: jlong,
    name: JString,
    pw: JString,
) -> jobject {
    let state = StateWrapped::from_ptr(qaul as i64);
    match libqaul::ffi::java::users::create(&env, state.get_inner(), name, pw) {
        Err(e) => {
            error!("Error occured while creating user: {:?}", e);
            *JObject::null()
        }
        Ok(auth) => {
            let id = auth.0;
            state.set_auth(Some(auth));
            *JObject::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_idTest(
    env: JNIEnv,
    _: JObject,
    id: JObject,
) {
    let rust_id = JavaId::from_obj(&env, id);
    
    info!("Successfully extracted ID object: '{}'", rust_id.0);
}

// #[no_mangle]
// pub unsafe extern "C" fn Java_net_qaul_app_ffi_NativeQaul_usersLogin(
//     env: JNIEnv,
//     _this: JObject,
//     qaul: jlong,
//     id: JString,
//     pw: JString,
// ) -> jboolean {
//     info!("Getting login request...");
//     let mut android_state = get_android_state(qaul);
//     let mut state = block_on(async { android_state.write().await });

//     let success = match libqaul::ffi::java::users::login(&env, Arc::clone(&state.libqaul), id, pw) {
//         Ok(auth) => {
//             state.auth = Some(auth);
//             true
//         }
//         Err(_) => false,
//     };

//     drop(state);
//     // Don't drop the outer wrapper
//     std::mem::forget(android_state);

//     // Communicate happiness
//     success as u8
// }
