use axum::{extract, extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::handlers::database_error_handler::handle_db_error;
use crate::models::team::{Team, TeamDAO};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

pub async fn get_all_teams(State(db): State<DB>) -> impl IntoResponse {
    match Repo::<Team>::get_all(db).await {
        Ok(teams) => (StatusCode::OK, Json(teams)).into_response(),
        Err(e) => handle_db_error(e).into_response(),
    }
}

pub async fn create_team(
    State(db): State<DB>,
    extract::Json(payload): extract::Json<TeamDAO>,
) -> impl IntoResponse {
    match Repo::<Team>::create(db, payload).await {
        Ok(team) => (StatusCode::CREATED, Json(team)).into_response(),
        Err(e) => handle_db_error(e).into_response(),
    }
}
