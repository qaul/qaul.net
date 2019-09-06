//! A small example showing how to setup an Alexandria library

use alexandria::{keys::KeyAttr, scope::ScopeAttr, *};

fn main() {
    let mut a = Alexandria::new();
    a.create_path(
        "lib:/test".into(),
        ScopeAttr {
            ns_auth: false,
            encryption: KeyAttr::Off,
            offset: "/home/test".into(),
        },
    );

    dbg!(a).sync();
}
