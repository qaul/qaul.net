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

/// An error generated during plugging or unplugging routes
#[derive(Debug)]
pub enum HotPlugError {
    /// Method would have modified a core route
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
pub (crate) struct HotPlugMount {
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
            // TODO: Should NoPath provide a landing page?
            // it's like you'd end up there because you don't know how the
            // api works, we could potentially provide some guidence to the
            // documentation
            HotPlugHandlerError::NoPath => Status::BadRequest,
            HotPlugHandlerError::NoService(s) => Status::NotFound,
            HotPlugHandlerError::ServiceNotAuthorized(_) => Status::Forbidden,
            HotPlugHandlerError::QaulError(_) => Status::InternalServerError,
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

#[cfg(test)]
mod test {
    use anneal::RequestBuilder;
    use iron::{
        headers::{
            Location,
            Referer,
            Server,
        },
        middleware::BeforeMiddleware,
        method::Method,
        modifiers::Header,
    };
    use super::*;

    // says it's own name, like a pokemon
    struct NamedHandler { name: String }

    impl Handler for NamedHandler { 
        fn handle(&self, req: &mut Request) -> IronResult<Response> {
            Ok(Response::with((
                    // "this is blatant misuse of headers to exfiltrate information
                    // why not put it in the body or have it set a field or literally
                    // anything else"
                    //
                    // the only answer is a chilling laugh
                    Header(Referer(self.name.clone())),
                    Header(Server(req.url.clone().to_string())),
                    Header(Location(req.extensions.get::<OriginalUrl>().unwrap().to_string())),
            )))
        }
    }

    // the base set of routes used in these tests
    fn build_mount() -> HotPlugMount {
        let mount = HotPlugMount::new();
        assert!(!mount.mount("a".into(), NamedHandler { name: "a".into() }).unwrap());
        assert!(!mount.mount_core("b".into(), NamedHandler { name: "b".into() }));
        mount
    }

    #[test]
    fn basic_route() {
        let mount = build_mount();

        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/a")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "a");
            });

        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });

        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/c")
            .request(|mut req| {
                assert!(mount.handle(&mut req).is_err());
            });
    }

    #[test]
    fn add_route() {
        let mount = build_mount();

        assert!(mount.mount("a".into(), NamedHandler { name: "c".into() }).unwrap());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/a")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "c");
            });

        assert!(mount.mount("b".into(), NamedHandler { name: "d".into() }).is_err());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });

        assert!(mount.mount_core("b".into(), NamedHandler { name: "e".into() }));
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "e");
            });
    }

    #[test]
    fn unmount() {
        let mount = build_mount();

        assert!(mount.unmount("a").unwrap());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/a")
            .request(|mut req| {
                assert!(mount.handle(&mut req).is_err());
            });

        assert!(mount.unmount("b").is_err());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });

        assert!(mount.unmount_core("b"));
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                assert!(mount.handle(&mut req).is_err());
            });
    }

    // this tests for variants of routes being matched as the actual route
    // if in the future routing is changed to be a little more sophisticated
    // and this test starts failing that's because it relies on the current
    // way routes are handled.
    #[test]
    fn malicious_routes() {
        let mount = build_mount();

        assert!(!mount.mount("/b".into(), NamedHandler { name: "c".into(), }).unwrap());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });

        assert!(!mount.mount("b/".into(), NamedHandler { name: "c".into(), }).unwrap());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });

        assert!(!mount.mount("b/a".into(), NamedHandler { name: "c".into(), }).unwrap());
        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b/a")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
            });
    }

    #[test]
    fn paths() {
        let mount = build_mount();

        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b/a/b/c/d/e?f=g")
            .request(|mut req| {
                let res = mount.handle(&mut req).unwrap();
                assert_eq!(res.headers.get::<Referer>().unwrap().0, "b");
                assert_eq!(res.headers.get::<Server>().unwrap().0, 
                    "http://127.0.0.1:8080/a/b/c/d/e?f=g");
                assert_eq!(res.headers.get::<Location>().unwrap().0, 
                    "http://127.0.0.1:8080/b/a/b/c/d/e?f=g");
            });
    }

    // TODO: test the qaul integrated portion of this
    // ya know
    // once that exists
//    #[test]
//    fn service_auth() {
//        use libqaul::{
//            Qaul,
//            User,
//        };
//
//        let mount = build_mount();
//
//        // you are probably here because you just ran a full test after
//        // updating Qaul to actually do things
//        // at the time this code was written Qaul::start() created an
//        // empty instance for testing
//        // please put it in whatever the appropriate testing mode is
//        let qaul = Qaul::start();
//        let qaul_core = QaulCore::new(&qaul);
//
//        let u = User::new();
//        let user_auth = qaul.user_create("a").unwrap();
//
//        RequestBuilder::new(Method::Get, "http://127.0.0.1:8080/b")
//            .request(|mut req| {
//                req.extensions.insert::<CurrentUser>(user_auth.clone());
//
//                qaul_core.before(&mut req).unwrap();
//                assert!(mount.handle(&mut req).is_err());
//            });
//    }
}
