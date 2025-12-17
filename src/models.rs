use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Player {
    pub chungid: Uuid,
    pub name: String,
    pub frags: i32,
    pub deaths: i32,
    pub accuracy: f64,
    pub matches_played: i32,
    pub elo: i32,
    pub commendations: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, FromRow)]
pub struct IncomingPlayer {
    pub chungid: Uuid,
    pub name: String,
    pub frags: i32,
    pub deaths: i32,
    pub accuracy: f64,
    pub elo: i32,
}

#[derive(Serialize, FromRow)]
pub struct Match {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
}
