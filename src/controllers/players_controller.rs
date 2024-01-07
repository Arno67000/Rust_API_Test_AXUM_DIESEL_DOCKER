use axum::{extract, extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::handlers::database_error_handler::handle_db_error;
use crate::models::player::{CreatePlayerDTO, Player};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

pub async fn list_players(State(db): State<DB>) -> impl IntoResponse {
    match Repo::<Player>::get_all(db).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => handle_db_error(e).into_response(),
    }
}

pub async fn create_player(
    State(db): State<DB>,
    extract::Json(payload): extract::Json<CreatePlayerDTO>,
) -> impl IntoResponse {
    match Repo::<Player>::create(db, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => handle_db_error(e).into_response(),
    }
}
