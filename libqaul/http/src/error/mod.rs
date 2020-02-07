mod auth;
pub(crate) use auth::AuthError;

mod method;
pub(crate) use method::MethodError;

mod jsonapi;
pub(crate) use jsonapi::JsonApiError;

mod qaul;
pub(crate) use qaul::QaulError;

mod document;
pub(crate) use document::DocumentError;

mod generic;
pub(crate) use generic::GenericError;

mod service;
pub(crate) use service::ServiceError;

use crate::JSONAPI_MIME;
use iron::{status::Status, IronError};
use japi::{Document, Error as JError, ErrorSource, Link, Links, Meta};
use serde_json;
use std::{
    error::Error as StdError,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

pub trait Error: Debug + Send {
    /// The title of the error
    ///
    /// This should not change between occurances
    fn title(&self) -> String;

    /// A URL pointing to information about this error
    fn about(&self) -> Option<String> {
        None
    }

    /// An application-specific error code
    fn code(&self) -> Option<String> {
        None
    }

    /// Detailed information about this error
    fn detail(&self) -> Option<String> {
        None
    }

    /// A unique identifier for this particular occurance of the problem
    fn id(&self) -> Option<String> {
        None
    }

    /// The status HTTP status code applicable to this error
    ///
    /// Defaults to 400 (Bad Request)
    fn status(&self) -> Status {
        Status::BadRequest
    }

    /// Indicates which URI query parameter caused the error
    fn parameter(&self) -> Option<String> {
        None
    }

    /// A JSON Pointer to the associated entity in the request document
    fn pointer(&self) -> Option<String> {
        None
    }

    /// A meta object containing non-standard meta-information about the error
    fn meta(&self) -> Option<Meta> {
        None
    }
}

#[derive(Debug)]
pub struct ApiError(Box<Error>);

impl<E: Error + 'static> From<E> for ApiError {
    fn from(e: E) -> Self {
        Self(Box::new(e))
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if let Some(detail) = self.0.detail() {
            write!(f, "{}: {}", self.0.title(), detail)
        } else {
            write!(f, "{}", self.0.title())
        }
    }
}

impl StdError for ApiError {}
impl From<ApiError> for IronError {
    fn from(e: ApiError) -> Self {
        let status = e.0.status();

        let document = Document {
            errors: Some(vec![(&e).into()]),
            ..Default::default()
        };

        IronError::new(Box::new(e),
            (status, serde_json::to_string(&document).unwrap(), JSONAPI_MIME.clone()))
    }
}

impl From<&ApiError> for JError {
    fn from(e: &ApiError) -> Self {
        let pointer = e.0.pointer();
        let parameter = e.0.parameter();
        let source = if pointer.is_some() || parameter.is_some() {
            Some(ErrorSource { pointer, parameter })
        } else {
            None
        };

        let links = e.0.about().map(|about| {
            let mut links = Links::new();
            links.insert("about".into(), Link::Url(about));
            links
        });

        let status = e.0.status();

        JError {
            id: e.0.id(),
           links,
            status: Some(format!("{}", status.to_u16())),
            code: e.0.code(),
            title: Some(e.0.title()),
            detail: e.0.detail(),
            source,
            meta: e.0.meta(),
        }
    }
}

