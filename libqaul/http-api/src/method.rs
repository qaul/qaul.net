use crate::error::MethodError;
use iron::{method::Method, middleware::BeforeMiddleware, prelude::*};

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
        Self {
            methods: vec![Method::Options],
        }
    }

    pub fn get() -> Self {
        Self {
            methods: vec![Method::Get],
        }
    }

    pub fn post() -> Self {
        Self {
            methods: vec![Method::Post],
        }
    }

    pub fn put() -> Self {
        Self {
            methods: vec![Method::Put],
        }
    }

    pub fn delete() -> Self {
        Self {
            methods: vec![Method::Delete],
        }
    }

    pub fn head() -> Self {
        Self {
            methods: vec![Method::Head],
        }
    }

    pub fn trace() -> Self {
        Self {
            methods: vec![Method::Trace],
        }
    }

    pub fn connect() -> Self {
        Self {
            methods: vec![Method::Connect],
        }
    }

    pub fn patch() -> Self {
        Self {
            methods: vec![Method::Patch],
        }
    }
}

impl BeforeMiddleware for MethodGaurd {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        if self
            .methods
            .iter()
            .fold(false, |c, m| c || *m == req.method)
        {
            Ok(())
        } else {
            Err(MethodError {
                got: req.method.clone(),
                expected: self.methods.clone(),
            }
            .into())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anneal::RequestBuilder;

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
