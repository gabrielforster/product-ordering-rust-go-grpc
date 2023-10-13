use std::{sync::Arc, env, net::SocketAddr};
use axum::{http::StatusCode, Router, routing::get, response::IntoResponse};

mod models;
mod database;

use database::PostgresRepository;
use crate::models::Product;

type AppState = Arc<PostgresRepository>;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(3000);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

    let database_pool_size = env::var("DATABASE_POOL_SIZE")
        .ok()
        .and_then(|size| size.parse::<u32>().ok())
        .unwrap_or(30);

    // let repo = database::PostgresRepository::connect(&database_url, database_pool_size).await.unwrap();
    // let app_state = Arc::new(repo);

    let app = Router::new()
        .route("/products", get(get_products));
        // .with_state(app_state);

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_products() -> impl IntoResponse {
    return (StatusCode::OK, "ok");
}
