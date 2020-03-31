use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};

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
#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd, Deserialize)]
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

impl Serialize for Tag {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let human = ser.is_human_readable();
        let mut state = ser.serialize_struct("Tag", 2)?;
        state.serialize_field("key", &self.key)?;

        if human {
            state.serialize_field(
                "val",
                &hex::encode_upper(&self.val)
                    .as_bytes()
                    .chunks(4)
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<_>, _>>()
                    .unwrap()
                    .join(" "),
            )?;
        } else {
            state.serialize_field("key", &self.val)?;
        }

        state.end()
    }
}
