use super::{ApiError, Error};
use iron::{
    IronError,
    status::Status,
};

/// Errors relating to the currently authorized user
#[derive(Debug)]
pub enum AuthError {
    NotLoggedIn,
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
        }.into()
    }

    fn status(&self) -> Status {
        match self {
            AuthError::NotLoggedIn => Status::Unauthorized,
        }
    }

    fn detail(&self) -> Option<String> {
        Some(match self {
            AuthError::NotLoggedIn => "There is no user logged in",
        }.into())
    }
}
