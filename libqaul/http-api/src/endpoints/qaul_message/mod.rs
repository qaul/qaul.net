use iron::method::Method;
use router::Router;

mod get;
use get::qaul_message_get;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/qaul_messages/:service/:id", qaul_message_get, "qaul_message_get");
}
