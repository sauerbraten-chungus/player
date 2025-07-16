use axum::{Router, routing::get};

use crate::handlers::*;

mod db;
mod handlers;
mod logger;
mod models;

#[derive(Clone)]
struct AppState {
    db: db::Db,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().unwrap();
    logger::init().unwrap();

    let app_state = AppState {
        db: db::Db::new().await?,
    };

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/player", get(get_all_player_data))
        .route("/player/{id}", get(get_player_data))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
