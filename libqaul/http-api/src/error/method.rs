use super::{ApiError, Error};
use iron::{method::Method, status::Status, IronError};

/// Errors related to the http method of the request
#[derive(Debug, Clone)]
pub struct MethodError {
    /// The set of methods the endpoint supports
    pub expected: Vec<Method>,
    /// The method the request was sent with
    pub got: Method,
}

impl From<MethodError> for IronError {
    fn from(e: MethodError) -> Self {
        ApiError::from(e).into()
    }
}

impl Error for MethodError {
    fn title(&self) -> String {
        "Method Not Allowed".into()
    }

    fn status(&self) -> Status {
        Status::MethodNotAllowed
    }

    fn detail(&self) -> Option<String> {
        let mut method_string = String::new();
        for (i, method) in self.expected.iter().enumerate() {
            if i != 0 {
                method_string.push(',');
                method_string.push(' ');
            }

            method_string.push_str(match method {
                Method::Options => "Options",
                Method::Get => "Get",
                Method::Post => "Post",
                Method::Put => "Put",
                Method::Delete => "Delete",
                Method::Head => "Head",
                Method::Trace => "Trace",
                Method::Connect => "Connect",
                Method::Patch => "Patch",
                Method::Extension(s) => &s,
            });
        }

        Some(format!(
            "Request method was {} but endpoint only supports {}",
            self.got, method_string
        ))
    }
}
