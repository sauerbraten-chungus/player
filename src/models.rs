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
    pub created_at: chrono::NaiveDate,
}

impl Player {
    pub fn average_accuracy(&self) -> f64 {
        if self.matches_played == 0 {
            0.0
        } else {
            self.accuracy as f64 / self.matches_played as f64
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IncomingPlayer {
    pub name: String,
    pub frags: i32,
    pub deaths: i32,
    pub accuracy: i32,
}
