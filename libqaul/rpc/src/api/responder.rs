use crate::{api::Subscriber, QaulExt, QaulRpc, Request, Response, StreamResponder, Streamer};
use async_std::sync::Arc;
use libqaul::Qaul;

#[cfg(feature = "chat")]
use crate::{ChatExt, ChatRpc};
#[cfg(feature = "chat")]
use qaul_chat::Chat;

// #[cfg(feature = "voices")]
// use crate::{VoicesExt, VoicesRpc};
// #[cfg(feature = "voices")]
// use qaul_voices::Voices;

/// A type mapper to map RPC requests to libqaul and services
pub struct Responder<K: StreamResponder + Send + Sync + 'static> {
    /// Keeps track of subscription runs
    pub streamer: Arc<Streamer<K>>,

    pub qaul: Arc<Qaul>,

    #[cfg(feature = "chat")]
    pub chat: Arc<Chat>,
    // #[cfg(feature = "voices")]
    // pub voices: Arc<Voices>,
}

impl<K: StreamResponder + Send + Sync + 'static> Responder<K> {
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
        (&self.chat).apply(request).await
    }

    // #[cfg(feature = "voices")]
    // async fn respond_voices<R, T>(&self, request: R) -> T
    // where
    //     R: VoicesRpc<Response = T> + Send + Sync,
    //     T: Send + Sync,
    // {
    //     self.voices.apply(request).await
    // }

    /// Primary responder matcher
    ///
    /// Takes in a request, calls the appropriate submodule/service,
    /// and then returns the result to the caller.
    ///
    // With this function we avoid having 100 functions that all do
    // basically the same thing, and the switching logic stays in one
    // place.  If you need some context as to why this is required,
    // what it does, and how it actually works, there's an RPC chapter
    // in the contributors guide for you to read up on these things.
    // If you have more questions afterwards, feel free to reach out
    // to us.  When touching this function, try to leave comments for
    // anything that's not immediatly obvious.  Also: =^-^=!
    pub async fn respond(&self, req: Request) -> Response {
        // TODO: currently the ids all map into Response::UserId which is wrong
        match req {
            // =^-^= Generic RPC commands =^-^=
            Request::CancelSub(r) => match self.respond_qaul(r).await {
                Ok(id) => {
                    self.streamer.stop(id).await;
                    Response::Success
                }
                Err(e) => Response::Error(e.to_string()),
            },

            // =^-^= Chat Messages =^-^=
            #[cfg(feature = "chat")]
            Request::ChatMsgCreate(r) => self.respond_chat(r).await.into(),
            Request::ChatMsgSub(r) => self
                .respond_chat(r)
                .await
                .map(|sub| Response::Subscription(self.streamer.start(sub)))
                .unwrap_or_else(|e| Response::Error(e.to_string())),

            // =^-^= Chat Rooms =^-^=
            #[cfg(feature = "chat")]
            Request::ChatRoomList(r) => self.respond_chat(r).await.into(),
            #[cfg(feature = "chat")]
            Request::ChatRoomGet(r) => self.respond_chat(r).await.into(),
            #[cfg(feature = "chat")]
            Request::ChatLoadRoom(r) => self.respond_chat(r).await.into(),
            #[cfg(feature = "chat")]
            Request::ChatRoomCreate(r) => self.respond_chat(r).await.into(),

            // =^-^= Contacts =^-^=
            // FIXME: this should be a contacts type!
            Request::UserListRemote(r) => self.respond_qaul(r).await.into(),
            // Request::ContactModify(r) => self.respond_qaul(r).await.into(),
            // Request::ContactGet(r) => self.respond_qaul(r).await.into(),

            // TODO: Currently the "query" functions don't return
            // actual data, but just the IDs.  Maybe we should change
            // that in libqaul, but until then this RPC layer should
            // just mirror the base API.
            //
            // The usage here should probably be made nicer with a
            // From<Result<T, E>>, which is already implemented, but I
            // think we need to turbo-fish it somehow.  Anyway, future
            // me's problem :)
            Request::ContactQuery(r) => self
                .respond_qaul(r)
                .await
                .map(|ids| Response::UserId(ids))
                .unwrap_or_else(|e| Response::Error(e.to_string())),
            Request::ContactAll(r) => self
                .respond_qaul(r)
                .await
                .map(|ids| Response::UserId(ids))
                .unwrap_or_else(|e| Response::Error(e.to_string())),

            // =^-^= Messages =^-^=
            Request::MsgSend(r) => match self.respond_qaul(r).await {
                Ok(id) => Response::MsgId(id),
                Err(e) => Response::Error(e.to_string()),
            },

            // =^-^= Users =^-^=
            Request::UserList(r) => self.respond_qaul(r).await.into(),
            Request::UserIsAuthenticated(r) => match self.respond_qaul(r).await {
                Ok(()) => Response::Success,
                Err(_) => Response::Error("Not authorised".into()),
            },
            Request::UserCreate(r) => self.respond_qaul(r).await.into(),
            Request::UserDelete(r) => self.respond_qaul(r).await.into(),
            Request::UserChangePw(r) => self.respond_qaul(r).await.into(),
            Request::UserLogin(r) => self.respond_qaul(r).await.into(),
            Request::UserLogout(r) => self.respond_qaul(r).await.into(),
            Request::UserGet(r) => self.respond_qaul(r).await.into(),
            Request::UserUpdate(r) => self.respond_qaul(r).await.into(),

            // // =^-^= Voices =^-^=
            // #[cfg(feature = "voices")]
            // Request::VoicesMakeCall(r) => self
            //     .respond_voices(r)
            //     .await
            //     .map(|id| Response::CallId(id))
            //     .unwrap_or_else(|e| Response::Error(e.to_string())),
            // #[cfg(feature = "voices")]
            // Request::VoicesAcceptCall(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesRejectCall(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesHangUp(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesNextIncoming(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesGetMetadata(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesPushVoice(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesGetStatus(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesOnHangup(r) => self.respond_voices(r).await.into(),
            // #[cfg(feature = "voices")]
            // Request::VoicesNextVoice(r) => self
            //     .respond_voices(r)
            //     .await
            //     .map(|samples| Response::VoiceData(samples))
            //     .unwrap_or_else(|e| Response::Error(e.to_string())),
            tt => panic!(
                "Encountered unimplemented parse type: {:#?}\n...so sorry",
                tt
            ),
        }
    }
}
