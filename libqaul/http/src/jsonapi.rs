use crate::{
    error::{DocumentError, JsonApiError},
    JSONAPI_MIME,
};
use iron::{
    error::IronError,
    headers::{Accept, ContentType, QualityItem},
    mime::{Mime, SubLevel, TopLevel},
    modifiers::Header,
    prelude::*,
    status::Status,
    typemap, BeforeMiddleware,
};
use japi::{Document, Error, Link, Links, OptionalVec};
use serde_json;
use std::io::Read;

/// Use this key to get the request's `Document`
///
/// Will only decode documents when the `Content-Type` is
/// `application/vnd.api+json`. Also checks the headers as required by the
/// [JSON:API docs](https://jsonapi.org/format/#content-negotiation-clients).
/// If the `Content-Type` header indicates that a JSON:API document is present
/// and any of the header checks fail or the document fails to parse, an error
/// will be returned to the client and processing of the message will be aborted.
///
///
/// ```
/// # use iron::prelude::*;
/// # use libqaul_http::JsonApi;
/// fn handler(req: &mut Request) -> IronResult<Response> {
///     // Some(Document) if there was a document in the request
///     // None otherwise
///     let document = req.extensions.get::<JsonApi>();
///
///     // ...
/// # Ok(Response::with(""))
/// # }
/// ```
pub struct JsonApi;

impl typemap::Key for JsonApi {
    type Value = Document;
}

impl BeforeMiddleware for JsonApi {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let target_sublevel = SubLevel::Ext("vnd.api+json".into());

        // this block does two things:
        // firstly we skip any requests that don't have the Content-Type: application/vnd.api+json
        // header as they are not JSON:API requests
        // secondly we error on any requests that contain media type parameters as required by the
        // spec
        match req.headers.get::<ContentType>() {
            Some(ContentType(Mime(TopLevel::Application, sublevel, params)))
                if *sublevel == target_sublevel =>
            {
                if params.len() > 0 {
                    return Err(JsonApiError::MediaTypeParameters.into());
                }
            }
            _ => {
                return Ok(());
            }
        }

        // next up, we check the accept header
        // we have to error if it contains the JSON:API media type and all instanced of that media
        // type are modified with media type parameters
        if let Some(Accept(v)) = req.headers.get::<Accept>() {
            let mut json_api_type = false;
            let mut with_no_params = false;
            for mime in v {
                match mime {
                    QualityItem {
                        item: Mime(TopLevel::Application, _, params),
                        quality: _,
                    } => {
                        json_api_type = true;
                        if params.len() == 0 {
                            with_no_params = true;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if json_api_type && !with_no_params {
                return Err(JsonApiError::NoAcceptableType.into());
            }
        }

        // due to ownership requirements we read the body to an intermediate buffer
        // if this fails i'm honestly not sure it's recoverable but we'll return 500 INTERNAL
        // SERVER ERROR
        let mut buff = Vec::new();
        if let Err(e) = req.body.read_to_end(&mut buff) {
            return Err(JsonApiError::IoError(e).into());
        }

        // if the body was empty we shouldn't try to parse a document
        if buff.len() == 0 {
            return Ok(());
        }

        // now we try to parse the body to see if it contains a valid JSON:API request
        // if it doesn't we'll return 400 BAD REQUEST
        let doc: Document = match serde_json::from_slice(&buff) {
            Ok(d) => d,
            Err(e) => {
                return Err(JsonApiError::SerdeError(e).into());
            }
        };

        // after all that we put the document into the extensions map for some handler futher down
        // the chain to deal with
        req.extensions.insert::<Self>(doc);

        Ok(())
    }
}

/// Gaurds endpoints that only accept JSON:API requests
///
/// If a request passes through this middleware that does not
/// contain a valid JSON:API document, the request will be aborted and
/// an error will be returned to the client
pub struct JsonApiGaurd;
impl BeforeMiddleware for JsonApiGaurd {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        match req.extensions.get::<JsonApi>() {
            Some(_) => Ok(()),
            None => Err(DocumentError::NoDocument.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anneal::RequestBuilder;
    use iron::{
        headers::{qitem, Accept},
        mime,
    };
    use japi::{Identifier, OptionalVec};

    fn invalid_media_type() -> mime::Mime<Vec<(mime::Attr, mime::Value)>> {
        mime::Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Ext(String::from("vnd.api+json")),
            vec![(mime::Attr::Charset, mime::Value::Utf8)],
        )
    }

    // tests a completely valid request
    // this also tests that the gaurd passes successfully
    #[test]
    fn successful() {
        let doc = Document {
            data: OptionalVec::One(Some(Identifier::new("a".into(), "b".into()).into())),
            ..Default::default()
        };
        RequestBuilder::default_post()
            .set_document(&doc)
            .request(|mut req| {
                assert!(JsonApi.before(&mut req).is_ok());
                assert_eq!(*req.extensions.get::<JsonApi>().unwrap(), doc);
                assert!(JsonApiGaurd.before(&mut req).is_ok());
            });
    }

    // test that it doesn't process other content types
    // this also tests that the gaurd doesn't pass when there's no document
    #[test]
    fn skips_other() {
        let data = "abcdef";
        RequestBuilder::default_post()
            .set_string(data)
            .request(|mut req| {
                assert!(JsonApi.before(&mut req).is_ok());
                assert!(req.extensions.get::<JsonApi>().is_none());
                assert!(JsonApiGaurd.before(&mut req).is_err());

                let mut buff = String::new();
                req.body.read_to_string(&mut buff).unwrap();
                assert_eq!(buff, data);
            });
    }

    // test that it rejects requests with incorrect media types
    #[test]
    fn rejects_media_type() {
        RequestBuilder::default_post()
            .set_header(ContentType(invalid_media_type()))
            .request(|mut req| {
                assert!(JsonApi.before(&mut req).is_err());
            });
    }

    // test that it handles accept headers properly
    // this will test first that it fails when all are modified with parameters
    // and then that it doesn't fail if at least one isn't
    #[test]
    fn accept_headers() {
        let doc = Document {
            data: OptionalVec::One(Some(Identifier::new("a".into(), "b".into()).into())),
            ..Default::default()
        };

        let mut accept = vec![qitem(invalid_media_type())];

        let mut rb = RequestBuilder::default_post();
        rb.set_document(&doc);
        rb.set_header(Accept(accept.clone()));

        // should fail due to all acceptable types having media type parameters
        rb.request(|mut req| {
            assert!(JsonApi.before(&mut req).is_err());
        });

        accept.push(qitem(JSONAPI_MIME.clone()));
        rb.set_header(Accept(accept));
        rb.request(|mut req| {
            assert!(JsonApi.before(&mut req).is_ok());
            assert_eq!(*req.extensions.get::<JsonApi>().unwrap(), doc);
        });
    }

    // test to see if it handles a bad payload
    #[test]
    fn bad_payload() {
        RequestBuilder::default_post()
            .set_string("{\"data\": 1}".into())
            .set_header(ContentType(JSONAPI_MIME.clone()))
            .request(|mut req| {
                assert!(JsonApi.before(&mut req).is_err());
            });
    }
}
