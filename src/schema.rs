// @generated automatically by Diesel CLI.

diesel::table! {
    players (nickname) {
        #[max_length = 255]
        nickname -> Varchar,
        score -> Int8,
        #[max_length = 255]
        team_name -> Varchar,
    }
}

diesel::table! {
    teams (name) {
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::joinable!(players -> teams (team_name));

diesel::allow_tables_to_appear_in_same_query!(players, teams,);
