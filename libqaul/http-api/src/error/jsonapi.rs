use super::{ApiError, Error};
use iron::{status::Status, IronError};
use serde_json::Error as SerdeError;
use std::io::Error as IoError;

/// Errors relating to requests to JSON:API endpoints
#[derive(Debug)]
pub enum JsonApiError {
    MediaTypeParameters,
    NoAcceptableType,
    SerdeError(SerdeError),
    IoError(IoError),
}

impl From<JsonApiError> for IronError {
    fn from(e: JsonApiError) -> Self {
        ApiError::from(e).into()
    }
}

impl Error for JsonApiError {
    fn title(&self) -> String {
        match self {
            JsonApiError::MediaTypeParameters => "Invalid Media Type Parameters",
            JsonApiError::NoAcceptableType => "No Acceptable Type",
            JsonApiError::SerdeError(_) => "Deserialization Error",
            JsonApiError::IoError(_) => "Internal Error",
        }
        .into()
    }

    fn about(&self) -> Option<String> {
        match self {
            JsonApiError::MediaTypeParameters => {
                Some("https://jsonapi.org/format/#content-negotiation-servers".into())
            }
            JsonApiError::NoAcceptableType => {
                Some("https://jsonapi.org/format/#content-negotiation-servers".into())
            }
            _ => None,
        }
    }

    fn status(&self) -> Status {
        match self {
            JsonApiError::MediaTypeParameters => Status::UnsupportedMediaType,
            JsonApiError::NoAcceptableType => Status::NotAcceptable,
            JsonApiError::SerdeError(_) => Status::BadRequest,
            JsonApiError::IoError(_) => Status::InternalServerError,
        }
    }

    fn detail(&self) -> Option<String> {
        match self {
            JsonApiError::MediaTypeParameters => Some("Content type had media type parameters in violation of https://jsonapi.org/format/#content-negotiation-servers".into()),
            JsonApiError::NoAcceptableType => Some("Accept header had JSON:API media type but all instances included parameters in violation of https://jsonapi.org/format/#content-negotiation-servers".into()),
            JsonApiError::SerdeError(e) => Some(format!("Error deserializing document ({})", e)),
            JsonApiError::IoError(_) => None,
        }
    }
}
