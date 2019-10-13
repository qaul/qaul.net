//! General utility module

use rand::prelude::*;

pub(crate) fn random(len: usize) -> Vec<u8> {
    (0..)
        .map(|_| rand::thread_rng().next_u64())
        .take(len)
        .map(|x| x.to_be_bytes())
        .fold(Vec::new(), |mut acc, arr| {
            acc.extend(arr.iter().cloned());
            acc
        })
}

