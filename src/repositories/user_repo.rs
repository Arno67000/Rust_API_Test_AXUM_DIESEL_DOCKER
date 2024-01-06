use diesel::prelude::*;

use crate::models::user::{CreateUserDAO, User};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

impl Repo<User> {
    pub async fn get_all(db: DB) -> Result<Vec<User>, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        let conn = &mut db.db_connect();

        users.select(User::as_select()).load(conn)
    }

    pub async fn create(db: DB, user: CreateUserDAO) -> Result<User, diesel::result::Error> {
        use crate::schema::users;
        let conn = &mut db.db_connect();

        diesel::insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(conn)
    }
}
