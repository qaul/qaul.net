use crate::JSONAPI_MIME;
use iron::{
    prelude::*,
    method::Method,
    middleware::BeforeMiddleware,
    status::Status,
};
use json_api::{
    Document,
    Error,
};
use std::{
    error::Error as StdError,
    fmt::{
        Display,
        Formatter,
        Result,
    },
};

#[derive(Debug)]
struct MethodGaurdError {
    expected: Vec<Method>,
    got: Method,
}

impl MethodGaurdError {
    fn into_error(&self) -> (Error, Status) {
        let status = Status::MethodNotAllowed;
        let title = Some("Method Not Allowed".into());

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

        let detail = Some(format!("Request method was {} but endpoint only supports {}", 
            self.got, method_string));

        (
            Error {
                status: Some(format!("{}", status.to_u16())),
                title,
                detail,
                ..Default::default()
            },
            status
        )
    }
}

impl Display for MethodGaurdError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Method Gaurd Error: expected {:?}, got {}", &self.expected[..], self.got)
    }
}

impl StdError for MethodGaurdError {}

impl From<MethodGaurdError> for IronError {
    fn from(e: MethodGaurdError) -> IronError {
        let (err, status) = e.into_error();

        let document = Document {
            errors: Some(vec![err]),
            ..Default::default()
        };

        Self::new(e, (status, serde_json::to_string(&document).unwrap(), JSONAPI_MIME.clone()))
    }
}

/// Aborts requests made with incorrect methods
///
/// Add this to a request chain to deny all requests that do not use the 
/// required methods
pub struct MethodGaurd {
    methods: Vec<Method>,
}

impl MethodGaurd {
    pub fn new(methods: Vec<Method>) -> Self {
        Self { methods }
    }
    
    pub fn options() -> Self {
        Self { methods: vec![Method::Options] }
    }

    pub fn get() -> Self {
        Self { methods: vec![Method::Get] }
    }

    pub fn post() -> Self {
        Self { methods: vec![Method::Post] }
    }

    pub fn put() -> Self {
        Self { methods: vec![Method::Put] }
    }

    pub fn delete() -> Self {
        Self { methods: vec![Method::Delete] }
    }

    pub fn head() -> Self {
        Self { methods: vec![Method::Head] }
    }

    pub fn trace() -> Self {
        Self { methods: vec![Method::Trace] }
    }

    pub fn connect() -> Self {
        Self { methods: vec![Method::Connect] }
    }

    pub fn patch() -> Self {
        Self { methods: vec![Method::Patch] }
    }

}

impl BeforeMiddleware for MethodGaurd {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        if self.methods.iter().fold(false, |c, m| c || *m == req.method) {
            Ok(())
        } else {
            Err(MethodGaurdError{ got: req.method.clone(), expected: self.methods.clone() }.into())
        }
    }
}

#[cfg(test)]
mod test {
    use anneal::RequestBuilder;
    use super::*;

    #[test]
    fn same_method() {
        let gaurds = [
            (
                MethodGaurd::new(vec![Method::Get, Method::Extension("a".into())]), 
                Method::Extension("a".into()),
            ),
            (
                MethodGaurd::new(vec![Method::Get, Method::Extension("a".into())]), 
                Method::Get,
            ),
            (MethodGaurd::options(), Method::Options),
            (MethodGaurd::get(), Method::Get),
            (MethodGaurd::post(), Method::Post),
            (MethodGaurd::put(), Method::Put),
            (MethodGaurd::delete(), Method::Delete),
            (MethodGaurd::head(), Method::Head),
            (MethodGaurd::trace(), Method::Trace),
            (MethodGaurd::connect(), Method::Connect),
            (MethodGaurd::patch(), Method::Patch),
        ];

        for (gaurd, method) in gaurds.iter() {
            RequestBuilder::new(method.clone(), "http://127.0.0.1:8080/")
                .unwrap()
                .request(|mut req| {
                    gaurd.before(&mut req).unwrap();
                });
        }
    }

    #[test]
    fn different_method() {
        let gaurds = [
            (
                MethodGaurd::new(vec![Method::Get, Method::Extension("a".into())]), 
                Method::Options,
            ),
            (MethodGaurd::options(), Method::Get),
            (MethodGaurd::get(), Method::Post),
            (MethodGaurd::post(), Method::Put),
            (MethodGaurd::put(), Method::Delete),
            (MethodGaurd::delete(), Method::Head),
            (MethodGaurd::head(), Method::Trace),
            (MethodGaurd::trace(), Method::Connect),
            (MethodGaurd::connect(), Method::Patch),
            (MethodGaurd::patch(), Method::Extension("a".into())),
        ];

        for (gaurd, method) in gaurds.iter() {
            RequestBuilder::new(method.clone(), "http://127.0.0.1:8080/")
                .unwrap()
                .request(|mut req| {
                    if let Ok(_) = gaurd.before(&mut req) {
                        panic!("Request was successful");
                    }
                });
        }
    }
}
