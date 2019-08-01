use crate::{
    JSONAPI_MIME,
    models::ConversionError,
};
use libqaul::QaulError;
use identity::ID_LEN;
use iron::{
    IronError,
    status::Status,
};
use json_api::{
    Document,
    Error,
    ErrorSource,
    ObjectConversionError,
};
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub (crate) enum AuthError {
    MultipleData,
    NoData,
    ConversionError(ObjectConversionError),
    NoAttributes,
    InvalidIdentity(ConversionError),
    QaulError(QaulError),
    NotLoggedIn,
    InvalidToken,
}

impl AuthError {
    fn detail(&self) -> String {
        match self {
            AuthError::MultipleData => 
                "Multiple data were provided when the endpoint expects exactly one".into(),
            AuthError::NoData => "Document contains no data".into(),
            AuthError::ConversionError(e) => format!("Error converting generic object ({})", e),
            AuthError::NoAttributes => "Object has no attributes".into(),
            AuthError::InvalidIdentity(e) => format!("Conversion Error ({})", e), 
            AuthError::QaulError(e) => format!("Qaul Error ({:?})", e),
            AuthError::NotLoggedIn => "Not logged in".into(),
            AuthError::InvalidToken => "Invalid token".into(),
        }
    }

    fn into_error(&self) -> (Error, Status) {
        let status = match self {
            AuthError::QaulError(QaulError::NotAuthorised) => Status::Unauthorized,
            AuthError::QaulError(QaulError::UnknownUser) => Status::NotFound,
            AuthError::QaulError(QaulError::CallbackTimeout) => Status::InternalServerError,
            AuthError::NotLoggedIn => Status::Unauthorized,
            _ => Status::BadRequest,
        };

        let title = match self {
            AuthError::MultipleData => Some("Multiple Data".into()),
            AuthError::NoData => Some("No Data".into()),
            AuthError::ConversionError(_) => Some("Object Error".into()),
            AuthError::NoAttributes => Some("No Attributes".into()),
            AuthError::InvalidIdentity(_) => Some("Invalid identity".into()),
            AuthError::QaulError(QaulError::NotAuthorised) => Some("Not Authorized".into()),
            AuthError::QaulError(QaulError::UnknownUser) => Some("Unknown User".into()),
            AuthError::QaulError(QaulError::InvalidQuery) => Some("Invalid Query".into()),
            AuthError::QaulError(QaulError::InvalidPayload) => Some("Invalid Payload".into()),
            AuthError::QaulError(QaulError::CallbackTimeout) => None,
            AuthError::NotLoggedIn => Some("Not Logged In".into()),
            AuthError::InvalidToken => Some("Invalid Token".into()),
        };

        let detail = match self {
            AuthError::ConversionError(ObjectConversionError::ImproperType{ expected, got }) => 
                Some(format!("Primary data should be of type {} but is of type {} instead", 
                             expected, got)),
            AuthError::ConversionError(ObjectConversionError::FailedDeserialization(e)) =>
                Some(format!("Failed to deserialize attributes of primary data: {}", e)),
            AuthError::InvalidIdentity(ConversionError::Base64Decode(e)) => 
                Some(format!("Failed to decode identity, base 64 invalid: {}", e)),
            AuthError::InvalidIdentity(ConversionError::BadIdLength(l)) =>
                Some(format!("Failed to decode identity, decoded identity is {} bytes long when it should be {}", l, ID_LEN)),
            AuthError::QaulError(QaulError::NotAuthorised) => 
                Some("Current user is not authorised to perform this action".into()),
            AuthError::QaulError(QaulError::UnknownUser) => 
                Some("Target user is not known to Qaul".into()),
            AuthError::QaulError(QaulError::InvalidQuery) => None, 
            AuthError::QaulError(QaulError::InvalidPayload) => 
                Some("Most likely the payload is too large".into()),
            AuthError::QaulError(QaulError::CallbackTimeout) => None,
            AuthError::InvalidToken => 
                Some("The login token provided with your request is either no longer or never was valid".into()),
            _ => Some(self.detail()),
        };

        let pointer = match self {
            AuthError::MultipleData => Some("/data".into()),
            AuthError::NoData => Some("/".into()),
            AuthError::ConversionError(ObjectConversionError::ImproperType{ expected: _, got: _ }) =>
                Some("/data/type".into()),
            AuthError::ConversionError(ObjectConversionError::FailedDeserialization(_)) => 
                Some("/data/attributes".into()),
            AuthError::NoAttributes => Some("/data".into()),
            AuthError::InvalidIdentity(_) => Some("/data/id".into()),
            AuthError::QaulError(_) => None,
            AuthError::NotLoggedIn => None,
            AuthError::InvalidToken => None,
        };

        (
            Error {
                status: Some(format!("{}", status.to_u16())),
                title,
                detail,
                source: pointer.map(|p| ErrorSource { pointer: Some(p), ..Default::default() }),
                ..Default::default()
            },
            status
        )
    }
}

impl StdError for AuthError {}

impl Display for AuthError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Auth Error: {}", self.detail())
    }
}

impl From<AuthError> for IronError {
    fn from(e: AuthError) -> IronError {
        let (err, status) = e.into_error();

        let document = Document { 
            errors: Some(vec![err]),
            ..Default::default()
        };

        Self::new(e, (status, serde_json::to_string(&document).unwrap(), JSONAPI_MIME.clone()))
    }
}
