use iron::method::Method;
use router::Router;

mod create;
use create::grant_create;

mod delete;
use delete::grant_delete;

pub fn route(router: &mut Router) {
    router.route(Method::Post, "/api/grant", grant_create, "grant_create");
    router.route(
        Method::Delete,
        "/api/grant/:id",
        grant_delete,
        "grant_delete",
    );
}
