pub mod page;
pub mod feed;
pub mod messaging;
pub mod chat;


pub struct Services {

}

impl Services {
    pub fn init() {
        feed::Feed::init();
        chat::Chat::init();
    }
}