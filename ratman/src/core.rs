//! The routing core module of `RATMAN`

use identity::Identity;
use std::collections::HashMap;

pub struct RoutingCore {}

struct Node {
    id: Identity,
}
