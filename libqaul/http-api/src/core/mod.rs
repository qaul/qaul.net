/// Core service routes

use crate::{
    QaulCore,
};
use iron::{
    prelude::*,
    status,
};

pub fn get_all_users(req: &mut Request) -> IronResult<Response> {
    let qaul = req.extensions.get::<QaulCore>().unwrap();

    let all_users = format!("{:?}", qaul.user_get_all());

    Ok(Response::with((status::Ok, all_users)))
}
