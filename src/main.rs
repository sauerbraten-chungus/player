use std::env;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use middleware::jwt_auth;

use crate::handlers::*;

mod db;
mod handlers;
mod logger;
mod middleware;
mod models;

#[derive(Clone)]
struct AppState {
    db: db::Db,
    secret: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().unwrap();
    logger::init().unwrap();

    let app_state = AppState {
        db: db::Db::new().await?,
        secret: env::var("SECRET_CHUNGUS").unwrap_or_else(|_| "".to_string()),
    };

    let protected_routes = Router::new()
        .route("/players/batch", post(post_batch_player_data))
        .layer(axum_middleware::from_fn_with_state(
            app_state.clone(),
            jwt_auth,
        ));

    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/player", get(get_all_player_data))
        .route("/player/{id}", get(get_player_data))
        .merge(protected_routes)
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
