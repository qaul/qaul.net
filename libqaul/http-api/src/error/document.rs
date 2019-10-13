use iron::{
    status::Status,
    IronError,
};
use japi::ObjectConversionError;
use super::{ApiError, Error};

#[derive(Debug)]
pub enum DocumentError {
    /// The endpoint supports a single primary data when multiple were provided
    MultipleData,
    /// The endpoint requires primary data
    NoData,
    /// A japi conversion error
    ConversionError{ err: ObjectConversionError, pointer: Option<String> },
    /// The endpoint expected an attribute where none was provided
    NoAttributes { pointer: String },
}

impl DocumentError {
    pub fn no_attributes(pointer: String) -> DocumentError {
        DocumentError::NoAttributes { pointer }
    }

    pub fn conversion_error(e: ObjectConversionError, pointer: String) -> DocumentError {
        DocumentError::ConversionError { err: e, pointer: Some(pointer) }
    }
}

impl From<ObjectConversionError> for DocumentError {
    fn from(e: ObjectConversionError) -> Self {
        DocumentError::ConversionError{ err: e, pointer: None }
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
            DocumentError::MultipleData => "Multiple Data",
            DocumentError::NoData => "No Data",
            DocumentError::ConversionError { err: _, pointer: _ } => "Object Error",
            DocumentError::NoAttributes { pointer: _ } => "Missing Attributes",
        }.into()
    }
    
    fn detail(&self) -> Option<String> {
        Some(match self {
            DocumentError::MultipleData => 
                "The document contains multiple primary data when the endpoint expects only one".into(),
            DocumentError::NoData => "The document contains no primary data".into(),
            DocumentError::ConversionError {
                    err: ObjectConversionError::ImproperType{ expected, got },
                    pointer: _,
                } => 
                format!("Primary data should be of type {} but is of type {} instead", expected, got),
            DocumentError::ConversionError { 
                    err: ObjectConversionError::FailedDeserialization(e),
                    pointer: _,
                } =>
                format!("Failed to deserialize attributes of primary data: {}", e),
            DocumentError::NoAttributes { pointer: _ } => "Primary data is missing attributes".into(),
        })
    }

    fn pointer(&self) -> Option<String> {
        match self {
            DocumentError::MultipleData => Some("/data".into()),
            DocumentError::NoData => Some("/data".into()),
            DocumentError::ConversionError { err: _, pointer: p } => p.clone(),
            DocumentError::NoAttributes { pointer } => Some(pointer.into()),
        }
    }
}
