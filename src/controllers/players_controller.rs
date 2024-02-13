use axum::extract::Path;
use axum::{extract, extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::handlers::database_error_handler::handle_db_error;
use crate::models::player::{CreatePlayerDTO, Player, UpdateScoreDTO, UpdateTeamDTO};
use crate::models::team::{Team, TeamDAO};
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

pub async fn update_score(
    State(db): State<DB>,
    Path(nickname): Path<String>,
    extract::Json(score): extract::Json<UpdateScoreDTO>,
) -> impl IntoResponse {
    match Repo::<Player>::get_one(db.clone(), nickname).await {
        Ok(player) => match Repo::<Player>::increment_score(db, player, score).await {
            Ok(p) => (StatusCode::OK, Json(p)).into_response(),
            Err(e) => handle_db_error(e).into_response(),
        },
        Err(e) => handle_db_error(e).into_response(),
    }
}

pub async fn change_team(
    State(db): State<DB>,
    Path(nickname): Path<String>,
    extract::Json(team): extract::Json<UpdateTeamDTO>,
) -> impl IntoResponse {
    let existing_team = Repo::<Team>::get_one(
        db.clone(),
        TeamDAO {
            name: team.team_name.clone(),
        },
    )
    .await;
    if let Err(e) = existing_team {
        return handle_db_error(e).into_response();
    }

    match Repo::<Player>::get_one(db.clone(), nickname).await {
        Ok(player) => match Repo::<Player>::change_team(db, player, team).await {
            Ok(p) => (StatusCode::OK, Json(p)).into_response(),
            Err(e) => handle_db_error(e).into_response(),
        },
        Err(e) => handle_db_error(e).into_response(),
    }
}
