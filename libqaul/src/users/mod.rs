//! User storage

use identity::Identity;
use rand::prelude::*;
use std::collections::BTreeMap;

/// User context
pub(crate) struct User {
    pub(crate) id: Identity,
    pub(crate) display_name: Option<String>,
    pub(crate) real_name: Option<String>,
    pub(crate) bio: BTreeMap<String, String>,
    pub(crate) services: Vec<String>,
    pub(crate) avatar: Option<Vec<u8>>,
}

impl User {
    pub(crate) fn new() -> Self {
        let mut rng = rand::thread_rng();
        let buf: [u8; 12] = rng.gen();
        Self {
            id: buf.into(),
            display_name: None,
            real_name: None,
            bio: Default::default(),
            services: vec![],
            avatar: None,
        }
    }
}

