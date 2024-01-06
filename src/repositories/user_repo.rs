use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,
    pub nickname: String,
    pub level: i64,
    pub score: i64,
    pub team_name: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InsertableUser {
    pub nickname: String,
}

impl Repo<User> {
    pub async fn get_all(db: DB) -> Result<Vec<User>, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let conn = &mut db.db_connect();

        users.select(User::as_select()).load(conn)
    }

    pub async fn create(db: DB, user: InsertableUser) -> Result<User, diesel::result::Error> {
        use crate::schema::users;
        let conn = &mut db.db_connect();

        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(conn)
    }
}
