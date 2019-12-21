use iron::method::Method;
use router::Router;

mod get;
use get::text_message_get;

mod create;
use create::text_message_create;

mod poll;
use poll::text_message_poll;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/text_messages/:id", text_message_get, "text_message_get");
    router.route(Method::Post, "/api/text_messages", text_message_create, "text_message_create");
    router.route(Method::Get, "/api/text_messages", text_message_poll, "text_message_poll");
}
