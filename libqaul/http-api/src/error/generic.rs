use super::{ApiError, Error};
use iron::{status::Status, IronError};
use japi::Meta;

#[derive(Debug, Clone)]
pub struct GenericError {
    /// The title of the error
    ///
    /// This should not change between occurances
    title: String,

    /// A URL pointing to information about this error
    about: Option<String>,

    /// An application-specific error code
    code: Option<String>,

    /// Detailed information about this error
    detail: Option<String>,

    /// A unique identifier for this particular occurance of the problem
    id: Option<String>,

    /// The status HTTP status code applicable to this error
    ///
    /// Defaults to 400 (Bad Request)
    status: Status,

    /// Indicates which URI query parameter caused the error
    parameter: Option<String>,

    /// A JSON Pointer to the associated entity in the request document
    pointer: Option<String>,

    /// A meta object containing non-standard meta-information about the error
    meta: Option<Meta>,
}

impl GenericError {
    pub fn new(title: String) -> GenericError {
        GenericError {
            title,
            about: None,
            code: None,
            detail: None,
            id: None,
            status: Status::BadRequest,
            parameter: None,
            pointer: None,
            meta: None,
        }
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn about(mut self, about: String) -> Self {
        self.about = Some(about);
        self
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    pub fn detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn parameter(mut self, parameter: String) -> Self {
        self.parameter = Some(parameter);
        self
    }

    pub fn pointer(mut self, pointer: String) -> Self {
        self.pointer = Some(pointer);
        self
    }

    pub fn meta(mut self, meta: Meta) -> Self {
        self.meta = Some(meta);
        self
    }


    pub fn from_err<E: Error>(e: E) -> GenericError {
        GenericError {
            title: e.title(),
            about: e.about(),
            code: e.code(),
            detail: e.detail(),
            id: e.id(),
            status: e.status(),
            parameter: e.parameter(),
            pointer: e.pointer(),
            meta: e.meta(),
        }
    }
}

impl From<GenericError> for IronError {
    fn from(e: GenericError) -> IronError {
        ApiError::from(e).into()
    }
}

impl Error for GenericError {
    fn title(&self) -> String {
        self.title.clone()
    }
    fn about(&self) -> Option<String> {
        self.about.clone()
    }
    fn code(&self) -> Option<String> {
        self.code.clone()
    }
    fn detail(&self) -> Option<String> {
        self.detail.clone()
    }
    fn id(&self) -> Option<String> {
        self.id.clone()
    }
    fn status(&self) -> Status {
        self.status.clone()
    }
    fn parameter(&self) -> Option<String> {
        self.parameter.clone()
    }
    fn pointer(&self) -> Option<String> {
        self.pointer.clone()
    }
    fn meta(&self) -> Option<Meta> {
        self.meta.clone()
    }
}
