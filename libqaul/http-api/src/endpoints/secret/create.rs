use crate::{
    error::{DocumentError, QaulError},
    models::Secret,
    JsonApi, QaulCore, JSONAPI_MIME,
};
use iron::{prelude::*, status::Status};
use japi::{Document, OptionalVec, ResourceObject};
use serde_json;
use std::convert::TryFrom;

pub fn secret_create(req: &mut Request) -> IronResult<Response> {
    let attr = req
        .extensions
        .get::<JsonApi>()
        .ok_or(DocumentError::NoDocument)
        .and_then(|d| match &d.data {
            OptionalVec::One(Some(go)) => Ok(go),
            OptionalVec::Many(_) => Err(DocumentError::MultipleData),
            _ => Err(DocumentError::NoData),
        })
        .and_then(|go| ResourceObject::<Secret>::try_from(go).map_err(|e| DocumentError::from(e)))
        .and_then(|ro| {
            ro.attributes
                .ok_or(DocumentError::no_attributes("/data/attributes".into()))
        })?;

    let core = req.extensions.get::<QaulCore>().unwrap();
    let ua = core
        .users()
        .create(&attr.value)
        .map_err(|e| QaulError::from(e))?;
    core.users()
        .logout(ua.clone())
        .map_err(|e| QaulError::from(e))?;
    let id = ua.0.clone();

    let doc = Document {
        data: OptionalVec::One(Some(Secret::from_identity(&id).into())),
        ..Default::default()
    };

    Ok(Response::with((
        Status::Created,
        JSONAPI_MIME.clone(),
        serde_json::to_string(&doc).unwrap(),
    )))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{models::into_identity, JsonApi, QaulCore};
    use anneal::RequestBuilder;
    use libqaul::Qaul;
    use std::sync::Arc;

    fn build() -> (Arc<Qaul>, RequestBuilder) {
        let qaul = Arc::new(Qaul::dummy());
        let mut rb = RequestBuilder::post("http://127.0.0.1:8000/api/secrets").unwrap();
        rb.add_middleware(QaulCore::new(qaul.clone()));
        rb.add_middleware(JsonApi);
        (qaul, rb)
    }

    #[test]
    fn works() {
        let (qaul, mut rb) = build();
        let go = rb
            .set_primary_data(
                ResourceObject {
                    attributes: Some(Secret {
                        value: "test".into(),
                        old_value: None,
                    }),
                    id: None,
                    relationships: None,
                    links: None,
                    meta: None,
                }
                .into(),
            )
            .request_response(|mut req| secret_create(&mut req))
            .unwrap()
            .get_primary_data()
            .unwrap();
        let ro = ResourceObject::<Secret>::try_from(go).unwrap();
        let rels = ro.relationships.unwrap();
        let rel = rels.get("user").unwrap();
        let rel = match &rel.data {
            OptionalVec::One(Some(rel)) => rel,
            _ => panic!("No primary data"),
        };
        let id = ro.id.unwrap();
        assert_eq!(id, rel.id);
        assert_eq!(rel.kind, "user");
        qaul.users().login(into_identity(&id).unwrap(), "test")
            .unwrap();
    }
}
