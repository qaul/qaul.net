mod grant;
mod secret;
mod user;
#[cfg(feature = "messaging")]
mod text_message;
mod qaul_message;

use router::Router;
pub fn route(router: &mut Router) {
    secret::route(router);
    grant::route(router);
    user::route(router);
    #[cfg(feature = "messaging")]
    text_message::route(router);
    qaul_message::route(router);
}
