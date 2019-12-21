use super::{from_message_id, from_identity, into_message_id, into_identity, Trust, User};
use crate::error::{ApiError, DocumentError, GenericError};
use japi::{
    Attributes, Identifier, Link, Links, OptionalVec, Relationship, Relationships, ResourceObject,
};
use serde_derive::{Deserialize, Serialize};
use libqaul::{messages::{Recipient, MsgId, Message}, Identity};
use std::convert::TryFrom;
use base64::{encode_config, decode_config, URL_SAFE};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct QaulMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<Trust>,
    pub payload: String,
}

impl Attributes for QaulMessage {
    fn kind() -> String {
        "qaul_message".into()
    }
}

impl QaulMessage { 
    pub fn from_message(m: Message) -> ResourceObject<QaulMessage> {
        let attr = QaulMessage {
            sign: Some(m.sign.into()),
            payload: encode_config(&m.payload, URL_SAFE),
        };

        let id = from_message_id(&m.id);
        let mut ro = ResourceObject::new(id.clone(), Some(attr));

        let mut links = Links::new();
        links.insert("self".into(), Link::Url(format!("/api/qaul_messages/{}", id)));
        ro.links = Some(links);

        let mut relationships = Relationships::new();

        relationships.insert(
            "sender".into(),
            Relationship {
                data: OptionalVec::One(Some(Identifier::new(from_identity(&m.sender), "user".into()))),
                ..Default::default()
            },
        );

        let recipient = match m.recipient {
            Recipient::User(id) => OptionalVec::One(Some(
                Identifier::new(from_identity(&id), "user".into()))),
            Recipient::Group(ids) => OptionalVec::Many(ids.iter()
                .map(|id| Identifier::new(from_identity(id), "user".into())).collect()),
            Recipient::Flood => OptionalVec::One(None),
        };
        relationships.insert(
            "recipient".into(),
            Relationship { data: recipient, ..Default::default() },
        );

        ro.relationships = Some(relationships);

        ro
    }

    pub fn message_id(
        ro: &ResourceObject<QaulMessage>,
        pointer: String,
    ) -> Result<MsgId, ApiError> {
        ro.id
            .as_ref()
            .ok_or(
                DocumentError::NoId {
                    pointer: Some(format!("{}/id", pointer)),
                }
                .into(),
            )
            .and_then(|id| Ok(into_message_id(&id)?))
    }

    pub fn sender(
        ro: &ResourceObject<QaulMessage>,
        pointer: String,
    ) -> Result<Identity, ApiError> {
        Ok(into_identity(
            &ResourceObject::<User>::try_from(ro.relationships
                .as_ref()
                .ok_or(DocumentError::no_relationships(pointer.clone()))?
                .get("sender")
                .ok_or(DocumentError::no_relationship("sender".into(), pointer.clone()))?
                .data
                .clone()
                .one_or(DocumentError::many_items(format!("{}/relationships/sender", pointer)))?
                .ok_or(DocumentError::null_item(format!("{}/relationships/sender", pointer)))?
            )
            .map_err(|e| DocumentError::from(e))?
            .id
            .unwrap())?)
    }

    pub fn recipient(
        ro: &ResourceObject<QaulMessage>,
        pointer: String
    ) -> Result<Recipient, ApiError> {
        Ok(match &ro.relationships
                .as_ref()
                .ok_or(DocumentError::no_relationships(pointer.clone()))?
                .get("recipient".into())
                .ok_or(DocumentError::no_relationship("recipient".into(), pointer.clone()))?
                .data {
            OptionalVec::One(None) => Recipient::Flood,
            OptionalVec::One(Some(id)) => {
                Recipient::User(into_identity(&ResourceObject::<User>::try_from(id)
                    .map_err(|e| DocumentError::from(e))?
                    .id
                    .unwrap()
                )?)
            },
            OptionalVec::Many(ids) => {
                let mut idents = Vec::new();
                for id in ids.iter() {
                    let id = into_identity(&ResourceObject::<User>::try_from(id)
                        .map_err(|e| DocumentError::from(e))?
                        .id
                        .unwrap()
                    )?;
                    idents.push(id);
                }
                Recipient::Group(idents)
            },
            // if it's null, it'll be One(Some), if it's not present it'll fail earlier with no
            // relationship
            OptionalVec::NotPresent => unreachable!(),
        })
    }

    pub fn sign(
        ro: &ResourceObject<QaulMessage>,
        pointer: String
    ) -> Result<Trust, ApiError> {
        Ok(ro.attributes
           .as_ref()
           .ok_or(DocumentError::no_attributes(pointer.clone()))?
           .sign
           .clone()
           .ok_or(DocumentError::no_attribute("sign".into(), pointer.clone()))?
           .into())
    }

    pub fn payload(
        ro: &ResourceObject<QaulMessage>,
        pointer: String
    ) -> Result<Vec<u8>, ApiError> {
        Ok(decode_config(&ro.attributes
                .as_ref()
                .ok_or(DocumentError::no_attributes(pointer))?
                .payload, URL_SAFE)
           .map_err(|e| GenericError::new("Invalid Payload".into())
                .detail(format!("{}", e))
            )?
        )
    }
}
