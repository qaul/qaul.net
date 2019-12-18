use iron::method::Method;
use router::Router;

mod get;
use get::user_get;

mod delete;
use delete::user_delete;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/users/:id", user_get, "user_get");
    router.route(Method::Delete, "/api/users/:id", user_delete, "user_delete");
}
