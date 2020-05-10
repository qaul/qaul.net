//! # RPC Test Environment Without any Netmods
//! 
//! The test environment provides common functions
//! for the RPC tests.

#[cfg(test)]
pub(crate) mod tests {
    use async_std::sync::Arc;
    use libqaul_rpc::{
        json::{RequestEnv, ResponseEnv},
        Envelope, Responder,
    };
    use ratman::Router;
    use {
        qaul_chat::Chat,
        qaul_voices::Voices,
    }; 
    use libqaul::{Qaul, Identity};

    /// RPC test state
    pub(crate) struct RPC {
        pub responder: Responder,
    }

    impl RPC {
        /// initialized the RPC test environment
        pub(crate) async fn init() -> RPC {
            // Initialize a basic libqaul stack with no interfaces
            let rat = Router::new();
            let dir = tempfile::tempdir().unwrap();
            let qaul = Qaul::new(rat, dir.path());
            let chat = Chat::new(Arc::clone(&qaul)).await.unwrap();
            let voices = Voices::new(Arc::clone(&qaul)).await.unwrap();
            
            RPC {
                responder: Responder {qaul, chat, voices}
            }
        }

        /// send a RPC call
        pub(crate) async fn send(self, json_string: &str) -> ResponseEnv {
            let req_env: RequestEnv =
                serde_json::from_str(json_string).unwrap();
            let Envelope { id, data: req } = req_env.clone().generate_envelope().unwrap();

            // Call into libqaul via the rpc utilities
            //let responder: Arc<_> = Arc::clone(r.state());
            let resp = self.responder.respond(req).await;

            let env = Envelope { id, data: resp };
            let resp_env: ResponseEnv = (env, req_env).into();
            resp_env
        }
    }
}