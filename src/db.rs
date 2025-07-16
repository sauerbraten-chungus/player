use crate::models::Player;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

#[derive(Clone)]
pub struct Db {
    pool: PgPool,
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let url = env::var("DATABASE_URL").unwrap();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;
        Ok(Db { pool })
    }

    pub async fn get_player_by_id(&self, id: i32) -> Result<Player, sqlx::Error> {
        let player = sqlx::query_as!(Player, "SELECT * FROM players WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(player)
    }

    pub async fn get_all_players(&self) -> Result<Vec<Player>, sqlx::Error> {
        let players = sqlx::query_as!(Player, "SELECT * FROM players")
            .fetch_all(&self.pool)
            .await?;

        Ok(players)
    }
}
