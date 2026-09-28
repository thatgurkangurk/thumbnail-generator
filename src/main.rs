mod error;
mod ffmpeg;
mod handlers;
mod image_processing;

use anyhow::Result;
use axum::{Router, http::Method, routing::get};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/api/thumbnail", get(handlers::get_thumbnail_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("server running on http://localhost:3000");
    axum::serve(listener, app).await?;

    Ok(())
}
