//! Common utilities module

use heapless::{consts::*, Vec as Hec};
use serde::Serialize;

pub(crate) fn serialise<S>(s: &S) -> Vec<u8>
where
    S: Serialize,
{
    let v: Hec<u8, U11> = postcard::to_vec(s).unwrap();
    v.to_vec()
}
