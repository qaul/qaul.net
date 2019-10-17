mod secret;
mod grant;

use router::Router;
pub fn route(router: &mut Router) {
    secret::route(router);
    grant::route(router);
}
