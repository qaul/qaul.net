//! # Routes of REST Endpoint
//!
//! Adds REST Routes for WebGUI

use async_std::sync::Arc;
use libqaul_rpc::Responder;
use tide::Server;

use crate::rest::rest2rpc;

/// Creates the Tide server and routes for the REST endpoint
pub fn rest_routes(rest_state: Arc<Responder>) -> Server<Arc<Responder>> {
    let mut app_rest = tide::with_state(rest_state);

    // authentication
    app_rest
        .at("/login")
        .post(|req| async move { rest2rpc::rest2rpc(req, "users", "login").await });
    app_rest
        .at("/logout")
        .get(|req| async move { rest2rpc::rest2rpc(req, "users", "logout").await });

    // user management
    app_rest
        .at("/users")
        .get(|req| async move { rest2rpc::rest2rpc(req, "users", "list").await })
        .post(|req| async move { rest2rpc::rest2rpc(req, "users", "create").await });
    app_rest
        .at("/users/:id")
        .get(|req| async move {
            let params = vec!["id"];
            rest2rpc::rest2rpc_params(req, "users", "get", Some(params)).await
        })
        .patch(|req| async move { rest2rpc::rest2rpc(req, "users", "modify").await })
        .delete(|req| async move { rest2rpc::rest2rpc(req, "users", "delete").await });

    // contacts
    // TODO: contacts query
    app_rest.at("/contacts").get(|req| async move {
        rest2rpc::rest2rpc(req, "contacts", "list").await
        //rest2rpc::rest2rpc(req, "contacts", "query").await
    });
    app_rest
        .at("/contacts/:id")
        .get(|req| async move {
            let params = vec!["id"];
            rest2rpc::rest2rpc_params(req, "contacts", "get", Some(params)).await
        })
        .patch(|req| async move { rest2rpc::rest2rpc(req, "contacts", "modify").await });

    // chat service
    // chat-rooms
    app_rest
        .at("/chat-rooms")
        .get(|req| async move { rest2rpc::rest2rpc(req, "chat-rooms", "list").await })
        .post(|req| async move { rest2rpc::rest2rpc(req, "chat-rooms", "create").await });
    app_rest
        .at("/chat-rooms/:id")
        .get(|req| async move {
            let params = vec!["id"];
            rest2rpc::rest2rpc_params(req, "chat-rooms", "get", Some(params)).await
        })
        .patch(|req| async move { rest2rpc::rest2rpc(req, "chat-rooms", "modify").await })
        //.delete(|req| async move { rest2rpc::rest2rpc(req, "chat-rooms", "delete").await })
        ;

    // chat-messages
    app_rest
        .at("/chat-messages/:id")
        .get(|req| async move { rest2rpc::rest2rpc(req, "chat-messages", "get").await })
        .post(|req| async move { rest2rpc::rest2rpc(req, "chat-messages", "create").await });
    app_rest
        .at("/chat-messages/:id/next")
        .get(|req| async move { rest2rpc::rest2rpc(req, "chat-messages", "next").await });

    app_rest
}
