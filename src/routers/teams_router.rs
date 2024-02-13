use axum::{routing::get, Router};

use crate::{
    controllers::teams_controller::{create_team, get_all_teams},
    providers::database::DB,
};

pub fn routes() -> Router<DB> {
    Router::new().route("/", get(get_all_teams).post(create_team))
}
