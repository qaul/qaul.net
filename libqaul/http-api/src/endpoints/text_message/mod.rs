use iron::{method::Method, prelude::*};
use router::Router;

mod get;
use get::text_message_get;

mod create;
use create::text_message_create;

mod query;
use query::text_message_query;

mod poll;
use poll::text_message_poll;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/text_messages/:id", text_message_get, "text_message_get");
    router.route(Method::Post, "/api/text_messages", text_message_create, "text_message_create");

    // routes /api/text_messages? differently than /api/text_messages
    fn poll_handler(req: &mut Request) -> IronResult<Response> {
        if req.url.query().is_some() {
            text_message_query(req)
        } else {
            text_message_poll(req)
        }
    }
    router.route(Method::Get, "/api/text_messages", poll_handler, "text_message_poll");
}
