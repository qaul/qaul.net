//! libqaul users module

use crate::{
    error::Result,
    users::{UserAuth, UserUpdate},
    Qaul,
};
use async_std::task::block_on;
use jni::{
    objects::{JList, JString},
    JNIEnv,
};
use std::sync::Arc;

#[no_mangle]
pub unsafe extern "C" fn create(
    this: &JNIEnv,
    q: Arc<Qaul>,
    name: JString,
    pw: JString,
) -> Result<UserAuth> {
    let name = super::conv_jstring(this, name);
    let pw = super::conv_jstring(this, pw);
    let auth = block_on(async { q.users().create(&pw).await })?;
    block_on(async {
        q.users()
            .update(auth.clone(), UserUpdate::DisplayName(Some(name)))
            .await
    })?;

    Ok(auth)
}

#[no_mangle]
pub unsafe extern "C" fn list<'this>(
    this: &JNIEnv<'this>,
    q: Arc<Qaul>,
) -> Result<JList<'this, 'this>> {
    let users = block_on(async { q.users().list().await });

    unimplemented!()
}
