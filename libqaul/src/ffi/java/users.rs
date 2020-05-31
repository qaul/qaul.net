//! libqaul users module

use super::ToJObject;
use crate::{
    error::Result,
    users::{UserAuth, UserUpdate},
    Identity, Qaul,
};

use async_std::task::block_on;
use jni::{
    objects::{JClass, JList, JObject, JString},
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
pub unsafe extern "C" fn login(
    env: &JNIEnv,
    q: Arc<Qaul>,
    id: JString,
    pw: JString,
) -> Result<UserAuth> {
    let id = Identity::from_string(&super::conv_jstring(env, id));
    let pw = super::conv_jstring(env, pw);
    block_on(async { q.users().login(id, &pw).await })
}

pub fn list<'env>(env: &'env JNIEnv<'env>, q: Arc<Qaul>) -> JList<'env, 'env> {
    let users = block_on(async { q.users().list().await });
    let class = env.find_class("java/util/ArrayList").unwrap();

    let arraylist = env.new_object(class, "()V", &[]).unwrap();
    let list = JList::from_env(env, arraylist).unwrap();

    users
        .into_iter()
        .map(|user| user.to_jobject(&env))
        .fold(list, |list, jobj| {
            list.add(jobj);
            list
        })
}
