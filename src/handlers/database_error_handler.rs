pub fn handle_db_error(e: diesel::result::Error) -> (axum::http::StatusCode, String) {
    use axum::http::StatusCode;
    use diesel::result::{DatabaseErrorKind, Error};

    match e {
        Error::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, e) => {
            (StatusCode::BAD_REQUEST, format!("{:?}", e))
        }
        Error::DatabaseError(DatabaseErrorKind::UniqueViolation, e) => {
            (StatusCode::BAD_REQUEST, format!("{:?}", e))
        }
        Error::DatabaseError(DatabaseErrorKind::CheckViolation, e) => {
            (StatusCode::BAD_REQUEST, format!("{:?}", e))
        }
        Error::NotFound => (StatusCode::NOT_FOUND, format!("{:?}", e)),
        _ => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal server error : {:?}", e),
        ),
    }
}
