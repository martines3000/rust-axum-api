// Built-in Lints
// Clippy lints
#![allow(clippy::map_unwrap_or)]
#![warn(
    clippy::if_not_else,
    clippy::items_after_statements,
    clippy::mut_mut,
    clippy::non_ascii_literal,
    clippy::similar_names,
    clippy::unicode_not_nfc,
    clippy::used_underscore_binding,
    missing_copy_implementations
)]
#![cfg_attr(test, allow(clippy::unwrap_used))]
use std::net::SocketAddr;

use axum::{routing::get, Json, Router};
use serde_json::json;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new().route(
        "/",
        get(Json(json!({
            "message": "Hello from auth server!",
        }))),
    );

    // Run with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
