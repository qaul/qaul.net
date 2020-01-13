use iron::method::Method;
use router::Router;

mod create;
use create::secret_create;

mod update;
use update::secret_update;

pub fn route(router: &mut Router) {
    router.route(
        Method::Patch,
        "/api/secrets/:id",
        secret_update,
        "secret_update",
    );
    router.route(
        Method::Post,
        "/api/secrets",
        secret_create,
        "secret_create",
    );
}
