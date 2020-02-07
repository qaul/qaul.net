use iron::{method::Method, prelude::*};
use router::Router;

mod get;
use get::qaul_message_get;

mod create;
use create::qaul_message_create;

mod query;
use query::qaul_message_query;

mod poll;
use poll::qaul_message_poll;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/qaul_messages/:service/:id", qaul_message_get, "qaul_message_get");
    router.route(
        Method::Post, 
        "/api/qaul_messages/:service", 
        qaul_message_create, 
        "qaul_message_create"
    );

    fn poll_handler(req: &mut Request) -> IronResult<Response> {
        if req.url.query().is_some() {
            qaul_message_query(req)
        } else {
            qaul_message_poll(req)
        }
    }
    router.route(Method::Get, "/api/qaul_messages/:service", poll_handler, "qaul_message_poll");
}
