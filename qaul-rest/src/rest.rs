//! Define all REST routes

use models::*;
use rocket::Rocket;
use rocket_contrib::Json;
use rocket::response::status;

#[put("/users")]
fn create_user() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/users/<id>")]
fn update_user(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/users/<id>")]
fn get_user(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/users")]
fn list_users() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/users/<id>/login")]
fn login(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/users/<id>/logout")]
fn logout(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/messages")]
fn list_messages() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/messges/<id>")]
fn get_message(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[put("/messages/<user>")]
fn send_message_to_user(user: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[put("/messages")]
fn send_message_to_all() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/files")]
fn get_files() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/files/<id>")]
fn get_files_by_user(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/file/<id>/status")]
fn get_file_status(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[put("/file")]
fn add_file() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[put("/file/<group>")]
fn add_file_to_group(group: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[put("/voip/<user>")]
fn start_voip_with_user(user: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[delete("/voip/<user>")]
fn stop_voip_with_user(user: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/voip/accept")]
fn accept_call() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/voip/reject")]
fn reject_call() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/interfaces")]
fn get_network_interfaces() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/interfaces/<id>")]
fn get_interface_information(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/interfaces/<id>")]
fn set_interface_information(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/network")]
fn get_network() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/network/<id>")]
fn get_network_node(id: u64) -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[get("/binaries")]
fn get_binaries() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

#[post("/binaries")]
fn update_binaries() -> Json<Dummy> {
    Json(Dummy {
        name: String::from("qaul.net"),
        awesome: true,
    })
}

use rocket::response::Response;
use rocket::http::ContentType;
use rocket::http::Status;
use std::io::Cursor;

#[get("/")]
fn index() -> Result<Response<'static>, Status> {
    let s = "<!DOCTYPE html>
<html>
    <head>
        <title>qaul.net RESTful API</title>
    </head>
    <body>
        <h1>Invalid API usage</h1>
        <p>The following API endpoints are supported</p>
        <ul>
            <li> GET / - Print this help text</li>
            <li> PUT /users - Create a new user</li>
            <li> POST /users/&lt;id&gt; - Update user information</li>
            <li> GET /users/&lt;id&gt; - Get user information</li>
            <li> GET /users - Get all known users</li>
            <li> POST /users/&lt;id&gt;/login - Authenticate as user (receive token)</li>
            <li> POST /users/&lt;id&gt;/logout - De-authenticate as user (hand-in token)</li>
            <li> GET /messages - Get all messages</li>
            <li> GET /messges/&lt;id&gt; - Get messages for user</li>
            <li> PUT /messages/&lt;user&gt;> - Send message to user</li>
            <li> PUT /messages - Send message to all</li>
            <li> GET /files - Get all known files</li>
            <li> GET /files/&lt;id&gt; - Get file with id</li>
            <li> GET /file/&lt;id&gt;/status - Get file status with id</li>
            <li> PUT /file - Add a new file</li>
            <li> PUT /file/&lt;gr&gt;up> - Add a new file to specific group</li>
            <li> PUT /voip/&lt;user&gt;> - Voip stuff</li>
            <li> DELETE /voip/&lt;user&gt;> - Voip stuff</li>
            <li> POST /voip/accept - Voip stuff</li>
            <li> POST /voip/reject - Voip stuff</li>
            <li> GET /interfaces - </li>
            <li> GET /interfaces/&lt;id&gt; - </li>
            <li> POST /interfaces/&lt;id&gt; - </li>
            <li> GET /network - </li>
            <li> GET /network/&lt;id&gt; - </li>
            <li> GET /binaries - </li>
            <li> POST /binaries - </li>
        </ul>
    </body>
</html>";

    return Response::build()
        .header(ContentType::Plain)
        .raw_header("Content-Type", "text/html")
        .raw_header("Charset", "UTF-8")
        .sized_body(Cursor::new(s))
        .ok();
    // return status::Accepted(Some(format!("{}", s)));
}

pub(crate) fn initialise() -> Rocket {
    Rocket::ignite().mount(
        "/",
        routes![
            index,
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
