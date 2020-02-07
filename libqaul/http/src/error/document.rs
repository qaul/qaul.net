use super::{ApiError, Error};
use iron::{status::Status, IronError};
use japi::ObjectConversionError;

#[derive(Debug)]
pub enum DocumentError {
    /// There is no document in the request
    NoDocument,
    /// The endpoint supports a single primary data when multiple were provided
    MultipleData,
    /// The endpoint requires primary data
    NoData,
    /// The data has no id
    NoId { pointer: Option<String> },
    /// The data has the wrong type
    WrongType {
        expected: Vec<String>,
        got: String,
        pointer: Option<String>,
    },
    /// A japi conversion error
    ConversionError {
        err: ObjectConversionError,
        pointer: Option<String>,
    },
    /// The endpoint expected an attribute where none was provided
    NoAttributes { pointer: Option<String> },
    /// The endpoint expected an attribute where none was provided
    NoAttribute {
        attr: String,
        pointer: Option<String>,
    },
    /// The endpoint expected there to be a relationships key in the document
    NoRelationships { pointer: Option<String> },
    /// The named relationship was missing from the document
    NoRelationship {
        rel: String,
        pointer: Option<String>,
    },
    /// The field was expected to contain a singular item but in fact contained multiple
    ManyItems { pointer: Option<String>, },
    NullItem { pointer: Option<String>, },
}

impl DocumentError {
    pub fn conversion_error(e: ObjectConversionError, pointer: String) -> DocumentError {
        DocumentError::ConversionError {
            err: e,
            pointer: Some(pointer),
        }
    }

    pub fn no_attributes(pointer: String) -> DocumentError {
        DocumentError::NoAttributes {
            pointer: Some(pointer),
        }
    }

    pub fn no_attribute(attr: String, pointer: String) -> DocumentError {
        DocumentError::NoAttribute {
            attr,
            pointer: Some(pointer),
        }
    }

    pub fn no_relationships(pointer: String) -> DocumentError {
        DocumentError::NoRelationships {
            pointer: Some(pointer),
        }
    }

    pub fn no_relationship(key: String, pointer: String) -> DocumentError {
        DocumentError::NoRelationship {
            rel: key,
            pointer: Some(pointer),
        }
    }

    pub fn wrong_type(expected: Vec<String>, got: String, pointer: String) -> DocumentError {
        DocumentError::WrongType {
            expected,
            got,
            pointer: Some(pointer),
        }
    }

    pub fn many_items(pointer: String) -> DocumentError {
        DocumentError::ManyItems {
            pointer: Some(pointer),
        }
    }

    pub fn null_item(pointer: String) -> DocumentError {
        DocumentError::NullItem {
            pointer: Some(pointer),
        }
    }
}

impl From<ObjectConversionError> for DocumentError {
    fn from(e: ObjectConversionError) -> Self {
        match &e {
            ObjectConversionError::MissingId => DocumentError::NoId { pointer: None },
            _ => DocumentError::ConversionError {
                err: e,
                pointer: None,
            },
        }
    }
}

impl From<DocumentError> for IronError {
    fn from(e: DocumentError) -> Self {
        ApiError::from(e).into()
    }
}

impl Error for DocumentError {
    fn title(&self) -> String {
        match self {
            DocumentError::NoDocument => "No Document",
            DocumentError::MultipleData => "Multiple Data",
            DocumentError::NoData => "No Data",
            DocumentError::NoId { pointer: _ } => "No Id",
            DocumentError::WrongType {
                expected: _,
                got: _,
                pointer: _,
            } => "Wrong Type",
            DocumentError::ConversionError { err: _, pointer: _ } => "Object Error",
            DocumentError::NoAttributes { pointer: _ } => "Missing Attributes",
            DocumentError::NoAttribute {
                attr: _,
                pointer: _,
            } => "Missing Attribute",
            DocumentError::NoRelationships { pointer: _ } => "Missing Relationships",
            DocumentError::NoRelationship { rel: _, pointer: _ } => "Missing Relationship",
            DocumentError::ManyItems { pointer: _ } => "Many Items",
            DocumentError::NullItem { pointer: _ } => "Null Item",
        }
        .into()
    }

    fn detail(&self) -> Option<String> {
        Some(match self {
            DocumentError::NoDocument => "The request does not contain a document".into(),
            DocumentError::MultipleData => {
                "The document contains multiple primary data when the endpoint expects only one"
                    .into()
            }
            DocumentError::NoData => "The document contains no primary data".into(),
            DocumentError::NoId { pointer: _ } => "The object does not have an id".into(),
            DocumentError::WrongType {
                expected,
                got,
                pointer: _,
            } => format!(
                "Object is of the wrong type (expected {}, got {})",
                {
                    let mut s = String::new();
                    for (i, e) in expected.iter().enumerate() {
                        if i != 0 {
                            s.push_str(", ");
                        }
                        s.push_str(e);
                    }
                    s
                },
                got
            ),
            DocumentError::ConversionError {
                err: ObjectConversionError::ImproperType { expected, got },
                pointer: _,
            } => format!(
                "Data should be of type {} but is of type {} instead",
                expected, got
            ),
            DocumentError::ConversionError {
                err: ObjectConversionError::FailedDeserialization(e),
                pointer: _,
            } => format!("Failed to deserialize attributes of primary data: {}", e),
            DocumentError::NoAttributes { pointer: _ } => "Data is missing attributes".into(),
            DocumentError::NoAttribute { attr, pointer: _ } => {
                format!("Data is missing attribute {}", attr)
            }
            DocumentError::NoRelationships { pointer: _ } => "Data is missing relationships".into(),
            DocumentError::NoRelationship { rel, pointer: _ } => {
                format!("Data is missing relationship {}", rel)
            }
            DocumentError::ConversionError {
                err: ObjectConversionError::MissingId,
                pointer: _,
            } => panic!("No id"),
            DocumentError::ManyItems { pointer: _ } => {
                "Many items were provided when one was expected".into()
            },
            DocumentError::NullItem { pointer: _ } => {
                "An item was null when it was expected to have a value".into()
            },
        })
    }

    fn pointer(&self) -> Option<String> {
        match self {
            DocumentError::NoDocument => None,
            DocumentError::MultipleData => Some("/data".into()),
            DocumentError::NoData => Some("/data".into()),
            DocumentError::NoId { pointer } => pointer.clone(),
            DocumentError::WrongType {
                expected: _,
                got: _,
                pointer,
            } => pointer.clone(),
            DocumentError::ConversionError { err: _, pointer } => pointer.clone(),
            DocumentError::NoAttributes { pointer } => pointer.clone(),
            DocumentError::NoAttribute { attr: _, pointer } => pointer.clone(),
            DocumentError::NoRelationships { pointer } => pointer.clone(),
            DocumentError::NoRelationship { rel: _, pointer } => pointer.clone(),
            DocumentError::ManyItems { pointer } => pointer.clone(),
            DocumentError::NullItem { pointer } => pointer.clone(),
        }
    }
}
