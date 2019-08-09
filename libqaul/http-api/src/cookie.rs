use cookie::{CookieJar, Cookie, ParseError}; 
use crate::JSONAPI_MIME;
use iron::{
    headers::{
        Cookie as CookieHeader,
        SetCookie,
    },
    prelude::*,
    middleware::{
        BeforeMiddleware,
        AfterMiddleware,
    },
    typemap::Key,
    status::Status,
};
use json_api::{
    Document,
    Error,
};
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

#[derive(Debug)]
struct CookieJarError(ParseError); 

impl CookieJarError {
    fn detail(&self) -> String {
        format!("Error parsing cookies ({})", self.0)
    }

    fn into_error(&self) -> (Error, Status) {
        let status = Status::BadRequest;
        let title = Some("Cookie Parse Error".into());
        let detail = Some(self.detail());

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

impl Display for CookieJarError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Cookie jar error: {}", self.detail())
    }
}

impl StdError for CookieJarError {}

impl From<CookieJarError> for IronError {
    fn from(e: CookieJarError) -> Self {
        let (err, status) = e.into_error();

        let document = Document {
            errors: Some(vec![err]),
            ..Default::default()
        };

        Self::new(e, (status, serde_json::to_string(&document).unwrap(), JSONAPI_MIME.clone()))
    }
}

impl From<ParseError> for CookieJarError {
    fn from(e: ParseError) -> Self {
        CookieJarError(e)
    }
}

/// User this key to get the CookieJar for the request
/// 
/// Any changes performed on the cookie jar will be sent back to the 
/// client in the response
///
/// ```
/// # use iron::prelude::*;
/// # use qaul_http::Cookies;
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     let cookie_jar = req.extensions.get::<Cookies>().unwrap();
///
///     // ...
/// # Ok(Response::with(""))
/// # }
/// ```
pub struct Cookies;

impl Key for Cookies { type Value = CookieJar; }

pub (crate) struct CookieManager;

impl CookieManager {
    fn build_jar(req: &mut Request) -> Result<CookieJar, CookieJarError> {
        let mut jar = CookieJar::new();

        if let Some(cookies) = req.headers.get::<CookieHeader>() {
            for cookie in cookies.iter() {
                jar.add_original(Cookie::parse(cookie)?.into_owned());
            }
        }

        Ok(jar)
    }

    pub fn new() -> (Self, Self) { (Self, Self) }
}

impl BeforeMiddleware for CookieManager {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let jar = Self::build_jar(req)?;
        req.extensions.insert::<Cookies>(jar);
        Ok(())
    }
}

impl AfterMiddleware for CookieManager {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let cookies : Vec<String> = req.extensions.get::<Cookies>().unwrap()
            .delta()
            .map(|c| c.to_string())
            .collect();

        if cookies.len() != 0 { res.headers.set::<SetCookie>(SetCookie(cookies)); }

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use anneal::RequestBuilder; 
    use iron::method::Method;
    use super::*;

    #[test]
    fn no_cookies() {
        RequestBuilder::new(Method::Get, "https://127.0.0.1:8080/")
            .unwrap()
            .request(|mut req| {
                CookieManager.before(&mut req).unwrap();
                assert_eq!(req.extensions.get::<Cookies>().unwrap().iter().count(), 0);
                let res = CookieManager.after(&mut req, Response::with("")).unwrap();
                assert!(!res.headers.has::<SetCookie>());
            });
    }

    #[test]
    fn valid_cookies() {
        RequestBuilder::new(Method::Get, "https://127.0.0.1:8080/")
            .unwrap()
            .set_header(CookieHeader(vec!["a=b".into(), "c=d".into()]))
            .request(|mut req| {
                CookieManager.before(&mut req).unwrap();

                assert_eq!(req.extensions.get::<Cookies>().unwrap().iter().count(), 2);
                let mut cookies = req.extensions.get_mut::<Cookies>().unwrap();
                assert_eq!(cookies.get("a").unwrap().value(), "b");
                assert_eq!(cookies.get("c").unwrap().value(), "d");

                cookies.add(Cookie::new("e", "f"));
                let res = CookieManager.after(&mut req, Response::with("")).unwrap();
                assert_eq!(*res.headers.get::<SetCookie>().unwrap(), 
                           SetCookie(vec!["e=f".into()]));
            })
    }

    #[test]
    fn invalid_cookies() {
        RequestBuilder::new(Method::Get, "https://127.0.0.1:8080/")
            .unwrap()
            .set_header(CookieHeader(vec!["a".into()]))
            .request(|mut req| {
                let err = match CookieManager.before(&mut req) {
                    Ok(_) => panic!("Request completed successfully"),
                    Err(e) => e.error,
                };
                assert_eq!(err.to_string(), 
                    "Cookie jar error: Error parsing cookies (the cookie is missing a name/value pair)");
            });
    }
}
