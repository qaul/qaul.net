use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

/// A generic metadata tag
///
/// Because searching through message or file payloads might be slow,
/// and I/O intensive (especially within thi secret storage module),
/// all public types have a tag metadata interface.  These are
/// included in the wire-format, meaning that they will get
/// transferred across to another node.
///
/// This can be used to implement things like conversation ID's,
/// In-Reply-To, and more.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Tag {
    /// A string key for a tag
    pub key: String,
    /// Some binary data that is up to a service to interpret
    pub val: Vec<u8>,
}

impl Tag {
    /// Create a new MsgTag with key and value
    pub fn new<K, I>(key: K, val: I) -> Self
    where
        K: Into<String>,
        I: IntoIterator<Item = u8>,
    {
        Self {
            key: key.into(),
            val: val.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
struct HumanVec(Vec<u8>);

impl Serialize for HumanVec {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if dbg!(ser.is_human_readable()) {
            ser.serialize_str(
                &hex::encode_upper(&self.0)
                    .as_bytes()
                    .chunks(4)
                    .map(std::str::from_utf8)
                    .collect::<Result<String, _>>()
                    .unwrap(),
            )
        } else {
            ser.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for HumanVec {
    fn deserialize<D>(der: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HumanVecVis;

        impl HumanVecVis {
            fn from_str<E: Error>(s: &str) -> Result<HumanVec, E> {
                Self::from_bytes(&hex::decode(s).map_err(|e| E::custom(e))?)
            }

            fn from_bytes<E: Error, V: AsRef<[u8]>>(v: V) -> Result<HumanVec, E> {
                let v = v.as_ref();
                Ok(HumanVec(v.iter().cloned().collect()))
            }
        }

        impl<'de> Visitor<'de> for HumanVecVis {
            type Value = HumanVec;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "A byte array or a hex string encoded byte array",)
            }

            fn visit_borrowed_str<E: Error>(self, v: &'de str) -> Result<Self::Value, E> {
                Self::from_str(v)
            }

            fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
                Self::from_str(&v)
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'de [u8]) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_byte_buf<E: Error>(self, v: Vec<u8>) -> Result<Self::Value, E> {
                Self::from_bytes(v)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut v = Vec::new();
                while let Some(b) = seq.next_element::<u8>()? {
                    v.push(b);
                }

                Self::from_bytes(v)
            }
        }

        if der.is_human_readable() {
            der.deserialize_str(HumanVecVis)
        } else {
            der.deserialize_bytes(HumanVecVis)
        }
    }
}

impl Serialize for Tag {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        dbg!();
        let mut state = ser.serialize_struct("Tag", 2)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("val", &HumanVec(self.val.clone()))?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for Tag {
    fn deserialize<D>(der: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Responsible for deserialising hex-encoded payloads
        ///
        /// This visitor is called when the deserialiser is working
        /// for a human readable format, such as json.
        struct TagVisitor;

        impl<'de> Visitor<'de> for TagVisitor {
            type Value = Tag;

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let key: String = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;

                let hvec: HumanVec = seq
                    .next_element()?
                    .ok_or_else(|| Error::invalid_length(0, &self))?;
                let val: Vec<u8> = hvec.0;

                Ok(Tag { key, val })
            }

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                fmt.write_str("struct Tag { key, val }")
            }
        }

        der.deserialize_struct("Tag", &["key", "val"], TagVisitor)
    }
}

#[test]
fn serialize_tag_json() {
    let t = Tag {
        key: "blorp".into(),
        val: vec![172, 171],
    };

    use serde_json;
    let json = serde_json::to_string(&t).unwrap();
    assert_eq!(json.as_str(), r#"{"key":"blorp","val":"ACAB"}"#);
}

#[test]
fn serialize_tag_bincode() {
    let t = Tag {
        key: "blorp".into(),
        val: vec![172, 171],
    };

    use bincode;
    let bc = bincode::serialize(&t).unwrap();
    assert_eq!(
        bc.as_slice(),
        &[5, 0, 0, 0, 0, 0, 0, 0, 98, 108, 111, 114, 112, 2, 0, 0, 0, 0, 0, 0, 0, 172, 171]
    );
}

#[test]
fn deserialize_tag_json() {
    let json = r#"{"key":"blorp","val":"ACAB"}"#;

    use serde_json;
    let t: Tag = serde_json::from_str(&json).unwrap();

    assert_eq!(
        t,
        Tag {
            key: "blorp".into(),
            val: vec![172, 171],
        }
    );
}

#[test]
fn deserialize_tag_bincode() {
    let bin = [
        5, 0, 0, 0, 0, 0, 0, 0, 98, 108, 111, 114, 112, 2, 0, 0, 0, 0, 0, 0, 0, 172, 171,
    ];

    use bincode;
    let t: Tag = bincode::deserialize(&bin).unwrap();

    assert_eq!(
        t,
        Tag {
            key: "blorp".into(),
            val: vec![172, 171],
        }
    );
}
