mod secret;
mod grant;
mod user;

use router::Router;
pub fn route(router: &mut Router) {
    secret::route(router);
    grant::route(router);
    user::route(router);
}
