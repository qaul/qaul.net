use iron::method::Method;
use router::Router;

mod create;
use create::grant_create;

pub fn route(router: &mut Router) {
    router.route(Method::Post, "/api/grant", grant_create, "grant_create"); 
}
