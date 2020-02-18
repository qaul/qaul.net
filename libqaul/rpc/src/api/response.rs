use {
    serde::{Serialize, Deserialize},
    super::request::Request,
};

/// A wrapped response object carrying with it additional
/// information to allow correlating the response with a sent
/// request.
///
/// If the client provided a transaction id with the request
/// the respose will carry that same id. If the client did not
/// the response will carry the request sent by the client. As such
/// it is **highly** recommended clients include a transaction id *even*
/// if they don't care about correlation.
#[derive(Serialize, Deserialize)]
pub struct TransactionResponse {
    #[serde(default)]
    pub transaction_id: Option<String>,
    #[serde(default)]
    pub request: Option<Request>,
    pub response: Response,
}

/// A struct holding the required context to construct a `TransactionResponse`
/// with a minimum of copying
pub struct ResponseContext {
    pub transaction_id: Option<String>,
    pub request: Option<Request>,
}

impl ResponseContext {
    /// Create a transaction with this context and the given response
    pub fn with_response(self, response: Response) -> TransactionResponse {
        TransactionResponse {
            transaction_id: self.transaction_id,
            request: self.request,
            response
        }
    }
}

/// In some systems all responses are channeled over a single pipe. In
/// such systems, this object is provided to contain all possible responses.
#[derive(Serialize, Deserialize)]
pub enum Response {}
