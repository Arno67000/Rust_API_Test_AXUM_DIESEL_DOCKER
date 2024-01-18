use axum::{
    routing::{get, put},
    Router,
};

use crate::{
    controllers::players_controller::{change_team, create_player, list_players, update_score},
    providers::database::DB,
};

pub fn routes() -> Router<DB> {
    Router::new()
        .route("/", get(list_players).post(create_player))
        .route("/:nickname/score", put(update_score))
        .route("/:nickname/team", put(change_team))
}
