use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub nickname: String,
    pub score: i64,
    pub team_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlayerDTO {
    pub nickname: String,
    pub score: Option<i64>,
    pub team_name: Option<String>,
}

impl CreatePlayerDTO {
    pub fn into_player(self) -> Player {
        Player {
            nickname: self.nickname,
            team_name: self.team_name.unwrap_or(String::from("FREE_PLAYERS")),
            score: self.score.unwrap_or(0),
        }
    }
}
