//! A small example showing how to setup an Alexandria library

use alexandria::{Address, Alexandria, Data, Delta, KeyAttr, ScopeAttr, Value};

fn main() {
    let mut a = Alexandria::new();
    a.modify_path(
        Address::scope(None, "test"),
        Delta::Insert(ScopeAttr {
            ns_auth: false,
            encryption: KeyAttr::Off,
            offset: "/home/test".into(),
        }),
    );

    // a.insert(
    //     Address::root("test", "foo"),
    //     Data::KV(
    //         vec![("name".into(), Value::String("Alice".into()))]
    //             .into_iter()
    //             .collect(),
    //     ),
    // );

    a.insert(
        Address::root("test", "bar"),
        Data::Blob([0; 32 * 1024].into_iter().map(|i| *i as u8).collect()),
    );

    dbg!(a).sync();
}
