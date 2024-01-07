use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use super::player::PlayerDAO;

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub nb_player: i64,
    pub score: i64,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TeamDAO {
    pub name: String,
}

impl TeamDAO {
    pub fn into_team(self, players: Vec<PlayerDAO>) -> Team {
        Team {
            name: self.name,
            nb_player: players.len().try_into().unwrap_or_default(),
            score: players.into_iter().map(|p| p.score).sum::<i64>(),
        }
    }
}
