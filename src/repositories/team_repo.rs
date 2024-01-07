use diesel::prelude::*;

use crate::models::player::Player;
use crate::models::team::{Team, TeamDAO};
use crate::providers::database::DB;
use crate::repositories::repo_model::Repo;

impl Repo<Team> {
    pub async fn get_all(db: DB) -> Result<Vec<Team>, diesel::result::Error> {
        use crate::schema::players;
        use crate::schema::players::team_name;
        use crate::schema::teams::dsl::*;
        let conn = &mut db.db_connect();

        let res = match teams.select(TeamDAO::as_select()).load(conn) {
            Ok(t) => t,
            Err(e) => return Err(e),
        };

        Ok(res
            .into_iter()
            .map(|t| {
                let players = match players::table
                    .filter(team_name.eq(&t.name))
                    .select(Player::as_select())
                    .load(conn)
                {
                    Ok(players) => players,
                    Err(_e) => vec![],
                };
                t.into_team(players)
            })
            .collect::<Vec<Team>>())
    }

    pub async fn create(db: DB, team: TeamDAO) -> Result<Team, diesel::result::Error> {
        use crate::schema::teams;
        let conn = &mut db.db_connect();

        match diesel::insert_into(teams::table)
            .values(team)
            .returning(TeamDAO::as_returning())
            .get_result(conn)
        {
            Ok(t) => Ok(t.into_team(vec![])),
            Err(e) => Err(e),
        }
    }
}
