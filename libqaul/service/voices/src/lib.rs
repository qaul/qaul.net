use {
    crate::api::{CallId, StreamMetadata},
    futures::lock::Mutex,
    libqaul::{
        error::Result,
        Identity, Qaul,
    },
    std::{
        collections::BTreeMap,
        sync::Arc,
    },
};

pub mod api;
mod wire;

const ASC_NAME: &'static str = "net.qaul.voices";

#[derive(Clone)]
pub struct Voices {
    calls: Arc<Mutex<BTreeMap<CallId, CallState>>>,
    qaul: Arc<Qaul>,
}

impl Voices {
    pub fn new(qaul: Arc<Qaul>) -> Result<Self> {
        qaul.services().register(ASC_NAME)?;
        Ok(Self { 
            calls: Arc::new(Mutex::new(BTreeMap::new())),
            qaul 
        })
    }
}

struct CallState {
    local: Identity,
    remote: Identity,
    remote_metadata: StreamMetadata,
}
