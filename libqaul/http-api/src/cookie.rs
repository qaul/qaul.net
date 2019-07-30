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
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     let cookie_jar = req.extensions.get::<CookieJar>().unwrap();
/// }
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
