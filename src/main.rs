use std::net::SocketAddr;

use axum::{Router, routing::get, Json};
use parser::PowerStats;

mod parser;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(stats));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3135));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn stats() -> Result<Json<PowerStats>, String> {
    PowerStats::new()
        .map(|x| Json(x))
}
