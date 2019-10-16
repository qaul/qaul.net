//! The routing core module of `RATMAN`

use identity::Identity;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct Core;

#[derive(Clone)]
pub(crate) struct Node {
    id: Identity,
}
