use crate::AppState;
use crate::models::Player;
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use log::error;

pub async fn get_player_data(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Player>, StatusCode> {
    let player = state.db.get_player_by_id(id).await.map_err(|e| {
        error!("Failed to fetch player {}: {}", id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(player))
}
pub async fn get_all_player_data(
    State(state): State<AppState>,
) -> Result<Json<Vec<Player>>, StatusCode> {
    let players = state.db.get_all_players().await.map_err(|e| {
        error!("Failed to fetch players: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(players))
}
