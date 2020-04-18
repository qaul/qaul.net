//! libqaul specific storage wrappers
//!
//! The database and storage library in use is called alexandria,
//! which encodes data as diffs.  This module is repsonsible for
//! making Diffs and records map onto each other.  Each operation on a
//! type yields in a diff that the storage system can then apply, and
//! reading a data type from a record.

mod users;
pub(crate) use users::LocalUser;

use crate::Identity;
use alexandria::record::kv::Value;
use std::collections::{BTreeMap, BTreeSet};

struct Conv;

impl Conv {
    pub(self) fn id(v: &Value) -> Identity {
        match v {
            Value::Vec(vec) => Identity::from_bytes(vec),
            v => panic!("Invalid conversion: {:?} -> Id", v),
        }
    }

    pub(self) fn string(v: &Value) -> String {
        match v {
            Value::String(s) => s.clone(),
            v => panic!("Invalid conversion: {:?} -> String", v),
        }
    }

    pub(self) fn map(v: &Value) -> BTreeMap<String, String> {
        match v {
            Value::Map(map) => map
                .iter()
                .map(|(k, v)| match v {
                    Value::String(s) => (k.clone(), s.clone()),
                    v => panic!("Invalid map-inner conversion: {:?} -> String", v),
                })
                .collect(),
            v => panic!("Invalid conversion: {:?} -> BTreeMap<String, String>", v),
        }
    }

    pub(self) fn set(v: &Value) -> BTreeSet<String> {
        match v {
            Value::List(list) => list
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    v => panic!("Invalid set-inner conversion: {:?} -> String", v),
                })
                .collect(),
            v => panic!("Invalid conversion: {:?} -> BTreeSet<String>", v),
        }
    }

    pub(self) fn binvec(v: &Value) -> Vec<u8> {
        match v {
            Value::Vec(v) => v.clone(),
            v => panic!("Invalid conversion: {:?} -> Vec<u8>", v),
        }
    }
}
