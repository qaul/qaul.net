pub mod page;
pub mod feed;


pub struct Services {

}

impl Services {
    pub fn init() {
        feed::Feed::init();
    }
}