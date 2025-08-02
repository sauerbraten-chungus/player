use crate::models::{IncomingPlayer, Player};
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

    pub async fn get_player_by_id(&self, id: i64) -> Result<Player, sqlx::Error> {
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

    pub async fn upsert_batch_players(
        &self,
        incoming: Vec<IncomingPlayer>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for inc in incoming {
            sqlx::query!(
                r#"
                INSERT INTO players (name, frags, deaths, accuracy, matches_played, elo, commendations, created_at)
                VALUES ($1, $2, $3, $4, 1, 1000, 0, $5)
                ON CONFLICT (name) DO UPDATE
                SET frags = players.frags + $2,  -- Cumulative add
                    deaths = players.deaths + $3,
                    accuracy = (players.accuracy * players.matches_played + $4) / (players.matches_played + 1)::float,
                    matches_played = players.matches_played + 1
                "#,
                inc.name,
                inc.frags,
                inc.deaths,
                inc.accuracy as f64,  // Cast if incoming.accuracy is i32
                chrono::Utc::now().naive_utc().date()
            )
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;

        Ok(())
    }
}
