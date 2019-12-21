use iron::method::Method;
use router::Router;

mod create;
use create::text_message_create;

pub fn route(router: &mut Router) {
    router.route(Method::Post, "/api/text_messages", text_message_create, "text_message_create");
}
