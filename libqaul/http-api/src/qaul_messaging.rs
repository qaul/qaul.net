use iron::{
    middleware::BeforeMiddleware,
    typemap::Key,
    prelude::*,
};
use text_messaging::Messaging;
use std::sync::Arc;

pub struct QaulMessaging {
    messaging: Arc<Messaging>,
}

impl QaulMessaging {
    pub fn new(msg: &Messaging) -> Self {
        Self {
            messaging: Arc::new(msg.clone()),
        }
    }
}

impl Key for QaulMessaging {
    type Value = Arc<Messaging>;
}

impl BeforeMiddleware for QaulMessaging {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Self>(self.messaging.clone());
        Ok(())
    }
}
