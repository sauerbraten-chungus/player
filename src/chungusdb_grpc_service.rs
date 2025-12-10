use tonic::{Request, Response, Status};
use log::{info, error};

use crate::db::Db;
use crate::models::IncomingPlayer;

// Include the generated code from the proto file
// tonic_build generates a module based on the package name in the proto
pub mod chungusdb {
    tonic::include_proto!("chungusdb");
}

use chungusdb::chungus_db_service_server::ChungusDbService;
use chungusdb::{MatchStats, MatchStatsResponse};

pub struct ChungusDbServiceImpl {
    db: Db,
}

impl ChungusDbServiceImpl {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

#[tonic::async_trait]
impl ChungusDbService for ChungusDbServiceImpl {
    async fn send_match_stats(
        &self,
        request: Request<MatchStats>,
    ) -> Result<Response<MatchStatsResponse>, Status> {
        let match_stats = request.into_inner();

        info!("Received match stats for {} players", match_stats.player_stats.len());

        // Transform the proto map into Vec<IncomingPlayer>
        let incoming_players: Vec<IncomingPlayer> = match_stats
            .player_stats
            .into_iter()
            .map(|(chungid, stats)| {
                info!("Processing player: {} with {} kills", chungid, stats.kills);
                IncomingPlayer {
                    name: chungid,
                    frags: stats.kills as i32, // kills maps to frags
                    deaths: 0,                  // Not tracked in current proto
                    accuracy: 0,                // Not tracked in current proto
                }
            })
            .collect();

        // Use the existing database upsert logic
        self.db
            .upsert_batch_players(incoming_players)
            .await
            .map_err(|e| {
                error!("Failed to upsert match stats: {}", e);
                Status::internal(format!("Database error: {}", e))
            })?;

        info!("Successfully processed match stats");

        let response = MatchStatsResponse {
            message: "Match stats received and processed".to_string(),
        };

        Ok(Response::new(response))
    }
}
