//!

use crate::{ChatExt, ChatRpc, EnvelopeType, QaulExt, QaulRpc, Request, Response};
use async_std::sync::Arc;
use libqaul::Qaul;
use qaul_chat::Chat;

/// A type mapper to map RPC requests to libqaul and services
pub struct Responder {
    pub qaul: Arc<Qaul>,

    #[cfg(feature = "chat")]
    pub chat: Arc<Chat>,
}

impl Responder {
    async fn respond_qaul<R, T>(&self, request: R) -> T
    where
        R: QaulRpc<Response = T> + Send + Sync,
        T: Send + Sync,
    {
        self.qaul.apply(request).await
    }

    #[cfg(feature = "chat")]
    async fn respond_chat<R, T>(&self, request: R) -> T
    where
        R: ChatRpc<Response = T> + Send + Sync,
        T: Send + Sync,
    {
        self.chat.apply(request).await
    }

    pub async fn respond(&self, req: Request) -> Response {
        match req {
            // =^-^= Chat Messages =^-^=
            #[cfg(features = "chat")]
            Request::ChatMessageNext(r) => self.respond_chat(r).await.into(),
            #[cfg(features = "chat")]
            Request::ChatMessageSend(r) => self.respond_chat(r).await.into(),

            // =^-^= Chat Rooms =^-^=
            #[cfg(features = "chat")]
            Request::ChatRoomList(r) => self.respond_chat(r).await.into(),
            #[cfg(features = "chat")]
            Request::ChatRoomGet(r) => self.respond_chat(r).await.into(),
            #[cfg(features = "chat")]
            Request::ChatRoomCreate(r) => self.respond_chat(r).await.into(),
            #[cfg(features = "chat")]
            Request::ChatRoomModify(r) => self.respond_chat(r).await.into(),
            #[cfg(features = "chat")]
            Request::ChatRoomDelete(r) => self.respond_chat(r).await.into(),

            // =^-^= Contacts =^-^=
            Request::ContactModify(r) => self.respond_qaul(r).await.into(),
            Request::ContactGet(r) => self.respond_qaul(r).await.into(),

            // TODO: Currently the "query" functions don't return
            // actual data, but just the IDs.  Maybe we should change
            // that in libqaul, but until then this RPC layer should
            // just mirror the base API.
            //
            // The usage here should probably be made nicer with a
            // From<Result<T, E>>, which is already implemented, but I
            // think we need to turbo-fish it somehow.  Anyway, future
            // me's problem :)
            Request::ContactQuery(r) => match self.respond_qaul(r).await.map(|r| r.into()) {
                Ok(users) => users,
                Err(e) => Response::Error(e.to_string()),
            },
            Request::ContactAll(r) => match self.respond_qaul(r).await.map(|r| r.into()) {
                Ok(users) => users,
                Err(e) => Response::Error(e.to_string()),
            },

            // =^-^= Messages =^-^=
            Request::MsgSend(r) => match self.respond_qaul(r).await {
                Ok(id) => Response::MsgId(id),
                Err(e) => Response::Error(e.to_string()),
            },
            Request::MsgNext(r) => self.respond_qaul(r).await.into(),
            Request::MsgQuery(r) => self.respond_qaul(r).await.into(),

            // =^-^= Users =^-^=
            Request::UserList(r) => self.respond_qaul(r).await.into(),
            Request::UserCreate(r) => self.respond_qaul(r).await.into(),
            Request::UserDelete(r) => self.respond_qaul(r).await.into(),
            Request::UserChangePw(r) => self.respond_qaul(r).await.into(),
            Request::UserLogin(r) => self.respond_qaul(r).await.into(),
            Request::UserLogout(r) => self.respond_qaul(r).await.into(),
            Request::UserGet(r) => self.respond_qaul(r).await.into(),
            Request::UserUpdate(r) => self.respond_qaul(r).await.into(),
            _ => unimplemented!(),
        }
    }
}
 
