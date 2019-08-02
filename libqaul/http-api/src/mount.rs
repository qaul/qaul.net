use crate::{
    CurrentUser,
    JSONAPI_MIME,
    QaulCore,
};
use libqaul::QaulError;
use iron::{
    middleware::Handler,
    prelude::*,
    Url,
    typemap,
    status::Status,
};
use json_api::{
    Document,
    Error,
    ErrorSource,
};
use std::{
    collections::BTreeMap,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub enum HotPlugError {
    CoreRoute,
}

impl Display for HotPlugError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Hot Plug Error: ")?;
        match self {
            HotPlugError::CoreRoute => write!(f, "Tried to replace a core route"),
        }
    }
}

impl StdError for HotPlugError {}

#[derive(Clone)]
enum Route {
    Service(Arc<Box<dyn Handler>>),
    Core(Arc<Box<dyn Handler>>),
}

#[derive(Clone)]
pub struct HotPlugMount {
    routes: Arc<Mutex<BTreeMap<String, Route>>>,
}

impl HotPlugMount {
    pub fn new() -> Self {
        Self { routes: Default::default(), } 
    }

    pub fn mount<T: Handler>(&self, path: String, handler: T) -> Result<bool, HotPlugError> {
        let mut routes = self.routes.lock().unwrap();

        if let Some(Route::Core(_)) = routes.get(&path) {
            return Err(HotPlugError::CoreRoute);
        }

        Ok(routes.insert(path, Route::Service(Arc::new(Box::new(handler)))).is_some())
    }

    pub fn unmount(&self, path: &str) -> Result<bool, HotPlugError> {
        let mut routes = self.routes.lock().unwrap();

        if let Some(Route::Core(_)) = routes.get(path) {
            return Err(HotPlugError::CoreRoute);
        }

        Ok(routes.remove(path).is_some())
    }

    pub fn mount_core<T: Handler>(&self, path: String, handler: T) -> bool {
        let mut routes = self.routes.lock().unwrap();

        routes.insert(path, Route::Core(Arc::new(Box::new(handler)))).is_some()
    }

    pub fn unmount_core(&self, path: &str) -> bool {
        let mut routes = self.routes.lock().unwrap();

        routes.remove(path).is_some()
    }
}

pub struct OriginalUrl;
impl typemap::Key for OriginalUrl { type Value = Url; }

#[derive(Debug)]
enum HotPlugHandlerError {
    NoPath,
    NoService(String),
    ServiceNotAuthorized(String),
    QaulError(QaulError),
}

impl HotPlugHandlerError {
    fn detail(&self) -> String {
        match self {
            HotPlugHandlerError::NoPath => "Url provided no path to a service".into(),
            HotPlugHandlerError::NoService(s) => format!("No mounted service named {}", s),
            HotPlugHandlerError::ServiceNotAuthorized(s) =>
                format!("Current user has not authorized service {}", s),
            HotPlugHandlerError::QaulError(e) => format!("Qaul Error: {:?}", e),
        }
    }

    fn into_error(&self) -> (Error, Status) {
        let status = match self {
            HotPlugHandlerError::ServiceNotAuthorized(_) => Status::Forbidden,
            HotPlugHandlerError::QaulError(_) => Status::InternalServerError,
            _ => Status::BadRequest,
        };

        let title = match self {
            HotPlugHandlerError::NoPath => Some("No Path".into()),
            HotPlugHandlerError::NoService(_) => Some("No Service".into()),
            HotPlugHandlerError::ServiceNotAuthorized(_) => Some("Service Not Authorized".into()),
            HotPlugHandlerError::QaulError(_) => None,
        };

        let detail = match self {
            HotPlugHandlerError::QaulError(_) => None,
            _ => Some(self.detail()),
        };

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

impl StdError for HotPlugHandlerError {}

impl Display for HotPlugHandlerError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Hot Plug Hander World: {}", self.detail())
    }
}

impl From<HotPlugHandlerError> for IronError {
    fn from(e: HotPlugHandlerError) -> Self {
        let (err, status) = e.into_error();

        let document = Document {
            errors: Some(vec![err]),
            ..Default::default()
        };

        Self::new(e, (status, serde_json::to_string(&document).unwrap(), JSONAPI_MIME.clone()))
    }
}

impl Handler for HotPlugMount {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut path = req.url.path();

        // is there a service name in the path?
        let service = match path.first() {
            Some(p) => p.to_string(),
            None => { return Err(HotPlugHandlerError::NoPath.into()); },
        };

        // if a user is logged in, do they have the service enabled?
        match req.extensions.get::<CurrentUser>()
                .map(|user| req.extensions.get::<QaulCore>().unwrap().user_get(user.clone())) {
            Some(Ok(user)) => {
                if !user.data.services.iter().fold(false, |c, s| c || *s == service) {
                    return Err(HotPlugHandlerError::ServiceNotAuthorized(service).into());
                }
            },
            Some(Err(e)) => {
                return Err(HotPlugHandlerError::QaulError(e).into());
            },
            None => {},
        }

        // does the path point to a real handler? 
        let handler = {
            match self.routes.lock().unwrap().get(&service) {
                Some(Route::Core(h)) => h.clone(),
                Some(Route::Service(h)) => h.clone(),
                None => { return Err(HotPlugHandlerError::NoService(service).into()); },
            }
        };

        // stash the original url
        req.extensions.insert::<OriginalUrl>(req.url.clone());

        // remove the prefix from the url
        let path = path[1..].join("/");
        req.url.as_mut().set_path(&path);


        handler.handle(req)
    }
}
