use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Player {
    pub id: i32,
    pub name: String,
    pub steam_id: String,
    pub matches_played: i32,
    pub elo: i32,
    pub commendations: i32,
    pub created_at: chrono::NaiveDate,
}
