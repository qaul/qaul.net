mod secret;

use router::Router;
pub fn route(router: &mut Router) {
    secret::route(router);
}
