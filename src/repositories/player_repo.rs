use diesel::prelude::*;

use crate::models::player::{CreatePlayerDTO, Player, UpdateScoreDTO, UpdateTeamDTO};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

impl Repo<Player> {
    pub async fn get_all(db: DB) -> Result<Vec<Player>, diesel::result::Error> {
        use crate::schema::players::dsl::*;
        let conn = &mut db.db_connect();

        players.select(Player::as_select()).load(conn)
    }

    pub async fn get_one(db: DB, name: String) -> Result<Player, diesel::result::Error> {
        use crate::schema::players::dsl::*;
        let conn = &mut db.db_connect();

        players.filter(nickname.eq(name)).first(conn)
    }

    pub async fn create(db: DB, user: CreatePlayerDTO) -> Result<Player, diesel::result::Error> {
        use crate::schema::players;
        let conn = &mut db.db_connect();

        diesel::insert_into(players::table)
            .values(user.into_player())
            .returning(Player::as_returning())
            .get_result(conn)
    }

    pub async fn increment_score(
        db: DB,
        player: Player,
        data: UpdateScoreDTO,
    ) -> Result<Player, diesel::result::Error> {
        use crate::schema::players::dsl::*;
        let conn = &mut db.db_connect();

        diesel::update(players)
            .filter(nickname.eq(player.nickname))
            .set(score.eq(score + data.score))
            .returning(Player::as_returning())
            .get_result(conn)
    }

    pub async fn change_team(
        db: DB,
        player: Player,
        data: UpdateTeamDTO,
    ) -> Result<Player, diesel::result::Error> {
        use crate::schema::players::dsl::*;
        let conn = &mut db.db_connect();

        diesel::update(players)
            .filter(nickname.eq(player.nickname))
            .set(team_name.eq(data.team_name))
            .returning(Player::as_returning())
            .get_result(conn)
    }
}
