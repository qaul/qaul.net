use super::{ApiError, Error};
use iron::{status::Status, IronError};

#[derive(Debug, Clone)]
pub struct ServiceError {
    kind: ServiceErrorKind,
    service: String,
}

#[derive(Debug, Clone)]
pub enum ServiceErrorKind {
    NotMounted
}

impl ServiceError {
    pub fn not_mounted(service: String) -> ServiceError {
        ServiceError {
            kind: ServiceErrorKind::NotMounted,
            service,
        }
    }
}

impl From<ServiceError> for IronError {
    fn from(e: ServiceError) -> IronError {
        ApiError::from(e).into()
    }
}

impl Error for ServiceError {
    fn title(&self) -> String {
        match self.kind {
            ServiceErrorKind::NotMounted => "Service Not Mounted",
        }
        .into()
    }

    fn status(&self) -> Status {
        match self.kind {
            ServiceErrorKind::NotMounted => Status::InternalServerError,
        }
    }

    fn detail(&self) -> Option<String> {
        Some(
            match self.kind {
                ServiceErrorKind::NotMounted => {
                    format!("Middleware for service {} is not mounted", self.service)
                }
            }
        )
    }
}
