/// Core service routes

use crate::{
    Cookies,
    QaulCore,
    JSONAPI_MIME,
    models::Success,
};
use libqaul::UserAuth;
use iron::{
    prelude::*,
    status::Status,
};
use json_api::{
    Document,
    OptionalVec,
};

pub fn get_all_users(req: &mut Request) -> IronResult<Response> {
    

    unimplemented!()
}
