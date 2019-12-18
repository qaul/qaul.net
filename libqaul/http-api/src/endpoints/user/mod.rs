use iron::method::Method;
use router::Router;

mod get;
use get::user_get;

mod update;
use update::user_update;

mod delete;
use delete::user_delete;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/users/:id", user_get, "user_get");
    router.route(Method::Patch, "/api/users/:id", user_update, "'user_update");
    router.route(Method::Delete, "/api/users/:id", user_delete, "user_delete");
}
