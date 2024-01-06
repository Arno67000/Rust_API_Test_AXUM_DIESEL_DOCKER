use axum::{extract, extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::models::user::{CreateUserDAO, User};
use crate::{providers::database::DB, repositories::repo_model::Repo};

pub async fn get_all_users(State(db): State<DB>) -> impl IntoResponse {
    match Repo::<User>::get_all(db).await {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(e) => {
            eprintln!("Error : {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error : {:?}", e),
            )
                .into_response()
        }
    }
}

pub async fn create_user(
    State(db): State<DB>,
    extract::Json(payload): extract::Json<CreateUserDAO>,
) -> impl IntoResponse {
    match Repo::<User>::create(db, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error : {:?}", e),
        )
            .into_response(),
    }
}
