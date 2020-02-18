use {
    crate::{
        api::{
            request::{TransactionRequest, Request}, 
            response::{TransactionResponse, Response},
            QaulExt, QaulRPC,
        },
        subtask::{SubTaskManager, Spawner},
    },
    futures::{
        channel::mpsc,
        future::FutureExt,
        task::{Context, Poll, SpawnExt, SpawnError},
        stream::{Stream, StreamExt},
        sink::{Sink, SinkExt},
    },
    libqaul::{
        api::{Subscription, SubId},
        Qaul,
    },
    std::{
        pin::Pin,
        sync::Arc,
    },
};
#[feature(chat)]
use {
    crate::api::chat::{ChatExt, ChatRPC},
    qaul_chat::Chat,
};

/// The set of services a responder should respond to requests for. If a responder
/// does not have a service mounted to respond to a request it will return an error
/// instead.
#[derive(Clone)]
pub struct ResponderServices {
    qaul: Option<Arc<Qaul>>,
    #[feature(chat)]
    chat: Option<Arc<Chat>>,
}

impl ResponderServices {
    /// Create a new set with no services mounted
    pub fn new() -> Self {
        Self {
            qaul: None,
            #[feature(chat)]
            chat: None,
        }
    }

    /// Mount the core set of services
    pub fn qaul(mut self, qaul: Arc<Qaul>) -> Self {
        self.qaul = Some(qaul);
        self
    }

    /// Mount the chat service
    #[feature(chat)]
    pub fn chat(mut self, chat: Arc<Chat>) -> Self {
        self.chat = Some(chat);
        self
    }

    /// Build a responder using this set of services
    pub fn build(self) -> Responder {
        Responder::new(self)
    }

    /// If the core service is mounted respond to the given request, otherwise
    /// error
    async fn respond_qaul<R, T>(&self, request: R) -> Result<T, &'static str> 
    where R: QaulRPC<Response = T> + Send + Sync, T: Send + Sync {
        if let Some(qaul) = &self.qaul {
            Ok(qaul.apply(request).await)
        } else {
            Err("The core Qaul service was not mounted on this responder")
        }
    }

    /// If the chat service is mounted respond to the given request, otherwise
    /// error
    #[feature(chat)]
    async fn respond_chat<R, T>(&self, request: R) -> Result<T, &'static str> 
    where R: ChatRPC<Response = T> + Send + Sync, T: Send + Sync {
        if let Some(chat) = &self.chat {
            Ok(chat.apply(request).await)
        } else {
            Err("The chat service was not mounted on this responder")
        }
    }
}

/// Take a stream of Request objects and produce a stream of Response objects.
///
/// This abstracts away most of the logic in doing so and is meant for RPC systems
/// like websockets or ipc which don't have statically typed routes and just recieve
/// a stream of requests and give out responses. These implementations should handle
/// serialization and deserialization and route the streams through this Responder.
///
/// The responder will only process input when the output is polled. The input message
/// queue is extremely small so it will back up quickly if the output side has not
/// been polled. 
///
/// Each incoming message will be processed in a seperate sub-task and as such the order
/// of responses depends on the order they are fufilled by the underlying `Qaul` 
/// implementation. Each request and response is wrapped in a `Transaction` for this reason.
pub struct Responder {
    input: mpsc::Sender<TransactionRequest>,
    output: mpsc::Receiver<TransactionResponse>,
    manager: SubTaskManager,
}

impl Stream for Responder {
    type Item = TransactionResponse;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        // if there's something in the stream output it
        if let Poll::Ready(resp) = self.output.poll_next_unpin(cx) {
            return Poll::Ready(resp);
        }

        // otherwise drive any pending futures the manager has forward and 
        // then check if that's put anything in the output queue
        self.manager.poll_unpin(cx);
        self.output.poll_next_unpin(cx)
    }
}

impl Sink<TransactionRequest> for Responder {
    type Error = mpsc::SendError;

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.input).poll_close(cx)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.input).poll_flush(cx)
    }
    
    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.input).poll_ready(cx)
    }
    
    fn start_send(mut self: Pin<&mut Self>, item: TransactionRequest) -> Result<(), Self::Error> {
        Pin::new(&mut self.input).start_send(item)
    }
}

impl Responder {
    /// Construct a new responder for the given set of services
    pub fn new(services: ResponderServices) -> Self { 
        let (input, _input) = mpsc::channel(1);
        let (_output, output) = mpsc::channel(1);
        let manager = SubTaskManager::new();
        let mut spawner = manager.spawner();

        // start a sub-task looping over incoming tasks 
        //
        // notably this could just as easily be in our `poll_next` impl,
        // but we have the manager regardless so no harm in using it imo
        manager.spawner().spawn(async move { 
            let mut input = _input;
            let mut output = _output;
            let mut spawner = spawner;
            loop {
                // wait for incoming messages, ending the task if the channel has closed
                let request = match input.next().await {
                    Some(request) => request,
                    None => { break; },
                };

                // spawn a task for each incoming message
                // this is a little bit of overhead but given even simple requests
                // might block for a long time this helps ensure the quickest possible responses
                let mut output = output.clone();
                let services = services.clone();
                let _spawner = spawner.clone();
                spawner.spawn( async move {
                    let response = Responder::process(
                        request, 
                        services, 
                        output.clone(), 
                        _spawner)
                    .await;
                    output.send(response).await;
                });
            }
        }).unwrap();

        Self {
            input,
            output,
            manager,
        }
    }

    /// Actually process the incoming requests, spinning up additional tasks if required
    async fn process(
        request: TransactionRequest,
        services: ResponderServices, 
        output: mpsc::Sender<TransactionResponse>, 
        spawner: Spawner
    ) -> TransactionResponse {
        let (request, response_ctx) = request.split();
        let response = match request {
            // =^-^= Chat Messages =^-^=
            #[feature(chat)]
            Request::ChatMessageNext(r) => services.respond_chat(r).await.into(),
            //#[feature(chat)]
            //Request::ChatMessageSubscribe(r) => {
            //    services.respond_chat(r)
            //        .await
            //        .map(|r| 
            //            r.map(|subscription| 
            //                  Responder::subscribe(subscription, output, spawner)
            //            )
            //        )
            //        .into()
            //},
            #[feature(chat)]
            Request::ChatMessageSend(r) => services.respond_chat(r).await.into(),

            // =^-^= Chat Rooms =^-^=
            #[feature(chat)]
            Request::ChatRoomList(r) => {
                services.respond_chat(r)
                    .await
                    .map(|ids| Response::RoomId(ids))
                    .into()
            },
            #[feature(chat)]
            Request::ChatRoomGet(r) => services.respond_chat(r).await.into(),
            #[feature(chat)]
            Request::ChatRoomCreate(r) => {
                services.respond_chat(r)
                    .await
                    .map(|r| r.map(|id| Response::RoomId(vec![id])))
                    .into()
            },
            #[feature(chat)]
            Request::ChatRoomModify(r) => services.respond_chat(r).await.into(),
            #[feature(chat)]
            Request::ChatRoomDelete(r) => services.respond_chat(r).await.into(),


            // =^-^= Contacts =^-^=
            Request::ContactModify(r) => services.respond_qaul(r).await.into(),
            Request::ContactGet(r) => services.respond_qaul(r).await.into(),
            Request::ContactQuery(r) => {
                services.respond_qaul(r)
                    .await
                    .map(|r| r.map(|ids| Response::UserId(ids)))
                    .into()
            },
            Request::ContactAll(r) => {
                services.respond_qaul(r)
                    .await
                    .map(|r| r.map(|ids| Response::UserId(ids)))
                    .into()
            },

            // =^-^= Messages =^-^=
            Request::MessageSend(r) => {
                services.respond_qaul(r)
                    .await
                    .map(|r| r.map(|id| Response::MessageId(id)))
                    .into()
            },
            Request::MessagePoll(r) => services.respond_qaul(r).await.into(),
            Request::MessageSubscribe(r) => {
                services.respond_qaul(r)
                    .await
                    .map(|r| 
                        r.map(|subscription| 
                              Responder::subscribe(subscription, output, spawner)
                        )
                    )
                    .into()
            },
            Request::MessageQuery(r) => services.respond_qaul(r).await.into(),

            // =^-^= Users =^-^=
            Request::UserList(r) => services.respond_qaul(r).await.into(),
            Request::UserCreate(r) => services.respond_qaul(r).await.into(), 
            Request::UserDelete(r) => services.respond_qaul(r).await.into(),
            Request::UserChangePw(r) => services.respond_qaul(r).await.into(),
            Request::UserLogin(r) => services.respond_qaul(r).await.into(),
            Request::UserLogout(r) => services.respond_qaul(r).await.into(),
            Request::UserGet(r) => services.respond_qaul(r).await.into(),
            Request::UserUpdate(r) => services.respond_qaul(r).await.into(),

            _ => { unimplemented!(); },
        };
        response_ctx.with_response(response)
    }

    /// Spawn a future moving the results of this subscription to the output
    fn subscribe<T: Into<Response> + Send + Sync + 'static>(
        mut subscription: Subscription<T>,
        mut output: mpsc::Sender<TransactionResponse>,
        mut spawner: Spawner,
    ) -> Response {
        let id = subscription.id.clone();
        spawner.spawn(async move {
                loop {
                    let t = match subscription.next().await {
                        Some(t) => t,
                        None => { break; },
                    };

                    let trans = TransactionResponse::subscription(
                        t.into(),
                        subscription.id.clone(),
                    );

                    if let Err(e) = output.send(trans).await {
                        break;
                    }
                }
            })
            .map(|_| Response::Subscription(id))
            .into()
    }
}
