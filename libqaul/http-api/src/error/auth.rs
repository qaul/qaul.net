use super::{ApiError, Error};
use iron::{status::Status, IronError};

/// Errors relating to the currently authorized user
#[derive(Debug, Clone)]
pub enum AuthError {
    NotLoggedIn,
    NotAuthorised,
}

impl From<AuthError> for IronError {
    fn from(e: AuthError) -> IronError {
        ApiError::from(e).into()
    }
}

impl Error for AuthError {
    fn title(&self) -> String {
        match self {
            AuthError::NotLoggedIn => "Not Logged In",
            AuthError::NotAuthorised => "Not Authorised",
        }
        .into()
    }

    fn status(&self) -> Status {
        match self {
            _ => Status::Unauthorized,
        }
    }

    fn detail(&self) -> Option<String> {
        Some(
            match self {
                AuthError::NotLoggedIn => "There is no user logged in",
                AuthError::NotAuthorised => {
                    "The current user is not authorised to perform the requested action"
                }
            }
            .into(),
        )
    }
}
