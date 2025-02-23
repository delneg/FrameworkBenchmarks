mod common;
mod models_common;
mod server;

use models_common::Message;

use axum::http::StatusCode;
use axum::http::{header, HeaderValue};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::get, Router};
use dotenv::dotenv;
use tower_http::set_header::SetResponseHeaderLayer;

pub async fn plaintext() -> &'static str {
    "Hello, World!"
}

pub async fn json() -> impl IntoResponse {
    let message = Message {
        message: "Hello, World!",
    };

    (StatusCode::OK, Json(message))
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/plaintext", get(plaintext))
        .route("/json", get(json))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("Axum"),
        ));

    server::builder()
        .http1_pipeline_flush(true)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
