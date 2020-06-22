//! # Routes of REST Endpoint
//!
//! Adds REST Routes for WebGUI

use async_std::sync::Arc;
use tide::Server;

use crate::{http::http2rpc, Responder};

/// Creates the Tide server and routes for the REST endpoint
pub fn http_routes(http_state: Arc<Responder>) -> Server<Arc<Responder>> {
    let mut app_http = tide::with_state(http_state);

    // authentication
    app_http
        .at("/auth")
        .get(|req| async move { http2rpc::http2rpc(req, "user", "validate").await })
        .post(|req| async move { http2rpc::http2rpc(req, "user", "login").await })
        .delete(|req| async move { http2rpc::http2rpc(req, "user", "logout").await });

    // user management
    app_http
        .at("/user")
        .get(|req| async move { http2rpc::http2rpc(req, "user", "list").await })
        .post(|req| async move { http2rpc::http2rpc(req, "user", "create").await });
    app_http
        .at("/user/:id")
        .get(|req| async move {
            let params = vec!["id"];
            http2rpc::http2rpc_params(req, "user", "get", Some(params)).await
        })
        .patch(|req| async move { http2rpc::http2rpc(req, "user", "modify").await })
        .delete(|req| async move { http2rpc::http2rpc(req, "user", "delete").await });

    // contacts
    // TODO: contacts query
    app_http
        .at("/contact")
        .get(|req| async move { http2rpc::http2rpc(req, "contact", "list").await });

    // chat service
    // chat_room
    app_http
        .at("/chat_room")
        .get(|req| async move { http2rpc::http2rpc(req, "chat_room", "list").await })
        .post(|req| async move { http2rpc::http2rpc(req, "chat_room", "create").await });
    app_http
        .at("/chat_room/:id")
        .get(|req| async move {
            let params = vec!["id"];
            http2rpc::http2rpc_params(req, "chat_room", "get", Some(params)).await
        })
        .patch(|req| async move { http2rpc::http2rpc(req, "chat_room", "modify").await })
        //.delete(|req| async move { http2rpc::http2rpc(req, "chat_room", "delete").await })
        ;

    // chat_message
    app_http
        .at("/chat_message")
        .get(|req| async move {
            http2rpc::http2rpc_query(req, "chat_message", "query").await
        })
        .post(|req| async move { http2rpc::http2rpc(req, "chat_message", "create").await });
    app_http
        .at("/chat_message/next")
        .get(|req| async move { http2rpc::http2rpc_query(req, "chat_message", "next").await });

    app_http
}
