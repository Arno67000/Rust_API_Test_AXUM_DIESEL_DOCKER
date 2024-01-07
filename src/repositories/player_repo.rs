use diesel::prelude::*;

use crate::models::player::{CreatePlayerDTO, PlayerDAO};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

impl Repo<PlayerDAO> {
    pub async fn get_all(db: DB) -> Result<Vec<PlayerDAO>, diesel::result::Error> {
        use crate::schema::players::dsl::*;
        let conn = &mut db.db_connect();

        players.select(PlayerDAO::as_select()).load(conn)
    }

    pub async fn create(db: DB, user: CreatePlayerDTO) -> Result<PlayerDAO, diesel::result::Error> {
        use crate::schema::players;
        let conn = &mut db.db_connect();

        diesel::insert_into(players::table)
            .values(user.into_player())
            .returning(PlayerDAO::as_returning())
            .get_result(conn)
    }
}
