//! User storage

use identity::Identity;
use rand::prelude::*;
use std::collections::{BTreeMap, BTreeSet};

mod store;
mod profile;
mod contacts;

pub use profile::*;
