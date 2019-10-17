use router::Router;
use iron::method::Method;

mod get;
use get::user_get;

pub fn route(router: &mut Router) {
    router.route(Method::Get, "/api/users/:id", user_get, "user_get");
}
