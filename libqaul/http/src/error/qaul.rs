use super::{ApiError, Error};
use iron::{status::Status, IronError};
use libqaul::error::Error as QaulInternalError;

/// Wrapper for internal qaul errors
///
/// You can provide optional context for the cause of the error
#[derive(Debug, Clone)]
pub struct QaulError {
    err: QaulInternalError,
    context: Option<String>,
}

impl From<QaulError> for IronError {
    fn from(e: QaulError) -> Self {
        ApiError::from(e).into()
    }
}

impl From<QaulInternalError> for QaulError {
    fn from(e: QaulInternalError) -> Self {
        Self {
            err: e,
            context: None,
        }
    }
}

impl QaulError {
    fn with_context(e: QaulInternalError, context: String) -> QaulError {
        Self {
            err: e,
            context: Some(context),
        }
    }
}

impl Error for QaulError {
    fn title(&self) -> String {
        match self.err {
            QaulInternalError::NotAuthorised => "Not Authorised",
            QaulInternalError::NoUser => "Unknown User",
            QaulInternalError::InvalidQuery => "Invalid Query",
            QaulInternalError::InvalidPayload => "Invalid Payload",
            QaulInternalError::CallbackTimeout => "Callback Timeout",
            _ => "Unknown error!",
        }
        .into()
    }

    fn detail(&self) -> Option<String> {
        let reason = match self.err {
            QaulInternalError::NotAuthorised => {
                "The authenticated user is not authorised to perform the requested action"
            }
            QaulInternalError::NoUser => "The authenticated user is not known to libqaul",
            QaulInternalError::InvalidQuery => "The query sent to libqaul was invalid",
            QaulInternalError::InvalidPayload => "The payload of the request was invalid",
            QaulInternalError::CallbackTimeout => "An internal callback timed out",
            _ => "Unknown error!",
        };

        Some(if let Some(context) = &self.context {
            format!("{} at {}", reason, context)
        } else {
            reason.into()
        })
    }

    fn status(&self) -> Status {
        match self.err {
            QaulInternalError::NoUser => Status::InternalServerError,
            _ => Status::BadRequest,
        }
    }
}
