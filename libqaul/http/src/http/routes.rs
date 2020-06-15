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
        .at("/login")
        .post(|req| async move { http2rpc::http2rpc(req, "users", "login").await });
    app_http
        .at("/logout")
        .post(|req| async move { http2rpc::http2rpc(req, "users", "logout").await });

    app_http
        .at("/validate_token")
        .get(|req| async move { http2rpc::http2rpc(req, "users", "validate").await });

    // user management
    app_http
        .at("/users")
        .get(|req| async move { http2rpc::http2rpc(req, "users", "list").await })
        .post(|req| async move { http2rpc::http2rpc(req, "users", "create").await });
    app_http
        .at("/users/:id")
        .get(|req| async move {
            let params = vec!["id"];
            http2rpc::http2rpc_params(req, "users", "get", Some(params)).await
        })
        .patch(|req| async move { http2rpc::http2rpc(req, "users", "modify").await })
        .delete(|req| async move { http2rpc::http2rpc(req, "users", "delete").await });

    // contacts
    // TODO: contacts query
    app_http
        .at("/contacts")
        .get(|req| async move { http2rpc::http2rpc(req, "contacts", "list").await });

    // chat service
    // chat-rooms
    app_http
        .at("/chat-rooms")
        .get(|req| async move { http2rpc::http2rpc(req, "chat-rooms", "list").await })
        .post(|req| async move { http2rpc::http2rpc(req, "chat-rooms", "create").await });
    app_http
        .at("/chat-rooms/:id")
        .get(|req| async move {
            let params = vec!["id"];
            http2rpc::http2rpc_params(req, "chat-rooms", "get", Some(params)).await
        })
        .patch(|req| async move { http2rpc::http2rpc(req, "chat-rooms", "modify").await })
        //.delete(|req| async move { http2rpc::http2rpc(req, "chat-rooms", "delete").await })
        ;

    // chat-messages
    app_http
        .at("/chat-messages")
        .get(|req| async move {
            http2rpc::http2rpc_query(req, "chat-messages", "query").await
        })
        .post(|req| async move { http2rpc::http2rpc(req, "chat-messages", "create").await });
    app_http
        .at("/chat-messages/next")
        .get(|req| async move { http2rpc::http2rpc_query(req, "chat-messages", "next").await });

    app_http
}
