use libqaul::Qaul;
use iron::typemap;

mod auth;

struct QaulCore;
impl typemap::Key for QaulCore { type Value = Qaul; }
