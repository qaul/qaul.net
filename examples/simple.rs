//! A small example showing how to setup an Alexandria library

use alexandria::namespace::Address;
use alexandria::{
    data::{Data, Value},
    keys::KeyAttr,
    scope::ScopeAttr,
    *,
};

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

    a.insert(
        Address::root("test", "foo"),
        Data::KV(
            vec![("name".into(), Value::String("Alice".into()))]
                .into_iter()
                .collect(),
        ),
    );
    
    a.insert(
        Address::root("test", "bar"),
        Data::Blob([0; 32*1024].into_iter().map(|i| *i as u8).collect())
    );
    
    dbg!(a).sync();
}
