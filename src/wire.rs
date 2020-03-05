//! Wire serialiser formats

use bincode::{self, Result};
use serde::{de::DeserializeOwned, Serialize};

/// A generic trait for anything that can be serialised
pub(crate) trait Encoder<T> {
    fn encode(&self) -> Result<Vec<u8>>;
    fn decode(data: &Vec<u8>) -> Result<T>;
}

// Blanket impl for anything than can be `Encoder<T>`
impl<T> Encoder<T> for T
where
    T: Serialize + DeserializeOwned,
{
    fn encode(&self) -> Result<Vec<u8>> {
        bincode::serialize(self)
    }

    fn decode(data: &Vec<u8>) -> Result<T> {
        bincode::deserialize(data)
    }
}

#[test]
fn encode_simple() {
    use {identity::Identity as Id, serde::Deserialize};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestStruct {
        id: Id,
    }

    let t = TestStruct { id: Id::random() };

    let enc = t.encode().unwrap();
    let dec = TestStruct::decode(&enc).unwrap();

    assert_eq!(dec, t);
}

#[test]
fn encode_skip() {
    use std::cell::Cell;
    use {identity::Identity as Id, serde::Deserialize};

    #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct TestStruct {
        #[serde(skip)]
        _dont: Option<Cell<*const usize>>,
        id: Id,
    }

    let t = TestStruct {
        _dont: Some(Cell::new(0 as *const usize)), // NullPtr
        id: Id::random(),
    };

    let enc = t.encode().unwrap();
    let dec = TestStruct::decode(&enc).unwrap();

    assert_eq!(dec.id, t.id);
}
