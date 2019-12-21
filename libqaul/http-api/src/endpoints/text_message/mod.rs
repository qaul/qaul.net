use iron::method::Method;
use router::Router;

mod create;
use create::text_message_create;

mod poll;
use poll::text_message_poll;

pub fn route(router: &mut Router) {
    router.route(Method::Post, "/api/text_messages", text_message_create, "text_message_create");
    router.route(Method::Get, "/api/text_messages", text_message_poll, "text_message_poll");
}
