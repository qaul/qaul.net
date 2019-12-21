use iron::method::Method;
use router::Router;

mod get;
use get::qaul_message_get;

mod create;
use create::qaul_message_create;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/qaul_messages/:service/:id", qaul_message_get, "qaul_message_get");
    router.route(
        Method::Post, 
        "/api/qaul_messages/:service", 
        qaul_message_create, 
        "qaul_message_create"
    );
}
