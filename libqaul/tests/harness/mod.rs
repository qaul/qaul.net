//! A libqaul specific harness for arbitrary API types

use std::sync::Arc;
use libqaul::Qaul;
use ratman_harness::{temp, Initialize, ThreePoint};

pub async fn init() -> ThreePoint<Arc<Qaul>> {
    let mut tp = ThreePoint::new().await;
    tp.init_with(|_, arc| Qaul::new(arc, temp().path()));
    tp
}
