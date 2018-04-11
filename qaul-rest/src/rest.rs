//! Define all REST routes

use models::*;
use rocket::Rocket;
use rocket_contrib::Json;

#[put("/users")]
fn create_user() {}

#[post("/users/<id>")]
fn update_user(id: u64) {}

#[get("/users/<id>")]
fn get_user(id: u64) {}

#[get("/users")]
fn list_users() {}

#[post("/users/<id>/login")]
fn login(id: u64) {}

#[post("/users/<id>/logout")]
fn logout(id: u64) {}

#[get("/messages")]
fn list_messages() {}

#[get("/messges/<id>")]
fn get_message(id: u64) {}

#[put("/messages/<user>")]
fn send_message_to_user(user: u64) {}

#[put("/messages")]
fn send_message_to_all() {}

#[get("/files")]
fn get_files() {}

#[get("/files/<id>")]
fn get_files_by_user(id: u64) {}

#[get("/file/<id>/status")]
fn get_file_status(id: u64) {}

#[put("/file")]
fn add_file() {}

#[put("/file/<group>")]
fn add_file_to_group(group: u64) {}

#[put("/voip/<user>")]
fn start_voip_with_user(user: u64) {}

#[delete("/voip/<user>")]
fn stop_voip_with_user(user: u64) {}

#[post("/voip/accept")]
fn accept_call() {}

#[post("/voip/reject")]
fn reject_call() {}

#[get("/interfaces")]
fn get_network_interfaces() {}

#[get("/interfaces/<id>")]
fn get_interface_information(id: u64) {}

#[post("/interfaces/<id>")]
fn set_interface_information(id: u64) {}

#[get("/network")]
fn get_network() {}

#[get("/network/<id>")]
fn get_network_node(id: u64) {}

#[get("/binaries")]
fn get_binaries() {}

#[post("/binaries")]
fn update_binaries() {}

pub(crate) fn initialise() -> Rocket {
    Rocket::ignite().mount(
        "/",
        routes![
            create_user,
            update_user,
            get_user,
            list_users,
            login,
            logout,
            list_messages,
            get_message,
            send_message_to_user,
            send_message_to_all,
            get_files,
            get_files_by_user,
            get_file_status,
            add_file,
            add_file_to_group,
            start_voip_with_user,
            stop_voip_with_user,
            accept_call,
            reject_call,
            get_network_interfaces,
            get_interface_information,
            set_interface_information,
            get_network,
            get_network_node,
            get_binaries,
            update_binaries
        ],
    )
}
